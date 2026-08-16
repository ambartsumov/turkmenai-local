# TurkmenAI Local — Production Report (v0.3.0)

Date: 2026-08-17 · Repository: https://github.com/ambartsumov/turkmenai-local ·
Site: https://turkmenai.tech

This report is evidence-backed: every claim maps to a committed change, a
published artifact, or a live URL.

---

## 1. Shipped release — v0.3.0 (verified)

- Tag `v0.3.0` published with **17 assets across 5 platforms**:
  - Linux x64 — AppImage, deb, rpm
  - Linux arm64 — AppImage, deb, rpm
  - Windows x64 — .exe (NSIS) + .msi
  - Windows arm64 — .exe (NSIS) + .msi
  - macOS arm64 — .dmg
  - plus `SHA256SUMS.txt` and per-platform manifests.
- Every checksum on the site/manifest is read from the **real published
  `SHA256SUMS.txt`**, never computed/guessed.
- **macOS Intel (x64)** is intentionally not built yet: the `macos-13` hosted
  runner repeatedly stalls in queue for hours and blocks the whole publish. It
  shows honestly as "building" on the site; to be re-added when a healthy runner
  path exists.

### Two real bugs found and fixed during release
1. **Windows bundler** — `Couldn't find a .ico icon`: `tauri.conf.json` listed
   only `icon.png`. Fixed by listing the full icon set (incl. `.ico`/`.icns`).
2. **Windows CI false-failure** — `pnpm run check` failed *only* on Windows
   because git checked out the LF-generated `product.ts` as CRLF, so the drift
   check read it as stale. Fixed with line-ending-insensitive comparison +
   `.gitattributes (eol=lf)`.

---

## 2. One product: repo ↔ website synchronization (directive §UNIFIED SYNC)

- **Single source of truth**: `metadata/product.json` (version, tagline,
  requirements, security contract), `metadata/features.json` (feature registry,
  3-language, status/since), `metadata/platforms.json`.
- `scripts/sync-metadata.mjs` propagates the ONE version into Cargo workspace,
  `package.json`, `tauri.conf.json`, and generates `client/src/generated/
  product.ts`. API health + desktop `core_version` read `env!(CARGO_PKG_VERSION)`.
- **Machine-readable, static, cacheable** endpoints shipped under
  `client/public/api/`: `product.json`, `features/`, `platforms/`, `models/`,
  `datasets/`, `releases/latest.json` (+ `index.json`), `changelog.json`,
  `product-status.json`. `latest.json` carries real sizes + SHA-256.
- **Website consumes generated data** (no hardcoded links): the download matrix
  (`client/src/releases.ts`) and the version/feature grid on the homepage are
  generated from real release assets and metadata.
- **SEO from metadata**: JSON-LD (SoftwareApplication + SoftwareSourceCode)
  inline; `sitemap.xml` / `robots.txt` regenerated from `product.website`.
- **CI gates**: `check-consistency.mjs` fails on version drift or stale
  download links (opt-in HTTP link check); a post-deploy smoke test verifies the
  live site, manifest, sitemap, robots.
- **Auto-deploy on release**: `sync-site.yml` triggers on `workflow_run(Release
  completed)` — chosen over `on: release` because a release published by
  `GITHUB_TOKEN` does not fire `on: release` — regenerates the site data and
  builds+deploys Pages inline. No manual site edits, ever.

Live evidence: `https://turkmenai.tech/api/releases/latest.json` → `version
0.3.0`, Windows x64/arm64 present.

---

## 3. Product features (directive §Models+Datasets+Download+Benchmarks)

- **Model catalog from Hugging Face**, by specialization, ranked to local
  hardware, 3-language cards. Discovered live; offline starter catalog + cache.
- **Dataset catalog from Hugging Face**, by specialization, license/risk flags,
  free-disk checks.
- **Managed llama.cpp engine** — auto-downloads the official build for the OS
  (prefers driver-free CPU), verifies and unpacks. No manual `llama-server` path.
- **Resilient downloader** — resumable HTTP Range, TCP keep-alive, exponential
  backoff + jitter, no fatal total timeout, crash-safe journal; emits live
  progress (real MB/s, interruptions survived).
- **One-click install** — user picks a model; download → SHA-256 verify →
  install → path handed to the runtime. Picks Xet transport when available, else
  the built-in downloader; falls back transparently.
- **Managed Xet transport** — detects the `hf` CLI (`hf_xet`), best-effort
  provisions it; when missing, reports `not_installed` with localized (RU/TK/EN)
  setup instructions and keeps the built-in downloader. Never a home-made slower
  replacement.
- **On-device benchmarks in the console** — real download speed vs a *modelled*
  no-resume client (advantage only claimed when interruptions actually happened —
  no invented multipliers); inference tokens/sec, time-to-first-token, RAM.

Verification: 33 core unit tests pass; workspace tests green; desktop crate
cargo-checked on Linux x64/arm64, Windows x64/arm64, macOS arm64 in CI.

---

## 4. Marketing video (Remotion)

- On-brand vertical product tour (1080×1920, ~36s) recreating the real UI in
  fast single-focus beats (headline → one UI element → one short line). Covers
  all nine capabilities. Three languages: `Tour` / `TourRU` / `TourTK`.
- `video/kaggle-render.sh` renders all three and sends them to Telegram
  (`send_telegram.py`, token from env — never committed).
- Social captions (EN/RU/TK) in `video/POSTS.md`.

---

## 5. What remains (honest)

Directive items not yet done, in rough priority:

1. **Real on-device numbers** — install a model on real hardware, run the
   download + inference benchmarks, and record actual figures. The video and any
   "N× faster" claim intentionally use **no fabricated numbers** until this
   exists.
2. **Release archive pages** — `/releases`, `/releases/vX.Y.Z` (current marked,
   history preserved).
3. **Per-item pages** — `/models/<slug>`, `/datasets/<slug>` generated from the
   catalog metadata.
4. **Training Advisor** — "what you can train on this PC" (LoRA/QLoRA) as a
   planner orchestrating Axolotl/PEFT/TRL. Currently `status: planned`.
5. **macOS Intel (x64) build** — re-add when a reliable runner path exists.
6. **Desktop REPORT BUG** prefill + richer diagnostics panel (website side done).

---

## 6. Pending user actions

- **Video**: run the two Kaggle cells in `video/README.md` (Internet on). Three
  MP4s arrive in Telegram; captions to post are in `video/POSTS.md`.
- **Rotate** any GitHub PAT / bot token that was pasted in chat.
