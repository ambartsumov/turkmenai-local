# Contributing

Please open an issue before broad architectural changes. Every change must preserve the separation between model source analysis, immutable content storage, versioned state, runtime process control and UI.

Run `./scripts/test` and `cargo fmt --all -- --check` before a pull request. Any upstream-derived work needs its source revision, license implications and clear attribution. Do not add model weights, access tokens, large build artifacts, personal data or generated dependency caches to the repository.
