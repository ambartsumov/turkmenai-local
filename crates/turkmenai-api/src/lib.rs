//! Local API surface shared by the desktop shell, CLI and future web control panel.
//! Default binding is loopback only; LAN exposure and cloud inference are deliberately outside this crate.

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use turkmenai_core::{
    catalog::Catalog,
    llama::{LlamaHealth, LlamaServerEndpoint},
    runtime::{discover_llama_server, RuntimeRecord, RuntimeSupervisor},
    state::{default_data_root, RuntimeConfig},
    BackendCapabilityRegistry, ExecutionPlanner, HardwareProfile, ModelResolver, Objective,
};

const LLAMA_RUNTIME_ID: &str = "llama-server";

struct ApiRuntime {
    supervisor: RuntimeSupervisor,
    endpoint: Option<LlamaServerEndpoint>,
    config: Option<RuntimeConfig>,
}

impl Default for ApiRuntime {
    fn default() -> Self {
        Self {
            supervisor: RuntimeSupervisor::default(),
            endpoint: None,
            config: None,
        }
    }
}

#[derive(Clone)]
pub struct ApiState {
    registry: Arc<BackendCapabilityRegistry>,
    runtime: Arc<Mutex<ApiRuntime>>,
}

impl Default for ApiState {
    fn default() -> Self {
        Self {
            registry: Arc::new(BackendCapabilityRegistry::with_builtin()),
            runtime: Arc::new(Mutex::new(ApiRuntime::default())),
        }
    }
}

#[derive(Debug, Serialize)]
struct ApiError {
    error: ErrorBody,
}
#[derive(Debug, Serialize)]
struct ErrorBody {
    code: String,
    message: String,
}

struct AppError(StatusCode, String, String);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(ApiError {
                error: ErrorBody {
                    code: self.1,
                    message: self.2,
                },
            }),
        )
            .into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub source: String,
}
#[derive(Debug, Deserialize)]
pub struct PlanRequest {
    pub source: String,
    pub objective: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct RuntimeActivationRequest {
    pub port: u16,
}

#[derive(Debug, Serialize)]
struct RuntimeStatusResponse {
    loopback_only: bool,
    active: bool,
    config: Option<RuntimeConfig>,
    process: Option<RuntimeRecord>,
    health: Option<LlamaHealth>,
}

fn parse_objective(value: Option<&str>) -> Objective {
    match value {
        Some("fastest") => Objective::Fastest,
        Some("best_quality") => Objective::BestQuality,
        Some("lowest_ram") => Objective::LowestRam,
        Some("lowest_vram") => Objective::LowestVram,
        Some("lowest_download") => Objective::LowestDownload,
        _ => Objective::Balanced,
    }
}

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/api/v1/health", get(health))
        .route("/api/v1/hardware", get(hardware))
        .route("/api/v1/capabilities", get(capabilities))
        .route("/api/v1/analyze", post(analyze))
        .route("/api/v1/plan", post(plan))
        .route("/api/v1/catalog", get(catalog))
        .route(
            "/api/v1/catalog/recommendations",
            get(catalog_recommendations),
        )
        .route("/api/v1/datasets", get(datasets))
        .route(
            "/api/v1/datasets/recommendations",
            get(dataset_recommendations),
        )
        .route(
            "/api/v1/runtime",
            get(runtime_status).post(activate_external_runtime),
        )
        .route("/api/v1/runtime/status", get(runtime_status))
        .route("/api/v1/runtime/start", post(start_runtime))
        .route("/api/v1/runtime/stop", post(stop_runtime))
        .route("/v1/models", get(openai_models))
        .route("/v1/chat/completions", post(chat_completions))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(
        serde_json::json!({"status":"ok","mode":"localhost","telemetry":"off","cloud_inference":"off","lan_sharing":"off","version":"0.1.0"}),
    )
}
async fn hardware() -> Json<HardwareProfile> {
    Json(HardwareProfile::detect())
}
async fn capabilities(State(state): State<ApiState>) -> Json<BackendCapabilityRegistry> {
    Json((*state.registry).clone())
}

