//! One-click model installation. The user picks *which* model in the catalog;
//! from there everything is automatic: pick the fastest available transport,
//! download resiliently, verify the hash, and hand back the local path plus an
//! honest download benchmark. Nothing is auto-installed without the user's pick.

use crate::bench::{download_benchmark, DownloadBenchmark};
use crate::download::{HttpDownloader, Progress};
use crate::{transfer, CoreError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledModel {
    pub model_id: String,
    pub path: String,
    pub bytes: u64,
    /// "hf_xet" when the accelerated transport was used, else "builtin".
    pub backend: String,
    pub benchmark: DownloadBenchmark,
}

/// Everything needed to fetch one model file, mirrored from a catalog
/// `Recommendation` so the installer does not depend on the catalog types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallRequest {
    pub model_id: String,
    pub repo: String,
    pub revision: String,
    pub file: String,
    pub download_url: String,
    pub sha256: Option<String>,
}

/// Install a model into `dest_dir`, reporting live progress. Uses Xet when the
/// managed transport is ready (speed), otherwise the built-in resilient
/// downloader (which also yields interruption/resume metrics). Falls back to the
/// built-in downloader if the accelerated path errors — the user never has to
/// retry by hand.
pub fn install_model(
    request: &InstallRequest,
    dest_dir: &Path,
    on_progress: &mut dyn FnMut(Progress),
) -> Result<InstalledModel, CoreError> {
    std::fs::create_dir_all(dest_dir)?;
    let file_name = Path::new(&request.file)
        .file_name()
        .map(|n| n.to_owned())
        .ok_or_else(|| CoreError::UnsupportedSource("model file has no name".into()))?;
    let dest = dest_dir.join(&file_name);

    let transport = transfer::detect();
    if transport.xet.state == transfer::XetState::Ready {
        if let Ok(installed) = install_via_xet(request, dest_dir, &dest) {
            return Ok(installed);
        }
        // Accelerated path failed — transparently fall back, no user action.
    }
    install_via_builtin(request, &dest, on_progress)
}

fn install_via_builtin(
    request: &InstallRequest,
    dest: &Path,
    on_progress: &mut dyn FnMut(Progress),
) -> Result<InstalledModel, CoreError> {
    let downloader = HttpDownloader::default();
    let journal = downloader.download_with_progress(
        &request.download_url,
        dest,
        request.sha256.clone(),
        on_progress,
    )?;
    let bytes = journal.total_bytes.unwrap_or(journal.bytes_downloaded);
    let elapsed_ms = journal.elapsed_ms.unwrap_or(0);
    let avg_bps = journal.avg_bps.unwrap_or(0);
    Ok(InstalledModel {
        model_id: request.model_id.clone(),
        path: dest.display().to_string(),
        bytes,
        backend: "builtin".into(),
        benchmark: download_benchmark(bytes, elapsed_ms, avg_bps, journal.retries),
    })
}

fn install_via_xet(
    request: &InstallRequest,
    dest_dir: &Path,
    dest: &Path,
) -> Result<InstalledModel, CoreError> {
    let start = Instant::now();
    let path = transfer::hf_download(&request.repo, &request.file, &request.revision, dest_dir)
        .map_err(CoreError::UnsupportedSource)?;
    let elapsed_ms = start.elapsed().as_millis() as u64;
    let bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
    // Verify the hash ourselves — Xet transferred it, but the security contract
    // still requires an end-to-end checksum before we trust the file.
    if let Some(expected) = &request.sha256 {
        let actual = crate::sha256_file(&path)?;
        if &actual != expected {
            let _ = std::fs::remove_file(&path);
            return Err(CoreError::UnsupportedSource("HASH_MISMATCH".into()));
        }
    }
    if path != dest {
        let _ = std::fs::rename(&path, dest);
    }
    let avg_bps = if elapsed_ms > 0 {
        (bytes as u128 * 1000 / elapsed_ms as u128) as u64
    } else {
        0
    };
    // Xet manages its own retries internally, so we report 0 survived
    // interruptions here rather than inventing a resume advantage.
    Ok(InstalledModel {
        model_id: request.model_id.clone(),
        path: dest.display().to_string(),
        bytes,
        backend: "hf_xet".into(),
        benchmark: download_benchmark(bytes, elapsed_ms, avg_bps, 0),
    })
}

/// The default per-user directory where installed model files live.
pub fn default_models_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("turkmenai-local")
        .join("models")
}
