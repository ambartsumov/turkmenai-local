# Release Package Contents

The delivery folder is named `TURKMENAI_LOCAL_RELEASE`. It is deliberately separated into source, documentation, branding, installers and build evidence.

| Directory | Contents |
|---|---|
| `source/` | Buildable TurkmenAI Local source code, tests, workflows and documentation; dependency caches and build output are excluded. |
| `installers/linux/` | The real Debian, RPM and AppImage artifacts built for this revision. |
| `branding/` | Generated transparent PNG mark, social image, deterministic SVG variants and brand guide. |
| `docs/` | User manual, administrator guide, architecture, API, security, installation and release checklist. |
| `checksums.txt` | SHA-256 values for only the included Linux installer artifacts. |

The package intentionally does not contain renamed files or placeholder installers for Windows or macOS. Native release jobs for those platforms are configured in `.github/workflows/release.yml` and must produce their own artifacts on matching runners.
