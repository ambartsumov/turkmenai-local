# FINAL TAKEOVER AUDIT — TurkmenAI Local

Date: 2026-08-15 · Branch of record: `main` @ `166ff82` · Auditor: autonomous engineering pass.

This audit classifies the repository **as found**, before the v0.2 changes on
`feat/hf-catalog-datasets`. It is evidence-based: every "WORKING" claim maps to a
test or a file that was read. Network in the working environment intermittently
blocks crates.io, npmjs and huggingface.co (GitHub reachable), so Rust/TS
compilation and live Hub calls are verified on GitHub Actions, not locally.

## Foundation

- **Stack:** Rust workspace (`turkmenai-core`, `turkmenai-api`, `apps/cli`,
  `desktop/src-tauri` = Tauri 2) + React 19 / Vite / Tailwind 4 front-end, pnpm.
- **Architecture is sound and honest:** UI holds no model-runtime logic; the core
  owns resolver, hardware profile, execution planner, content-addressed store,
  resumable downloader, versioned state; the API is loopback-only; `llama-server`
  is an explicitly-configured external sidecar; no repository code is executed.

## Component classification

| Component | Status | Evidence / note |
|---|---|---|
| Rust core: resolver, planner, graph, doctor | **WORKING** | Unit tests in `crates/turkmenai-core/src/lib.rs`. |
| Content-addressed store (SHA-256 blobs, dedup) | **WORKING** | `store.rs` tests. |
| Resumable HTTP downloader (Range, `.part`, journal, hash) | **WORKING (core)** | `download.rs` test; no beginner GUI flow. |
| Versioned app state + config backup/restore + migration | **WORKING** | `state.rs` tests. |
| Runtime supervisor (spawn/stop external llama-server) | **WORKING** | `runtime.rs` fixture-process test. |
| Loopback API (health/hardware/analyze/plan/capabilities/runtime) | **WORKING** | `turkmenai-api` tests; inference gated on ready runtime. |
| llama-server adapter (loopback health/models/chat) | **WORKING (adapter)** | `llama.rs`; requires user-provided binary. |
| CLI (`tmai`) | **WORKING** | `apps/cli`. |
| First-run wizard (language→hardware→use case→runtime) | **PARTIALLY WORKING** | No recommended-model install; **UX bug:** Continue button clipped on short windows. |
| **Model library / catalog in UI** | **MISSING** | No way to discover or pick a model — the headline gap. |
| **Dataset catalog** | **MISSING** | No dataset domain at all. |
| Managed/bundled runtime | **MISSING** | Runtime is manual-only; feels broken to non-technical users. |
| Chat UI | **MISSING** | No local chat surface. |
| Website download matrix | **SECURITY/HONESTY RISK** | `Home.tsx` hard-codes all 8 platforms as `verified` with direct links to a release that contains **Linux artifacts only** — buttons to non-existent files. |
| Release workflow (6-platform matrix, checksums) | **WORKING (config)** | `.github/workflows/release.yml`; only `v0.1.0` Linux pre-release actually published. |
| CI (quality + 6-OS native check) | **WORKING (config)** | `.github/workflows/ci.yml`. |
| Docs (README, KNOWN_LIMITATIONS, etc.) | **WORKING but drifting** | Reference features not yet built; must track v0.2. |
| Signing / notarization | **NOT AVAILABLE** | Honestly declared; needs paid credentials. |

## Release-blockers identified

1. Website presents unverified Windows/macOS downloads as verified (honesty).
2. No model catalog / no dataset catalog / no managed runtime — product cannot
   deliver "time to first inference" for a normal user.
3. `latestRelease` on GitHub is a Linux-only pre-release; the site implies more.

## v0.2 direction (this pass)

Implement the missing catalog layer the honest way, per the product brief:
discover **models and datasets from the Hugging Face Hub** on the user's machine,
group them by specialization, and rank against the local hardware; download from
Hugging Face; keep a small embedded starter set + cache for offline/blocked
networks. Then rebuild the 6-platform matrix through CI and correct the website
download matrix to derive its status from real release assets.
