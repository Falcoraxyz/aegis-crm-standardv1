# Contributing to Aegis CRM Standard

Thanks for taking interest in contributing.

Aegis CRM Standard is a cryptographic standard + reference implementation, so **correctness and spec compliance** matter more than speed.

---

## What We Care About
- Spec compliance: `SPEC.md`
- Stable public API: `docs/API.md` (**do not break without discussion**)
- Deterministic encoding (CBOR canonical)
- No insecure crypto shortcuts
- High-quality tests (fixtures + regression tests)

---

## How to Contribute
### 1) Fork & Branch
Create a feature branch:
```bash
git checkout -b feat/my-change
