# Final Launch Audit — TurkmenAI Local `0.1.0`

**Audit date:** 2026-08-14  
**Decision:** **Not eligible for a stable general-availability launch.** The project is a tested, production-oriented Linux technical release with a working public website and a real local runtime integration boundary. It must remain pre-1.0 until user-critical flows have end-to-end evidence.

## Evidence examined

The audit reviewed the Rust workspace, Tauri shell, local API, React website and console, release scripts and workflow, local production folder, current git state, and available sandbox runtime files. `pnpm run check` passed with **117** keys in English, Russian and Turkmen; `pnpm run build` passed; `cargo test --workspace` passed with **13** tests; and `cargo fmt --all -- --check` passed after the runtime integration changes. The local environment contains neither a `llama-server` executable nor a `.gguf` model, so it cannot truthfully demonstrate first inference.

## COMPLETE

| Surface | Status | Evidence |
|---|---|---|
| Core resolver, planner, Model Doctor, content store, resumable Range downloader, state migration and runtime supervisor | Implemented and unit-tested | Rust workspace tests; source in `crates/turkmenai-core/` |
| Loopback API safety | Implemented and tested | API binds to `127.0.0.1`; chat returns `409 NO_ACTIVE_RUNTIME` until a ready local runtime is verified |
| Local `llama-server` integration boundary | Implemented, not E2E-proven | Explicit executable/model paths, isolated child process, `/health` gate, model/chat forwarding, lifecycle API and Tauri commands |
| Desktop first-run status UI | Implemented | Language, hardware, use-case and truthful runtime-readiness steps; runtime setup controls appear only in Tauri |
| Localization | Implemented and checked | English default; Russian and Turkmen dictionaries have matching 117-key sets |
| Public site | Published | `https://turkmenai-lxyvv4qu.manus.space`; responsive first-run Console captured from the real web build |
| Linux installer evidence | Previously built and checksummed | Real DEB, RPM and AppImage records in [Production Readiness Report](./PRODUCTION_READINESS_REPORT.md) |
| Release CI source | Implemented | Native x64/ARM matrix, artifact manifests, checksums and tag-triggered GitHub Release workflow |
| Core documentation | Present | Architecture, security, limitations, build, platform and runtime documents are in the repository and production package |

## INCOMPLETE

| Required public flow | Current state | Release condition |
|---|---|---|
| Zero-config first install | First-run UI exists; model installation is not zero-config | Real recommended-model selection, download, verification and launch flow |
| Model browser and Hugging Face selection | Core resolver supports analysis; no finished public desktop model-browser UI | Implement and test a provider/browser flow using real metadata |
| Download-to-chat path | Downloader and lifecycle primitives exist; no full GUI orchestration | E2E download/resume/hash/install/runtime/chat test using a legal model |
| Local chat and model switching | API forwarding exists, but no verified ready `llama-server`/GGUF session or chat UI | Test on a real runtime and surface real conversations in the app |
| Windows, Linux ARM64 and macOS artifacts | CI build matrix is configured; no artifacts have been built or verified in this audit | Successful runner builds, artifact inspection and platform-specific install smoke tests |
| Installer clean-machine validation | Not performed | Install, launch, shutdown and restart each verified package on clean supported OSes |
| Website release links | Linux links are release-package references, not GitHub Release assets | Publish the repository and verified release assets, then point downloads to them |
| Real desktop screenshot set | One public Console screenshot has been captured; requested product-flow screenshots cannot be captured truthfully | Capture only actual running UI after the missing flows exist |
| Marketing carousel/video | Not started | Use only actual product screenshots and implemented capabilities |
| Custom domain | Domain registered; hosting binding is not enabled on current plan | Add domain through platform settings, use platform-issued records, then verify HTTPS |

## BROKEN

| Finding | Impact | Repair plan |
|---|---|---|
| `pnpm run release:check` currently fails after a clean target directory because it correctly requires a freshly built native bundle | Expected pre-packaging failure, not an application defect | Run `pnpm desktop:build` before the release check; CI does so in that order |
| Existing release workflow’s asset glob may not be portable in all shells | Could prevent automatic GitHub Release upload | Replace the glob with an explicit `find`-based upload list before tagging |
| The prior report and limitations describe the pre-integration runtime state | Could understate newly implemented lifecycle code | Update the public documentation to say “implemented but not E2E-verified,” never “available” |

## MOCK

| Surface | Assessment |
|---|---|
| Website/Console runtime indicator | **Not mock.** It reports native discovery/health only when Tauri is present and otherwise states that browser runtime control is unavailable. |
| Windows/macOS download affordances | **Not binary downloads.** They must stay labelled as CI targets until real artifacts exist. |
| Console model/chat product screens | **Absent, not represented as working.** The product must not generate or publish screenshots implying those flows currently exist. |
| Recommendations and execution plans | Planner outputs are genuine logic; they are not evidence of a model having run. |

## NEEDS CREDENTIAL

No supplied credential is required for the code audit. The connected GitHub CLI may be used only for the authorized `ambartsumov/turkmenai-local` repository action; no user-supplied personal access token will be used or stored. Optional signing/notarization requires separate provider secrets and is not requested for this pre-1.0 release.

## NEEDS USER ACTION

| Action | Exact place | Why |
|---|---|---|
| Unlock custom-domain access, if desired | Project **Settings → Domains** | The platform must issue the exact apex and `www` records; they must not be guessed |
| Confirm DNS changes immediately before save | Namecheap **Domain List → turkmenai.tech → Advanced DNS** | Remove the existing redirect and replace it only with the platform-issued records |
| Revoke the GitHub PAT previously pasted in chat | GitHub account security settings | It was exposed in conversation and must be treated as compromised; it is not used by this project |
| Provide code-signing credentials only if signed installers are required | Secure provider secret store | Windows signing and Apple notarization cannot be truthfully claimed otherwise |

## BLOCKED BY ENVIRONMENT

| Blocker | Consequence |
|---|---|
| No `llama-server` executable or GGUF model in the sandbox | No truthful local inference smoke test, chat screenshot or runtime performance measurement |
| Linux-only local build environment | Windows and macOS packages cannot be locally built or installed here |
| Custom-domain feature unavailable on the present plan | `turkmenai.tech` remains registered but not connected; current public URL remains valid |
| No published GitHub repository/release at audit time | GitHub Release URLs and release-asset download buttons cannot yet be real |

## Release recommendation

Keep the current version as **`0.1.0` experimental / pre-1.0**. Do not create a `v1.0.0` tag until the incomplete user-critical install, inference, cross-platform and security gates have verifiable evidence. The repository may still be published as a transparent technical release once the real Linux artifacts and release notes are recreated from the final revision.
