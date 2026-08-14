$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")
pnpm install --frozen-lockfile
cargo fetch
Write-Host "Setup complete. Run .\scripts\test.ps1, then .\scripts\dev.ps1."
