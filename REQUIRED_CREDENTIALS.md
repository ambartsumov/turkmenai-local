# Required Credentials and User Actions

This document lists only actions that cannot be automated safely from the current environment. **Never paste credentials into an issue, source file, chat transcript, screenshot, or repository.**

| Credential or action | Why it is needed | Minimum scope | Exact location | What it enables | Revocable? |
|---|---|---|---|---|---|
| GitHub authorization for `ambartsumov/turkmenai-local` | Publish source, topics, tag, release and assets | Repository administration and release creation for this repository only | Existing authenticated GitHub CLI session; no PAT in source | Creates the public repository and release assets | Yes; remove/revoke access in GitHub settings |
| Custom-domain plan access | Obtain platform-issued DNS records for `turkmenai.tech` | Project custom-domain feature only | Project **Settings → Domains** | Starts safe domain verification and binding | Yes; subscription access can be changed later |
| Explicit confirmation of DNS mutation | DNS changes alter where the public domain routes | One record-set change for `turkmenai.tech` | Namecheap **Advanced DNS** | Removes redirect and adds only platform-issued apex/`www` records | Yes; records can be reverted |
| Windows code-signing certificate — optional | Avoid unsigned Windows installer warnings | Signing the TurkmenAI Local release only | GitHub Actions secret or secure signing provider | Signed Windows packages | Yes; revoke/replace certificate |
| Apple Developer signing/notarization credentials — optional | Gatekeeper-compatible macOS distribution | App signing/notarization for TurkmenAI Local only | GitHub Actions secret or secure signing provider | Signed/notarized macOS DMG | Yes; revoke/rotate in Apple Developer account |

## Not required now

No Hugging Face credential is required for the public codebase or metadata-only resolver. A token would be needed only for a private or gated model test explicitly selected by the owner. No Namecheap API token is requested: manual browser confirmation is safer for the single outstanding DNS change.
