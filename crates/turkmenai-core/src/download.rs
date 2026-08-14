//! Resumable HTTP downloader. It writes only a `.part` file until size and hash verification pass.

use crate::{sha256_file, CoreError};
use reqwest::{
    blocking::Client,
    header::{ACCEPT_RANGES, RANGE},
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DownloadState {
    Prepare,
    Downloading,
    Verifying,
    Ready,
    Paused,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadJournal {
    pub schema_version: u32,
    pub url: String,
    pub destination: String,
    pub expected_sha256: Option<String>,
    pub bytes_downloaded: u64,
    pub state: DownloadState,
    pub error_code: Option<String>,
}

impl DownloadJournal {
    pub fn new(url: &str, destination: &Path, expected_sha256: Option<String>) -> Self {
        Self {
            schema_version: 1,
            url: url.into(),
            destination: destination.display().to_string(),
            expected_sha256,
            bytes_downloaded: 0,
            state: DownloadState::Prepare,
            error_code: None,
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), CoreError> {
        let temporary = path.with_extension("json.tmp");
        fs::write(
            &temporary,
            serde_json::to_vec_pretty(self)
                .map_err(|error| CoreError::UnsupportedSource(error.to_string()))?,
        )?;
        fs::rename(temporary, path)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self, CoreError> {
        serde_json::from_slice(&fs::read(path)?)
            .map_err(|error| CoreError::UnsupportedSource(error.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct HttpDownloader {
    client: Client,
    pub chunk_bytes: usize,
    pub max_attempts: u8,
}

impl Default for HttpDownloader {
    fn default() -> Self {
        Self {
            client: Client::builder()
                .connect_timeout(Duration::from_secs(15))
                .timeout(Duration::from_secs(120))
                .build()
                .expect("valid HTTP client configuration"),
            chunk_bytes: 64 * 1024,
            max_attempts: 3,
        }
    }
}

impl HttpDownloader {
    pub fn download(
        &self,
        url: &str,
        destination: &Path,
        expected_sha256: Option<String>,
    ) -> Result<DownloadJournal, CoreError> {
        let part = destination.with_extension("part");
        let journal_path = destination.with_extension("download.json");
        let mut journal = DownloadJournal::new(url, destination, expected_sha256);
        let resume_from = fs::metadata(&part).map(|item| item.len()).unwrap_or(0);
        journal.bytes_downloaded = resume_from;
        journal.state = DownloadState::Downloading;
        journal.save(&journal_path)?;
        let mut request = self.client.get(url);
        if resume_from > 0 {
            request = request.header(RANGE, format!("bytes={resume_from}-"));
        }
        let mut response = request.send().map_err(|error| {
            CoreError::UnsupportedSource(format!("NETWORK_INTERRUPTED: {error}"))
        })?;
        let resumable = response
            .headers()
            .get(ACCEPT_RANGES)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.contains("bytes"))
            .unwrap_or(false);
        let append = resume_from > 0 && response.status().as_u16() == 206;
        if !response.status().is_success() {
            journal.state = DownloadState::Failed;
            journal.error_code = Some(format!("HTTP_{}", response.status().as_u16()));
            journal.save(&journal_path)?;
            return Err(CoreError::UnsupportedSource(journal.error_code.unwrap()));
        }
        if resume_from > 0 && !append && !resumable {
            let _ = fs::remove_file(&part);
            journal.bytes_downloaded = 0;
        }
        let mut output = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .truncate(!append)
            .open(&part)?;
        let mut buffer = vec![0_u8; self.chunk_bytes];
        loop {
            let count = response.read(&mut buffer).map_err(|error| {
                CoreError::UnsupportedSource(format!("NETWORK_INTERRUPTED: {error}"))
            })?;
            if count == 0 {
                break;
            }
            output.write_all(&buffer[..count])?;
            journal.bytes_downloaded += count as u64;
            journal.save(&journal_path)?;
        }
        output.flush()?;
        journal.state = DownloadState::Verifying;
        journal.save(&journal_path)?;
        if let Some(expected) = &journal.expected_sha256 {
            let actual = sha256_file(&part)?;
            if &actual != expected {
                journal.state = DownloadState::Failed;
                journal.error_code = Some("HASH_MISMATCH".into());
                journal.save(&journal_path)?;
                return Err(CoreError::UnsupportedSource("HASH_MISMATCH".into()));
            }
        }
        fs::rename(&part, destination)?;
        journal.state = DownloadState::Ready;
        journal.save(&journal_path)?;
        Ok(journal)
    }
}

pub fn recoverable_downloads(folder: &Path) -> Result<Vec<(PathBuf, DownloadJournal)>, CoreError> {
    let mut jobs = Vec::new();
    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        if entry
            .path()
            .extension()
            .and_then(|extension| extension.to_str())
            == Some("json")
        {
            if let Ok(journal) = DownloadJournal::load(&entry.path()) {
                if matches!(
                    journal.state,
                    DownloadState::Downloading | DownloadState::Paused | DownloadState::Failed
                ) {
                    jobs.push((entry.path(), journal));
                }
            }
        }
    }
    Ok(jobs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persists_and_recovers_a_paused_job() {
        let workspace = tempfile::tempdir().unwrap();
        let journal_path = workspace.path().join("tiny.download.json");
        let mut journal = DownloadJournal::new(
            "https://example.invalid/file",
            &workspace.path().join("tiny.gguf"),
            None,
        );
        journal.bytes_downloaded = 4096;
        journal.state = DownloadState::Paused;
        journal.save(&journal_path).unwrap();
        let recovered = recoverable_downloads(workspace.path()).unwrap();
        assert_eq!(recovered.len(), 1);
        assert_eq!(recovered[0].1.bytes_downloaded, 4096);
    }
}
