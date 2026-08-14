# TurkmenAI Local

> **Local AI. Built for Turkmenistan.**

TurkmenAI Local is an **offline-first desktop control plane** for discovering compatible local AI models, selecting an explainable execution plan, managing verified model files and exposing a private loopback API. It is designed so that complex compatibility logic stays inside the product while the public-facing contract stays direct: **download once, work locally, and never silently send inference to the cloud.**

**Website:** [turkmenai-lxyvv4qu.manus.space](https://turkmenai-lxyvv4qu.manus.space)  
**Release channel:** `0.1.0` experimental / pre-1.0  
**Launch status:** Read [FINAL_LAUNCH_AUDIT.md](FINAL_LAUNCH_AUDIT.md) before treating this as a general-availability installer.

## What is verified in `0.1.0`

| Surface | Implemented behaviour | Intentional boundary |
|---|---|---|
| Model resolution | Local paths, Hugging Face-style IDs and URLs, basic format/task classification, custom-code risk state | Repository code is never executed automatically. |
| Planning | Capability registry, hardware fit, ranked safe fallbacks and explainable scores | Scores remain estimates until a local benchmark is recorded. |
| Storage and delivery | SHA-256 content store, atomic ingest, persistent download journal and Range-capable HTTP downloader | No model weights are bundled. |
| Runtime integration | Explicit local `llama-server` discovery, validated executable/model paths, isolated sidecar process and loopback health gate | No runtime or GGUF model is bundled; end-to-end inference needs a real local model. |
| Local API | Health, hardware, analysis, planning, capabilities, runtime lifecycle and OpenAI-compatible `/v1/*` forwarding | Chat returns `409 NO_ACTIVE_RUNTIME` until an activated runtime reports ready. |
| Desktop UX | Tauri shell, native hardware profile, English-first first-run wizard, Russian and Turkmen UI | Model library, beginner download flow and chat UI are not finished. |
| Privacy | `127.0.0.1` default binding; telemetry, cloud inference and LAN sharing off | LAN sharing, plugins and auto-update are deliberately not exposed. |

## Product evidence

The optimized Linux desktop binary has been launched in an isolated display session and the resulting home and first-run Console screenshots are recorded in [Screenshot Evidence](docs/SCREENSHOT_EVIDENCE.md). No generated screenshot is used to imply an unimplemented model, chat or download flow.

## Quick start for contributors

```bash
./scripts/setup
./scripts/test
./scripts/dev
```

To start the loopback API without opening the desktop shell:

```bash
cargo run -p tmai -- server 8742
curl http://127.0.0.1:8742/api/v1/health
```

The API stays private by default. A local runtime must be explicitly configured with an existing `llama-server` executable and GGUF path; it is never downloaded or enabled automatically.

## Runtime contract

The desktop shell and API use `llama-server` only as an explicit local sidecar. The process is started in an isolated workspace, forced onto `127.0.0.1`, and checked through `GET /health`. A healthy runtime is required before either `GET /v1/models` or `POST /v1/chat/completions` will forward a request.

| Endpoint | Purpose |
|---|---|
| `GET /api/v1/health` | Confirms the TurkmenAI Local loopback service. |
| `GET /api/v1/hardware` | Returns a local hardware profile. |
| `POST /api/v1/analyze` | Parses a model source without running repository code. |
| `POST /api/v1/plan` | Produces ranked, explainable execution plans. |
| `GET /api/v1/runtime/status` | Returns local runtime discovery/process/health state. |
| `POST /api/v1/runtime/start` | Starts an explicitly configured `llama-server` sidecar. |
| `POST /api/v1/runtime/stop` | Stops the supervised local sidecar. |
| `POST /v1/chat/completions` | Forwards only to a verified ready loopback runtime. |

See [First Run and Runtime](docs/FIRST_RUN_AND_RUNTIME.md) and [API documentation](docs/API.md) for the complete contract.

## Installation and release policy

The current final local build produced real Linux x64 packages:

| Artifact | Current local verification |
|---|---|
| AppImage | SHA-256 verified after local Tauri build. |
| DEB | SHA-256 verified after local Tauri build. |
| RPM | SHA-256 verified after local Tauri build. |
| Windows x64 / ARM64 | CI target only; no artifact is claimed until its runner build and install test pass. |
| Linux ARM64 | CI target only; no artifact is claimed until its runner build and install test pass. |
| macOS Intel / Apple Silicon | CI target only; no artifact is claimed until its runner build and install test pass. |

Run these checks after packaging on Linux:

```bash
pnpm desktop:build
pnpm run release:check
RELEASE_RUNNER=linux-x64 RELEASE_MANIFEST_PATH=release-manifest/linux-x64.json node scripts/generate-release-manifest.mjs
SHA256SUMS_PATH=release-manifest/linux-x64-SHA256SUMS.txt node scripts/generate-sha256sums.mjs
```

GitHub releases are intentionally **not** linked until a repository and real release assets exist. The tag workflow packages each matching runner, generates JSON manifests and `SHA256SUMS.txt`, then creates a release only after all matrix jobs pass.

## Architecture

```text
React public site / desktop console
                │ typed Tauri commands
                ▼
Tauri desktop shell ──────► turkmenai-core
                                   │ resolver · planner · downloader · store · state
                                   ▼
                           isolated llama-server sidecar
                                   │ 127.0.0.1 only
                                   ▼
                       OpenAI-compatible loopback API
```

Detailed component boundaries are documented in [ARCHITECTURE.md](ARCHITECTURE.md).

## Repository guide

| Path | Responsibility |
|---|---|
| `crates/turkmenai-core/` | Resolver, planning, Model Doctor, store, download journal, runtime and state. |
| `crates/turkmenai-api/` | Shared loopback HTTP and OpenAI-compatible API surface. |
| `apps/cli/` | `tmai` command-line tool. |
| `desktop/src-tauri/` | Native shell, lifecycle commands and packaging configuration. |
| `client/` | Public site and desktop Console user interface. |
| `docs/` | Runtime, platform, security, evidence and implementation documents. |
| `branding/` | Logo variants and brand guide. |
| `.github/workflows/` | Quality, native matrix and tagged-release automation. |

## Development commands

| Command | Purpose |
|---|---|
| `./scripts/setup` | Installs JavaScript dependencies and fetches Rust packages. |
| `./scripts/test` | Runs the project’s primary TypeScript, Rust and native checks. |
| `pnpm run check` | Runs TypeScript and localization-completeness checks. |
| `cargo test --workspace` | Runs Core, API, CLI and desktop crate tests. |
| `pnpm desktop:build` | Builds native artifacts for the current host platform. |
| `pnpm run release:check` | Verifies expected native release artifacts and prints SHA-256 values. |
| `tmai hardware` | Prints detected hardware as JSON. |
| `tmai analyze <source>` | Resolves a source without executing it. |
| `tmai plan <source>` | Produces ranked local execution plans. |
| `tmai doctor <source>` | Reports compatibility and trust findings. |
| `tmai server [port]` | Starts the private loopback HTTP API. |

## Security, privacy and licensing

TurkmenAI Local binds to `127.0.0.1`; cloud inference, telemetry and LAN sharing are OFF by default. Model repositories are untrusted inputs, custom code is recorded as risk information rather than executed, and model content is stored as immutable SHA-256 blobs. Read [SECURITY.md](SECURITY.md), [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md) and [REQUIRED_CREDENTIALS.md](REQUIRED_CREDENTIALS.md) before publication.

TurkmenAI Local source is available under the [MIT License](LICENSE). Upstream runtime and model licenses remain separate; see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

## Contributing

Contributions are welcome when they preserve the product’s privacy-first and evidence-first contract. Start with [CONTRIBUTING.md](CONTRIBUTING.md), run the full test suite, and do not claim a model, platform or integration as working without reproducible proof.
