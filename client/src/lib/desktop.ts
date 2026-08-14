/** Design note — UI calls the native shell only through typed Core commands; no model logic exists in React. */
export type DesktopStatus = { platform: string; core_version: string; loopback_default: boolean; telemetry: boolean };
export type Hardware = { cpu: string; ram_mib: number; free_disk_mib: number; accelerators: string[]; vram_mib: number; os: string };

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
