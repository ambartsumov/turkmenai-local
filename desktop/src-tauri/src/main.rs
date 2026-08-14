//! Native shell only. Resolver, hardware fit, and execution planning are owned by turkmenai-core.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use turkmenai_core::{
    BackendCapabilityRegistry, Capability, ExecutionPlanner, HardwareProfile, ModelFormat,
    ModelResolver, Objective, Task,
};

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
        core_version: "0.1.0".into(),
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hardware, desktop_status, plan])
        .run(tauri::generate_context!())
        .expect("TurkmenAI desktop shell failed to start");
}
