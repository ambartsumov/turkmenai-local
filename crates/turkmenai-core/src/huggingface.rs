//! Hugging Face Hub adapter.
//!
//! Discovery and downloads both target the Hugging Face Hub, where the models and
//! datasets actually live. This module treats every field of the Hub JSON as
//! untrusted data: it parses normalized metadata, never executes repository code,
//! and never downloads weights here (that is the download engine's job). It only
//! lists candidates and reads file sizes so the recommender can judge hardware fit.

use crate::{
    catalog::{Catalog, CatalogModel, ModelCategory},
    datasets::{DatasetCatalog, DatasetCategory, DatasetRecord, DatasetRisk},
    CoreError, ModelFormat, Objective, Task, TrustLevel,
};
use serde::Deserialize;
use std::{collections::BTreeMap, time::Duration};

const HF_API: &str = "https://huggingface.co/api";

/// How many models to keep per specialization. Kept small to respect slow and
/// metered connections — each kept model costs one extra file-listing request.
const PER_CATEGORY: usize = 6;

pub struct HfClient {
    client: reqwest::blocking::Client,
    token: Option<String>,
}

/// One raw model row from `GET /api/models`.
#[derive(Debug, Deserialize)]
struct HfModelRow {
    #[serde(alias = "modelId")]
    id: String,
    #[serde(default)]
    downloads: u64,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    pipeline_tag: Option<String>,
}

/// One entry from `GET /api/models/{id}/tree/main?recursive=true`.
#[derive(Debug, Deserialize, Clone)]
pub struct HfTreeEntry {
    #[serde(rename = "type")]
    pub kind: String,
    pub path: String,
    #[serde(default)]
    pub size: u64,
}

/// A specialization mapped to a concrete Hub query.
struct CategoryQuery {
    category: ModelCategory,
    pipeline_tag: &'static str,
    search: Option<&'static str>,
    task: Task,
}

fn category_queries() -> Vec<CategoryQuery> {
    use ModelCategory::*;
    vec![
        CategoryQuery {
            category: Chat,
            pipeline_tag: "text-generation",
            search: Some("instruct"),
            task: Task::TextGeneration,
        },
        CategoryQuery {
            category: Reasoning,
            pipeline_tag: "text-generation",
            search: Some("distill"),
            task: Task::TextGeneration,
        },
        CategoryQuery {
            category: Code,
            pipeline_tag: "text-generation",
            search: Some("coder"),
            task: Task::TextGeneration,
        },
        CategoryQuery {
            category: Translation,
            pipeline_tag: "translation",
            search: None,
            task: Task::TextGeneration,
        },
        CategoryQuery {
            category: Embeddings,
            pipeline_tag: "feature-extraction",
            search: None,
            task: Task::Embeddings,
        },
        CategoryQuery {
            category: Vision,
            pipeline_tag: "image-text-to-text",
            search: None,
            task: Task::Vision,
        },
        CategoryQuery {
            category: SpeechRecognition,
            pipeline_tag: "automatic-speech-recognition",
            search: None,
            task: Task::SpeechRecognition,
        },
    ]
}

impl HfClient {
    pub fn new(token: Option<String>) -> Self {
        let client = reqwest::blocking::Client::builder()
            .connect_timeout(Duration::from_secs(15))
            .timeout(Duration::from_secs(45))
            .user_agent("TurkmenAI-Local")
            .build()
            .unwrap_or_default();
        Self { client, token }
    }

