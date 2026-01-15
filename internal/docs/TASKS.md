Generated: 2026-01-12 14:20 UTC

# Phase 0 — Workspace Initialization

Create Cargo workspace with crate aegis-crm-core.

Add dependencies: k256, sha2, rand_core/getrandom, serde, serde_cbor (or ciborium with canonical mode), thiserror.

Enable CI-friendly commands: cargo fmt, cargo clippy, cargo test.

# Phase 1 — Crypto Primitives

Implement SHA-256 helper: sha256(bytes)-\>\[u8;32\].

Implement ECDSA compact 64-byte sign(priv, digest)-\>\[u8;64\].

Implement ECDSA verify(pub, digest, sig)-\>bool.

Implement pubkey_from_privkey().

Unit tests: signature roundtrip.

# Phase 2 — Canonical CBOR CERT

Implement LicensePayload and LicenseCert structs.

Implement canonical CBOR encoding for payload map {v,pid,lid,iat,exp,upk,feat}.

Implement issue_cert(): digest(payload_bytes) and sign with vendor_priv.

Implement verify_cert(): verify vendor sig and enforce version + expiry.

Implement encode_cert()/decode_cert() with strict parsing & validation.

# Phase 3 — PoP (Proof of Possession)

Implement challenge(): 32-byte random nonce.

Implement prove(): sign sha256(nonce) with user_priv.

Implement verify(): verify using user_pub.

# Phase 4 — Ergonomic Verify

Implement verify_license(): Stage A + Stage B combined.

# Phase 5 — Tests & Fixtures

Add golden vector tests reading fixtures from ./fixtures.

Integration test: issue-\>encode-\>decode-\>verify-\>PoP.

# Phase 6 — Documentation

Add Rust doc comments to all public items.

Add README usage examples for library consumers.
