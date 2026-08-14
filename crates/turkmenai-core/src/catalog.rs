//! Offline-first model catalog and hardware-aware recommendation.
//!
//! The catalog is a small, checksum-friendly manifest that ships with the app so
//! the product is useful immediately, without a network round-trip on every open.
//! It never scrapes HTML model cards and never downloads anything by itself; it
//! only reasons over normalized metadata and the local `HardwareProfile`.

use crate::{CoreError, HardwareProfile, ModelFormat, Objective, Task, TrustLevel};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

/// Embedded starter catalog. Kept intentionally small and honest. It exists so the
/// app is useful on the very first run and in fully offline / blocked-network
/// environments. The live catalog is discovered directly from Hugging Face at
/// runtime on the user's machine, which is where models and datasets are hosted.
const BUILTIN_CATALOG: &str = include_str!("../../../registry/catalog.json");

/// Where the discovered catalog is cached so a later offline launch still shows the
/// last known catalog instead of only the small embedded one.
pub fn cache_path() -> PathBuf {
    crate::state::default_data_root().join("catalog.cache.json")
}

/// Where the catalog shown to the user came from — surfaced in the UI so freshness
/// is never implied dishonestly.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CatalogSource {
    /// Just discovered live from the Hugging Face Hub.
    Remote,
    /// Loaded from a previously discovered local cache.
    Cache,
    /// The small manifest embedded in the application binary.
    Builtin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogSnapshot {
    pub source: CatalogSource,
    pub catalog: Catalog,
}

/// How well a model fits *this specific computer* — always separate from the
/// model's maximum capability.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FitLevel {
    /// Comfortable headroom, GPU-accelerated where available.
    Excellent,
    /// Fits with sensible headroom on this machine.
    Good,
    /// Runs, but with little headroom or reduced context.
    Usable,
    /// Technically fits in memory but will be noticeably slow (e.g. large CPU-only).
    Slow,
    /// Does not fit available RAM/VRAM/disk on this machine.
    Unsupported,
}

impl FitLevel {
    /// A coarse rank used for ordering recommendations (higher is better).
    pub fn rank(self) -> u8 {
        match self {
            FitLevel::Excellent => 4,
            FitLevel::Good => 3,
            FitLevel::Usable => 2,
            FitLevel::Slow => 1,
            FitLevel::Unsupported => 0,
        }
    }
}

/// Specialization used to group the catalog into human-friendly sections.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum ModelCategory {
    #[default]
    Chat,
    Code,
    Translation,
    Reasoning,
    Multilingual,
    Embeddings,
    Vision,
    SpeechRecognition,
    SpeechSynthesis,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CatalogModel {
    pub id: String,
    pub name: String,
    /// Hugging Face repo id, e.g. `Qwen/Qwen2.5-0.5B-Instruct-GGUF`.
    pub repo: String,
    pub revision: String,
    /// The single artifact needed for the listed quantization.
    pub file: String,
    pub sha256: Option<String>,
    pub license: String,
    pub task: Task,
    /// Product-facing specialization, e.g. `chat`, `code`, `translation`,
    /// `reasoning`, `embeddings`, `vision`, `speech_recognition`, `speech_synthesis`.
    #[serde(default)]
    pub category: ModelCategory,
    pub format: ModelFormat,
    pub params_b: f32,
    pub quant: String,
    /// Approximate download size for `file`, in MiB. Estimated until measured.
    pub download_mib: u64,
    pub min_ram_mib: u64,
    pub rec_ram_mib: u64,
    pub context: u32,
    pub trust: TrustLevel,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub objectives: Vec<Objective>,
    /// Short localized descriptions keyed by language code (`en`/`ru`/`tk`).
    #[serde(default)]
    pub description: BTreeMap<String, String>,
}

impl CatalogModel {
    /// Direct Hugging Face `resolve` URL for the single artifact. Shown to the
    /// user with the source before any download; the download engine verifies it.
    pub fn download_url(&self) -> String {
        format!(
            "https://huggingface.co/{}/resolve/{}/{}",
            self.repo, self.revision, self.file
        )
    }

