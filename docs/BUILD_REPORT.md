# Build Report — `0.1.0`

The following checks were run in the Linux build environment for this source revision.

| Check | Result |
|---|---|
| `pnpm run check` | Passed. |
| `pnpm run build` | Passed. |
| `cargo test --workspace` | Passed: 8 tests across core, API and runtime supervisor. |
| `cargo check -p turkmenai-desktop` | Passed. |
| `pnpm desktop:build` | Passed. |
| Debian package inspection | Passed: `turkmen-ai-local` `0.1.0`, `amd64`. |
| RPM file inspection | Passed: RPM v3.0 package. |
| AppImage file inspection | Passed: x86-64 ELF AppImage. |

## Linux artifact checksums

| Artifact | SHA-256 |
|---|---|
| `TurkmenAI Local_0.1.0_amd64.deb` | `250180b9240a24d988768679ef2bcaaa40595dcaaacc3b9cb693d9666a4cfa7a` |
| `TurkmenAI Local-0.1.0-1.x86_64.rpm` | `a960768463a79ceebd299a9632e52933470d66407c0b4db25db0a490a6b19347` |
| `TurkmenAI Local_0.1.0_amd64.AppImage` | `fce432bdb0a297a31f1bb10b3e236adf82065ba15774e7a04b8c203e32b3e4ac` |

No Windows or macOS binary was created in this Linux environment. Matching native release jobs are configured in `.github/workflows/release.yml`; an artifact should be distributed for those platforms only after that job has completed successfully.