    fn get_json<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T, CoreError> {
        let mut request = self.client.get(url);
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        request
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| CoreError::Runtime(format!("HF_REQUEST_FAILED: {error}")))?
            .json::<T>()
            .map_err(|error| CoreError::Runtime(format!("HF_PARSE_FAILED: {error}")))
    }

    /// List the most-downloaded GGUF models for one specialization.
    fn list_models(&self, query: &CategoryQuery) -> Result<Vec<HfModelRow>, CoreError> {
        let mut url = format!(
            "{HF_API}/models?filter=gguf&pipeline_tag={}&sort=downloads&direction=-1&limit={}",
            query.pipeline_tag,
            PER_CATEGORY * 3
        );
        if let Some(search) = query.search {
            url.push_str(&format!("&search={search}"));
        }
        self.get_json(&url)
    }

    fn list_files(&self, id: &str) -> Result<Vec<HfTreeEntry>, CoreError> {
        self.get_json(&format!("{HF_API}/models/{id}/tree/main?recursive=true"))
    }

    /// Build a hardware-agnostic catalog across every specialization. The caller
    /// (`Catalog::recommend`) applies the local hardware filter afterwards.
    pub fn discover_catalog(&self) -> Result<Catalog, CoreError> {
        let mut models: Vec<CatalogModel> = Vec::new();
        let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut last_error: Option<CoreError> = None;

        for query in category_queries() {
            let rows = match self.list_models(&query) {
                Ok(rows) => rows,
                Err(error) => {
                    last_error = Some(error);
                    continue;
                }
            };
            let mut kept = 0usize;
            for row in rows {
                if kept >= PER_CATEGORY || seen.contains(&row.id) {
                    continue;
                }
                let files = match self.list_files(&row.id) {
                    Ok(files) => files,
                    Err(_) => continue,
                };
                if let Some(model) = map_model(&row, &files, query.category, query.task.clone()) {
                    seen.insert(row.id.clone());
                    models.push(model);
                    kept += 1;
                }
            }
        }

        if models.is_empty() {
            return Err(last_error.unwrap_or_else(|| CoreError::Runtime("HF_NO_MODELS".into())));
        }
        Ok(Catalog {
            schema_version: 1,
            models,
        })
    }
}

// ---- Datasets -------------------------------------------------------------

const DATASETS_SERVER: &str = "https://datasets-server.huggingface.co";

#[derive(Debug, Deserialize)]
struct HfDatasetRow {
    id: String,
    #[serde(default)]
    downloads: u64,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct HfSizeResponse {
    size: HfSizeOuter,
}
#[derive(Debug, Deserialize)]
struct HfSizeOuter {
    dataset: HfSizeInner,
}
#[derive(Debug, Deserialize, Default)]
struct HfSizeInner {
    #[serde(default)]
    num_bytes_original_files: u64,
    #[serde(default)]
    num_bytes_parquet_files: u64,
    #[serde(default)]
    num_rows: u64,
}

struct DatasetQuery {
    category: DatasetCategory,
    task_category: &'static str,
    search: Option<&'static str>,
}

fn dataset_queries() -> Vec<DatasetQuery> {
    use DatasetCategory::*;
    vec![
        DatasetQuery {
            category: Instruction,
            task_category: "text-generation",
            search: Some("instruct"),
        },
        DatasetQuery {
            category: Chat,
            task_category: "text-generation",
            search: Some("chat"),
        },
        DatasetQuery {
            category: Code,
            task_category: "text-generation",
            search: Some("code"),
        },
        DatasetQuery {
            category: Reasoning,
            task_category: "text-generation",
            search: Some("math"),
        },
        DatasetQuery {
            category: Translation,
            task_category: "translation",
            search: None,
        },
        DatasetQuery {
            category: Summarization,
            task_category: "summarization",
            search: None,
        },
        DatasetQuery {
            category: Classification,
            task_category: "text-classification",
            search: None,
        },
        DatasetQuery {
            category: Speech,
            task_category: "automatic-speech-recognition",
            search: None,
        },
    ]
}

const DATASETS_PER_CATEGORY: usize = 5;

impl HfClient {
    fn list_datasets(&self, query: &DatasetQuery) -> Result<Vec<HfDatasetRow>, CoreError> {
        let mut url = format!(
            "{HF_API}/datasets?filter=task_categories:{}&sort=downloads&direction=-1&limit={}",
            query.task_category,
            DATASETS_PER_CATEGORY * 3
        );
        if let Some(search) = query.search {
            url.push_str(&format!("&search={search}"));
        }
        self.get_json(&url)
    }

