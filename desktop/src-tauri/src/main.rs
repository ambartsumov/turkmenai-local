//! Native shell only. Resolver, hardware fit, execution planning, and isolated runtime control are owned by turkmenai-core.

use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, path::PathBuf, sync::Mutex};
use tauri::ipc::Channel;
use tauri::Manager;
use turkmenai_core::{
    bench::{inference_benchmark, InferenceBenchmark},
    catalog::{Catalog, CatalogSnapshot, Recommendation},
    datasets::{DatasetCatalog, DatasetEvaluation, DatasetSnapshot},
    download::Progress,
    install::{self, install_model, InstallRequest, InstalledModel},
    llama::{LlamaHealth, LlamaServerEndpoint},
    provision::{self, EngineState, ManagedEngine},
    runtime::{discover_llama_server, RuntimeRecord, RuntimeSupervisor},
    state::{AppStateStore, RuntimeConfig},
    transfer::{self, TransferStatus},
    BackendCapabilityRegistry, Capability, ExecutionPlanner, HardwareProfile, ModelFormat,
    ModelResolver, Objective, Task,
};

const LLAMA_RUNTIME_ID: &str = "llama-server";

struct RuntimeManager {
    supervisor: Mutex<RuntimeSupervisor>,
    state_store: AppStateStore,
}

impl RuntimeManager {
    fn open() -> Result<Self, String> {
        Ok(Self {
            supervisor: Mutex::new(RuntimeSupervisor::default()),
            state_store: AppStateStore::open_default().map_err(|error| error.to_string())?,
        })
    }

    fn saved_config(&self) -> Result<RuntimeConfig, String> {
        self.state_store
            .load()
            .map(|state| state.runtime)
            .map_err(|error| error.to_string())
    }

    fn save_config(&self, runtime: RuntimeConfig) -> Result<(), String> {
        let mut state = self.state_store.load().map_err(|error| error.to_string())?;
        state.runtime = runtime;
        self.state_store
            .save(&state)
            .map_err(|error| error.to_string())
    }
}

#[derive(Deserialize)]
struct PlanRequest {
    source: String,
    objective: Option<String>,
}

#[derive(Serialize)]
struct DesktopStatus {
    platform: String,
    core_version: String,
    loopback_default: bool,
    telemetry: bool,
}

#[derive(Serialize)]
struct RuntimeStatus {
    loopback_only: bool,
    executable_path: Option<String>,
    config: RuntimeConfig,
    process: Option<RuntimeRecord>,
    health: Option<LlamaHealth>,
    /// The managed llama.cpp engine, when the app has set one up automatically.
    engine: Option<ManagedEngine>,
    /// Whether a managed engine is ready without any manual configuration.
    engine_state: EngineState,
}

#[derive(Serialize)]
struct EngineStatus {
    state: EngineState,
    engine: Option<ManagedEngine>,
}

/// Environment that lets a managed engine binary find its bundled shared
/// libraries, using the correct variable for each platform.
fn engine_library_env(engine: &ManagedEngine) -> Vec<(String, String)> {
    let var = if cfg!(target_os = "windows") {
        "PATH"
    } else if cfg!(target_os = "macos") {
        "DYLD_LIBRARY_PATH"
    } else {
        "LD_LIBRARY_PATH"
    };
    let existing = std::env::var(var).unwrap_or_default();
    let separator = if cfg!(windows) { ";" } else { ":" };
    let value = if existing.is_empty() {
        engine.lib_dir.clone()
    } else {
        format!("{}{}{}", engine.lib_dir, separator, existing)
    };
    vec![(var.to_string(), value)]
}

fn objective(value: Option<&str>) -> Objective {
    match value {
        Some("fastest") => Objective::Fastest,
        Some("best_quality") => Objective::BestQuality,
        Some("lowest_ram") => Objective::LowestRam,
        Some("lowest_vram") => Objective::LowestVram,
        Some("lowest_download") => Objective::LowestDownload,
        _ => Objective::Balanced,
    }
}

#[tauri::command]
fn hardware() -> HardwareProfile {
    HardwareProfile::detect()
}

#[tauri::command]
fn desktop_status() -> DesktopStatus {
    DesktopStatus {
        platform: std::env::consts::OS.into(),
        core_version: "0.2.0".into(),
        loopback_default: true,
        telemetry: false,
    }
}

#[tauri::command]
fn plan(request: PlanRequest) -> Result<Vec<turkmenai_core::ExecutionPlan>, String> {
    let mut model = ModelResolver::resolve(&request.source).map_err(|error| error.to_string())?;
    if model.format == ModelFormat::Unknown {
        model.format = ModelFormat::Gguf;
        model.task = Task::TextGeneration;
        model.capabilities = BTreeSet::from([Capability::Text, Capability::Streaming]);
    }
    let registry = BackendCapabilityRegistry::with_builtin();
    Ok(ExecutionPlanner {
        registry: &registry,
    }
    .plan(
        &model,
        &HardwareProfile::detect(),
        objective(request.objective.as_deref()),
    ))
}

