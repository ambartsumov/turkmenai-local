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
    time::{Duration, Instant},
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
    /// Total size once learned from the server, so resume knows when it is done.
    #[serde(default)]
    pub total_bytes: Option<u64>,
    pub state: DownloadState,
    pub error_code: Option<String>,
    /// How many transient network failures have been survived so far — surfaced so
    /// the UI can honestly show "reconnecting" on unstable links.
    #[serde(default)]
    pub retries: u32,
    /// Wall-clock time actually spent transferring, set on completion.
    #[serde(default)]
    pub elapsed_ms: Option<u64>,
    /// Overall average throughput in bytes/sec, set on completion.
    #[serde(default)]
    pub avg_bps: Option<u64>,
}

/// A live progress tick, emitted roughly every few MiB during a download.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Progress {
    pub bytes_downloaded: u64,
    pub total_bytes: Option<u64>,
    pub elapsed_ms: u64,
    /// Instantaneous throughput since the previous tick (bytes/sec).
    pub speed_bps: u64,
    /// Overall average throughput so far (bytes/sec).
    pub avg_bps: u64,
    /// Interruptions survived so far (resumes), for an honest "reconnecting" UI.
    pub retries: u32,
}

impl DownloadJournal {
    pub fn new(url: &str, destination: &Path, expected_sha256: Option<String>) -> Self {
        Self {
            schema_version: 1,
            url: url.into(),
            destination: destination.display().to_string(),
            expected_sha256,
            bytes_downloaded: 0,
            total_bytes: None,
            state: DownloadState::Prepare,
            error_code: None,
            retries: 0,
            elapsed_ms: None,
            avg_bps: None,
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
            // No *total* timeout: a multi-gigabyte model over a ~2-4 Mbit/s link
            // would otherwise be killed mid-way. We bound connection setup and use
            // TCP keep-alive so a dead connection surfaces as an error (which the
            // retry loop resumes) while a slow-but-live download keeps going.
            client: Client::builder()
                .connect_timeout(Duration::from_secs(30))
                .tcp_keepalive(Duration::from_secs(30))
                .build()
                .expect("valid HTTP client configuration"),
            chunk_bytes: 256 * 1024,
            max_attempts: 12,
        }
    }
}

/// Outcome of a single streaming attempt.
enum Attempt {
    /// The connection reached a clean end-of-stream.
    Eof,
}

enum AttemptError {
    /// A definitive server refusal — retrying will not help.
    Permanent(String),
    /// A network/5xx hiccup — safe to back off and resume.
    Transient(String),
}

impl HttpDownloader {
    /// Download `url` to `destination` resiliently: resumable, retrying transient
    /// failures with exponential backoff + jitter, and verifying the hash before an
    /// atomic rename. Built for slow, unstable connections — progress is journaled
    /// and never re-downloads bytes that are already on disk.
    pub fn download(
        &self,
        url: &str,
        destination: &Path,
        expected_sha256: Option<String>,
    ) -> Result<DownloadJournal, CoreError> {
        self.download_with_progress(url, destination, expected_sha256, &mut |_| {})
    }

