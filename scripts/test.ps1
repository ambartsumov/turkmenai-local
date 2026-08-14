$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")
pnpm run check
cargo test --workspace
cargo check -p turkmenai-desktop
