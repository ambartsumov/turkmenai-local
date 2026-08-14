//! Immutable SHA-256 blob storage. Metadata points to content, never the reverse.

use crate::{sha256_file, CoreError, ModelDescriptor};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub struct ContentStore {
    root: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlobRef {
    pub sha256: String,
    pub bytes: u64,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredManifest {
    pub schema_version: u32,
    pub descriptor: ModelDescriptor,
    pub blobs: Vec<BlobRef>,
}

impl ContentStore {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, CoreError> {
        let root = root.into();
        for directory in [
            "blobs/sha256",
            "manifests",
            "models",
            "derived",
            "runtimes",
            "journal",
        ] {
            fs::create_dir_all(root.join(directory))?;
        }
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
    pub fn blob_path(&self, sha256: &str) -> PathBuf {
        self.root.join("blobs/sha256").join(sha256)
    }

    pub fn ingest_file(&self, source: &Path) -> Result<BlobRef, CoreError> {
        let hash = sha256_file(source)?;
        let destination = self.blob_path(&hash);
        let bytes = fs::metadata(source)?.len();
        if !destination.exists() {
            let temporary = destination.with_extension("tmp");
            fs::copy(source, &temporary)?;
            if sha256_file(&temporary)? != hash {
                let _ = fs::remove_file(&temporary);
                return Err(CoreError::UnsupportedSource(
                    "copy integrity check failed".into(),
                ));
            }
            fs::rename(temporary, &destination)?;
        }
        Ok(BlobRef {
            sha256: hash.clone(),
            bytes,
            path: format!("blobs/sha256/{hash}"),
        })
    }

    pub fn verify_blob(&self, blob: &BlobRef) -> Result<bool, CoreError> {
        let path = self.blob_path(&blob.sha256);
        Ok(path.exists()
            && fs::metadata(&path)?.len() == blob.bytes
            && sha256_file(&path)? == blob.sha256)
    }

    pub fn save_manifest(
        &self,
        model_id: &str,
        descriptor: ModelDescriptor,
        blobs: Vec<BlobRef>,
    ) -> Result<PathBuf, CoreError> {
        let safe_id = model_id
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                    character
                } else {
                    '_'
                }
            })
            .collect::<String>();
        let manifest = StoredManifest {
            schema_version: 1,
            descriptor,
            blobs,
        };
        let target = self.root.join("manifests").join(format!("{safe_id}.json"));
        let temporary = target.with_extension("json.tmp");
        fs::write(
            &temporary,
            serde_json::to_vec_pretty(&manifest)
                .map_err(|error| CoreError::UnsupportedSource(error.to_string()))?,
        )?;
        fs::rename(temporary, &target)?;
        Ok(target)
    }

    pub fn load_manifest(&self, model_id: &str) -> Result<StoredManifest, CoreError> {
        let safe_id = model_id
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                    character
                } else {
                    '_'
                }
            })
            .collect::<String>();
        let value = fs::read(self.root.join("manifests").join(format!("{safe_id}.json")))?;
        serde_json::from_slice(&value)
            .map_err(|error| CoreError::UnsupportedSource(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModelResolver;

    #[test]
    fn deduplicates_identical_content() {
        let workspace = tempfile::tempdir().unwrap();
        let store = ContentStore::open(workspace.path().join("store")).unwrap();
        let first = workspace.path().join("first.gguf");
        let second = workspace.path().join("second.gguf");
        fs::write(&first, b"one immutable model blob").unwrap();
        fs::write(&second, b"one immutable model blob").unwrap();
        let first_ref = store.ingest_file(&first).unwrap();
        let second_ref = store.ingest_file(&second).unwrap();
        assert_eq!(first_ref.sha256, second_ref.sha256);
        assert!(store.verify_blob(&first_ref).unwrap());
        let descriptor = ModelResolver::resolve_file(&first).unwrap();
        store
            .save_manifest("demo", descriptor, vec![first_ref])
            .unwrap();
        assert_eq!(store.load_manifest("demo").unwrap().schema_version, 1);
    }
}
