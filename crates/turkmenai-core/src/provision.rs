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
/// Prefers the portable CPU build (works without any GPU driver toolkit) and
/// avoids vendor-specific CUDA/HIP/ROCm/SYCL/OpenVINO/OpenCL packages. Windows
/// ships `.zip`; Linux and macOS ship `.tar.gz` — both are accepted.
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
    let accel = [
        "cuda", "hip", "rocm", "sycl", "cann", "openvino", "opencl", "adreno", "cudart",
    ];
    let candidates: Vec<&GithubAsset> = assets
        .iter()
        .filter(|asset| {
            let name = asset.name.to_ascii_lowercase();
            (name.ends_with(".zip") || name.ends_with(".tar.gz"))
                && name.contains("bin")
                && !name.contains("xcframework")
                && matches(&name, os_keys)
                && (arch_keys.is_empty() || matches(&name, arch_keys))
                && !accel.iter().any(|key| name.contains(key))
        })
        .collect();
    // Rank: a build that mentions "cpu" or mentions no accelerator at all is most
    // portable; Vulkan (needs a loader) is a fallback; then smallest wins.
    candidates
        .iter()
        .find(|asset| {
            let name = asset.name.to_ascii_lowercase();
            name.contains("cpu") || !name.contains("vulkan")
        })
        .copied()
        .or_else(|| candidates.iter().min_by_key(|asset| asset.size).copied())
}

fn is_tar_gz(name: &str) -> bool {
    name.to_ascii_lowercase().ends_with(".tar.gz")
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
    let tar_gz = is_tar_gz(&asset.name);
    let archive = root.join(if tar_gz {
        "engine.tar.gz"
    } else {
        "engine.zip"
    });
    HttpDownloader::default().download(&asset.browser_download_url, &archive, None)?;
    if tar_gz {
        extract_tar_gz(&archive, &root)?;
    } else {
        extract_zip(&archive, &root)?;
    }
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

/// Extract a `.tar.gz` (Linux/macOS engine builds). The `tar` crate refuses
/// absolute paths and `..` components, so extraction stays inside `dest`; unix
/// permissions (the executable bit on `llama-server`) are preserved.
fn extract_tar_gz(archive: &Path, dest: &Path) -> Result<(), CoreError> {
    let file = fs::File::open(archive)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut tar = tar::Archive::new(decoder);
    tar.set_preserve_permissions(true);
    tar.unpack(dest)
        .map_err(|error| CoreError::Runtime(format!("ENGINE_ARCHIVE_INVALID: {error}")))?;
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

    /// Mirrors the real llama.cpp release: Linux/macOS ship .tar.gz, Windows .zip,
    /// plus many accelerator variants that must be avoided for a driver-free setup.
    fn realistic_assets() -> Vec<GithubAsset> {
        [
            ("cudart-llama-bin-win-cuda-12.4-x64.zip", 391),
            ("llama-b1-bin-macos-arm64.tar.gz", 11),
            ("llama-b1-bin-macos-x64.tar.gz", 11),
            ("llama-b1-bin-ubuntu-arm64.tar.gz", 13),
            ("llama-b1-bin-ubuntu-vulkan-x64.tar.gz", 33),
            ("llama-b1-bin-ubuntu-x64.tar.gz", 16),
            ("llama-b1-bin-ubuntu-sycl-fp16-x64.tar.gz", 53),
            ("llama-b1-bin-win-cpu-x64.zip", 18),
            ("llama-b1-bin-win-cpu-arm64.zip", 12),
            ("llama-b1-bin-win-cuda-12.4-x64.zip", 250),
            ("llama-b1-bin-win-vulkan-x64.zip", 34),
            ("llama-b1-xcframework.zip", 283),
        ]
        .into_iter()
        .map(|(n, s)| asset(n, s))
        .collect()
    }

    #[test]
    fn selects_portable_cpu_tar_gz_on_linux_x64() {
        let assets = realistic_assets();
        let chosen = select_asset(&assets, "linux", "x86_64").unwrap();
        assert_eq!(chosen.name, "llama-b1-bin-ubuntu-x64.tar.gz");
    }

    #[test]
    fn selects_cpu_zip_on_windows_x64_not_vulkan_or_cuda() {
        let assets = realistic_assets();
        let chosen = select_asset(&assets, "windows", "x86_64").unwrap();
        assert_eq!(chosen.name, "llama-b1-bin-win-cpu-x64.zip");
    }

    #[test]
    fn selects_macos_tar_gz() {
        let assets = realistic_assets();
        assert_eq!(
            select_asset(&assets, "macos", "aarch64").unwrap().name,
            "llama-b1-bin-macos-arm64.tar.gz"
        );
        assert_eq!(
            select_asset(&assets, "macos", "x86_64").unwrap().name,
            "llama-b1-bin-macos-x64.tar.gz"
        );
    }

    #[test]
    fn never_selects_accelerator_or_xcframework_builds() {
        let assets = realistic_assets();
        for (os, arch) in [
            ("linux", "x86_64"),
            ("windows", "x86_64"),
            ("macos", "aarch64"),
        ] {
            let name = select_asset(&assets, os, arch).unwrap().name.to_lowercase();
            for bad in ["cuda", "sycl", "rocm", "cudart", "xcframework", "vulkan"] {
                assert!(!name.contains(bad), "{os}/{arch} picked {name}");
            }
        }
    }

    #[test]
    fn returns_none_when_no_platform_match() {
        let assets = vec![asset("llama-b1-bin-macos-arm64.tar.gz", 20)];
        assert!(select_asset(&assets, "windows", "x86_64").is_none());
    }
}
