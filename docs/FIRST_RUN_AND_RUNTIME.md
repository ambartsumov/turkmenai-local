# First-Run and Runtime Contract

## First-run flow

The desktop client opens in English on a fresh install. The wizard is local-only and persists only a small preference record in the application’s browser storage:

1. **Language.** English, Russian or Turkmen. The selected language drives the desktop UI and can be changed later.
2. **Hardware report.** The UI reads a local hardware profile from the native shell. It presents detected memory and operating system without uploading it.
3. **Use case.** The user selects balanced, speed, quality, low RAM or low download.
4. **Runtime readiness.** The app discovers only a locally installed, explicitly configured `llama-server` executable. It never downloads or executes model-repository scripts. The desktop Console can persist explicit local executable/model paths, a loopback port, context size and GPU-layer preference; it validates those paths before starting a sidecar.
5. **Plan preview.** The app asks the Core planner for a compatible execution plan. A missing runtime results in an actionable local-installation message, not a fabricated chat service.

## Runtime boundary

`llama-server` is treated as a local sidecar. The product validates that the executable and the selected model file are explicit local paths, starts it inside an isolated workspace, and only enables chat when a loopback health request succeeds. The client does not include weights or call a cloud endpoint. Runtime processes receive an isolated environment and bind to `127.0.0.1`; LAN listening is not offered by the Console.

The integration uses the documented `GET /health` endpoint. A `200` response with `{"status":"ok"}` marks a model as ready; a `503` means that model loading is still in progress. This allows the desktop app to expose a truthful readiness state before opening inference controls. [1]

The local API exposes `GET /api/v1/runtime/status`, `POST /api/v1/runtime/start`, `POST /api/v1/runtime/stop`, and the compatibility activation route `POST /api/v1/runtime`. The OpenAI-compatible `GET /v1/models` and `POST /v1/chat/completions` routes forward only to an activated and ready loopback `llama-server`. Otherwise they return `409 NO_ACTIVE_RUNTIME`; there is no synthetic answer or remote fallback.

## Failure behavior

Runtime detection, startup, health checks and model loading expose stable, user-facing state: **not installed**, **not configured**, **starting**, **ready** or **failed**. Failure keeps the planner and diagnostic interface available and does not block app preferences or model inventory.

## References

[1]: https://github.com/ggml-org/llama.cpp/blob/master/tools/server/README.md "llama.cpp HTTP Server — health endpoint"
