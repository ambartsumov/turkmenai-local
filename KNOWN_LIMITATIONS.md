# Known Limitations — `0.1.0`

This document is user-facing. A limitation here is not hidden behind a marketing claim.

| Capability | Status | What that means |
|---|---|---|
| Native installers | **Partial** | Linux DEB, RPM and AppImage were built locally. Windows and macOS jobs are configured in CI but were not built in this Linux environment. |
| First-run wizard | **Partial** | A local language → hardware → use case → runtime-readiness wizard is implemented. Automatic recommended-model installation is not implemented. |
| Inference | **Partial** | The shell and loopback API can start an explicit local `llama-server` and proxy only after its health check is ready. No verified runtime or model is bundled, so `/v1/chat/completions` returns `NO_ACTIVE_RUNTIME` until a user configures and verifies one. |
| Model download | **Partial** | A resumable HTTP Range downloader, SHA-256 verification and journal exist in Core. A beginner GUI install flow and real 5GB network fixture validation are not complete. |
| Model lifecycle | **Partial** | State types cover transactional install states; full GUI/runtime orchestration through all states is not complete. |
| Local API | **Partial** | Loopback health, hardware, analysis, planning, capability and runtime lifecycle endpoints are verified. Inference requires a locally configured, ready runtime and still lacks an E2E model smoke test. |
| Storage migration | **Partial** | Versioned app state and configuration backup/restore are verified. A full SQLite/chat/workspace migration system is not yet implemented. |
| LAN sharing, plugins, updates | **Not available** | These remain disabled rather than exposed without authentication, permissions and security review. |
| GPU matrix | **Not verified** | CPU fallback was tested in this environment. NVIDIA, AMD and Apple runtime execution remain release blockers for claims about those paths. |
| Signing and notarization | **Not available locally** | No false certificate or notarization claim is made. CI release jobs are the intended credential-injected path. |

The stable `1.0.0` gate remains closed until first-run model installation, verified inference, clean-machine packaging, downloader stress tests, update/recovery flows, cross-platform runtime tests and independent security review have passed.