    /// RAM the plan is expected to need: the artifact plus a conservative runtime
    /// and KV-cache overhead. Deliberately pessimistic so we never over-promise.
    pub fn estimated_ram_mib(&self) -> u64 {
        self.download_mib
            .saturating_add(runtime_overhead_mib(self.params_b))
    }
}

fn runtime_overhead_mib(params_b: f32) -> u64 {
    // Base runtime + a rough KV/context allowance that grows with model size.
    let kv = (params_b * 300.0) as u64;
    768 + kv
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub model: CatalogModel,
    pub fit: FitLevel,
    pub download_url: String,
    pub estimated_ram_mib: u64,
    pub fits_disk: bool,
    /// Whether an accelerator would be used for this plan on this machine.
    pub gpu_accelerated: bool,
    /// Human-readable, evidence-first reasons. Positive reasons for why it fits,
    /// or the honest reason it does not.
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub schema_version: u32,
    pub models: Vec<CatalogModel>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawCatalog {
    schema_version: u32,
    #[serde(default)]
    models: Vec<CatalogModel>,
}

/// Optional Hugging Face read token. Only used to raise anonymous rate limits and
/// reach gated public repos; never required for ordinary public downloads.
pub(crate) fn hf_token() -> Option<String> {
    std::env::var("HF_TOKEN")
        .or_else(|_| std::env::var("HUGGING_FACE_HUB_TOKEN"))
        .ok()
        .filter(|token| !token.trim().is_empty())
}

impl Catalog {
    /// The catalog embedded in the binary. Parsing is infallible in practice
    /// because the manifest is validated in tests; a malformed override yields
    /// an empty catalog rather than a panic.
    pub fn builtin() -> Self {
        Self::from_json(BUILTIN_CATALOG).unwrap_or(Self {
            schema_version: 1,
            models: Vec::new(),
        })
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let raw: RawCatalog = serde_json::from_str(json)?;
        Ok(Self {
            schema_version: raw.schema_version,
            models: raw.models,
        })
    }

    /// Discover a live catalog directly from the Hugging Face Hub, ranked so the
    /// most-used GGUF models across every specialization surface first, then cache
    /// it atomically. This is an explicit, cacheable action — never called
    /// silently on every screen. Models are downloaded from Hugging Face too.
    pub fn fetch_remote() -> Result<Self, CoreError> {
        let hub = crate::huggingface::HfClient::new(hf_token());
        let catalog = hub.discover_catalog()?;
        if catalog.models.is_empty() {
            return Err(CoreError::Runtime("CATALOG_EMPTY".into()));
        }
        if let Ok(body) = serde_json::to_string_pretty(&RawCatalog {
            schema_version: catalog.schema_version,
            models: catalog.models.clone(),
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

    /// The most recently cached catalog, if one was ever fetched.
    pub fn cached() -> Option<Self> {
        Self::from_path(&cache_path())
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|body| Self::from_json(&body).ok())
    }

    /// Resolve the best available catalog with an honest source label:
    /// remote → cache → builtin. When `allow_network` is false, the network step
    /// is skipped entirely (offline-first, low-bandwidth respect).
    pub fn resolve(allow_network: bool) -> CatalogSnapshot {
        if allow_network {
            if let Ok(catalog) = Self::fetch_remote() {
                return CatalogSnapshot {
                    source: CatalogSource::Remote,
                    catalog,
                };
            }
        }
        if let Some(catalog) = Self::cached() {
            return CatalogSnapshot {
                source: CatalogSource::Cache,
                catalog,
            };
        }
        CatalogSnapshot {
            source: CatalogSource::Builtin,
            catalog: Self::builtin(),
        }
    }

    /// Score one model against a hardware profile, independent of objective.
    pub fn evaluate(&self, model: &CatalogModel, hardware: &HardwareProfile) -> Recommendation {
        let estimated_ram_mib = model.estimated_ram_mib();
        let fits_disk = model.download_mib.saturating_add(256) <= hardware.free_disk_mib;
        let fits_ram = estimated_ram_mib <= hardware.usable_ram_mib();
        let gpu_accelerated = hardware.accelerators.iter().any(|item| item != "cpu")
            && hardware.usable_vram_mib() >= model.download_mib;

        let mut reasons = Vec::new();
        let fit = if !fits_disk {
            reasons.push(format!(
                "Needs about {} MiB of disk but only {} MiB is free.",
                model.download_mib, hardware.free_disk_mib
            ));
            FitLevel::Unsupported
        } else if !fits_ram {
            reasons.push(format!(
                "Estimated {} MiB RAM exceeds the {} MiB usable budget on this computer.",
                estimated_ram_mib,
                hardware.usable_ram_mib()
            ));
            FitLevel::Unsupported
        } else {
            reasons.push(format!(
                "Fits the local memory budget (~{} MiB estimated of {} MiB usable).",
                estimated_ram_mib,
                hardware.usable_ram_mib()
            ));
            if gpu_accelerated {
                reasons.push("An accelerator is available for this artifact.".into());
                FitLevel::Excellent
            } else {
                // CPU-only: rank by model size, which drives throughput most.
                match model.params_b {
                    p if p <= 1.6 => {
                        reasons.push("Small enough for responsive CPU-only inference.".into());
                        FitLevel::Good
                    }
                    p if p <= 3.5 => {
                        reasons.push("Runs on CPU at a usable speed.".into());
                        FitLevel::Usable
                    }
                    _ => {
                        reasons
                            .push("Fits in RAM but will be noticeably slow without a GPU.".into());
                        FitLevel::Slow
                    }
                }
            }
        };

        Recommendation {
            download_url: model.download_url(),
            estimated_ram_mib,
            fits_disk,
            gpu_accelerated,
            reasons,
            fit,
            model: model.clone(),
        }
    }

    /// Every model with an honest fit verdict for this machine (compatible first).
    pub fn evaluate_all(&self, hardware: &HardwareProfile) -> Vec<Recommendation> {
        let mut all: Vec<Recommendation> = self
            .models
            .iter()
            .map(|model| self.evaluate(model, hardware))
            .collect();
        all.sort_by(|a, b| {
            b.fit
                .rank()
                .cmp(&a.fit.rank())
                .then(a.model.download_mib.cmp(&b.model.download_mib))
        });
        all
    }

    /// All models of one specialization, with fit verdicts, compatible first.
    pub fn by_category(
        &self,
        category: ModelCategory,
        hardware: &HardwareProfile,
    ) -> Vec<Recommendation> {
        self.evaluate_all(hardware)
            .into_iter()
            .filter(|rec| rec.model.category == category)
            .collect()
    }

    /// The set of specializations present in the catalog, in a stable order.
    pub fn categories(&self) -> Vec<ModelCategory> {
        let order = [
            ModelCategory::Chat,
            ModelCategory::Reasoning,
            ModelCategory::Code,
            ModelCategory::Translation,
            ModelCategory::Multilingual,
            ModelCategory::Vision,
            ModelCategory::SpeechRecognition,
            ModelCategory::SpeechSynthesis,
            ModelCategory::Embeddings,
        ];
        order
            .into_iter()
            .filter(|category| self.models.iter().any(|model| model.category == *category))
            .collect()
    }

    /// Recommendations that actually fit this computer, ranked for the objective.
    /// Models that do not fit are excluded here — the caller can still show them
    /// through `evaluate_all` in an explicit "incompatible" view.
    pub fn recommend(
        &self,
        hardware: &HardwareProfile,
        objective: Objective,
    ) -> Vec<Recommendation> {
        let mut compatible: Vec<Recommendation> = self
            .evaluate_all(hardware)
            .into_iter()
            .filter(|rec| rec.fit != FitLevel::Unsupported)
            .collect();
        compatible.sort_by(|a, b| {
            objective_bonus(objective, &b.model)
                .cmp(&objective_bonus(objective, &a.model))
                .then(b.fit.rank().cmp(&a.fit.rank()))
                .then(a.model.download_mib.cmp(&b.model.download_mib))
        });
        compatible
    }
}

/// Objective preference as a small additive bonus, so ranking stays explainable.
fn objective_bonus(objective: Objective, model: &CatalogModel) -> i32 {
    let mut bonus = if model.objectives.contains(&objective) {
        10
    } else {
        0
    };
    match objective {
        Objective::BestQuality => bonus += (model.params_b * 2.0) as i32,
        Objective::Fastest | Objective::LowestRam => {
            bonus += (8.0 - model.params_b).max(0.0) as i32
        }
        Objective::LowestDownload => {
            bonus += (5000u64.saturating_sub(model.download_mib) / 1000) as i32
        }
        Objective::Balanced => {
            if (1.0..=4.0).contains(&model.params_b) {
                bonus += 3;
            }
        }
        Objective::LowestVram => bonus += (8.0 - model.params_b).max(0.0) as i32,
    }
    bonus
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn cpu_profile(ram_mib: u64, disk_mib: u64) -> HardwareProfile {
        HardwareProfile {
            cpu: "test".into(),
            ram_mib,
            free_disk_mib: disk_mib,
            accelerators: BTreeSet::from(["cpu".into()]),
            vram_mib: 0,
            os: "linux".into(),
            reserve_ram_mib: 1024,
            reserve_vram_mib: 0,
        }
    }

    #[test]
    fn builtin_catalog_parses_and_is_non_empty() {
        let catalog = Catalog::builtin();
        assert_eq!(catalog.schema_version, 1);
        assert!(!catalog.models.is_empty());
        // Every model must expose a resolvable-looking artifact and a license.
        for model in &catalog.models {
            assert!(model.download_url().starts_with("https://huggingface.co/"));
            assert!(!model.license.is_empty());
            assert!(model.download_mib > 0);
        }
    }

    #[test]
    fn tiny_model_fits_a_modest_cpu() {
        let catalog = Catalog::builtin();
        let hardware = cpu_profile(24 * 1024, 200 * 1024);
        let recs = catalog.recommend(&hardware, Objective::Balanced);
        assert!(!recs.is_empty());
        // Nothing unsupported may appear in the recommended list.
        assert!(recs.iter().all(|rec| rec.fit != FitLevel::Unsupported));
    }

    #[test]
    fn oversized_model_is_unsupported_on_tiny_ram() {
        let catalog = Catalog::builtin();
        let hardware = cpu_profile(2 * 1024, 200 * 1024);
        // The 7B model cannot fit ~2 GB RAM; it must never be recommended.
        let recs = catalog.recommend(&hardware, Objective::BestQuality);
        assert!(recs.iter().all(|rec| rec.model.params_b < 7.0));
        // But it must still be visible with an honest Unsupported verdict.
        let all = catalog.evaluate_all(&hardware);
        assert!(all
            .iter()
            .any(|rec| rec.model.params_b >= 7.0 && rec.fit == FitLevel::Unsupported));
    }

    #[test]
    fn no_disk_space_blocks_everything() {
        let catalog = Catalog::builtin();
        let hardware = cpu_profile(24 * 1024, 10);
        let recs = catalog.recommend(&hardware, Objective::Balanced);
        assert!(recs.is_empty());
    }

    #[test]
    fn lowest_download_prefers_the_smallest_artifact() {
        let catalog = Catalog::builtin();
        let hardware = cpu_profile(24 * 1024, 200 * 1024);
        let recs = catalog.recommend(&hardware, Objective::LowestDownload);
        let smallest = catalog.models.iter().map(|m| m.download_mib).min().unwrap();
        assert_eq!(recs.first().unwrap().model.download_mib, smallest);
    }
}
