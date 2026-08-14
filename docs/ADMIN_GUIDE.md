# Administrator Guide

TurkmenAI Local is administered as local software. The current `0.1.0` implementation has no user account system, remote admin plane or automatic cloud enrollment. Administration focuses on supply-chain hygiene, controlled runtimes and release verification.

## Build environment

Use Node.js 22+, pnpm 10+ and stable Rust. A Linux desktop build requires GTK 3 and WebKitGTK development packages listed in [Installation](INSTALLATION.md). Build and verify with `./scripts/test` before `./scripts/package`.

## Local data boundary

| Domain | Administrative rule |
|---|---|
| API | Keep loopback binding unless a future authenticated LAN module is enabled by an administrator. |
| Models | Keep source identifiers, revisions, hashes, license notes and integrity state. Treat all source code as untrusted. |
| Store | Content blobs are immutable and SHA-256 addressed. Do not edit blob files in place. |
| Runtime | Register an explicit executable and isolated workspace; do not use a model repository directory as a runtime working directory. |
| Logs | Redact local paths and prompts before sharing diagnostic output. |
| Updates | Use signed, real platform artifacts only after successful matching-platform CI. |

## Release gate

The release workflow creates Windows and macOS bundles on native GitHub Actions runners. Never rename a Linux installer to another extension. Before a stable release, confirm unit tests, web build, native check, package metadata, checksums, icon family and model/runtime smoke tests; the current smoke-test gap is recorded transparently in [Release Checklist](RELEASE_CHECKLIST.md).

## Incident handling

If a model reports custom code, do not execute it to “make it work.” Preserve the source URL, revision and hash, isolate the file, and record why access was requested. If a runtime becomes unstable, stop it through the supervisor and retain the evidence; recovery must preserve source artifacts and journal state.
