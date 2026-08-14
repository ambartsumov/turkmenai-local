//! Strictly loopback-only adapter for an explicitly configured llama-server.
//! It never downloads model files, accepts a remote URL, or falls back to cloud inference.

use crate::CoreError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LlamaServerEndpoint {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LlamaHealth {
    Ready,
    Loading,
    Unreachable,
    Failed,
}

impl LlamaServerEndpoint {
    pub fn new(port: u16) -> Result<Self, CoreError> {
        if port == 0 {
            return Err(CoreError::Runtime("loopback port must be non-zero".into()));
        }
        Ok(Self { port })
    }

    pub fn base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.port)
    }

    fn client() -> Result<reqwest::blocking::Client, CoreError> {
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|error| CoreError::Runtime(error.to_string()))
    }

    pub fn health(&self) -> LlamaHealth {
        match Self::client().and_then(|client| {
            client
                .get(format!("{}/health", self.base_url()))
                .send()
                .map_err(|error| CoreError::Runtime(error.to_string()))
        }) {
            Ok(response) if response.status().is_success() => LlamaHealth::Ready,
            Ok(response) if response.status().as_u16() == 503 => LlamaHealth::Loading,
            Ok(_) => LlamaHealth::Failed,
            Err(_) => LlamaHealth::Unreachable,
        }
    }

    pub fn models(&self) -> Result<Value, CoreError> {
        if self.health() != LlamaHealth::Ready {
            return Err(CoreError::Runtime(
                "llama-server is not ready on loopback".into(),
            ));
        }
        Self::client()?
            .get(format!("{}/v1/models", self.base_url()))
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| CoreError::Runtime(error.to_string()))?
            .json()
            .map_err(|error| CoreError::Runtime(error.to_string()))
    }

    pub fn chat(&self, payload: &Value) -> Result<Value, CoreError> {
        if self.health() != LlamaHealth::Ready {
            return Err(CoreError::Runtime(
                "llama-server is not ready on loopback".into(),
            ));
        }
        Self::client()?
            .post(format!("{}/v1/chat/completions", self.base_url()))
            .json(payload)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| CoreError::Runtime(error.to_string()))?
            .json()
            .map_err(|error| CoreError::Runtime(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_only_loopback_urls() {
        let endpoint = LlamaServerEndpoint::new(8080).unwrap();
        assert_eq!(endpoint.base_url(), "http://127.0.0.1:8080");
        assert!(LlamaServerEndpoint::new(0).is_err());
    }
}
