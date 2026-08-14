//! RuntimeSupervisor controls only explicitly configured backend executables.
//! It does not inspect or execute source files shipped with model repositories.

use crate::CoreError;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::{Child, Command},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeState {
    Stopped,
    Starting,
    Running,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeRecord {
    pub id: String,
    pub backend: String,
    pub executable: String,
    pub arguments: Vec<String>,
    pub workspace: String,
    pub state: RuntimeState,
    pub pid: Option<u32>,
    pub started_unix_ms: Option<u128>,
    pub error: Option<String>,
}

struct ManagedRuntime {
    record: RuntimeRecord,
    child: Child,
}

#[derive(Default)]
pub struct RuntimeSupervisor {
    processes: BTreeMap<String, ManagedRuntime>,
}

impl RuntimeSupervisor {
    pub fn start(
        &mut self,
        id: &str,
        backend: &str,
        executable: &Path,
        arguments: &[String],
        workspace: &Path,
        extra_env: &[(String, String)],
    ) -> Result<RuntimeRecord, CoreError> {
        if !workspace.is_dir() {
            return Err(CoreError::MissingPath(workspace.display().to_string()));
        }
        if !executable.is_file() {
            return Err(CoreError::MissingPath(executable.display().to_string()));
        }
        if self.processes.contains_key(id) {
            return Err(CoreError::UnsupportedSource(format!(
                "runtime already active: {id}"
            )));
        }
        let mut command = Command::new(executable);
        command
            .args(arguments)
            .current_dir(workspace)
            .env_clear()
            .env("PATH", std::env::var("PATH").unwrap_or_default())
            .env("HOME", workspace);
        // Extra environment (e.g. a managed engine's shared-library directory) is
        // applied last so it can extend PATH / LD_LIBRARY_PATH deterministically.
        for (key, value) in extra_env {
            command.env(key, value);
        }
        let child = command.spawn().map_err(CoreError::Io)?;
        let record = RuntimeRecord {
            id: id.into(),
            backend: backend.into(),
            executable: executable.display().to_string(),
            arguments: arguments.to_vec(),
            workspace: workspace.display().to_string(),
            state: RuntimeState::Running,
            pid: Some(child.id()),
            started_unix_ms: Some(now_ms()),
            error: None,
        };
        self.processes.insert(
            id.into(),
            ManagedRuntime {
                record: record.clone(),
                child,
            },
        );
        Ok(record)
    }

    pub fn refresh(&mut self, id: &str) -> Option<RuntimeRecord> {
        let managed = self.processes.get_mut(id)?;
        match managed.child.try_wait() {
            Ok(Some(status)) => {
                managed.record.state = RuntimeState::Failed;
                managed.record.error = Some(format!("process exited: {status}"));
            }
            Ok(None) => {
                managed.record.state = RuntimeState::Running;
            }
            Err(error) => {
                managed.record.state = RuntimeState::Failed;
                managed.record.error = Some(error.to_string());
            }
        }
        Some(managed.record.clone())
    }

    pub fn stop(&mut self, id: &str) -> Option<RuntimeRecord> {
        let mut managed = self.processes.remove(id)?;
        let _ = managed.child.kill();
        let _ = managed.child.wait();
        managed.record.state = RuntimeState::Stopped;
        managed.record.pid = None;
        Some(managed.record)
    }

    pub fn list(&mut self) -> Vec<RuntimeRecord> {
        let ids = self.processes.keys().cloned().collect::<Vec<_>>();
        ids.into_iter().filter_map(|id| self.refresh(&id)).collect()
    }
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

pub fn discover_llama_server(explicit: Option<&Path>) -> Option<PathBuf> {
    if let Some(path) = explicit.filter(|path| path.is_file()) {
        return Some(path.to_path_buf());
    }
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|entry| {
            entry.join(if cfg!(windows) {
                "llama-server.exe"
            } else {
                "llama-server"
            })
        })
        .find(|candidate| candidate.is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn supervises_a_real_fixture_process() {
        let workspace = tempfile::tempdir().unwrap();
        let mut supervisor = RuntimeSupervisor::default();
        let command = Path::new("/bin/sh");
        let record = supervisor
            .start(
                "fixture",
                "fixture",
                command,
                &["-c".into(), "sleep 5".into()],
                workspace.path(),
                &[],
            )
            .unwrap();
        assert_eq!(record.state, RuntimeState::Running);
        assert_eq!(
            supervisor.stop("fixture").unwrap().state,
            RuntimeState::Stopped
        );
    }
}
