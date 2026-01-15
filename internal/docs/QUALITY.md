Generated: 2026-01-12 14:20 UTC

# Quality Requirements

All code formatted with rustfmt.

Clippy must pass with -D warnings (no warnings).

No panics in normal flows (only for unreachable/internal invariants).

No unsafe Rust allowed.

Public API has doc comments with examples where helpful.

# Testing Requirements

Minimum unit tests: 12.

Minimum integration tests: 2 (issue/verify, encode/decode).

Golden vectors must be included and validated.

# Security Requirements

Use audited crypto crates only (k256).

No custom ECC, no custom random.

CSPRNG required for nonce & license_id.

Strict CBOR parsing: reject missing/extra critical fields.