#[derive(Debug, Deserialize)]
struct CatalogQuery {
    objective: Option<String>,
    /// When true, pull the live catalog from the repository before ranking.
    #[serde(default)]
    refresh: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct RecommendationsResponse {
    source: turkmenai_core::catalog::CatalogSource,
    categories: Vec<turkmenai_core::catalog::ModelCategory>,
    recommendations: Vec<turkmenai_core::catalog::Recommendation>,
}

async fn catalog(
    Query(query): Query<CatalogQuery>,
) -> Json<turkmenai_core::catalog::CatalogSnapshot> {
    // A non-refresh call must never touch the network (offline-first).
    Json(Catalog::resolve(query.refresh))
}

async fn catalog_recommendations(
    Query(query): Query<CatalogQuery>,
) -> Json<RecommendationsResponse> {
    let hardware = HardwareProfile::detect();
    let objective = parse_objective(query.objective.as_deref());
    let refresh = query.refresh;
    let snapshot = tokio::task::spawn_blocking(move || Catalog::resolve(refresh))
        .await
        .unwrap_or_else(|_| turkmenai_core::catalog::CatalogSnapshot {
            source: turkmenai_core::catalog::CatalogSource::Builtin,
            catalog: Catalog::builtin(),
        });
    Json(RecommendationsResponse {
        source: snapshot.source,
        categories: snapshot.catalog.categories(),
        recommendations: snapshot.catalog.recommend(&hardware, objective),
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct DatasetsResponse {
    source: turkmenai_core::datasets::DatasetSource,
    categories: Vec<turkmenai_core::datasets::DatasetCategory>,
    datasets: Vec<turkmenai_core::datasets::DatasetEvaluation>,
}

async fn datasets(
    Query(query): Query<CatalogQuery>,
) -> Json<turkmenai_core::datasets::DatasetSnapshot> {
    Json(turkmenai_core::datasets::DatasetCatalog::resolve(
        query.refresh,
    ))
}

async fn dataset_recommendations(Query(query): Query<CatalogQuery>) -> Json<DatasetsResponse> {
    let hardware = HardwareProfile::detect();
    let snapshot = tokio::task::spawn_blocking(move || {
        turkmenai_core::datasets::DatasetCatalog::resolve(query.refresh)
    })
    .await
    .unwrap_or_else(|_| turkmenai_core::datasets::DatasetSnapshot {
        source: turkmenai_core::datasets::DatasetSource::Builtin,
        catalog: turkmenai_core::datasets::DatasetCatalog::builtin(),
    });
    Json(DatasetsResponse {
        source: snapshot.source,
        categories: snapshot.catalog.categories(),
        datasets: snapshot.catalog.evaluate_all(&hardware),
    })
}

async fn analyze(
    Json(request): Json<AnalyzeRequest>,
) -> Result<Json<turkmenai_core::ModelDescriptor>, AppError> {
    ModelResolver::resolve(&request.source)
        .map(Json)
        .map_err(|error| {
            AppError(
                StatusCode::BAD_REQUEST,
                "MODEL_SOURCE_INVALID".into(),
                error.to_string(),
            )
        })
}

async fn plan(
    State(state): State<ApiState>,
    Json(request): Json<PlanRequest>,
) -> Result<Json<Vec<turkmenai_core::ExecutionPlan>>, AppError> {
    let mut model = ModelResolver::resolve(&request.source).map_err(|error| {
        AppError(
            StatusCode::BAD_REQUEST,
            "MODEL_SOURCE_INVALID".into(),
            error.to_string(),
        )
    })?;
    if model.format == turkmenai_core::ModelFormat::Unknown {
        model.format = turkmenai_core::ModelFormat::Gguf;
        model.task = turkmenai_core::Task::TextGeneration;
        model.capabilities = std::collections::BTreeSet::from([
            turkmenai_core::Capability::Text,
            turkmenai_core::Capability::Streaming,
        ]);
    }
    let plans = ExecutionPlanner {
        registry: &state.registry,
    }
    .plan(
        &model,
        &HardwareProfile::detect(),
        parse_objective(request.objective.as_deref()),
    );
    Ok(Json(plans))
}

fn no_active_runtime() -> AppError {
    AppError(StatusCode::CONFLICT, "NO_ACTIVE_RUNTIME".into(), "No verified local runtime and READY model are active. Start llama-server on loopback and wait for GET /health to report ready.".into())
}

fn runtime_snapshot(
    state: &ApiState,
) -> Result<
    (
        Option<RuntimeConfig>,
        Option<LlamaServerEndpoint>,
        Option<RuntimeRecord>,
    ),
    AppError,
> {
    let mut runtime = state.runtime.lock().map_err(|_| {
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "RUNTIME_STATE_LOCKED".into(),
            "Could not read local runtime state.".into(),
        )
    })?;
    let process = runtime.supervisor.refresh(LLAMA_RUNTIME_ID);
    Ok((runtime.config.clone(), runtime.endpoint.clone(), process))
}

async fn runtime_status(
    State(state): State<ApiState>,
) -> Result<Json<RuntimeStatusResponse>, AppError> {
    let (config, endpoint, process) = runtime_snapshot(&state)?;
    let health = match endpoint {
        Some(endpoint) => Some(
            tokio::task::spawn_blocking(move || endpoint.health())
                .await
                .map_err(|error| {
                    AppError(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "RUNTIME_TASK_FAILED".into(),
                        error.to_string(),
                    )
                })?,
        ),
        None => None,
    };
    let active = health == Some(LlamaHealth::Ready);
    Ok(Json(RuntimeStatusResponse {
        loopback_only: true,
        active,
        config,
        process,
        health,
    }))
}

fn validate_runtime_config(config: &RuntimeConfig) -> Result<(PathBuf, PathBuf), AppError> {
    if config.port == 0 {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "RUNTIME_CONFIG_INVALID".into(),
            "port must be non-zero".into(),
        ));
    }
    if !(512..=131_072).contains(&config.context_size) {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "RUNTIME_CONFIG_INVALID".into(),
            "context_size must be between 512 and 131072".into(),
        ));
    }
    if !(0..=1000).contains(&config.gpu_layers) {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "RUNTIME_CONFIG_INVALID".into(),
            "gpu_layers must be between 0 and 1000".into(),
        ));
    }
    let model = config
        .model_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            AppError(
                StatusCode::BAD_REQUEST,
                "RUNTIME_CONFIG_INVALID".into(),
                "model_path is required".into(),
            )
        })?;
    if !model.is_file() {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "MODEL_PATH_MISSING".into(),
            model.display().to_string(),
        ));
    }
    let explicit = config.executable_path.as_ref().map(PathBuf::from);
    let executable = discover_llama_server(explicit.as_deref()).ok_or_else(|| {
        AppError(
            StatusCode::BAD_REQUEST,
            "LLAMA_SERVER_NOT_FOUND".into(),
            "Select a verified local llama-server executable or add it to PATH.".into(),
        )
    })?;
    Ok((model, executable))
}

