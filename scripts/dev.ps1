$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")
pnpm desktop:dev