    /// Best-effort exact size from the datasets-server. `None` when the dataset is
    /// not viewer-enabled; the caller then estimates from `size_categories` tags.
    fn dataset_size_mib(&self, id: &str) -> Option<u64> {
        let response: HfSizeResponse = self
            .get_json(&format!("{DATASETS_SERVER}/size?dataset={id}"))
            .ok()?;
        let bytes = response
            .size
            .dataset
            .num_bytes_parquet_files
            .max(response.size.dataset.num_bytes_original_files);
        if bytes == 0 {
            None
        } else {
            Some((bytes / (1024 * 1024)).max(1))
        }
    }

    pub fn discover_datasets(&self) -> Result<DatasetCatalog, CoreError> {
        let mut datasets: Vec<DatasetRecord> = Vec::new();
        let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut last_error: Option<CoreError> = None;

        for query in dataset_queries() {
            let rows = match self.list_datasets(&query) {
                Ok(rows) => rows,
                Err(error) => {
                    last_error = Some(error);
                    continue;
                }
            };
            let mut kept = 0usize;
            for row in rows {
                if kept >= DATASETS_PER_CATEGORY || seen.contains(&row.id) {
                    continue;
                }
                let size = self.dataset_size_mib(&row.id);
                let record = map_dataset(&row, size, query.category);
                seen.insert(row.id.clone());
                datasets.push(record);
                kept += 1;
            }
        }

        if datasets.is_empty() {
            return Err(last_error.unwrap_or_else(|| CoreError::Runtime("HF_NO_DATASETS".into())));
        }
        Ok(DatasetCatalog {
            schema_version: 1,
            datasets,
        })
    }
}

/// Rough size (MiB) from a `size_categories:<range>` tag when the exact size is
/// unavailable. Uses the lower bound of the range and a small per-row estimate.
fn size_from_categories(tags: &[String]) -> u64 {
    let rows = tags
        .iter()
        .find_map(|tag| tag.strip_prefix("size_categories:"));
    let lower_rows = match rows {
        Some("n<1K") => 500,
        Some("1K<n<10K") => 5_000,
        Some("10K<n<100K") => 50_000,
        Some("100K<n<1M") => 500_000,
        Some("1M<n<10M") => 5_000_000,
        Some("10M<n<100M") => 50_000_000,
        Some("100M<n<1B") => 500_000_000,
        _ => 0,
    };
    // ~1 KiB/row is a deliberately rough, conservative estimate for text data.
    (lower_rows / 1024).max(if lower_rows > 0 { 1 } else { 0 })
}

fn localized_dataset_desc(
    name: &str,
    category: DatasetCategory,
    license: &str,
) -> BTreeMap<String, String> {
    use DatasetCategory::*;
    let (en, ru, tk) = match category {
        Instruction => ("instruction-tuning", "инструкций", "görkezme"),
        Chat => ("chat", "чата", "chat"),
        Code => ("code", "кода", "kod"),
        Reasoning => ("reasoning/math", "рассуждений", "pikirleniş"),
        Translation => ("translation", "перевода", "terjime"),
        Summarization => ("summarization", "суммаризации", "gysgaltma"),
        Classification => ("classification", "классификации", "klassifikasiýa"),
        Multilingual => ("multilingual", "многоязычный", "köpdilli"),
        Speech => ("speech", "речи", "ses"),
        Embeddings => ("embeddings", "эмбеддингов", "embedding"),
    };
    let mut map = BTreeMap::new();
    map.insert(
        "en".into(),
        format!("{name}: {en} dataset. License: {license}. Hosted on Hugging Face."),
    );
    map.insert(
        "ru".into(),
        format!("{name}: датасет для {ru}. Лицензия: {license}. Источник — Hugging Face."),
    );
    map.insert(
        "tk".into(),
        format!("{name}: {tk} dataseti. Ygtyýarnama: {license}. Çeşme — Hugging Face."),
    );
    map
}

fn dataset_risk(license: &str) -> DatasetRisk {
    match license.to_ascii_lowercase().as_str() {
        l if l.contains("nc") => DatasetRisk::NonCommercial,
        "mit" | "apache-2.0" | "cc0-1.0" | "bsd" => DatasetRisk::Permissive,
        "cc-by-4.0" | "cc-by-3.0" => DatasetRisk::Attribution,
        "cc-by-sa-4.0" | "cc-by-sa-3.0" => DatasetRisk::AttributionShareAlike,
        "unspecified" | "" => DatasetRisk::Unknown,
        _ => DatasetRisk::ReviewTerms,
    }
}

fn map_dataset(
    row: &HfDatasetRow,
    size_mib: Option<u64>,
    category: DatasetCategory,
) -> DatasetRecord {
    let license = parse_license(&row.tags);
    let languages: Vec<String> = row
        .tags
        .iter()
        .filter_map(|tag| tag.strip_prefix("language:").map(str::to_string))
        .take(6)
        .collect();
    let download_mib = size_mib.unwrap_or_else(|| size_from_categories(&row.tags));
    let name = row.id.split('/').next_back().unwrap_or(&row.id);
    let description = localized_dataset_desc(name, category, &license);
    DatasetRecord {
        id: sanitize_id(&row.id),
        name: row.id.split('/').next_back().unwrap_or(&row.id).to_string(),
        repo: row.id.clone(),
        revision: "main".into(),
        category,
        risk: dataset_risk(&license),
        license,
        languages,
        download_mib,
        unpacked_mib: download_mib.saturating_mul(2),
        num_examples: 0,
        description,
    }
}

/// Pick the artifact a beginner should get: prefer a balanced `Q4_K_M`, then any
/// `Q4`, otherwise the smallest single-file GGUF. Sharded GGUF (`-00001-of-`) is
/// skipped for the starter flow.
pub fn pick_gguf(files: &[HfTreeEntry]) -> Option<&HfTreeEntry> {
    let ggufs: Vec<&HfTreeEntry> = files
        .iter()
        .filter(|entry| entry.kind == "file")
        .filter(|entry| entry.path.to_ascii_lowercase().ends_with(".gguf"))
        .filter(|entry| !entry.path.to_ascii_lowercase().contains("-of-"))
        .collect();
    if ggufs.is_empty() {
        return None;
    }
    let lower = |entry: &HfTreeEntry| entry.path.to_ascii_lowercase();
    ggufs
        .iter()
        .find(|entry| lower(entry).contains("q4_k_m"))
        .or_else(|| ggufs.iter().find(|entry| lower(entry).contains("q4")))
        .copied()
        .or_else(|| ggufs.iter().min_by_key(|entry| entry.size).copied())
}

/// Read a license id from Hub `license:*` tags. Honest fallback when absent.
pub fn parse_license(tags: &[String]) -> String {
    tags.iter()
        .find_map(|tag| tag.strip_prefix("license:"))
        .map(|value| value.to_string())
        .unwrap_or_else(|| "unspecified".into())
}

/// Estimate parameter count in billions from the repo id, e.g. `...-3B-...` → 3.0,
/// `...-0.5B-...` → 0.5. Returns 0.0 when no size token is present.
pub fn parse_params_b(id: &str) -> f32 {
    let lower = id.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut best = 0.0f32;
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'b' {
            // Walk backwards over a number like `0.5`, `1.5`, `70`.
            let mut start = index;
            while start > 0 {
                let previous = bytes[start - 1];
                if previous.is_ascii_digit() || previous == b'.' {
                    start -= 1;
                } else {
                    break;
                }
            }
            if start < index {
                if let Ok(value) = lower[start..index].parse::<f32>() {
                    if (0.01..=1000.0).contains(&value) {
                        best = value;
                    }
                }
            }
        }
        index += 1;
    }
    best
}

