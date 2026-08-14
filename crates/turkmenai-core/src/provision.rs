//! Managed llama.cpp runtime provisioning.
//!
//! The user never installs or points at a runtime. On first use the app fetches
//! the official llama.cpp release for the current platform, unpacks it into the
//! app data directory, and remembers where the `llama-server` binary lives. This
//! is the "engine sets itself up automatically" path; models remain a separate,
//! explicit user choice from the catalog.

use crate::{download::HttpDownloader, state::default_data_root, CoreError};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

const LLAMA_RELEASES_LATEST: &str =
    "https://api.github.com/repos/ggml-org/llama.cpp/releases/latest";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubAsset {
    pub name: String,
    #[serde(default)]
    pub browser_download_url: String,
    #[serde(default)]
    pub size: u64,
}

#[derive(Debug, Deserialize)]
pub struct GithubRelease {
    #[serde(default)]
    pub tag_name: String,
    #[serde(default)]
    pub assets: Vec<GithubAsset>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EngineState {
    /// No managed engine is present yet.
    NotInstalled,
    /// A managed engine binary is present and executable.
    Ready,
}

/// A resolved, locally-present llama.cpp engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedEngine {
    pub backend: String,
    pub version: String,
    pub server_path: String,
    /// Directory whose shared libraries the binary needs at load time.
    pub lib_dir: String,
}

pub fn engine_root() -> PathBuf {
    default_data_root().join("runtimes").join("llama.cpp")
}

fn manifest_path() -> PathBuf {
    engine_root().join("engine.json")
}

/// The managed engine, if one is already unpacked and its binary still exists.
pub fn installed_engine() -> Option<ManagedEngine> {
    let engine: ManagedEngine = serde_json::from_slice(&fs::read(manifest_path()).ok()?).ok()?;
    if Path::new(&engine.server_path).is_file() {
        Some(engine)
    } else {
        None
    }
}

pub fn engine_state() -> EngineState {
    if installed_engine().is_some() {
        EngineState::Ready
    } else {
        EngineState::NotInstalled
    }
}

/// Pick the most portable release asset for the given OS/arch. Pure and testable.
/// Prefers a Vulkan or plain CPU build and avoids vendor-specific CUDA/HIP/SYCL
/// packages that would require separate driver toolkits.
pub fn select_asset<'a>(
    assets: &'a [GithubAsset],
    os: &str,
    arch: &str,
) -> Option<&'a GithubAsset> {
    let arch_keys: &[&str] = match arch {
        "x86_64" | "x64" => &["x64", "x86_64", "amd64"],
        "aarch64" | "arm64" => &["arm64", "aarch64"],
        _ => &[],
    };
    let os_keys: &[&str] = match os {
        "linux" => &["ubuntu", "linux"],
        "windows" => &["win"],
        "macos" => &["macos"],
        _ => &[],
    };
    let matches = |name: &str, keys: &[&str]| keys.iter().any(|key| name.contains(key));
    let candidates: Vec<&GithubAsset> = assets
        .iter()
        .filter(|asset| {
            let name = asset.name.to_ascii_lowercase();
            name.ends_with(".zip")
                && name.contains("bin")
                && matches(&name, os_keys)
                && (arch_keys.is_empty() || matches(&name, arch_keys))
                && !name.contains("cuda")
                && !name.contains("hip")
                && !name.contains("sycl")
                && !name.contains("cann")
        })
        .collect();
    candidates
        .iter()
        .find(|asset| asset.name.to_ascii_lowercase().contains("vulkan"))
        .copied()
        .or_else(|| candidates.iter().min_by_key(|asset| asset.size).copied())
}

fn client() -> Result<reqwest::blocking::Client, CoreError> {
    reqwest::blocking::Client::builder()
        .user_agent("TurkmenAI-Local")
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(45))
        .build()
        .map_err(|error| CoreError::Runtime(error.to_string()))
}

pub fn fetch_latest_release() -> Result<GithubRelease, CoreError> {
    client()?
        .get(LLAMA_RELEASES_LATEST)
        .send()
        .and_then(|response| response.error_for_status())
        .map_err(|error| CoreError::Runtime(format!("ENGINE_RELEASE_FETCH_FAILED: {error}")))?
        .json()
        .map_err(|error| CoreError::Runtime(format!("ENGINE_RELEASE_PARSE_FAILED: {error}")))
}

