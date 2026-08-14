/** Design note — UI calls the native shell only through typed Core commands; no model logic exists in React. */
export type DesktopStatus = { platform: string; core_version: string; loopback_default: boolean; telemetry: boolean };
export type Hardware = { cpu: string; ram_mib: number; free_disk_mib: number; accelerators: string[]; vram_mib: number; os: string };
export type RuntimeConfig = { executable_path: string | null; model_path: string | null; port: number; context_size: number; gpu_layers: number };
export type RuntimeProcess = { id: string; backend: string; executable: string; arguments: string[]; workspace: string; state: "stopped" | "starting" | "running" | "failed"; pid: number | null; started_unix_ms: number | null; error: string | null };
export type EngineState = "not_installed" | "ready";
export type ManagedEngine = { backend: string; version: string; server_path: string; lib_dir: string };
export type EngineStatus = { state: EngineState; engine: ManagedEngine | null };
export type RuntimeStatus = { loopback_only: boolean; executable_path: string | null; config: RuntimeConfig; process: RuntimeProcess | null; health: "ready" | "loading" | "unreachable" | "failed" | null; engine: ManagedEngine | null; engine_state: EngineState };

function inTauri() { return "__TAURI_INTERNALS__" in window; }

export async function getDesktopStatus(): Promise<DesktopStatus | null> {
  if (!inTauri()) return null;
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<DesktopStatus>("desktop_status");
}

export async function getHardware(): Promise<Hardware | null> {
  if (!inTauri()) return null;
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<Hardware>("hardware");
}

async function invokeRuntime<T>(command: string, args?: Record<string, unknown>): Promise<T | null> {
  if (!inTauri()) return null;
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(command, args);
}

export function discoverRuntime(): Promise<RuntimeStatus | null> {
  return invokeRuntime<RuntimeStatus>("runtime_discover");
}

export function startRuntime(config: RuntimeConfig): Promise<RuntimeStatus | null> {
  return invokeRuntime<RuntimeStatus>("runtime_start", { config });
}

export function getRuntimeHealth(): Promise<RuntimeStatus | null> {
  return invokeRuntime<RuntimeStatus>("runtime_health");
}

export function stopRuntime(): Promise<RuntimeStatus | null> {
  return invokeRuntime<RuntimeStatus>("runtime_stop");
}

export function getEngineStatus(): Promise<EngineStatus | null> {
  return invokeRuntime<EngineStatus>("engine_status");
}

/** Set up the local AI engine automatically (downloads llama.cpp for this OS). */
export function installEngine(): Promise<EngineStatus | null> {
  return invokeRuntime<EngineStatus>("engine_install");
}

// ---- Catalog (models) & datasets, discovered from Hugging Face -------------

export type FitLevel = "excellent" | "good" | "usable" | "slow" | "unsupported";
export type ModelCategory = "chat" | "reasoning" | "code" | "translation" | "multilingual" | "vision" | "speech_recognition" | "speech_synthesis" | "embeddings";
export type CatalogSource = "remote" | "cache" | "builtin";

export type CatalogModel = { id: string; name: string; repo: string; revision: string; file: string; sha256: string | null; license: string; task: string; category: ModelCategory; format: string; params_b: number; quant: string; download_mib: number; min_ram_mib: number; rec_ram_mib: number; context: number; trust: string; tags: string[]; description: Record<string, string> };
export type Recommendation = { model: CatalogModel; fit: FitLevel; download_url: string; estimated_ram_mib: number; fits_disk: boolean; gpu_accelerated: boolean; reasons: string[] };
export type RecommendationsResult = { source: CatalogSource; categories: ModelCategory[]; recommendations: Recommendation[] };

export type DatasetFit = "fits" | "tight" | "unsupported";
export type DatasetCategory = "instruction" | "chat" | "code" | "reasoning" | "translation" | "summarization" | "classification" | "multilingual" | "speech" | "embeddings";
export type DatasetRecord = { id: string; name: string; repo: string; revision: string; category: DatasetCategory; license: string; languages: string[]; download_mib: number; unpacked_mib: number; num_examples: number; risk: string; description: Record<string, string> };
export type DatasetEvaluation = { dataset: DatasetRecord; fit: DatasetFit; required_disk_mib: number; page_url: string; reasons: string[] };
export type DatasetsResult = { source: CatalogSource; categories: DatasetCategory[]; datasets: DatasetEvaluation[] };

export function getCatalogRecommendations(objective?: string, refresh = false): Promise<RecommendationsResult | null> {
  return invokeRuntime<RecommendationsResult>("catalog_recommendations", { objective: objective ?? null, refresh });
}

export function getCatalogAll(refresh = false): Promise<Recommendation[] | null> {
  return invokeRuntime<Recommendation[]>("catalog_all", { refresh });
}

export function getDatasetRecommendations(refresh = false): Promise<DatasetsResult | null> {
  return invokeRuntime<DatasetsResult>("dataset_recommendations", { refresh });
}