/// Trust from the owner: official publisher orgs get a higher default.
fn trust_for(id: &str) -> TrustLevel {
    let owner = id.split('/').next().unwrap_or("").to_ascii_lowercase();
    const OFFICIAL: [&str; 8] = [
        "qwen",
        "meta-llama",
        "google",
        "mistralai",
        "microsoft",
        "deepseek-ai",
        "ggml-org",
        "openai",
    ];
    if OFFICIAL.contains(&owner.as_str()) {
        TrustLevel::OfficialPublisher
    } else {
        TrustLevel::Community
    }
}

/// Map one Hub row plus its file list into a normalized catalog entry, or `None`
/// when the repo has no usable single-file GGUF.
pub fn map_model(
    row: &HfModelRow,
    files: &[HfTreeEntry],
    category: ModelCategory,
    task: Task,
) -> Option<CatalogModel> {
    let file = pick_gguf(files)?;
    let download_mib = (file.size / (1024 * 1024)).max(1);
    let params_b = parse_params_b(&row.id);
    let quant = file
        .path
        .rsplit('/')
        .next()
        .and_then(|name| {
            name.to_ascii_uppercase().rfind("Q").map(|idx| {
                name[idx..]
                    .trim_end_matches(".GGUF")
                    .trim_end_matches(".gguf")
                    .to_string()
            })
        })
        .unwrap_or_else(|| "GGUF".into());
    let name = row.id.split('/').next_back().unwrap_or(&row.id).to_string();
    let short = row.id.clone();
    let description = localized_model_desc(&name, category, params_b, &parse_license(&row.tags));
    Some(CatalogModel {
        id: sanitize_id(&row.id),
        name,
        repo: row.id.clone(),
        revision: "main".into(),
        file: file.path.clone(),
        sha256: None,
        license: parse_license(&row.tags),
        task,
        category,
        format: ModelFormat::Gguf,
        params_b,
        quant,
        download_mib,
        // Conservative memory floors derived from the artifact size.
        min_ram_mib: download_mib.saturating_add(768),
        rec_ram_mib: download_mib.saturating_add(1536),
        context: 8192,
        trust: trust_for(&row.id),
        tags: row
            .pipeline_tag
            .iter()
            .cloned()
            .chain(std::iter::once(short))
            .collect(),
        objectives: default_objectives(params_b),
        description,
    })
}

