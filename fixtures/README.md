# ⚠️ TEST FIXTURES ONLY - NOT FOR PRODUCTION

These are **deterministic test vectors** for CI/CD validation.

## 🔒 CRITICAL WARNINGS

- ✅ **Public keys ONLY** (`vendor_pub.hex`, `user_pub.hex`)
- ❌ **NEVER commit** `vendor_priv.hex` or `user_priv.hex`
- ✅ **These keys are for TESTING ONLY**
- ❌ **DO NOT use these keys in production systems**

## 📦 Contents

- `vendor_pub.hex` - Test vendor public key (33 bytes, secp256k1 compressed)
- `user_pub.hex` - Test user public key (33 bytes, secp256k1 compressed)
- `license_payload.json` - Sample license payload (CBOR short names)
- `license.cert.base64` - Test certificate (base64-encoded CBOR)
- `pop_nonce.hex` - Test nonce for PoP (dummy: all 'a's)

## 🧪 Generation

All fixtures are generated via:
```bash
cargo run -p aegis-crm-core --example generate_fixtures
```

**DO NOT edit manually** - CI will fail if fixtures are not deterministic.

## 🔐 Security

These fixtures use **test keys only**. The private keys used to generate these fixtures are:
- Ephemeral (generated for testing)
- Not used in any production system
- Publicly documented in the codebase

If you need to generate production licenses, use the `aegis` CLI:
```bash
aegis vendor keygen  # Generate NEW production keys
aegis user keygen    # Generate user keys
aegis issue ...      # Issue real certificates
```

**Never reuse test fixtures for production!**
