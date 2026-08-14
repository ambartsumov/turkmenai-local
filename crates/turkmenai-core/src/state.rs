//! Versioned, privacy-first local state. This is separate from immutable model blobs.
//! Migration never removes model content and always writes atomically.

use crate::{CoreError, ModelDescriptor};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

pub const APP_STATE_SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstallLifecycle {
    Preparing,
    Downloading,
    Verifying,
    Installing,
    Configuring,
    Testing,
    Ready,
    Failed,
    Rollback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: String,
    pub api_port: u16,
    pub telemetry_enabled: bool,
    pub lan_sharing_enabled: bool,
    pub update_channel: String,
    pub model_store: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: "en".into(),
            api_port: 8742,
            telemetry_enabled: false,
            lan_sharing_enabled: false,
            update_channel: "stable".into(),
            model_store: default_data_root().join("models").display().to_string(),
        }
    }
}

/// Persisted configuration for an explicit local llama-server. Hostnames are deliberately absent:
/// all runtime traffic is bound to 127.0.0.1 by the shell and adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RuntimeConfig {
    pub executable_path: Option<String>,
    pub model_path: Option<String>,
    pub port: u16,
    pub context_size: u32,
    pub gpu_layers: i32,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            executable_path: None,
            model_path: None,
            port: 8080,
            context_size: 4096,
            gpu_layers: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledModelRecord {
    pub descriptor: ModelDescriptor,
    pub lifecycle: InstallLifecycle,
    pub manifest_path: Option<String>,
    pub last_error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub schema_version: u32,
    pub settings: AppSettings,
    #[serde(default)]
    pub runtime: RuntimeConfig,
    pub models: BTreeMap<String, InstalledModelRecord>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            schema_version: APP_STATE_SCHEMA_VERSION,
            settings: AppSettings::default(),
            runtime: RuntimeConfig::default(),
            models: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppStateStore {
    root: PathBuf,
}

impl AppStateStore {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, CoreError> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn open_default() -> Result<Self, CoreError> {
        Self::open(default_data_root())
    }
    pub fn root(&self) -> &Path {
        &self.root
    }
    pub fn path(&self) -> PathBuf {
        self.root.join("state.json")
    }

    pub fn load(&self) -> Result<AppState, CoreError> {
        let path = self.path();
        if !path.exists() {
            return Ok(AppState::default());
        }
        let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path)?)
            .map_err(|error| CoreError::UnsupportedSource(format!("STATE_CORRUPTED: {error}")))?;
        let version = value
            .get("schema_version")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0) as u32;
        if version > APP_STATE_SCHEMA_VERSION {
            return Err(CoreError::UnsupportedSource(format!(
                "STATE_NEWER_THAN_APP: {version}"
            )));
        }
        if version == 0 {
            value["schema_version"] = serde_json::json!(APP_STATE_SCHEMA_VERSION);
            if value.get("settings").is_none() {
                value["settings"] = serde_json::to_value(AppSettings::default())
                    .map_err(|error| CoreError::UnsupportedSource(error.to_string()))?;
            }
            if value.get("models").is_none() {
                value["models"] = serde_json::json!({});
            }
        }
        if version < 2 && value.get("runtime").is_none() {
            value["runtime"] = serde_json::to_value(RuntimeConfig::default())
                .map_err(|error| CoreError::UnsupportedSource(error.to_string()))?;
        }
        let mut state: AppState = serde_json::from_value(value)
            .map_err(|error| CoreError::UnsupportedSource(format!("STATE_CORRUPTED: {error}")))?;
        state.schema_version = APP_STATE_SCHEMA_VERSION;
        Ok(state)
    }

    pub fn save(&self, state: &AppState) -> Result<(), CoreError> {
        let mut state = state.clone();
        state.schema_version = APP_STATE_SCHEMA_VERSION;
        let target = self.path();
        if target.exists() {
            fs::copy(&target, self.root.join("state.previous.json"))?;
        }
        let temporary = self.root.join("state.json.tmp");
        fs::write(
            &temporary,
            serde_json::to_vec_pretty(&state)
                .map_err(|error| CoreError::UnsupportedSource(error.to_string()))?,
        )?;
        fs::rename(temporary, target)?;
        Ok(())
    }

    pub fn backup_config(&self) -> Result<PathBuf, CoreError> {
        let state = self.load()?;
        let backup = self
            .root
            .join(format!("config-backup-{}.json", now_millis()));
        fs::write(
            &backup,
            serde_json::to_vec_pretty(&state.settings)
                .map_err(|error| CoreError::UnsupportedSource(error.to_string()))?,
        )?;
        Ok(backup)
    }

    pub fn restore_config(&self, path: &Path) -> Result<(), CoreError> {
        let settings: AppSettings = serde_json::from_slice(&fs::read(path)?)
            .map_err(|error| CoreError::UnsupportedSource(format!("BACKUP_INVALID: {error}")))?;
        let mut state = self.load()?;
        state.settings = settings;
        self.save(&state)
    }

    pub fn export_inventory(&self) -> Result<Vec<InstalledModelRecord>, CoreError> {
        Ok(self.load()?.models.into_values().collect())
    }
}

pub fn default_data_root() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("TurkmenAILocal")
}
fn now_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_legacy_state_and_preserves_defaults() {
        let temporary = tempfile::tempdir().unwrap();
        let store = AppStateStore::open(temporary.path()).unwrap();
        fs::write(store.path(), br#"{"models":{}}"#).unwrap();
        let migrated = store.load().unwrap();
        assert_eq!(migrated.schema_version, APP_STATE_SCHEMA_VERSION);
        assert!(!migrated.settings.telemetry_enabled);
        assert_eq!(migrated.runtime.port, 8080);
    }

    #[test]
    fn config_backup_and_restore_do_not_touch_model_inventory() {
        let temporary = tempfile::tempdir().unwrap();
        let store = AppStateStore::open(temporary.path()).unwrap();
        let mut state = AppState::default();
        state.settings.language = "tk".into();
        store.save(&state).unwrap();
        let backup = store.backup_config().unwrap();
        state.settings.language = "en".into();
        store.save(&state).unwrap();
        store.restore_config(&backup).unwrap();
        assert_eq!(store.load().unwrap().settings.language, "tk");
    }
}
