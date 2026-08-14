# Installation

TurkmenAI Local `0.1.0` is an experimental source release. Its native Linux bundle is built only on a Linux build runner; Windows and macOS bundles are produced by the included GitHub Actions workflow on their matching runners. The project does not publish placeholder installers.

For development, install Node.js 22+, pnpm 10+, Rust stable and the native prerequisites for your operating system. On Ubuntu 24.04 the documented build dependencies are `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev` and `librsvg2-dev`.

```bash
./scripts/setup
./scripts/test
./scripts/dev
```

To use the API without the desktop UI, run `cargo run -p tmai -- server 8742`. It listens on `127.0.0.1` only. An active LLM runtime is not bundled with `0.1.0`; the API correctly returns `NO_ACTIVE_RUNTIME` for inference until a verified runtime and model are installed.
