# Security

## Reporting a vulnerability

Please do **not** open a public issue for security problems. Report via a
[private security advisory](https://github.com/dsk-dev-ai/tabforge-private/security/advisories)
on GitHub. We aim to acknowledge reports within 3 business days.

## Current posture

- This is an early-stage desktop app; the capture pipeline is still being built.
- No network endpoints are exposed by the application itself; it communicates
  only with the browser extension over a local WebSocket.
- Never commit secrets, API keys, or credentials to the repository.