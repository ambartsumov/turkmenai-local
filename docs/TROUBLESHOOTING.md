# Troubleshooting

If `tmai server` does not start, first check that port `8742` is not already occupied and choose a different loopback port if necessary. The server prints a local URL only; it does not create a public endpoint.

If the native shell fails to compile on Linux, install the WebKitGTK and GTK development packages listed in the installation guide, then run `cargo check -p turkmenai-desktop`. On Windows and macOS, use the matching GitHub Actions runner rather than attempting to fabricate a foreign installer.

If a model source is marked `requires_code`, this is a security state, not a runtime failure. Inspect its files, repository revision and license before deliberately selecting an isolated integration route. Do not use `trust_remote_code` as a silent recovery action.
