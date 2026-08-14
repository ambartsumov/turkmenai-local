# Production Readiness Report — `0.1.0`

## Identity

| Field | Value |
|---|---|
| Product | TurkmenAI Local |
| Version | `0.1.0` |
| Channel | Experimental / production-oriented release candidate foundation |
| Source commit | Recorded by `scripts/generate-build-metadata.mjs` in the release package when Git metadata is available. |
| Core privacy defaults | Telemetry OFF; cloud inference OFF; LAN sharing OFF; loopback API only. |

## Verified builds and tests

| Item | Status | Evidence |
|---|---|---|
| Linux DEB | **PASS** | Real `amd64` package produced by Tauri. |
| Linux RPM | **PASS** | Real `x86_64` package produced by Tauri. |
| Linux AppImage | **PASS** | Real x86-64 AppImage produced by Tauri. |
| Windows installer | **NOT BUILT** | Requires matching Windows CI runner and signing workflow. |
| macOS DMG | **NOT BUILT** | Requires matching macOS CI runner and notarization workflow. |
| Rust unit tests | **PASS** | 13 passing tests: core resolver/planner/store/download/runtime/state plus API runtime-gating checks. |
| TypeScript check | **PASS** | `pnpm run check` passes. |
| i18n completeness | **PASS** | 117 matching Russian, Turkmen and English keys. |
| Native Tauri check | **PASS** | `cargo check -p turkmenai-desktop` passes. |
| Website visual review | **PARTIAL** | Public site and console were manually captured at desktop viewport; mobile capture was interrupted. |

## Readiness scorecard

| Area | Status | Basis |
|---|---|---|
| Installation | PARTIAL | Real Linux packages exist; clean-machine install E2E is outstanding. |
| First-run UX | PARTIAL | A four-step local wizard and truthful runtime discovery exist; zero-config model installation is outstanding. |
| Model discovery | PARTIAL | Resolver and local analysis exist; provider browser is outstanding. |
| Model download | PARTIAL | Journal + Range + hash implementation exists; real fixture stress test is outstanding. |
| Resume / verification | PARTIAL | Unit-tested journal recovery and SHA-256 verification; 5GB interruption test is outstanding. |
| Inference | PARTIAL | Explicit local `llama-server` lifecycle and loopback chat proxy are implemented; no bundled or E2E-verified runtime/model exists. |
| Quantization / low-VRAM | PARTIAL | Planner exposes safe fallbacks; a runtime-backed workflow is outstanding. |
| Local API | PARTIAL | Loopback health, hardware, analysis, planning, runtime lifecycle and OpenAI-compatible forwarding exist; inference remains gated until a ready local runtime passes health. |
| Offline boot | PARTIAL | Core and shell do not require a startup network call; full offline model session is outstanding. |
| Security | PARTIAL | Loopback default, custom-code warning, state separation and safe runtime workspace are implemented; independent review is outstanding. |
| Localization | PARTIAL | Russian, Turkmen and English public UI strings are supplied; automated completeness/pluralization test is outstanding. |
| Website | PARTIAL | Working responsive public site and console exist; performance bundle split and automated accessibility checks are outstanding. |
| Packaging | PASS for Linux | Three real Linux artifacts and checksums are included. |
| Documentation | PASS for current scope | User, admin, developer, security, API, install and limitation guides are included. |
| CI/CD | PARTIAL | x64/ARM native matrix, artifact manifests and tag-triggered release workflow are configured; they have not run in a connected GitHub repository yet. |

## Final Linux artifacts

| Artifact | SHA-256 |
|---|---|
| `TurkmenAI Local_0.1.0_amd64.deb` | `d78f95eae562762a90cdae645d5bb346631f813bf90fd7a66813e99fb99dfef6` |
| `TurkmenAI Local-0.1.0-1.x86_64.rpm` | `eb921eabab62e7a5e97348c565e7bdc715c5c485453b6b78128aa0c11235dbca` |
| `TurkmenAI Local_0.1.0_amd64.AppImage` | `586c0962e8d1b3194eea2d91ce899f0cb7f29fb6bf915a53cee935bfba53ec23` |

## Security and privacy

No known P0/P1 defect has been identified in the implemented, tested surface. This is **not** equivalent to a completed security audit. Repository code is never automatically executed, runtime children are launched only from explicit executables and isolated workspaces, and the API binds to `127.0.0.1`. Logs, opt-in crash export, update signing, LAN permissions and plugin enforcement require further work before general availability.

## Performance and compatibility

The environment verified CPU-only hardware detection and release builds. No NVIDIA, AMD, Apple Silicon, real-model first-inference, clean-machine installer or long-running soak measurement is claimed. The Core streams file hashing and download content; it does not load a model blob into React state.

## Release decision

**Do not label `0.1.0` as stable or mass-use general availability.** It is a packaged, tested Linux technical release with an honest migration and hardening plan. The stable gate opens only after every `NOT AVAILABLE` / `PARTIAL` user-critical scorecard item has real test evidence.
