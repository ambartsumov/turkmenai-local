//! Local API surface shared by the desktop shell, CLI and future web control panel.
//! Default binding is loopback only; LAN exposure is deliberately outside this crate.

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use turkmenai_core::{
    BackendCapabilityRegistry, ExecutionPlanner, HardwareProfile, ModelResolver, Objective,
};

#[derive(Clone)]
pub struct ApiState {
    registry: Arc<BackendCapabilityRegistry>,
}

impl Default for ApiState {
    fn default() -> Self {
        Self {
            registry: Arc::new(BackendCapabilityRegistry::with_builtin()),
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
        .route("/v1/models", get(openai_models))
        .route("/v1/chat/completions", post(no_runtime))
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
    let hardware = HardwareProfile::detect();
    let plans = ExecutionPlanner {
        registry: &state.registry,
    }
    .plan(
        &model,
        &hardware,
        parse_objective(request.objective.as_deref()),
    );
    Ok(Json(plans))
}

async fn openai_models() -> Json<serde_json::Value> {
    Json(serde_json::json!({"object":"list","data":[]}))
}
async fn no_runtime() -> AppError {
    AppError(StatusCode::CONFLICT, "NO_ACTIVE_RUNTIME".into(), "No verified runtime and READY model are active. Install and smoke-test a compatible model before requesting inference.".into())
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
}
