# TurkmenAI Local

> **Local AI. Built for Turkmenistan.**

TurkmenAI Local is an offline-first desktop control plane for local AI. It analyzes a model source without executing repository code, observes the local hardware profile, produces ranked execution plans, stores model content by SHA-256, and exposes a loopback-only API. The project keeps a strict distinction between **implemented behaviour**, **estimated plans**, and **future integrations**.

## What works in `0.1.0`

| Area | Available now | Deliberate boundary |
|---|---|---|
| Model resolution | Local paths, Hugging Face-style IDs and URLs, basic format/task classification, custom-code risk state | Does not execute model repository code. |
| Planning | Capability registry, hardware fit, ranked safe fallbacks, explainable scores | Scores are estimates until a local benchmark is recorded. |
| Storage and delivery | SHA-256 content store, atomic ingest, persistent download journal, Range-capable HTTP downloader | No bundled model or runtime is shipped. |
| API | `127.0.0.1:8742` health, hardware, analyze, plan, capability and OpenAI-compatible model-list endpoints | Inference returns `NO_ACTIVE_RUNTIME` until a verified runtime and READY model exist. |
| Desktop | Tauri shell with native hardware command and local-console UI | It is not a cloud proxy and does not silently enable LAN access. |

## Quick start

```bash
./scripts/setup
./scripts/test
./scripts/dev
```

To use the local API without opening the desktop shell:

```bash
cargo run -p tmai -- server 8742
curl http://127.0.0.1:8742/api/v1/health
```

## Developer commands

| Command | Purpose |
|---|---|
| `./scripts/setup` | Installs JavaScript dependencies and fetches Rust packages. |
| `./scripts/test` | Runs TypeScript, Rust workspace and native desktop checks. |
| `./scripts/build` | Builds web assets and release-mode Rust binaries. |
| `./scripts/package` | Produces real native packages on the current supported platform. |
| `./scripts/clean` | Removes local build output only; it never removes model data. |
| `tmai hardware` | Prints the detected hardware profile as JSON. |
| `tmai analyze <source>` | Resolves a source without executing it. |
| `tmai plan <source>` | Produces ranked execution plans. |
| `tmai doctor <source>` | Reports compatibility and trust findings. |
| `tmai server [port]` | Starts the loopback-only HTTP API. |

## Repository guide

| Path | Responsibility |
|---|---|
| `crates/turkmenai-core/` | Resolver, execution graph, planner, Model Doctor, store, download journal and runtime supervisor. |
| `crates/turkmenai-api/` | Shared native localhost API contract. |
| `apps/cli/` | `tmai` command-line tool. |
| `desktop/src-tauri/` | Native desktop shell and packaging configuration. |
| `client/` | Public site and desktop console user interface. |
| `registry/` | Backend capability manifests. |
| `schemas/` | Versioned JSON contracts. |
| `docs/` | Architecture, security, API and release documentation. |
| `branding/` | Logo variants, generated PNG master and brand guide. |

## Privacy and safety defaults

The service binds to `127.0.0.1`. Cloud inference, telemetry and LAN sharing are off by default. Model repositories are treated as untrusted inputs; the resolver records custom code as a risk rather than running it. Content storage uses immutable SHA-256 blobs. Read [Security Model](docs/SECURITY.md) before enabling any future runtime or connector.

## Releases

The local build produced real Linux packages in `target/release/bundle/`. Windows and macOS packages are built only on their matching runners through [the release workflow](.github/workflows/release.yml). See [Installation](docs/INSTALLATION.md), [Release Checklist](docs/RELEASE_CHECKLIST.md), and [Third-Party Notices](THIRD_PARTY_NOTICES.md).

## License

The TurkmenAI Local source in this repository is licensed under the [MIT License](LICENSE). Third-party components retain their own licenses and notices.
