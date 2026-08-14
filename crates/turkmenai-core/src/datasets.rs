//! Offline-first dataset catalog with hardware (disk) aware fit.
//!
//! Datasets are first-class and modeled separately from models: they carry their
//! own licenses, splits, languages, sizes and risk flags. Like the model catalog,
//! the live source is the Hugging Face Hub; the embedded manifest is only an
//! offline fallback. Nothing here executes dataset code — that is the Data
//! Inspector's contract, which analyzes files without running them.

use crate::{CoreError, HardwareProfile};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

const BUILTIN_DATASETS: &str = include_str!("../../../registry/datasets.json");

pub fn cache_path() -> PathBuf {
    crate::state::default_data_root().join("datasets.cache.json")
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum DatasetCategory {
    #[default]
    Instruction,
    Chat,
    Code,
    Reasoning,
    Translation,
    Summarization,
    Classification,
    Multilingual,
    Speech,
    Embeddings,
}

/// A risk/usage flag distilled from the license and tags. It is advisory only and
/// never a substitute for reading the upstream terms, which the UI always links.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatasetRisk {
    Permissive,
    Attribution,
    AttributionShareAlike,
    NonCommercial,
    ReviewTerms,
    Unknown,
}

impl Default for DatasetRisk {
    fn default() -> Self {
        DatasetRisk::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetRecord {
    pub id: String,
    pub name: String,
    pub repo: String,
    pub revision: String,
    #[serde(default)]
    pub category: DatasetCategory,
    pub license: String,
    #[serde(default)]
    pub languages: Vec<String>,
    /// Approximate download size in MiB. Estimated until measured.
    #[serde(default)]
    pub download_mib: u64,
    /// Approximate on-disk size after unpacking, in MiB.
    #[serde(default)]
    pub unpacked_mib: u64,
    #[serde(default)]
    pub num_examples: u64,
    #[serde(default)]
    pub risk: DatasetRisk,
    #[serde(default)]
    pub description: BTreeMap<String, String>,
}

impl DatasetRecord {
    pub fn page_url(&self) -> String {
        format!("https://huggingface.co/datasets/{}", self.repo)
    }

    /// Total disk the install needs: download + unpacked + a temporary reserve.
    pub fn required_disk_mib(&self) -> u64 {
        let unpacked = if self.unpacked_mib > 0 {
            self.unpacked_mib
        } else {
            self.download_mib.saturating_mul(2)
        };
        self.download_mib
            .saturating_add(unpacked)
            .saturating_add(512)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatasetFit {
    Fits,
    Tight,
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetEvaluation {
    pub dataset: DatasetRecord,
    pub fit: DatasetFit,
    pub required_disk_mib: u64,
    pub page_url: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatasetSource {
    Remote,
    Cache,
    Builtin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetCatalog {
    pub schema_version: u32,
    pub datasets: Vec<DatasetRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSnapshot {
    pub source: DatasetSource,
    pub catalog: DatasetCatalog,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawDatasetCatalog {
    schema_version: u32,
    #[serde(default)]
    datasets: Vec<DatasetRecord>,
}

impl DatasetCatalog {
    pub fn builtin() -> Self {
        Self::from_json(BUILTIN_DATASETS).unwrap_or(Self {
            schema_version: 1,
            datasets: Vec::new(),
        })
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let raw: RawDatasetCatalog = serde_json::from_str(json)?;
        Ok(Self {
            schema_version: raw.schema_version,
            datasets: raw.datasets,
        })
    }

    pub fn cached() -> Option<Self> {
        std::fs::read_to_string(cache_path())
            .ok()
            .and_then(|body| Self::from_json(&body).ok())
    }

    /// Discover live datasets from the Hugging Face Hub and cache them atomically.
    pub fn fetch_remote() -> Result<Self, CoreError> {
        let hub = crate::huggingface::HfClient::new(crate::catalog::hf_token());
        let catalog = hub.discover_datasets()?;
        if catalog.datasets.is_empty() {
            return Err(CoreError::Runtime("DATASET_CATALOG_EMPTY".into()));
        }
        if let Ok(body) = serde_json::to_string_pretty(&RawDatasetCatalog {
            schema_version: catalog.schema_version,
            datasets: catalog.datasets.clone(),
        }) {
            let path = cache_path();
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
                let temporary = path.with_extension("json.tmp");
                if std::fs::write(&temporary, &body).is_ok() {
                    let _ = std::fs::rename(&temporary, &path);
                }
            }
        }
        Ok(catalog)
    }

    pub fn resolve(allow_network: bool) -> DatasetSnapshot {
        if allow_network {
            if let Ok(catalog) = Self::fetch_remote() {
                return DatasetSnapshot {
                    source: DatasetSource::Remote,
                    catalog,
                };
            }
        }
        if let Some(catalog) = Self::cached() {
            return DatasetSnapshot {
                source: DatasetSource::Cache,
                catalog,
            };
        }
        DatasetSnapshot {
            source: DatasetSource::Builtin,
            catalog: Self::builtin(),
        }
    }

    pub fn evaluate(
        &self,
        dataset: &DatasetRecord,
        hardware: &HardwareProfile,
    ) -> DatasetEvaluation {
        let required = dataset.required_disk_mib();
        let free = hardware.free_disk_mib;
        let mut reasons = Vec::new();
        let fit = if required > free {
            reasons.push(format!(
                "Needs about {required} MiB of disk (download + unpacked + reserve) but {free} MiB is free."
            ));
            DatasetFit::Unsupported
        } else if required.saturating_mul(4) > free {
            reasons.push(format!(
                "Fits, but leaves little headroom (~{required} MiB of {free} MiB free)."
            ));
            DatasetFit::Tight
        } else {
            reasons.push(format!(
                "Fits comfortably (~{required} MiB of {free} MiB free)."
            ));
            DatasetFit::Fits
        };
        if dataset.risk == DatasetRisk::NonCommercial {
            reasons.push("Non-commercial license — review terms before any commercial use.".into());
        }
        DatasetEvaluation {
            required_disk_mib: required,
            page_url: dataset.page_url(),
            reasons,
            fit,
            dataset: dataset.clone(),
        }
    }

    pub fn evaluate_all(&self, hardware: &HardwareProfile) -> Vec<DatasetEvaluation> {
        let mut all: Vec<DatasetEvaluation> = self
            .datasets
            .iter()
            .map(|dataset| self.evaluate(dataset, hardware))
            .collect();
        all.sort_by(|a, b| {
            fit_rank(b.fit)
                .cmp(&fit_rank(a.fit))
                .then(a.required_disk_mib.cmp(&b.required_disk_mib))
        });
        all
    }

    pub fn by_category(
        &self,
        category: DatasetCategory,
        hardware: &HardwareProfile,
    ) -> Vec<DatasetEvaluation> {
        self.evaluate_all(hardware)
            .into_iter()
            .filter(|item| item.dataset.category == category)
            .collect()
    }

    pub fn categories(&self) -> Vec<DatasetCategory> {
        let order = [
            DatasetCategory::Instruction,
            DatasetCategory::Chat,
            DatasetCategory::Reasoning,
            DatasetCategory::Code,
            DatasetCategory::Translation,
            DatasetCategory::Summarization,
            DatasetCategory::Classification,
            DatasetCategory::Multilingual,
            DatasetCategory::Speech,
            DatasetCategory::Embeddings,
        ];
        order
            .into_iter()
            .filter(|category| self.datasets.iter().any(|d| d.category == *category))
            .collect()
    }
}

fn fit_rank(fit: DatasetFit) -> u8 {
    match fit {
        DatasetFit::Fits => 2,
        DatasetFit::Tight => 1,
        DatasetFit::Unsupported => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn profile(disk_mib: u64) -> HardwareProfile {
        HardwareProfile {
            cpu: "t".into(),
            ram_mib: 24 * 1024,
            free_disk_mib: disk_mib,
            accelerators: BTreeSet::from(["cpu".into()]),
            vram_mib: 0,
            os: "linux".into(),
            reserve_ram_mib: 1024,
            reserve_vram_mib: 0,
        }
    }

    #[test]
    fn builtin_parses_and_has_categories() {
        let catalog = DatasetCatalog::builtin();
        assert!(!catalog.datasets.is_empty());
        assert!(!catalog.categories().is_empty());
        for dataset in &catalog.datasets {
            assert!(dataset.page_url().contains("/datasets/"));
            assert!(!dataset.license.is_empty());
        }
    }

    #[test]
    fn small_disk_marks_large_dataset_unsupported() {
        let catalog = DatasetCatalog::builtin();
        let evaluations = catalog.evaluate_all(&profile(500));
        // The multilingual OPUS-100 (~1.5 GB) must not fit in 500 MiB.
        assert!(evaluations
            .iter()
            .any(|e| e.dataset.download_mib >= 1000 && e.fit == DatasetFit::Unsupported));
    }

    #[test]
    fn ample_disk_fits_small_datasets() {
        let catalog = DatasetCatalog::builtin();
        let evaluations = catalog.evaluate_all(&profile(200 * 1024));
        assert!(evaluations.iter().any(|e| e.fit == DatasetFit::Fits));
    }
}
