# 🔒 Push Protection Guide

**CRITICAL: Read this before committing to prevent security incidents!**

---

## ⚠️ NEVER COMMIT THESE FILES

The following files contain **cryptographic private keys** and must NEVER be committed to Git:

### Private Keys
```
**/*_priv.hex         # Vendor/User private keys
**/*.key              # Generic private keys
**/*.pem              # PEM-encoded keys
**/*.p12, **/*.pfx    # Certificate bundles
```

### Secrets & Environment
```
.env*                 # Environment variables
**/*.cert             # Certificate files (except fixtures/)
**/pop_sig.hex        # PoP signatures
```

---

## ✅ Pre-Commit Checklist

Before running `git commit`, verify:

- [ ] No `*_priv.hex` files in staging area
- [ ] No `.env` or secret files in staging area
- [ ] Fixtures haven't been manually edited
- [ ] CI checks will pass (run locally first)

### Quick Check Command
```powershell
# Check for private keys in staging
git diff --cached --name-only | Select-String "priv|\.key|\.pem|\.env"

# Should return EMPTY - if not, DO NOT COMMIT!
```

---

## 🛡️ CI Fixture Protection

Our CI pipeline validates fixture determinism:

### How It Works
1. CI runs `cargo run -p aegis-crm-core --example generate_fixtures`
2. Checks if fixtures changed: `git diff --exit-code`
3. **FAILS if fixtures are modified** (prevents non-deterministic generation)

### If CI Fails on Fixtures
```powershell
# Regenerate fixtures locally
cargo run -p aegis-crm-core --example generate_fixtures

# Verify no changes
git status

# If changed, investigate why (indicates non-deterministic bug)
git diff fixtures/
```

---

## 🚨 Accident Recovery

### If You Accidentally Committed Private Keys

**DO NOT PUSH!** If you haven't pushed yet:

```powershell
# Undo last commit, keep changes
git reset --soft HEAD~1

# Remove private keys from staging
git reset HEAD **/*_priv.hex

# Recommit without private keys
git commit -m "your message"
```

### If You Already Pushed Private Keys

**🚨 CRITICAL - ACT IMMEDIATELY:**

1. **Rotate ALL keys immediately** - old keys are compromised
2. Contact repository admin to purge history
3. Force push clean history (requires force push permissions)
4. Revoke all licenses issued with compromised keys

### Force History Rewrite (Use with Extreme Caution)
```powershell
# DANGER: This rewrites history - coordinate with team!
git filter-branch --tree-filter 'rm -f **/*_priv.hex' HEAD
git push --force
```

---

## 📝 .gitignore Configuration

Our `.gitignore` protects against common mistakes:

```gitignore
# Private keys (CRITICAL)
**/*_priv.hex
**/*.key
**/*.pem
**/*.p12
**/*.pfx

# Environment secrets
.env
.env.local
.env.*.local

# Certificates (except fixtures)
**/*.cert
**/pop_sig.hex

# IMPORTANT: fixtures/ IS tracked (intentionally)
# Only regenerate via example, never edit manually
```

---

## 🔐 Fixture Security

### Why Fixtures Are Tracked
- Fixtures use **test keys only** (not production keys)
- Provide deterministic test vectors for compatibility
- Enable CI to verify determinism

### Fixture Files (Safe to Commit)
```
fixtures/vendor_priv.hex      # Test vendor key
fixtures/vendor_pub.hex       # Test vendor pubkey
fixtures/user_priv.hex        # Test user key
fixtures/user_pub.hex         # Test user pubkey
fixtures/license.cert         # Test certificate
fixtures/pop_nonce.hex        # Test nonce
fixtures/pop_sig.hex          # Test signature
```

⚠️ **Never use fixture keys in production!**

---

## 🧪 Local Testing Before Push

### Run Full CI Suite Locally
```powershell
# Format check
cargo fmt --check

# Lint check
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Tests
cargo test --workspace --all-features

# Fixture validation
cargo run -p aegis-crm-core --example generate_fixtures
git diff --exit-code fixtures/
```

All checks must pass before pushing!

---

## 📚 Additional Resources

- [SECURITY.md](../SECURITY.md) - Security policy and reporting
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
- [CI Workflow](workflows/ci.yml) - Automated checks

---

## 🎯 Best Practices Summary

1. **Never commit private keys** - use `.gitignore` protection
2. **Always run pre-commit checks** - verify no secrets in staging
3. **Trust the CI** - if fixtures fail, investigate don't bypass
4. **Rotate immediately** - if keys are exposed, assume compromise
5. **Use fixture keys for tests only** - never in production

---

**Remember: One accidental commit can compromise the entire licensing system!**

Stay vigilant. When in doubt, double-check before pushing.
