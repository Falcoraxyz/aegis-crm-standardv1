# AEGIS CRM STANDARD — AI AGENT IMPLEMENTATION ORDER
You are an AI software engineering agent. Your job is to implement the Aegis CRM Standard v1.0 Rust SDK exactly as specified. Do not deviate from the specification or invent new formats.

## 0) Primary Goal
Implement a production-quality Rust workspace for **Aegis CRM Standard v1.0 (Genesis)**.

Output must be:
- clean, tested Rust code
- stable public API
- deterministic encoding/signing behavior
- compliant with docs/API contract
- passes Definition of Done checklist

---

## 1) MUST READ (Order of Priority)
You MUST follow these documents (in this order):
1. `SPEC.md` (Aegis CRM Standard v1.0)
2. `docs/API.md` (Public API contract — DO NOT CHANGE)
3. `docs/STRUCTURE.md` (Canonical repo structure — MUST MATCH)
4. `docs/SECURITY.md` (Hardening guidance)
5. `docs/QUALITY.md` (Quality bar)
6. `docs/TASKS.md` (Work breakdown checklist)
7. `docs/DOD.md` (Definition of Done — MUST PASS)

If any conflict exists, resolve using the priority order above.

---

## 2) Deliverables
You MUST deliver:
### A) Workspace + crates
Create a Cargo workspace with:
- `crates/aegis-crm-core/` (Rust library)
Optionally later:
- `crates/aegis-crm-cli/` (not required for MVP unless TASKS says so)

### B) Core functionality
Implement all of these in `aegis-crm-core`:
- Vendor keygen
- User keygen
- Canonical CBOR CERT encoding/decoding
- Vendor signature issuance (ECDSA secp256k1)
- CERT verification
- Expiry enforcement
- Proof-of-Possession (PoP) challenge-response
- One-call verification helper

### C) Unit tests + integration tests
Must include:
- unit tests for CBOR canonical stability
- unit tests for sign/verify
- unit tests for PoP
- integration tests using fixtures

---

## 3) Non-Negotiable Rules (STRICT)
### 3.1 Cryptography
- Curve MUST be `secp256k1`
- Signature MUST be ECDSA
- CERT signature encoding MUST be compact 64 bytes `(r || s)`
- Public key MUST be compressed SEC1 33 bytes
- Hash MUST be SHA-256
- Never implement cryptography manually.

Recommended dependency:
- `k256`

### 3.2 Encoding
- CERT MUST be CBOR canonical.
- Signing payload MUST be canonical CBOR bytes excluding `sig`.

### 3.3 API Freeze
- The API in `docs/API.md` is immutable.
- Do NOT rename modules or public functions.
- Do NOT change public structs or fields.

### 3.4 Safety
- No `unsafe` in core crate.
- No panic in normal error flows.
- Use typed errors: `AegisError` enum only.

### 3.5 Output Restrictions
Do not add networking.
Do not add telemetry.
Do not add database calls.

Everything MUST work 100% offline.

---

## 4) Implementation Plan (Follow TASKS.md)
Follow the tasks checklist in `docs/TASKS.md`.
Implement modules in this order:
1) `errors.rs`
2) `crypto.rs`
3) `keys.rs`
4) `cert.rs`
5) `pop.rs`
6) `verify.rs`
7) tests + fixtures

---

## 5) Test Fixtures (Golden Vectors)
Create / use fixtures in:
- `fixtures/`

Fixtures MUST include:
- vendor private key
- vendor public key
- user private key
- user public key
- license payload example
- expected cert bytes (or expected verified = true)

Tests MUST validate:
- CBOR decode/encode roundtrip stable
- issued cert verifies correctly
- tampered cert fails verification
- invalid PoP fails

---

## 6) Definition of Done (STRICT)
You MUST NOT mark the task done until all of this passes:

- `cargo fmt --check` passes
- `cargo clippy -- -D warnings` passes
- `cargo test` passes
- At least:
  - 10 unit tests
  - 2 integration tests
- The following must work:
  - key generation
  - license issuance
  - license verification
  - proof-of-possession

---

## 7) README (Minimum)
Create a minimal `README.md` with:
- what is Aegis CRM
- how to build
- example usage code snippet

---

## 8) If Unsure
If any requirement is unclear:
- do not guess
- infer from SPEC and API
- choose the simplest compliant approach

The goal is correctness + spec compliance.
