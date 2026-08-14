# Platform Support Matrix

## Product rule

The download page must never present a non-existent binary as a release. Every target has a stable identifier, a delivery status and a matching GitHub Actions job. The button becomes a direct release download only when that job has uploaded the named artifact and its SHA-256 is present in the release manifest.

| Platform | Architecture | Package | Current status | Release gate |
|---|---:|---|---|---|
| Windows 10/11 | x64 | NSIS `.exe` and MSI | CI target | Native Windows x64 build, installer smoke and checksum |
| Windows 10/11 | ARM64 | NSIS `.exe` and MSI | CI target | `windows-11-arm` build, installer smoke and checksum |
| Linux | x64 | AppImage, DEB, RPM | **Verified** | Current Linux x64 bundle and SHA-256 |
| Linux | ARM64 | AppImage | CI target | `ubuntu-24.04-arm` build and checksum |
| Linux | ARM64 | DEB, RPM | Planned | Native ARM packaging and distribution-specific validation |
| macOS | Apple Silicon | DMG | CI target | macOS ARM bundle, signing configuration and checksum |
| macOS | Intel | DMG | CI target | macOS Intel bundle, signing configuration and checksum |

## Installer policy

Windows must be built on a Windows runner for MSI packaging. Tauri supports WiX MSI and NSIS setup executables; the workflow emits both where the platform tooling supports them. [1]

GitHub’s hosted-runner documentation lists `windows-11-arm` and `ubuntu-24.04-arm`, while noting that third-party actions may need manual validation on ARM64. The workflow therefore installs and validates its build tooling directly and preserves a clear failure signal instead of silently substituting a different architecture. [2]

## Direct download policy

The site reads an `artifacts.json` release manifest. A **Download** button is rendered only for an artifact with `status: "published"` and an HTTPS URL. Other targets show a transparent status card with the target, expected package and next verification gate.

## References

[1]: https://v2.tauri.app/distribute/windows-installer/ "Tauri — Windows Installer"
[2]: https://docs.github.com/en/actions/reference/runners/github-hosted-runners "GitHub — hosted runners reference"
