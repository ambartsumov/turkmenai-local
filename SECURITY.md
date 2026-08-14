# Security Policy

## Supported release line

Only the latest published `0.1.x` release receives fixes while the project remains pre-`1.0.0`.

## Reporting a vulnerability

Do not put credentials, API tokens, private prompts, model weights or exploit steps in a public issue. Submit a minimal sanitized report to the project maintainer through a private channel configured by the repository owner. Include the app version, operating system, reproduction steps, expected behaviour and actual behaviour.

The project treats automatic arbitrary-code execution, unprotected LAN exposure, loss of model metadata, plaintext secret leakage and installer compromise as high-severity classes. See `docs/SECURITY.md` for the implemented security model and its current boundaries.
