//! Managed transfer engine.
//!
//! The built-in resilient downloader ([`crate::download`]) always works with
//! zero external dependencies — that is the out-of-the-box guarantee. Hugging
//! Face **Xet** (via the `hf` CLI with the `hf_xet` extension) is an OPTIONAL
//! acceleration: chunk-level deduplication + adaptive concurrency. The app
//! detects it, can best-effort provision it, and transparently falls back to
//! the built-in downloader when it is missing. When Xet is unavailable we
//! report it honestly as `not_installed` with localized setup instructions
//! rather than dead-ending — never a home-made "slower Xet replacement".

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum XetState {
    Ready,
    NotInstalled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalizedText {
    pub en: String,
    pub ru: String,
    pub tk: String,
}

impl LocalizedText {
    fn of(en: &str, ru: &str, tk: &str) -> Self {
        Self { en: en.into(), ru: ru.into(), tk: tk.into() }
    }
}

/// One numbered setup step, shown when Xet is not installed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstructionStep {
    pub command: Option<String>,
    pub text: LocalizedText,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct XetStatus {
    pub state: XetState,
    pub hf_version: Option<String>,
    pub detail: LocalizedText,
    /// Populated only when `state == NotInstalled`.
    pub instructions: Vec<InstructionStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransferStatus {
    /// The built-in downloader is always available.
    pub builtin_ready: bool,
    /// "hf_xet" when Xet will be used, otherwise "builtin".
    pub active_backend: String,
    pub xet: XetStatus,
}

/// Localized instructions for installing the accelerated Xet transport.
fn xet_instructions() -> Vec<InstructionStep> {
    vec![
        InstructionStep {
            command: Some("pipx install \"huggingface_hub[hf_xet]\"".into()),
            text: LocalizedText::of(
                "Install the Hugging Face CLI with the Xet extension (pipx keeps it isolated).",
                "Установите Hugging Face CLI с расширением Xet (pipx держит его изолированно).",
                "Xet giňeltmesi bilen Hugging Face CLI-ni guruň (pipx ony aýratyn saklaýar).",
            ),
        },
        InstructionStep {
            command: Some("pip install \"huggingface_hub[hf_xet]\"".into()),
            text: LocalizedText::of(
                "No pipx? Use pip instead (ideally inside a virtual environment).",
                "Нет pipx? Используйте pip (лучше внутри виртуального окружения).",
                "pipx ýokmy? pip ulanyň (has gowusy wirtual gurşawda).",
            ),
        },
        InstructionStep {
            command: Some("hf version".into()),
            text: LocalizedText::of(
                "Verify the CLI is on PATH. TurkmenAI will pick it up automatically on the next check.",
                "Проверьте, что CLI в PATH. TurkmenAI подхватит его автоматически при следующей проверке.",
                "CLI-niň PATH-de bardygyny barlaň. TurkmenAI ony indiki barlagda awtomatiki alar.",
            ),
        },
    ]
}

/// Parse a version string out of `hf version` / `huggingface-cli version` output.
fn parse_version(raw: &str) -> Option<String> {
    raw.split_whitespace()
        .find(|token| token.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
        .map(|token| token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '.').to_string())
        .filter(|version| !version.is_empty())
}

/// Try one CLI candidate; returns its reported version if it runs.
fn probe(binary: &str) -> Option<String> {
    let output = Command::new(binary).arg("version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    parse_version(&text).or_else(|| Some("unknown".into()))
}

/// Detect the accelerated transport without side effects.
pub fn detect() -> TransferStatus {
    // Newer CLI is `hf`; older is `huggingface-cli`. Either ships hf_xet in
    // recent releases, so presence of the CLI is our readiness signal.
    let hf_version = probe("hf").or_else(|| probe("huggingface-cli"));
    match hf_version {
        Some(version) => TransferStatus {
            builtin_ready: true,
            active_backend: "hf_xet".into(),
            xet: XetStatus {
                state: XetState::Ready,
                hf_version: Some(version),
                detail: LocalizedText::of(
                    "Xet accelerated transport is available; downloads use chunk dedup + adaptive concurrency.",
                    "Ускоренный транспорт Xet доступен; загрузки используют дедупликацию чанков и adaptive concurrency.",
                    "Xet çaltlaşdyrylan transport elýeterli; ýüklemeler çank dedup + adaptiw parallellik ulanýar.",
                ),
                instructions: Vec::new(),
            },
        },
        None => TransferStatus {
            builtin_ready: true,
            active_backend: "builtin".into(),
            xet: XetStatus {
                state: XetState::NotInstalled,
                hf_version: None,
                detail: LocalizedText::of(
                    "Xet not installed — using the built-in resilient downloader. Install the Hugging Face CLI to accelerate large downloads.",
                    "Xet не установлен — используется встроенный устойчивый загрузчик. Установите Hugging Face CLI, чтобы ускорить большие загрузки.",
                    "Xet gurulmadyk — içerki durnukly ýükleýji ulanylýar. Uly ýüklemeleri çaltlaşdyrmak üçin Hugging Face CLI guruň.",
                ),
                instructions: xet_instructions(),
            },
        },
    }
}

/// Best-effort, out-of-the-box provisioning: if the CLI is missing but a Python
/// package manager is present, install it for the user. Never fails the app —
/// on any problem the caller keeps using the built-in downloader and the status
/// still exposes manual instructions.
pub fn provision() -> TransferStatus {
    let current = detect();
    if current.xet.state == XetState::Ready {
        return current;
    }
    for (bin, args) in [
        ("pipx", vec!["install", "huggingface_hub[hf_xet]"]),
        ("pip", vec!["install", "--user", "huggingface_hub[hf_xet]"]),
        ("pip3", vec!["install", "--user", "huggingface_hub[hf_xet]"]),
    ] {
        if which(bin).is_none() {
            continue;
        }
        let ok = Command::new(bin)
            .args(&args)
            .status()
            .map(|status| status.success())
            .unwrap_or(false);
        if ok {
            let refreshed = detect();
            if refreshed.xet.state == XetState::Ready {
                return refreshed;
            }
        }
    }
    // Nothing worked — report honestly, built-in downloader remains active.
    current
}

/// Minimal cross-platform "is this binary on PATH" check.
fn which(binary: &str) -> Option<String> {
    let (probe, flag) = if cfg!(windows) { ("where", binary) } else { ("command", binary) };
    // `command -v` is a shell builtin; fall back to running the binary itself.
    if !cfg!(windows) {
        if let Ok(out) = Command::new("sh").arg("-c").arg(format!("command -v {binary}")).output() {
            if out.status.success() {
                let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
        return None;
    }
    let out = Command::new(probe).arg(flag).output().ok()?;
    if out.status.success() {
        let path = String::from_utf8_lossy(&out.stdout).lines().next().unwrap_or("").trim().to_string();
        if !path.is_empty() {
            return Some(path);
        }
    }
    None
}

/// Download a Hugging Face file through the accelerated `hf` CLI (Xet). Returns
/// the local path on success. Only call when [`detect`] reports `Ready`; the
/// caller must fall back to the built-in downloader on `Err`.
pub fn hf_download(
    repo: &str,
    file: &str,
    revision: &str,
    local_dir: &std::path::Path,
) -> Result<std::path::PathBuf, String> {
    std::fs::create_dir_all(local_dir).map_err(|e| e.to_string())?;
    let binary = if probe("hf").is_some() { "hf" } else { "huggingface-cli" };
    // `hf download <repo> <file> --revision <rev> --local-dir <dir>` writes the
    // file under local_dir preserving its repo-relative path.
    let status = Command::new(binary)
        .arg("download")
        .arg(repo)
        .arg(file)
        .arg("--revision")
        .arg(revision)
        .arg("--local-dir")
        .arg(local_dir)
        .env("HF_HUB_ENABLE_HF_TRANSFER", "0") // Xet supersedes deprecated hf_transfer.
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(format!("hf download exited with {status}"));
    }
    let path = local_dir.join(file);
    if path.exists() {
        Ok(path)
    } else {
        Err("hf download reported success but the file is missing".into())
    }
}

/// Small helper so callers can bound how long a probe may take on a wedged CLI.
pub fn detect_bounded(_timeout: Duration) -> TransferStatus {
    // Command spawning is quick; the timeout is a placeholder for a future async
    // probe. Kept so callers can express intent without a breaking change later.
    detect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_extracts_semverish() {
        assert_eq!(parse_version("huggingface_hub version 0.35.1"), Some("0.35.1".into()));
        assert_eq!(parse_version("hf 1.0.0 (xet)"), Some("1.0.0".into()));
        assert_eq!(parse_version("no digits here"), None);
    }

    #[test]
    fn detect_always_reports_builtin_ready() {
        // Whatever the machine has, the built-in path is always available and the
        // active backend is one of the two known values.
        let status = detect();
        assert!(status.builtin_ready);
        assert!(status.active_backend == "hf_xet" || status.active_backend == "builtin");
        // When not installed, we must hand the user real instructions.
        if status.xet.state == XetState::NotInstalled {
            assert!(!status.xet.instructions.is_empty());
        }
    }
}