/// Build a short, honest model-card description in all three UI languages from
/// structured metadata (never scraped prose). Populating en/ru/tk keeps every
/// catalog card readable in Russian, Turkmen and English.
fn localized_model_desc(
    name: &str,
    category: ModelCategory,
    params_b: f32,
    license: &str,
) -> BTreeMap<String, String> {
    let size = if params_b > 0.0 {
        format!("{params_b}B")
    } else {
        String::new()
    };
    let (cat_en, cat_ru, cat_tk) = model_category_words(category);
    let mut map = BTreeMap::new();
    map.insert(
        "en".into(),
        format!("{name}: {size} {cat_en} model. License: {license}. Hosted on Hugging Face.")
            .replace("  ", " "),
    );
    map.insert(
        "ru".into(),
        format!("{name}: {cat_ru}-модель {size}. Лицензия: {license}. Источник — Hugging Face.")
            .replace("  ", " "),
    );
    map.insert(
        "tk".into(),
        format!("{name}: {size} {cat_tk} model. Ygtyýarnama: {license}. Çeşme — Hugging Face.")
            .replace("  ", " "),
    );
    map
}

fn model_category_words(category: ModelCategory) -> (&'static str, &'static str, &'static str) {
    use ModelCategory::*;
    match category {
        Chat => ("chat", "чат", "chat"),
        Reasoning => ("reasoning", "рассуждающая", "pikirleniş"),
        Code => ("coding", "для кода", "kod"),
        Translation => ("translation", "перевод", "terjime"),
        Multilingual => ("multilingual", "многоязычная", "köpdilli"),
        Vision => ("vision", "зрение", "görüş"),
        SpeechRecognition => ("speech-recognition", "распознавание речи", "sesi tanamak"),
        SpeechSynthesis => ("speech-synthesis", "синтез речи", "ses sinteziniň"),
        Embeddings => ("embedding", "эмбеддинги", "embedding"),
    }
}