#[derive(Serialize)]
struct RecommendationsResult {
    source: turkmenai_core::catalog::CatalogSource,
    categories: Vec<turkmenai_core::catalog::ModelCategory>,
    recommendations: Vec<Recommendation>,
}

/// Recommend models for this computer. `refresh` pulls the live Hugging Face
/// catalog first; otherwise the cached/embedded catalog is used (offline-first).
#[tauri::command]
fn catalog_recommendations(
    objective: Option<String>,
    refresh: Option<bool>,
) -> RecommendationsResult {
    let snapshot: CatalogSnapshot = Catalog::resolve(refresh.unwrap_or(false));
    RecommendationsResult {
        source: snapshot.source,
        categories: snapshot.catalog.categories(),
        recommendations: snapshot.catalog.recommend(
            &HardwareProfile::detect(),
            self::objective(objective.as_deref()),
        ),
    }
}

/// Every catalog model with an honest fit verdict (including incompatible ones).
#[tauri::command]
fn catalog_all(refresh: Option<bool>) -> Vec<Recommendation> {
    Catalog::resolve(refresh.unwrap_or(false))
        .catalog
        .evaluate_all(&HardwareProfile::detect())
}

#[derive(Serialize)]
struct DatasetsResult {
    source: turkmenai_core::datasets::DatasetSource,
    categories: Vec<turkmenai_core::datasets::DatasetCategory>,
    datasets: Vec<DatasetEvaluation>,
}

#[tauri::command]
fn dataset_recommendations(refresh: Option<bool>) -> DatasetsResult {
    let snapshot: DatasetSnapshot = DatasetCatalog::resolve(refresh.unwrap_or(false));
    DatasetsResult {
        source: snapshot.source,
        categories: snapshot.catalog.categories(),
        datasets: snapshot.catalog.evaluate_all(&HardwareProfile::detect()),
    }
}

fn runtime_status(manager: &RuntimeManager, config: RuntimeConfig) -> RuntimeStatus {
    let engine = provision::installed_engine();
    let explicit = config.executable_path.as_ref().map(PathBuf::from);
    // Prefer an explicit user path, then a PATH-discovered binary, then the
    // managed engine the app installed for the user automatically.
    let executable_path = discover_llama_server(explicit.as_deref())
        .map(|path| path.display().to_string())
        .or_else(|| engine.as_ref().map(|engine| engine.server_path.clone()));
    let process = manager
        .supervisor
        .lock()
        .ok()
        .and_then(|mut supervisor| supervisor.refresh(LLAMA_RUNTIME_ID));
    let health = if process.is_some() || executable_path.is_some() {
        LlamaServerEndpoint::new(config.port)
            .ok()
            .map(|endpoint| endpoint.health())
    } else {
        None
    };
    let engine_state = if engine.is_some() {
        EngineState::Ready
    } else {
        EngineState::NotInstalled
    };
    RuntimeStatus {
        loopback_only: true,
        executable_path,
        config,
        process,
        health,
        engine,
        engine_state,
    }
}

/// Report whether a managed engine is already set up. Never touches the network.
#[tauri::command]
fn engine_status() -> EngineStatus {
    let engine = provision::installed_engine();
    EngineStatus {
        state: if engine.is_some() {
            EngineState::Ready
        } else {
            EngineState::NotInstalled
        },
        engine,
    }
}

/// Set up the local AI engine automatically: download and unpack the official
/// llama.cpp build for this platform. Explicit, user-triggered, no model download.
#[tauri::command]
fn engine_install() -> Result<EngineStatus, String> {
    let engine = provision::provision().map_err(|error| error.to_string())?;
    Ok(EngineStatus {
        state: EngineState::Ready,
        engine: Some(engine),
    })
}

/// Report the managed transfer engine: the built-in downloader is always ready;
/// Xet (hf CLI) is reported as ready or not_installed with setup instructions.
#[tauri::command]
fn transfer_status() -> TransferStatus {
    transfer::detect()
}

/// Best-effort, out-of-the-box provisioning of the accelerated Xet transport.
/// Never fails: on any problem the built-in downloader stays active and the
/// returned status still carries manual instructions.
#[tauri::command]
fn transfer_provision() -> TransferStatus {
    transfer::provision()
}

/// One-click model install: pick the fastest available transport, download
/// resiliently with live progress, verify the hash, return the local path and an
/// honest download benchmark. Progress ticks stream over `on_progress`.
#[tauri::command]
fn model_install(request: InstallRequest, on_progress: Channel<Progress>) -> Result<InstalledModel, String> {
    let dir = install::default_models_dir();
    install_model(&request, &dir, &mut |progress| {
        let _ = on_progress.send(progress);
    })
    .map_err(|error| error.to_string())
}