async fn activate_external_runtime(
    State(state): State<ApiState>,
    Json(request): Json<RuntimeActivationRequest>,
) -> Result<Json<RuntimeStatusResponse>, AppError> {
    let endpoint = LlamaServerEndpoint::new(request.port).map_err(|error| {
        AppError(
            StatusCode::BAD_REQUEST,
            "RUNTIME_CONFIG_INVALID".into(),
            error.to_string(),
        )
    })?;
    let endpoint_for_check = endpoint.clone();
    let health = tokio::task::spawn_blocking(move || endpoint_for_check.health())
        .await
        .map_err(|error| {
            AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "RUNTIME_TASK_FAILED".into(),
                error.to_string(),
            )
        })?;
    if health != LlamaHealth::Ready {
        return Err(AppError(
            StatusCode::CONFLICT,
            "RUNTIME_NOT_READY".into(),
            format!("The local llama-server health state is {health:?}."),
        ));
    }
    {
        let mut runtime = state.runtime.lock().map_err(|_| {
            AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "RUNTIME_STATE_LOCKED".into(),
                "Could not update local runtime state.".into(),
            )
        })?;
        runtime.endpoint = Some(endpoint);
        runtime.config = Some(RuntimeConfig {
            port: request.port,
            ..RuntimeConfig::default()
        });
    }
    runtime_status(State(state)).await
}