fn default_objectives(params_b: f32) -> Vec<Objective> {
    if params_b <= 1.6 {
        vec![
            Objective::Fastest,
            Objective::LowestRam,
            Objective::LowestDownload,
        ]
    } else if params_b <= 4.0 {
        vec![Objective::Balanced]
    } else {
        vec![Objective::BestQuality]
    }
}

fn sanitize_id(id: &str) -> String {
    id.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(path: &str, size: u64) -> HfTreeEntry {
        HfTreeEntry {
            kind: "file".into(),
            path: path.into(),
            size,
        }
    }

    #[test]
    fn parses_parameter_size_from_id() {
        assert_eq!(parse_params_b("Qwen/Qwen2.5-0.5B-Instruct-GGUF"), 0.5);
        assert_eq!(parse_params_b("Qwen/Qwen2.5-7B-Instruct-GGUF"), 7.0);
        assert_eq!(parse_params_b("bartowski/Llama-3.2-3B-Instruct-GGUF"), 3.0);
        assert_eq!(parse_params_b("some/random-embeddings-gguf"), 0.0);
    }

    #[test]
    fn prefers_q4_k_m_then_smallest() {
        let files = vec![
            entry("model-q8_0.gguf", 900),
            entry("model-q4_k_m.gguf", 400),
            entry("model-f16.gguf", 1600),
        ];
        assert_eq!(pick_gguf(&files).unwrap().path, "model-q4_k_m.gguf");

        let no_q4 = vec![entry("a-q8_0.gguf", 900), entry("b-f16.gguf", 300)];
        assert_eq!(pick_gguf(&no_q4).unwrap().path, "b-f16.gguf");
    }

    #[test]
    fn skips_sharded_and_non_gguf() {
        let files = vec![
            entry("model-00001-of-00003.gguf", 100),
            entry("readme.md", 5),
        ];
        assert!(pick_gguf(&files).is_none());
    }

    #[test]
    fn reads_license_tag() {
        let tags = vec!["gguf".to_string(), "license:apache-2.0".to_string()];
        assert_eq!(parse_license(&tags), "apache-2.0");
        assert_eq!(parse_license(&[]), "unspecified");
    }

    #[test]
    fn maps_row_to_catalog_model() {
        let row = HfModelRow {
            id: "Qwen/Qwen2.5-3B-Instruct-GGUF".into(),
            downloads: 1234,
            tags: vec!["license:apache-2.0".into()],
            pipeline_tag: Some("text-generation".into()),
        };
        let files = vec![entry(
            "qwen2.5-3b-instruct-q4_k_m.gguf",
            2_000 * 1024 * 1024,
        )];
        let model = map_model(&row, &files, ModelCategory::Chat, Task::TextGeneration).unwrap();
        assert_eq!(model.repo, "Qwen/Qwen2.5-3B-Instruct-GGUF");
        assert_eq!(model.params_b, 3.0);
        assert_eq!(model.license, "apache-2.0");
        assert_eq!(model.download_mib, 2000);
        assert_eq!(model.trust, TrustLevel::OfficialPublisher);
        assert!(model
            .download_url()
            .ends_with("qwen2.5-3b-instruct-q4_k_m.gguf"));
    }
}
