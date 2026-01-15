# START AI AGENT — Aegis CRM Standard (v1.0)

This file is a **one-page operational guide** to start an AI coding agent and validate results.
Use it as the canonical kickoff instruction for contributors/agents.

---

## 1) Prerequisites (Repo must contain)

✅ Core docs (must exist):
- `SPEC.md`
- `AGENT_PROMPT.md`
- `docs/API.md`
- `docs/STRUCTURE.md`
- `docs/TASKS.md`
- `docs/QUALITY.md`
- `docs/DOD.md`
- `fixtures/`

✅ Workflow:
- `.github/workflows/ci.yml`

---

## 2) Agent Kickoff Prompt (COPY-PASTE)

> Use this exact prompt as the **first** message to the AI agent.

```txt
Implement Aegis CRM Standard v1.0 Rust SDK.

Follow AGENT_PROMPT.md strictly.
Read SPEC.md + docs/* first.
Do NOT change docs/API.md.

Goal: implement crates/aegis-crm-core exactly with required API, CBOR canonical CERT, ECDSA secp256k1 signing/verification, PoP challenge-response, and tests + fixtures.

Definition of Done must pass:
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test --all
```

### Optional strict constraints (recommended)
```txt
Do not implement custom cryptography. Use k256 + sha2.
No unsafe.
No networking.
Do not add heavy dependencies.
```

---

## 3) Expected Deliverables (Agent Output)

The agent MUST implement:

### Workspace
- `Cargo.toml` (workspace)
- `crates/aegis-crm-core/Cargo.toml`
- `crates/aegis-crm-core/src/`

### Core modules (minimum)
- `errors.rs`
- `crypto.rs`
- `keys.rs`
- `cert.rs`
- `pop.rs`
- `verify.rs`
- `lib.rs`

### Tests
- ≥ 10 unit tests
- ≥ 2 integration tests
- Uses `fixtures/` golden vectors

---

## 4) Validation Commands (Definition of Done)

Run these locally at repo root:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all
```

All must pass with exit code 0.

---

## 5) Review Checklist (Before Accepting Work)

- [ ] Public API matches `docs/API.md` exactly
- [ ] CERT encoding is **canonical CBOR**
- [ ] CERT signature uses compact 64-byte ECDSA `(r||s)`
- [ ] Public keys are compressed 33 bytes (secp256k1)
- [ ] No `unsafe` in `aegis-crm-core`
- [ ] No network / database code
- [ ] Tampered CERT fails verification
- [ ] Invalid PoP fails verification
- [ ] CI passes on GitHub

---

## 6) Notes

- If agent is unsure, it must choose the **simplest compliant** approach.
- Correctness > speed.
- Determinism > convenience.

Math Trust > Server Trust.