/// Ensure a managed engine exists, downloading and unpacking it if needed. Safe to
/// call repeatedly: a present engine is returned immediately without network use.
pub fn provision() -> Result<ManagedEngine, CoreError> {
    if let Some(engine) = installed_engine() {
        return Ok(engine);
    }
    let release = fetch_latest_release()?;
    let asset = select_asset(
        &release.assets,
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
    .ok_or_else(|| {
        CoreError::Runtime(format!(
            "NO_ENGINE_ASSET_FOR_PLATFORM: {}/{}",
            std::env::consts::OS,
            std::env::consts::ARCH
        ))
    })?;
    let root = engine_root().join(sanitize(&release.tag_name));
    fs::create_dir_all(&root)?;
    let archive = root.join("engine.zip");
    HttpDownloader::default().download(&asset.browser_download_url, &archive, None)?;
    extract_zip(&archive, &root)?;
    let server =
        find_server(&root).ok_or_else(|| CoreError::Runtime("ENGINE_BINARY_NOT_FOUND".into()))?;
    #[cfg(unix)]
    make_executable(&server)?;
    let lib_dir = server
        .parent()
        .map(|parent| parent.display().to_string())
        .unwrap_or_default();
    let engine = ManagedEngine {
        backend: "llama.cpp".into(),
        version: release.tag_name.clone(),
        server_path: server.display().to_string(),
        lib_dir,
    };
    fs::create_dir_all(engine_root())?;
    fs::write(
        manifest_path(),
        serde_json::to_vec_pretty(&engine)
            .map_err(|error| CoreError::Runtime(error.to_string()))?,
    )?;
    let _ = fs::remove_file(&archive);
    Ok(engine)
}

fn sanitize(tag: &str) -> String {
    tag.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

/// Extract a zip with zip-slip / path-traversal protection: every entry must
/// resolve strictly inside `dest`.
fn extract_zip(archive: &Path, dest: &Path) -> Result<(), CoreError> {
    let file = fs::File::open(archive)?;
    let mut zip = zip::ZipArchive::new(file)
        .map_err(|error| CoreError::Runtime(format!("ENGINE_ARCHIVE_INVALID: {error}")))?;
    let dest = dest.canonicalize().unwrap_or_else(|_| dest.to_path_buf());
    for index in 0..zip.len() {
        let mut entry = zip
            .by_index(index)
            .map_err(|error| CoreError::Runtime(error.to_string()))?;
        let relative = match entry.enclosed_name() {
            Some(path) => path.to_path_buf(),
            None => continue, // reject unsafe names
        };
        let target = dest.join(&relative);
        if !target.starts_with(&dest) {
            return Err(CoreError::Runtime("ENGINE_ARCHIVE_PATH_TRAVERSAL".into()));
        }
        if entry.is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out = fs::File::create(&target)?;
            std::io::copy(&mut entry, &mut out)?;
        }
    }
    Ok(())
}

/// Locate the `llama-server` binary anywhere under `root`.
fn find_server(root: &Path) -> Option<PathBuf> {
    let wanted = if cfg!(windows) {
        "llama-server.exe"
    } else {
        "llama-server"
    };
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = fs::read_dir(&dir).ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.file_name().and_then(|n| n.to_str()) == Some(wanted) {
                return Some(path);
            }
        }
    }
    None
}

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<(), CoreError> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset(name: &str, size: u64) -> GithubAsset {
        GithubAsset {
            name: name.into(),
            browser_download_url: format!("https://example/{name}"),
            size,
        }
    }

    #[test]
    fn selects_linux_x64_over_other_platforms() {
        let assets = vec![
            asset("llama-b1-bin-ubuntu-x64.zip", 30),
            asset("llama-b1-bin-win-x64.zip", 25),
            asset("llama-b1-bin-macos-arm64.zip", 20),
        ];
        let chosen = select_asset(&assets, "linux", "x86_64").unwrap();
        assert_eq!(chosen.name, "llama-b1-bin-ubuntu-x64.zip");
    }

    #[test]
    fn prefers_vulkan_when_present() {
        let assets = vec![
            asset("llama-b1-bin-ubuntu-x64.zip", 30),
            asset("llama-b1-bin-ubuntu-vulkan-x64.zip", 40),
        ];
        let chosen = select_asset(&assets, "linux", "x86_64").unwrap();
        assert!(chosen.name.contains("vulkan"));
    }

    #[test]
    fn avoids_cuda_builds() {
        let assets = vec![
            asset("llama-b1-bin-win-cuda-x64.zip", 90),
            asset("llama-b1-bin-win-x64.zip", 25),
        ];
        let chosen = select_asset(&assets, "windows", "x86_64").unwrap();
        assert!(!chosen.name.contains("cuda"));
    }

    #[test]
    fn returns_none_when_no_platform_match() {
        let assets = vec![asset("llama-b1-bin-macos-arm64.zip", 20)];
        assert!(select_asset(&assets, "windows", "x86_64").is_none());
    }
}
