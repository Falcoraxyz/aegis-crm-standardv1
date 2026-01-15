# Security Policy

Aegis CRM Standard focuses on cryptographic correctness and offline-first licensing security.

## Supported Versions
Only the latest `main` branch and the latest tagged release are supported for security fixes.

## Reporting a Vulnerability
Please report security vulnerabilities responsibly.

- **Do NOT** open a public GitHub issue.
- Contact the maintainer privately:

**Email:** falcoraa@gmail.com  
**Discord:** Falcora_

Include the following information:
- A clear description of the vulnerability
- Steps to reproduce / PoC (if available)
- Impact analysis (what can be bypassed / exploited)
- Affected version/commit
- Suggested fix (optional)

## Scope
### In scope
- Cryptographic implementation bugs
- Invalid signature acceptance
- Canonical CBOR encoding/decoding issues
- CERT parsing vulnerabilities (panic, OOM, malformed input)
- Proof-of-Possession (PoP) bypasses
- Dependency vulnerabilities affecting security

### Out of scope
- Client-side binary patching / cracking of downstream apps (client-side is inherently patchable)
- Vulnerabilities caused by downstream misuse of the API
- Social engineering, phishing, user credential compromise

## Disclosure Policy
We follow coordinated disclosure:
- Acknowledgement within **72 hours**
- Fix / mitigation will be prepared as soon as possible
- A public advisory may be published after a fix is released

Thank you for helping keep Aegis secure.