/// Benchmark a short generation against the running loopback llama-server:
/// tokens/sec, time-to-first-token and the RAM the run used.
#[tauri::command]
fn benchmark_inference(
    port: u16,
    model: String,
    prompt: Option<String>,
    max_tokens: Option<u32>,
) -> Result<InferenceBenchmark, String> {
    let endpoint = LlamaServerEndpoint::new(port).map_err(|error| error.to_string())?;
    let prompt = prompt.unwrap_or_else(|| "Write one short sentence about Turkmenistan.".into());
    inference_benchmark(&endpoint, &model, &prompt, max_tokens.unwrap_or(64))
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn runtime_discover(manager: tauri::State<'_, RuntimeManager>) -> Result<RuntimeStatus, String> {
    let config = manager.saved_config()?;
    Ok(runtime_status(&manager, config))
}

#[tauri::command]
fn runtime_start(
    config: RuntimeConfig,
    manager: tauri::State<'_, RuntimeManager>,
) -> Result<RuntimeStatus, String> {
    if config.port == 0 {
        return Err("RUNTIME_CONFIG_INVALID: port must be non-zero".into());
    }
    if !(512..=131_072).contains(&config.context_size) {
        return Err("RUNTIME_CONFIG_INVALID: context_size must be between 512 and 131072".into());
    }
    if !(0..=1000).contains(&config.gpu_layers) {
        return Err("RUNTIME_CONFIG_INVALID: gpu_layers must be between 0 and 1000".into());
    }
    let model_path = config
        .model_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| "RUNTIME_CONFIG_INVALID: model_path is required".to_string())?;
    if !model_path.is_file() {
        return Err(format!("MODEL_PATH_MISSING: {}", model_path.display()));
    }
    // Resolve the engine automatically: explicit path → PATH → managed engine,
    // and if none is present yet, set one up now so the user never has to install
    // a runtime by hand.
    let explicit = config.executable_path.as_ref().map(PathBuf::from);
    let (executable, extra_env) = match discover_llama_server(explicit.as_deref()) {
        Some(path) => (path, Vec::new()),
        None => {
            let engine =
                provision::provision().map_err(|error| format!("ENGINE_SETUP_FAILED: {error}"))?;
            let env = engine_library_env(&engine);
            (PathBuf::from(&engine.server_path), env)
        }
    };
    let mut arguments = vec![
        "--model".into(),
        model_path.display().to_string(),
        "--host".into(),
        "127.0.0.1".into(),
        "--port".into(),
        config.port.to_string(),
        "--ctx-size".into(),
        config.context_size.to_string(),
    ];
    if config.gpu_layers > 0 {
        arguments.extend(["--n-gpu-layers".into(), config.gpu_layers.to_string()]);
    }
    let mut supervisor = manager
        .supervisor
        .lock()
        .map_err(|_| "RUNTIME_STATE_LOCKED".to_string())?;
    supervisor
        .start(
            LLAMA_RUNTIME_ID,
            "llama.cpp",
            &executable,
            &arguments,
            manager.state_store.root(),
            &extra_env,
        )
        .map_err(|error| error.to_string())?;
    drop(supervisor);
    manager.save_config(config.clone())?;
    Ok(runtime_status(&manager, config))
}

#[tauri::command]
fn runtime_health(manager: tauri::State<'_, RuntimeManager>) -> Result<RuntimeStatus, String> {
    let config = manager.saved_config()?;
    Ok(runtime_status(&manager, config))
}

#[tauri::command]
fn runtime_stop(manager: tauri::State<'_, RuntimeManager>) -> Result<RuntimeStatus, String> {
    let config = manager.saved_config()?;
    let mut supervisor = manager
        .supervisor
        .lock()
        .map_err(|_| "RUNTIME_STATE_LOCKED".to_string())?;
    supervisor.stop(LLAMA_RUNTIME_ID);
    drop(supervisor);
    Ok(runtime_status(&manager, config))
}

fn main() {
    let runtime_manager =
        RuntimeManager::open().expect("TurkmenAI runtime state could not be initialized");
    tauri::Builder::default()
        .manage(runtime_manager)
        .invoke_handler(tauri::generate_handler![
            hardware,
            desktop_status,
            plan,
            catalog_recommendations,
            catalog_all,
            dataset_recommendations,
            engine_status,
            engine_install,
            transfer_status,
            transfer_provision,
            model_install,
            benchmark_inference,
            runtime_discover,
            runtime_start,
            runtime_health,
            runtime_stop
        ])
        .build(tauri::generate_context!())
        .expect("TurkmenAI desktop shell failed to start")
        .run(|app, event| {
            if matches!(event, tauri::RunEvent::Exit) {
                let manager = app.state::<RuntimeManager>();
                if let Ok(mut supervisor) = manager.supervisor.lock() {
                    supervisor.stop(LLAMA_RUNTIME_ID);
                };
            }
        });
}
