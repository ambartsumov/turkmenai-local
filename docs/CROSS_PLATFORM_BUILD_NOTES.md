# Cross-Platform Build Notes

## Verified platform facts

Tauri v2 documents a GitHub Actions pipeline that can build native applications and upload release artifacts. Its guide has a dedicated section for ARM runner compilation, including a more involved Linux ARM AppImage path. [1]

For Windows, Tauri documents MSI packages built with WiX and NSIS setup executables. MSI packaging must run on Windows because WiX runs on Windows; Windows ARM/32-bit targeting is documented separately and is not claimed as locally validated by this project. [2]

## Release policy

The product website must distinguish **available now**, **CI build planned** and **unsupported**. A direct download is presented only for a real release artifact. Windows x64/ARM64, Linux x64/ARM64 and macOS targets are represented by CI matrix entries and release-manifest records; they cannot be labelled downloaded, signed or verified until their matching runners succeed.

## References

[1]: https://v2.tauri.app/distribute/pipelines/github/ "Tauri v2 — GitHub pipeline"
[2]: https://v2.tauri.app/distribute/windows-installer/ "Tauri v2 — Windows Installer"
