//! CLI facade over TurkmenAI Core. Business decisions stay inside the core crate.

use std::{collections::BTreeSet, process::ExitCode};
use turkmenai_core::{
    state::AppStateStore, BackendCapabilityRegistry, ExecutionPlanner, HardwareProfile,
    ModelDoctor, ModelExecutionGraph, ModelFormat, ModelResolver, Objective,
};

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), String> {
    serde_json::to_string_pretty(value)
        .map_err(|error| error.to_string())
        .map(|text| println!("{text}"))
}

fn usage() {
    eprintln!("TurkmenAI Local CLI\n\nUsage:\n  tmai hardware\n  tmai analyze <model-source>\n  tmai validate-model <model-source>\n  tmai plan <model-source> [balanced|fastest|quality|ram|vram|download]\n  tmai doctor <model-source>\n  tmai capabilities\n  tmai migrate [state-dir]\n  tmai backup-config [state-dir]\n  tmai restore-config <backup-file> [state-dir]\n  tmai models export-list [state-dir]\n  tmai support [state-dir]\n  tmai server [port]\n\nRuntime installation and inference require an explicit verified runtime/model configuration; no repository code is executed automatically.");
}

fn state_store(argument: Option<&String>) -> Result<AppStateStore, String> {
    argument
        .map(|path| AppStateStore::open(path).map_err(|error| error.to_string()))
        .unwrap_or_else(|| AppStateStore::open_default().map_err(|error| error.to_string()))
}

fn objective(value: Option<&String>) -> Objective {
    match value.map(String::as_str) {
        Some("fastest") => Objective::Fastest,
        Some("quality") => Objective::BestQuality,
        Some("ram") => Objective::LowestRam,
        Some("vram") => Objective::LowestVram,
        Some("download") => Objective::LowestDownload,
        _ => Objective::Balanced,
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("hardware") => print_json(&HardwareProfile::detect()),
        Some("capabilities") => print_json(&BackendCapabilityRegistry::with_builtin()),
        Some("migrate") => state_store(args.get(1)).and_then(|store| store.load().map_err(|error| error.to_string())).and_then(|state| print_json(&state)),
        Some("backup-config") => state_store(args.get(1)).and_then(|store| store.backup_config().map_err(|error| error.to_string())).and_then(|path| print_json(&serde_json::json!({"backup": path}))),
        Some("restore-config") => match args.get(1) {
            Some(backup) => state_store(args.get(2))
                .and_then(|store| store.restore_config(std::path::Path::new(backup)).map_err(|error| error.to_string()))
                .and_then(|_| print_json(&serde_json::json!({"restored": true}))),
            None => Err("A backup file is required.".to_string()),
        },
        Some("models") if args.get(1).map(String::as_str) == Some("export-list") => state_store(args.get(2)).and_then(|store| store.export_inventory().map_err(|error| error.to_string())).and_then(|models| print_json(&models)),
        Some("support") => state_store(args.get(1)).and_then(|store| store.load().map_err(|error| error.to_string())).and_then(|state| print_json(&serde_json::json!({"app_state_schema": state.schema_version, "telemetry": state.settings.telemetry_enabled, "lan_sharing": state.settings.lan_sharing_enabled, "installed_models": state.models.len(), "hardware": HardwareProfile::detect()}))),
        Some("validate-model") => args.get(1).ok_or_else(|| "A model source is required.".to_string()).and_then(|source| ModelResolver::resolve(source).map_err(|error| error.to_string())).and_then(|model| print_json(&serde_json::json!({"descriptor": model, "verdict": "inspect security and license before installing"}))),
        Some("server") => {
            let port = args
                .get(1)
                .and_then(|value| value.parse::<u16>().ok())
                .unwrap_or(8742);
            println!("TurkmenAI Local API: http://127.0.0.1:{port}");
            return match tokio::runtime::Runtime::new()
                .and_then(|runtime| runtime.block_on(turkmenai_api::serve_loopback(port)))
            {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("{error}");
                    ExitCode::from(1)
                }
            };
        }
        Some("analyze") => args
            .get(1)
            .ok_or_else(|| "A model source is required.".to_string())
            .and_then(|source| ModelResolver::resolve(source).map_err(|error| error.to_string()))
            .and_then(|model| print_json(&model)),
        Some("plan") | Some("doctor") => {
            let source = match args.get(1) {
                Some(source) => source,
                None => {
                    usage();
                    return ExitCode::from(2);
                }
            };
            let model = match ModelResolver::resolve(source) {
                Ok(model) => model,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(2);
                }
            };
            let model = if model.format == ModelFormat::Unknown {
                ModelDescriptorExt::as_gguf_for_planning(model)
            } else {
                model
            };
            let registry = BackendCapabilityRegistry::with_builtin();
            let hardware = HardwareProfile::detect();
            let plans = ExecutionPlanner {
                registry: &registry,
            }
            .plan(&model, &hardware, objective(args.get(2)));
            if args[0] == "doctor" {
                let graph = ModelExecutionGraph::from_descriptor(&model);
                print_json(&ModelDoctor::diagnose(&model, &graph, plans.first()))
            } else {
                print_json(&plans)
            }
        }
        _ => {
            usage();
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

trait ModelDescriptorExt {
    fn as_gguf_for_planning(self) -> Self;
}
impl ModelDescriptorExt for turkmenai_core::ModelDescriptor {
    fn as_gguf_for_planning(mut self) -> Self {
        self.format = ModelFormat::Gguf;
        self.task = turkmenai_core::Task::TextGeneration;
        self.capabilities = BTreeSet::from([
            turkmenai_core::Capability::Text,
            turkmenai_core::Capability::Streaming,
        ]);
        self
    }
}
