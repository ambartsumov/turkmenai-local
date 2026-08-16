//! TurkmenAI Core owns model analysis, safe planning, capability negotiation,
//! and offline-first metadata. It never executes repository-supplied code.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;
use url::Url;

pub mod bench;
pub mod catalog;
pub mod datasets;
pub mod download;
pub mod huggingface;
pub mod install;
pub mod llama;
pub mod provision;
pub mod runtime;
pub mod state;
pub mod store;
pub mod transfer;

pub const MODEL_PLAN_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("the model source is not recognised: {0}")]
    UnsupportedSource(String),
    #[error("the local path does not exist: {0}")]
    MissingPath(String),
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("local runtime failed: {0}")]
    Runtime(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    HuggingFace,
    LocalFile,
    LocalDirectory,
    DirectUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelSource {
    pub kind: SourceKind,
    pub locator: String,
    pub revision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum ModelFormat {
    Gguf,
    SafeTensors,
    Pytorch,
    Onnx,
    Mlx,
    Gptq,
    Awq,
    Exl3,
    Adapter,
    Archive,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Task {
    TextGeneration,
    Embeddings,
    Reranking,
    SpeechRecognition,
    SpeechSynthesis,
    Vision,
    ImageGeneration,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Text,
    Vision,
    Audio,
    Embeddings,
    Reranking,
    ToolCalling,
    StructuredOutput,
    SpeculativeDecoding,
    Streaming,
    Lora,
    KvCacheQuantization,
    MultiGpu,
    CpuOffload,
    GpuOffload,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    OfficialPublisher,
    Community,
    Unverified,
    RequiresCode,
    VerifiedLocally,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SecurityClass {
    WeightsOnly,
    CustomCode,
    ExecutableApplication,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelFile {
    pub path: String,
    pub bytes: Option<u64>,
    pub sha256: Option<String>,
    pub format: ModelFormat,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelDescriptor {
    pub id: String,
    pub source: ModelSource,
    pub format: ModelFormat,
    pub task: Task,
    pub capabilities: BTreeSet<Capability>,
    pub files: Vec<ModelFile>,
    pub security: SecurityClass,
    pub trust: TrustLevel,
    pub warnings: Vec<String>,
    pub license: Option<String>,
}

impl ModelDescriptor {
    pub fn is_weights_only(&self) -> bool {
        self.security == SecurityClass::WeightsOnly
    }

    pub fn total_known_bytes(&self) -> u64 {
        self.files.iter().filter_map(|f| f.bytes).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeKind {
    Weights,
    Tokenizer,
    Config,
    Processor,
    Projector,
    Adapter,
    Asr,
    Tts,
    Embedding,
    Reranker,
    VectorStore,
    Runtime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphNode {
    pub id: String,
    pub kind: GraphNodeKind,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModelExecutionGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl ModelExecutionGraph {
    pub fn from_descriptor(model: &ModelDescriptor) -> Self {
        let mut nodes = vec![GraphNode {
            id: "weights".into(),
            kind: GraphNodeKind::Weights,
            required: true,
        }];
        let mut edges = Vec::new();
        let lower: Vec<String> = model
            .files
            .iter()
            .map(|file| file.path.to_lowercase())
            .collect();
        for (needle, kind, id) in [
            ("tokenizer", GraphNodeKind::Tokenizer, "tokenizer"),
            ("config", GraphNodeKind::Config, "config"),
            ("processor", GraphNodeKind::Processor, "processor"),
            ("mmproj", GraphNodeKind::Projector, "projector"),
            ("adapter", GraphNodeKind::Adapter, "adapter"),
            ("lora", GraphNodeKind::Adapter, "adapter"),
        ] {
            if lower.iter().any(|path| path.contains(needle))
                && !nodes.iter().any(|node| node.id == id)
            {
                nodes.push(GraphNode {
                    id: id.into(),
                    kind,
                    required: true,
                });
                edges.push(GraphEdge {
                    from: id.into(),
                    to: "weights".into(),
                    relation: "required_by".into(),
                });
            }
        }
        nodes.push(GraphNode {
            id: "runtime".into(),
            kind: GraphNodeKind::Runtime,
            required: true,
        });
        edges.push(GraphEdge {
            from: "weights".into(),
            to: "runtime".into(),
            relation: "loaded_by".into(),
        });
        Self { nodes, edges }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackendCapability {
    pub id: String,
    pub version: Option<String>,
    pub formats: BTreeSet<ModelFormat>,
    pub tasks: BTreeSet<Task>,
    pub features: BTreeSet<Capability>,
    pub hardware: BTreeSet<String>,
    pub limitations: Vec<String>,
    pub installed: bool,
}

impl BackendCapability {
    pub fn llama_cpp(installed: bool) -> Self {
        Self {
            id: "llama.cpp".into(),
            version: None,
            formats: BTreeSet::from([ModelFormat::Gguf]),
            tasks: BTreeSet::from([Task::TextGeneration, Task::Embeddings, Task::Vision]),
            features: BTreeSet::from([
                Capability::Text,
                Capability::Vision,
                Capability::Embeddings,
                Capability::ToolCalling,
                Capability::StructuredOutput,
                Capability::SpeculativeDecoding,
                Capability::Streaming,
                Capability::Lora,
                Capability::KvCacheQuantization,
                Capability::MultiGpu,
                Capability::CpuOffload,
                Capability::GpuOffload,
            ]),
            hardware: BTreeSet::from([
                "cpu".into(),
                "cuda".into(),
                "metal".into(),
                "vulkan".into(),
                "rocm".into(),
            ]),
            limitations: vec![
                "Version, model artifact and hardware determine actual feature availability."
                    .into(),
            ],
            installed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackendCapabilityRegistry {
    pub backends: BTreeMap<String, BackendCapability>,
}

impl BackendCapabilityRegistry {
    pub fn with_builtin() -> Self {
        let backend = BackendCapability::llama_cpp(false);
        Self {
            backends: BTreeMap::from([(backend.id.clone(), backend)]),
        }
    }

    pub fn compatible_backends(
        &self,
        model: &ModelDescriptor,
        hardware: &HardwareProfile,
    ) -> Vec<&BackendCapability> {
        self.backends
            .values()
            .filter(|backend| backend.formats.contains(&model.format))
            .filter(|backend| {
                backend.tasks.contains(&model.task) || backend.tasks.contains(&Task::TextGeneration)
            })
            .filter(|backend| {
                backend.hardware.contains("cpu")
                    || hardware
                        .accelerators
                        .iter()
                        .any(|item| backend.hardware.contains(item))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HardwareProfile {
    pub cpu: String,
    pub ram_mib: u64,
    pub free_disk_mib: u64,
    pub accelerators: BTreeSet<String>,
    pub vram_mib: u64,
    pub os: String,
    pub reserve_ram_mib: u64,
    pub reserve_vram_mib: u64,
}

impl HardwareProfile {
    pub fn detect() -> Self {
        let cpu = fs::read_to_string("/proc/cpuinfo")
            .ok()
            .and_then(|contents| {
                contents
                    .lines()
                    .find(|line| line.starts_with("model name"))
                    .and_then(|line| line.split(':').nth(1))
                    .map(str::trim)
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| std::env::consts::ARCH.into());
        let ram_mib = fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|contents| {
                contents
                    .lines()
                    .find(|line| line.starts_with("MemTotal:"))
                    .and_then(|line| line.split_whitespace().nth(1))
                    .and_then(|kb| kb.parse::<u64>().ok())
                    .map(|kb| kb / 1024)
            })
            .unwrap_or(0);
        let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let free_disk_mib = disk_free_mib(&current).unwrap_or(0);
        Self {
            cpu,
            ram_mib,
            free_disk_mib,
            accelerators: BTreeSet::from(["cpu".into()]),
            vram_mib: 0,
            os: std::env::consts::OS.into(),
            reserve_ram_mib: 1024,
            reserve_vram_mib: 512,
        }
    }

    pub fn usable_ram_mib(&self) -> u64 {
        self.ram_mib.saturating_sub(self.reserve_ram_mib)
    }
    pub fn usable_vram_mib(&self) -> u64 {
        self.vram_mib.saturating_sub(self.reserve_vram_mib)
    }
}

#[cfg(target_family = "unix")]
fn disk_free_mib(path: &Path) -> Option<u64> {
    use std::mem::MaybeUninit;
    use std::os::unix::ffi::OsStrExt;
    let bytes = path.as_os_str().as_bytes();
    let c_path = std::ffi::CString::new(bytes).ok()?;
    let mut stat = MaybeUninit::<libc_statvfs>::uninit();
    let result = unsafe { statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };
    if result != 0 {
        return None;
    }
    let stat = unsafe { stat.assume_init() };
    Some((stat.f_bavail as u64).saturating_mul(stat.f_frsize as u64) / (1024 * 1024))
}

#[cfg(not(target_family = "unix"))]
fn disk_free_mib(_path: &Path) -> Option<u64> {
    None
}

#[cfg(target_family = "unix")]
#[repr(C)]
struct libc_statvfs {
    f_bsize: u64,
    f_frsize: u64,
    f_blocks: u64,
    f_bfree: u64,
    f_bavail: u64,
    f_files: u64,
    f_ffree: u64,
    f_favail: u64,
    f_fsid: u64,
    f_flag: u64,
    f_namemax: u64,
    __f_spare: [i32; 6],
}

#[cfg(target_family = "unix")]
unsafe extern "C" {
    fn statvfs(path: *const std::ffi::c_char, buf: *mut libc_statvfs) -> i32;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Objective {
    Balanced,
    Fastest,
    BestQuality,
    LowestRam,
    LowestVram,
    LowestDownload,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Evidence {
    Estimated,
    Measured,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OffloadMode {
    Auto,
    Gpu,
    GpuRam,
    Cpu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    pub ram_mib: u64,
    pub vram_mib: u64,
    pub disk_mib: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanScore {
    pub total: u8,
    pub quality: u8,
    pub memory_safety: u8,
    pub performance: u8,
    pub simplicity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryAction {
    pub id: String,
    pub description: String,
    pub automatic: bool,
    pub score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub schema_version: u32,
    pub model_id: String,
    pub backend: String,
    pub artifact: String,
    pub context_tokens: u32,
    pub offload: OffloadMode,
    pub requirements: Requirements,
    pub score: PlanScore,
    pub evidence: Evidence,
    pub enabled_features: BTreeSet<Capability>,
    pub fallbacks: Vec<RecoveryAction>,
    pub explanation: Vec<String>,
}

pub struct ExecutionPlanner<'a> {
    pub registry: &'a BackendCapabilityRegistry,
}

impl<'a> ExecutionPlanner<'a> {
    pub fn plan(
        &self,
        model: &ModelDescriptor,
        hardware: &HardwareProfile,
        objective: Objective,
    ) -> Vec<ExecutionPlan> {
        let artifact_mib = (model.total_known_bytes() / (1024 * 1024)).max(1);
        let estimated_ram = artifact_mib.saturating_add(1024);
        let compatible = self.registry.compatible_backends(model, hardware);
        compatible.into_iter().map(|backend| {
            let has_accelerator = hardware.accelerators.iter().any(|item| item != "cpu" && backend.hardware.contains(item));
            let offload = if has_accelerator { OffloadMode::GpuRam } else { OffloadMode::Cpu };
            let vram = if has_accelerator { artifact_mib.min(hardware.usable_vram_mib()) } else { 0 };
            let memory_ok = estimated_ram <= hardware.usable_ram_mib() && artifact_mib <= hardware.free_disk_mib;
            let mut quality = 72u8;
            let mut performance: u8 = if has_accelerator { 72 } else { 42 };
            if objective == Objective::BestQuality { quality = 88; }
            if objective == Objective::Fastest { performance = performance.saturating_add(12); }
            let memory_safety = if memory_ok { 92 } else { 35 };
            let simplicity = if backend.installed { 92 } else { 70 };
            let total = ((quality as u16 + performance as u16 + memory_safety as u16 + simplicity as u16) / 4) as u8;
            let mut enabled_features = model.capabilities.intersection(&backend.features).cloned().collect::<BTreeSet<_>>();
            if !has_accelerator { enabled_features.remove(&Capability::GpuOffload); }
            let mut fallbacks = vec![
                RecoveryAction { id: "lower_context".into(), description: "Reduce context before changing model files.".into(), automatic: true, score: 94 },
                RecoveryAction { id: "cpu_offload".into(), description: "Increase CPU/RAM offload if runtime supports it.".into(), automatic: true, score: 87 },
                RecoveryAction { id: "smaller_prebuilt_quant".into(), description: "Choose an existing compatible smaller quantization variant.".into(), automatic: false, score: 82 },
                RecoveryAction { id: "cpu_only".into(), description: "Run CPU-only as a safe lower-performance fallback.".into(), automatic: false, score: 61 },
            ];
            if !memory_ok { fallbacks.sort_by(|a, b| b.score.cmp(&a.score)); }
            ExecutionPlan {
                schema_version: MODEL_PLAN_SCHEMA_VERSION,
                model_id: model.id.clone(), backend: backend.id.clone(), artifact: format_name(&model.format),
                context_tokens: if memory_ok { 8192 } else { 4096 }, offload,
                requirements: Requirements { ram_mib: estimated_ram, vram_mib: vram, disk_mib: artifact_mib.saturating_add(256) },
                score: PlanScore { total, quality, memory_safety, performance, simplicity },
                evidence: Evidence::Estimated, enabled_features, fallbacks,
                explanation: vec![
                    format!("{} format is compatible with {}.", format_name(&model.format), backend.id),
                    if memory_ok { "The plan fits the configured RAM and disk reserve.".into() } else { "The plan exceeds a configured resource reserve; apply a ranked fallback.".into() },
                ],
            }
        }).collect()
    }
}

fn format_name(format: &ModelFormat) -> String {
    serde_json::to_value(format)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .unwrap_or_else(|| "unknown".into())
}

pub struct ModelResolver;

impl ModelResolver {
    pub fn resolve(input: &str) -> Result<ModelDescriptor, CoreError> {
        if let Some((repo, revision)) = parse_huggingface(input) {
            return Ok(descriptor_from_source(
                ModelSource {
                    kind: SourceKind::HuggingFace,
                    locator: repo,
                    revision,
                },
                ModelFormat::Unknown,
                Vec::new(),
                SecurityClass::Unknown,
            ));
        }
        if let Ok(url) = Url::parse(input) {
            if url.scheme() == "http" || url.scheme() == "https" {
                let format = format_from_path(url.path());
                return Ok(descriptor_from_source(
                    ModelSource {
                        kind: SourceKind::DirectUrl,
                        locator: input.into(),
                        revision: None,
                    },
                    format,
                    Vec::new(),
                    SecurityClass::Unknown,
                ));
            }
        }
        let path = Path::new(input);
        if path.is_file() {
            return Self::resolve_file(path);
        }
        if path.is_dir() {
            return Self::resolve_directory(path);
        }
        Err(CoreError::MissingPath(input.into()))
    }

    pub fn resolve_file(path: &Path) -> Result<ModelDescriptor, CoreError> {
        let metadata = fs::metadata(path)?;
        let format = format_from_path(&path.to_string_lossy());
        let security = security_from_name(&path.to_string_lossy());
        let file = ModelFile {
            path: file_name(path),
            bytes: Some(metadata.len()),
            sha256: sha256_file(path).ok(),
            format: format.clone(),
            required: true,
        };
        Ok(descriptor_from_source(
            ModelSource {
                kind: SourceKind::LocalFile,
                locator: path.display().to_string(),
                revision: None,
            },
            format,
            vec![file],
            security,
        ))
    }

    pub fn resolve_directory(path: &Path) -> Result<ModelDescriptor, CoreError> {
        let mut files = Vec::new();
        let mut security = SecurityClass::WeightsOnly;
        let mut primary = ModelFormat::Unknown;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let child = entry.path();
            let format = format_from_path(&child.to_string_lossy());
            if primary == ModelFormat::Unknown && format != ModelFormat::Unknown {
                primary = format.clone();
            }
            let child_security = security_from_name(&child.to_string_lossy());
            if child_security != SecurityClass::WeightsOnly {
                security = child_security;
            }
            files.push(ModelFile {
                path: file_name(&child),
                bytes: Some(entry.metadata()?.len()),
                sha256: sha256_file(&child).ok(),
                format,
                required: true,
            });
        }
        Ok(descriptor_from_source(
            ModelSource {
                kind: SourceKind::LocalDirectory,
                locator: path.display().to_string(),
                revision: None,
            },
            primary,
            files,
            security,
        ))
    }
}

fn descriptor_from_source(
    source: ModelSource,
    format: ModelFormat,
    files: Vec<ModelFile>,
    security: SecurityClass,
) -> ModelDescriptor {
    let id = source
        .locator
        .rsplit('/')
        .next()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("model")
        .trim_end_matches(".gguf")
        .to_string();
    let task = match format {
        ModelFormat::Gguf
        | ModelFormat::SafeTensors
        | ModelFormat::Gptq
        | ModelFormat::Awq
        | ModelFormat::Exl3 => Task::TextGeneration,
        ModelFormat::Onnx => Task::Unknown,
        _ => Task::Unknown,
    };
    let capabilities = if task == Task::TextGeneration {
        BTreeSet::from([Capability::Text, Capability::Streaming])
    } else {
        BTreeSet::new()
    };
    let trust = if security == SecurityClass::WeightsOnly {
        TrustLevel::Community
    } else {
        TrustLevel::RequiresCode
    };
    let warnings = match security { SecurityClass::WeightsOnly => vec![], _ => vec!["This source contains code or executable content. TurkmenAI will not execute it automatically.".into()] };
    ModelDescriptor {
        id,
        source,
        format,
        task,
        capabilities,
        files,
        security,
        trust,
        warnings,
        license: None,
    }
}

fn parse_huggingface(input: &str) -> Option<(String, Option<String>)> {
    let trimmed = input.trim().trim_end_matches('/');
    let candidate = if let Ok(url) = Url::parse(trimmed) {
        if url.host_str()? != "huggingface.co" {
            return None;
        }
        url.path()
            .trim_start_matches('/')
            .split('/')
            .take(2)
            .collect::<Vec<_>>()
            .join("/")
    } else {
        trimmed.to_owned()
    };
    let (repo, revision) = candidate
        .split_once('@')
        .map(|(repo, rev)| (repo.to_string(), Some(rev.to_string())))
        .unwrap_or((candidate, None));
    if repo.split('/').count() == 2 && !repo.contains(' ') {
        Some((repo, revision))
    } else {
        None
    }
}

fn format_from_path(path: &str) -> ModelFormat {
    let value = path.to_ascii_lowercase();
    if value.ends_with(".gguf") {
        ModelFormat::Gguf
    } else if value.ends_with(".safetensors") {
        ModelFormat::SafeTensors
    } else if value.ends_with(".onnx") {
        ModelFormat::Onnx
    } else if value.ends_with(".pt") || value.ends_with(".pth") || value.ends_with(".bin") {
        ModelFormat::Pytorch
    } else if value.ends_with(".gptq") {
        ModelFormat::Gptq
    } else if value.ends_with(".awq") {
        ModelFormat::Awq
    } else if value.ends_with(".exl3") {
        ModelFormat::Exl3
    } else if value.ends_with(".zip")
        || value.ends_with(".tar")
        || value.ends_with(".tar.gz")
        || value.ends_with(".tmai")
    {
        ModelFormat::Archive
    } else {
        ModelFormat::Unknown
    }
}

fn security_from_name(path: &str) -> SecurityClass {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".py")
        || lower.ends_with(".sh")
        || lower.ends_with(".ps1")
        || lower.ends_with(".bat")
    {
        SecurityClass::CustomCode
    } else if lower.ends_with(".exe") || lower.ends_with(".msi") || lower.ends_with(".appimage") {
        SecurityClass::ExecutableApplication
    } else {
        SecurityClass::WeightsOnly
    }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .and_then(|item| item.to_str())
        .unwrap_or("file")
        .to_owned()
}

pub fn sha256_file(path: &Path) -> Result<String, CoreError> {
    use std::io::Read;
    let mut file = fs::File::open(path)?;
    let mut buffer = [0_u8; 64 * 1024];
    let mut hasher = Sha256::new();
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckState {
    Pass,
    Warning,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorCheck {
    pub id: String,
    pub state: CheckState,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDoctorReport {
    pub model_id: String,
    pub checks: Vec<DoctorCheck>,
    pub recovery: Vec<RecoveryAction>,
}

pub struct ModelDoctor;
impl ModelDoctor {
    pub fn diagnose(
        model: &ModelDescriptor,
        graph: &ModelExecutionGraph,
        plan: Option<&ExecutionPlan>,
    ) -> ModelDoctorReport {
        let mut checks = vec![
            DoctorCheck {
                id: "security".into(),
                state: if model.is_weights_only() {
                    CheckState::Pass
                } else {
                    CheckState::Warning
                },
                message: if model.is_weights_only() {
                    "No executable repository content was detected in the analyzed local input."
                        .into()
                } else {
                    "Code or executable content was detected and remains blocked from automatic execution.".into()
                },
            },
            DoctorCheck {
                id: "dependency_graph".into(),
                state: if graph
                    .nodes
                    .iter()
                    .any(|node| node.kind == GraphNodeKind::Weights)
                {
                    CheckState::Pass
                } else {
                    CheckState::Fail
                },
                message: "Model graph includes a weights node.".into(),
            },
        ];
        if let Some(plan) = plan {
            let status = if plan.score.memory_safety >= 60 {
                CheckState::Pass
            } else {
                CheckState::Warning
            };
            checks.push(DoctorCheck {
                id: "hardware_fit".into(),
                state: status,
                message: plan.explanation.join(" "),
            });
        } else {
            checks.push(DoctorCheck {
                id: "execution_plan".into(),
                state: CheckState::Fail,
                message: "No compatible execution plan was produced.".into(),
            });
        }
        let recovery = plan.map(|item| item.fallbacks.clone()).unwrap_or_default();
        ModelDoctorReport {
            model_id: model.id.clone(),
            checks,
            recovery,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parses_huggingface_url_with_revision() {
        let model =
            ModelResolver::resolve("https://huggingface.co/example/tiny-gguf@abc123").unwrap();
        assert_eq!(model.source.kind, SourceKind::HuggingFace);
        assert_eq!(model.source.locator, "example/tiny-gguf");
        assert_eq!(model.source.revision.as_deref(), Some("abc123"));
    }

    #[test]
    fn detects_custom_code_without_execution() {
        let dir = tempfile::tempdir().unwrap();
        let script = dir.path().join("model.py");
        fs::write(&script, "print('never run')").unwrap();
        let model = ModelResolver::resolve_directory(dir.path()).unwrap();
        assert_eq!(model.security, SecurityClass::CustomCode);
        assert_eq!(model.trust, TrustLevel::RequiresCode);
    }

    #[test]
    fn calculates_hash_streaming() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tiny.gguf");
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(b"turkmenai").unwrap();
        assert_eq!(
            sha256_file(&path).unwrap(),
            "57037d4948167c58d8f075721c3e1d9eee46ca308af674e6476ce1e5aba15d5c"
        );
    }

    #[test]
    fn planner_emits_safe_fallbacks() {
        let model = ModelResolver::resolve("owner/example").unwrap();
        let registry = BackendCapabilityRegistry::with_builtin();
        let hardware = HardwareProfile {
            cpu: "test".into(),
            ram_mib: 8192,
            free_disk_mib: 10240,
            accelerators: BTreeSet::from(["cpu".into()]),
            vram_mib: 0,
            os: "linux".into(),
            reserve_ram_mib: 1024,
            reserve_vram_mib: 0,
        };
        let plans = ExecutionPlanner {
            registry: &registry,
        }
        .plan(
            &ModelDescriptor {
                format: ModelFormat::Gguf,
                ..model
            },
            &hardware,
            Objective::Balanced,
        );
        assert_eq!(plans.len(), 1);
        assert!(plans[0]
            .fallbacks
            .iter()
            .any(|item| item.id == "lower_context" && item.automatic));
    }
}