    /// Like [`Self::download`] but reports live [`Progress`] ticks (~every few
    /// MiB) so a UI can show real speed on the user's actual link.
    pub fn download_with_progress(
        &self,
        url: &str,
        destination: &Path,
        expected_sha256: Option<String>,
        on_progress: &mut dyn FnMut(Progress),
    ) -> Result<DownloadJournal, CoreError> {
        let started = Instant::now();
        let part = destination.with_extension("part");
        let journal_path = destination.with_extension("download.json");
        // Reuse an existing journal for the same URL so total size / retry counts
        // survive an app restart; otherwise start fresh.
        let mut journal = DownloadJournal::load(&journal_path)
            .ok()
            .filter(|existing| existing.url == url)
            .unwrap_or_else(|| DownloadJournal::new(url, destination, expected_sha256.clone()));
        journal.expected_sha256 = expected_sha256;

        let mut backoff_ms = 1_000u64;
        let mut consecutive_failures = 0u8;
        loop {
            let resume_from = fs::metadata(&part).map(|item| item.len()).unwrap_or(0);
            journal.bytes_downloaded = resume_from;
            journal.state = DownloadState::Downloading;
            journal.error_code = None;
            journal.save(&journal_path)?;

            match self.stream_once(url, &part, resume_from, &mut journal, &journal_path, started, on_progress) {
                Ok(Attempt::Eof) => {
                    // Done only if we have all bytes (when the total is known).
                    let complete = journal
                        .total_bytes
                        .map(|total| journal.bytes_downloaded >= total)
                        .unwrap_or(true);
                    if complete {
                        break;
                    }
                    // Server closed early without an error: treat as transient.
                    consecutive_failures += 1;
                }
                Err(AttemptError::Permanent(code)) => {
                    journal.state = DownloadState::Failed;
                    journal.error_code = Some(code.clone());
                    journal.save(&journal_path)?;
                    return Err(CoreError::UnsupportedSource(code));
                }
                Err(AttemptError::Transient(reason)) => {
                    journal.error_code = Some(reason);
                    // Making any progress resets the failure budget so a slow but
                    // advancing download can continue indefinitely.
                    let progressed =
                        fs::metadata(&part).map(|m| m.len()).unwrap_or(0) > resume_from;
                    if progressed {
                        consecutive_failures = 0;
                        backoff_ms = 1_000;
                    } else {
                        consecutive_failures += 1;
                    }
                }
            }

            if consecutive_failures >= self.max_attempts {
                journal.state = DownloadState::Failed;
                journal.error_code = Some("NETWORK_INTERRUPTED".into());
                journal.save(&journal_path)?;
                return Err(CoreError::UnsupportedSource("NETWORK_INTERRUPTED".into()));
            }

            journal.retries = journal.retries.saturating_add(1);
            journal.state = DownloadState::Paused;
            journal.save(&journal_path)?;
            std::thread::sleep(Duration::from_millis(backoff_ms + jitter_ms(backoff_ms)));
            backoff_ms = (backoff_ms * 2).min(30_000);
        }

        let elapsed_ms = started.elapsed().as_millis() as u64;
        journal.elapsed_ms = Some(elapsed_ms);
        journal.avg_bps = Some(bps(journal.bytes_downloaded, elapsed_ms));
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

    /// One streaming pass, resuming from `resume_from` bytes via an HTTP Range
    /// request. Returns on clean EOF or classifies the failure for the retry loop.
    #[allow(clippy::too_many_arguments)]
    fn stream_once(
        &self,
        url: &str,
        part: &Path,
        resume_from: u64,
        journal: &mut DownloadJournal,
        journal_path: &Path,
        started: Instant,
        on_progress: &mut dyn FnMut(Progress),
    ) -> Result<Attempt, AttemptError> {
        let mut request = self.client.get(url);
        if resume_from > 0 {
            request = request.header(RANGE, format!("bytes={resume_from}-"));
        }
        let mut response = request
            .send()
            .map_err(|error| AttemptError::Transient(format!("NETWORK_INTERRUPTED: {error}")))?;

        let status = response.status().as_u16();
        if status == 416 {
            // Range not satisfiable — the part is already at/over the full size.
            return Ok(Attempt::Eof);
        }
        if !response.status().is_success() {
            let code = format!("HTTP_{status}");
            return match status {
                408 | 429 | 500..=599 => Err(AttemptError::Transient(code)),
                _ => Err(AttemptError::Permanent(code)),
            };
        }

        let append = resume_from > 0 && status == 206;
        let resumable = response
            .headers()
            .get(ACCEPT_RANGES)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.contains("bytes"))
            .unwrap_or(false);
        // Learn the total size from Content-Range (206) or Content-Length (200).
        if journal.total_bytes.is_none() {
            journal.total_bytes = total_from_headers(&response, resume_from, append);
        }
        // Server ignored our Range and cannot resume: restart cleanly from zero.
        let mut resume_from = resume_from;
        if resume_from > 0 && !append && !resumable {
            let _ = fs::remove_file(part);
            resume_from = 0;
            journal.bytes_downloaded = 0;
        }

        let mut output = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(append)
            .truncate(!append)
            .open(part)
            .map_err(|error| AttemptError::Transient(error.to_string()))?;
        journal.bytes_downloaded = resume_from;

        let mut buffer = vec![0_u8; self.chunk_bytes];
        let mut since_flush = 0u64;
        let mut tick_at = Instant::now();
        let mut tick_bytes = journal.bytes_downloaded;
        loop {
            let count = match response.read(&mut buffer) {
                Ok(count) => count,
                Err(error) => {
                    let _ = output.flush();
                    let _ = journal.save(journal_path);
                    return Err(AttemptError::Transient(format!(
                        "NETWORK_INTERRUPTED: {error}"
                    )));
                }
            };
            if count == 0 {
                break;
            }
            if output.write_all(&buffer[..count]).is_err() {
                return Err(AttemptError::Transient("DISK_WRITE_FAILED".into()));
            }
            journal.bytes_downloaded += count as u64;
            since_flush += count as u64;
            // Persist progress about every 4 MiB instead of every chunk to spare
            // the disk on a long download while staying crash-safe.
            if since_flush >= 4 * 1024 * 1024 {
                let _ = output.flush();
                let _ = journal.save(journal_path);
                since_flush = 0;
                // Emit a live progress tick with instantaneous + overall speed.
                let now = Instant::now();
                let dt_ms = now.duration_since(tick_at).as_millis() as u64;
                let speed_bps = bps(journal.bytes_downloaded.saturating_sub(tick_bytes), dt_ms);
                let elapsed_ms = now.duration_since(started).as_millis() as u64;
                on_progress(Progress {
                    bytes_downloaded: journal.bytes_downloaded,
                    total_bytes: journal.total_bytes,
                    elapsed_ms,
                    speed_bps,
                    avg_bps: bps(journal.bytes_downloaded, elapsed_ms),
                    retries: journal.retries,
                });
                tick_at = now;
                tick_bytes = journal.bytes_downloaded;
            }
        }
        output
            .flush()
            .map_err(|error| AttemptError::Transient(error.to_string()))?;
        let _ = journal.save(journal_path);
        Ok(Attempt::Eof)
    }
}

/// Bytes/sec from a byte count over a millisecond interval (0 when no time yet).
fn bps(bytes: u64, elapsed_ms: u64) -> u64 {
    if elapsed_ms == 0 {
        return 0;
    }
    (bytes as u128 * 1000 / elapsed_ms as u128) as u64
}

/// Small randomized jitter (up to ~30% of the backoff) to avoid synchronized
/// retries hammering a fragile link at the same instant.
fn jitter_ms(backoff_ms: u64) -> u64 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    nanos % ((backoff_ms / 3).max(1))
}

/// Total size in bytes from `Content-Range: bytes X-Y/Z`, or `Content-Length`
/// (adjusted for the resume offset on a 200 response).
fn total_from_headers(
    response: &reqwest::blocking::Response,
    resume_from: u64,
    append: bool,
) -> Option<u64> {
    if let Some(range) = response
        .headers()
        .get(reqwest::header::CONTENT_RANGE)
        .and_then(|value| value.to_str().ok())
    {
        if let Some(total) = range.rsplit('/').next().and_then(|n| n.parse::<u64>().ok()) {
            return Some(total);
        }
    }
    response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .map(|length| if append { resume_from + length } else { length })
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