async fn start_runtime(
    State(state): State<ApiState>,
    Json(config): Json<RuntimeConfig>,
) -> Result<Json<RuntimeStatusResponse>, AppError> {
    let (model, executable) = validate_runtime_config(&config)?;
    let workspace = default_data_root();
    std::fs::create_dir_all(&workspace).map_err(|error| {
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "RUNTIME_WORKSPACE_FAILED".into(),
            error.to_string(),
        )
    })?;
    let mut arguments = vec![
        "--model".into(),
        model.display().to_string(),
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
    let endpoint = LlamaServerEndpoint::new(config.port).map_err(|error| {
        AppError(
            StatusCode::BAD_REQUEST,
            "RUNTIME_CONFIG_INVALID".into(),
            error.to_string(),
        )
    })?;
    {
        let mut runtime = state.runtime.lock().map_err(|_| {
            AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "RUNTIME_STATE_LOCKED".into(),
                "Could not start local runtime.".into(),
            )
        })?;
        runtime
            .supervisor
            .start(
                LLAMA_RUNTIME_ID,
                "llama.cpp",
                &executable,
                &arguments,
                &workspace,
                &[],
            )
            .map_err(|error| {
                AppError(
                    StatusCode::CONFLICT,
                    "RUNTIME_START_FAILED".into(),
                    error.to_string(),
                )
            })?;
        runtime.config = Some(config);
        runtime.endpoint = Some(endpoint);
    }
    runtime_status(State(state)).await
}

async fn stop_runtime(
    State(state): State<ApiState>,
) -> Result<Json<RuntimeStatusResponse>, AppError> {
    {
        let mut runtime = state.runtime.lock().map_err(|_| {
            AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "RUNTIME_STATE_LOCKED".into(),
                "Could not stop local runtime.".into(),
            )
        })?;
        runtime.supervisor.stop(LLAMA_RUNTIME_ID);
        runtime.endpoint = None;
    }
    runtime_status(State(state)).await
}

async fn ready_runtime(state: &ApiState) -> Result<LlamaServerEndpoint, AppError> {
    let (_, endpoint, _) = runtime_snapshot(state)?;
    let endpoint = endpoint.ok_or_else(no_active_runtime)?;
    let endpoint_for_check = endpoint.clone();
    let health = tokio::task::spawn_blocking(move || endpoint_for_check.health())
        .await
        .map_err(|error| {
            AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "RUNTIME_TASK_FAILED".into(),
                error.to_string(),
            )
        })?;
    if health == LlamaHealth::Ready {
        Ok(endpoint)
    } else {
        Err(no_active_runtime())
    }
}

async fn openai_models(State(state): State<ApiState>) -> Result<Json<serde_json::Value>, AppError> {
    let endpoint = ready_runtime(&state).await?;
    let response = tokio::task::spawn_blocking(move || endpoint.models())
        .await
        .map_err(|error| {
            AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "RUNTIME_TASK_FAILED".into(),
                error.to_string(),
            )
        })?
        .map_err(|_error| no_active_runtime())?;
    Ok(Json(response))
}

async fn chat_completions(
    State(state): State<ApiState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, AppError> {
    let endpoint = ready_runtime(&state).await?;
    let response = tokio::task::spawn_blocking(move || endpoint.chat(&payload))
        .await
        .map_err(|error| {
            AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "RUNTIME_TASK_FAILED".into(),
                error.to_string(),
            )
        })?
        .map_err(|_error| no_active_runtime())?;
    Ok(Json(response))
}

pub async fn serve_loopback(port: u16) -> Result<(), std::io::Error> {
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, router(ApiState::default())).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_is_private_by_default() {
        let response = router(ApiState::default())
            .oneshot(Request::get("/api/v1/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn catalog_recommendations_never_include_unsupported() {
        let response = router(ApiState::default())
            .oneshot(
                Request::get("/api/v1/catalog/recommendations?objective=balanced")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = axum::body::to_bytes(response.into_body(), 1 << 20)
            .await
            .unwrap();
        let body: RecommendationsResponse = serde_json::from_slice(&bytes).unwrap();
        assert!(body
            .recommendations
            .iter()
            .all(|rec| rec.fit != turkmenai_core::catalog::FitLevel::Unsupported));
    }

    #[tokio::test]
    async fn runtime_activation_rejects_non_ready_loopback() {
        let response = router(ApiState::default())
            .oneshot(
                Request::post("/api/v1/runtime")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"port":1}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn chat_refuses_requests_without_verified_runtime() {
        let response = router(ApiState::default())
            .oneshot(
                Request::post("/v1/chat/completions")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"messages":[]}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}
