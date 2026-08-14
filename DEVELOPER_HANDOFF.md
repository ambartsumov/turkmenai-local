# Developer Handoff

TurkmenAI Local uses a separable architecture. `turkmenai-core` owns resolver, hardware fit, planning, Model Doctor, content storage, download journal, runtime supervision and versioned local state. `turkmenai-api` owns the localhost HTTP contract. `apps/cli` is a thin facade. `desktop/src-tauri` exposes native commands without duplicating planner decisions. `client` contains the public website and desktop console.

## Build and test

Run `./scripts/setup`, then `./scripts/test`. A native Linux package requires WebKitGTK/GTK packages listed in `docs/INSTALLATION.md`. Build with `./scripts/package`; only distribute artifacts created by that command or matching-platform GitHub Actions releases.

## Upstreams and licensing

The selected integration reference is Jan at the revision recorded in `docs/FOUNDATION_DECISION.md`. It remains outside the build root to avoid accidental inclusion. The product source is licensed under Apache-2.0; consult `THIRD_PARTY_NOTICES.md` before adding or redistributing any external component or model.

## Maintenance rule

Keep model artifact content separate from app state, and keep app state versioned. A migration must be atomic, preserve model records, create a prior-state backup and have an explicit recovery path. Do not expose experimental backend/runtime features to ordinary users until both a real integration and matching test evidence exist.
