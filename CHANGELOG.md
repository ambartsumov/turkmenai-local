# Changelog

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
