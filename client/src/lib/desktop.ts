/** Design note — UI calls the native shell only through typed Core commands; no model logic exists in React. */
export type DesktopStatus = { platform: string; core_version: string; loopback_default: boolean; telemetry: boolean };
export type Hardware = { cpu: string; ram_mib: number; free_disk_mib: number; accelerators: string[]; vram_mib: number; os: string };
export type RuntimeConfig = { executable_path: string | null; model_path: string | null; port: number; context_size: number; gpu_layers: number };
export type RuntimeProcess = { id: string; backend: string; executable: string; arguments: string[]; workspace: string; state: "stopped" | "starting" | "running" | "failed"; pid: number | null; started_unix_ms: number | null; error: string | null };
export type RuntimeStatus = { loopback_only: boolean; executable_path: string | null; config: RuntimeConfig; process: RuntimeProcess | null; health: "ready" | "loading" | "unreachable" | "failed" | null };

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
