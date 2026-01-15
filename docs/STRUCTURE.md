Generated: 2026-01-12 14:20 UTC

# Canonical Workspace Layout

aegis-crm/  
crates/  
aegis-crm-core/  
Cargo.toml  
src/  
lib.rs  
keys.rs  
crypto.rs  
cert.rs  
pop.rs  
verify.rs  
errors.rs  
aegis-crm-cli/ (optional later)  
SPEC.docx  
SPEC.md (exported)  
README.md  
CHANGELOG.md  
LICENSE

# Module Ownership

keys.rs: key generation & conversions only.

crypto.rs: ECDSA sign/verify, sha256 helpers, canonical CBOR helper primitives.

cert.rs: LicensePayload/LicenseCert types, issue_cert, verify_cert, encode/decode.

pop.rs: challenge/prove/verify PoP flow.

verify.rs: verify_license one-call API.

errors.rs: AegisError enum.
