# Changelog

## 0.3.0 — 2026-08-16

From control plane to a product you can actually run — and one product across
repo, app and website.

- **One-click model install**: pick a model in the catalog and the app does the
  rest — resilient download with a live progress bar (real MB/s, reconnect count
  on unstable links), SHA-256 verification, and the local model path handed to
  the runtime so it is one step from chat. Nothing installs itself without your
  pick.
- **On-device benchmarks, in the console**: real download speed vs a *modelled*
  no-resume client on the same link (the resume advantage is only claimed when
  interruptions actually happened — no invented multipliers), plus inference
  tokens/sec, time-to-first-token and RAM used for the model on your hardware.
- **Managed Xet transport**: the app detects Hugging Face Xet (hf CLI + hf_xet)
  and best-effort provisions it for accelerated downloads; when unavailable it
  says so honestly with localized setup steps and keeps using the always-on
  built-in downloader. Never a home-made slower Xet replacement.
- **One product, one source of truth**: `metadata/*.json` drives the version
  (Cargo/npm/Tauri), features and platforms; the website consumes generated
  `/api/*.json` incl. `releases/latest.json` with real sizes and SHA-256; SEO
  (JSON-LD, sitemap, robots) is generated; CI fails on version drift or stale
  download links and the site auto-updates on release. No manual site edits.
- **Windows bundles fixed**: the NSIS/MSI bundler now finds the `.ico` icon, so
  Windows x64/ARM64 installers build again.

## 0.2.0 — 2026-08-15

Adds the product surface the first release was missing, the honest way.

- **Model catalog from Hugging Face, by specialization** (chat, reasoning, code,
  translation, embeddings, vision, speech). Discovered live on the user's machine,
  ranked against the local hardware (RAM/VRAM/disk) with honest fit verdicts;
  incompatible models are hidden behind an explicit view. Three-language cards
  (RU/TK/EN). A small embedded starter catalog + cache keep it useful offline.
- **Dataset catalog from Hugging Face, by specialization**, with license/risk
  flags and free-disk checks.
- **Managed runtime**: the llama.cpp engine sets itself up automatically — the app
  downloads the official build for the platform (`.tar.gz` on Linux/macOS, `.zip`
  on Windows), prefers the driver-free CPU build, verifies and unpacks it. No more
  manual `llama-server` path.
- **Resilient downloader** for slow/unstable links: no fatal total timeout, TCP
  keep-alive, resumable HTTP Range, exponential backoff + jitter, progress-based
  failure reset, total-size awareness, crash-safe journaling.
- Console gains **Models**, **Datasets** and a real **API** section; first-run
  priority now drives recommendations.
- Public site cleaned of template preview runtime (index.html 369 kB → 2.3 kB, no
  injected telemetry); download matrix reflects real release assets; 404 restyled.

Still pre-1.0: unsigned builds, no first-run auto model install, no LAN sharing,
no updater. See `KNOWN_LIMITATIONS.md`.

## 0.1.0 — 2026-08-14

This initial production-oriented release introduces a Tauri desktop shell, a multilingual public site and local console, safe model-source resolution, capability-driven execution planning, Model Doctor, SHA-256 content storage, resumable download journal, loopback API, versioned app state, backup/restore configuration commands, Linux DEB/RPM/AppImage packaging, CI workflows and release documentation.

It does **not** ship a verified model runtime or claim first-run inference, automatic model installation, signed cross-platform releases, updater, LAN sharing, plugin system or general availability.
