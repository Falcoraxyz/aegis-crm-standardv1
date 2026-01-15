# Aegis CRM CLI - Design Document

## Overview

Aegis CLI is a command-line tool for vendors to manage cryptographic license operations. It provides a user-friendly interface to the `aegis-crm-core` library for key generation, certificate issuance, inspection, and Proof-of-Possession operations.

## Core Principles

- **Offline-first**: No network calls, works in air-gapped environments
- **Secure by default**: Refuses to overwrite files without `--force`, warns on private key operations
- **Deterministic**: Same inputs produce same outputs (except random keygen and license_id)
- **Cross-platform**: Works on Windows, Linux, macOS

## Architecture

```
aegis (binary)
  ├── main.rs (CLI entry point, clap setup)
  ├── commands/
  │   ├── mod.rs
  │   ├── vendor.rs (vendor keygen)
  │   ├── user.rs (user keygen)
  │   ├── issue.rs (issue certificates)
  │   ├── inspect.rs (inspect certificates)
  │   └── pop.rs (PoP operations)
  ├── utils/
  │   ├── mod.rs
  │   ├── io.rs (file I/O helpers)
  │   ├── hex.rs (hex encoding helpers)
  │   └── payload.rs (JSON payload validation)
  └── error.rs (CLI error types)
```

## Command Structure

### 1. Vendor Keygen
```bash
aegis vendor keygen [--out <dir>] [--force] [--json]
```

**Behavior**:
- Generate secp256k1 keypair using `aegis_crm_core::keys::vendor_keygen()`
- Write to `<out>/vendor_priv.hex` and `<out>/vendor_pub.hex`
- Default output: `./vendor_keys/`
- Refuse to overwrite unless `--force`
- Display loud warning about private key security

**Output** (human):
```
⚠️  WARNING: Vendor private key is CRITICAL - store in cold storage!
✅ Vendor keypair generated successfully!
   Private key: ./vendor_keys/vendor_priv.hex
   Public key:  ./vendor_keys/vendor_pub.hex
```

**Output** (`--json`):
```json
{
  "private_key_path": "./vendor_keys/vendor_priv.hex",
  "public_key_path": "./vendor_keys/vendor_pub.hex"
}
```

### 2. User Keygen
```bash
aegis user keygen [--out <dir>] [--force] [--json]
```

Same as vendor keygen but for user keys, default output: `./user_keys/`

### 3. Issue Certificate
```bash
aegis issue --vendor-priv <path> --user-pub <path|hex> --payload <json> --out <cert> [--force] [--json]
```

**Payload Schema**:
```json
{
  "plan": "lifetime_pro" | "campus",
  "expires_at": <unix_seconds> | null,
  "features": ["feature1", "feature2"],
  "seat_limit": 1,
  "metadata": { "custom": "data" }
}
```

**Validation Rules**:
- `campus` plan MUST have `expires_at` != null
- `lifetime_pro` may have null expiry
- Generate random `license_id` using CSPRNG
- Convert features array to bitmask (first 8 features map to bits 0-7)
- Use current timestamp for `issued_at`

**Outputs**:
- Binary CBOR: `<out>`
- Base64 CBOR: `<out>.base64`

### 4. Inspect Certificate
```bash
aegis inspect --cert <path> --vendor-pub <path|hex> [--json]
```

**Behavior**:
- Decode and verify certificate
- Display payload (NO private keys)
- Show validity status and expiry

**Output** (human):
```
Certificate: license.cert
Status: ✅ VALID
Product: falcora_terminal
License ID: 9f33ae...
Issued: 2026-01-13 14:30:00 UTC
Expires: Never (perpetual)
Features: 0b1111 (4 features)
User Public Key: 02a1b2c3...
```

### 5. Proof-of-Possession
```bash
# Generate challenge
aegis pop challenge
# Output: <32-byte-hex>

# Prove ownership
aegis pop prove --user-priv <path|hex> --nonce <hex>
# Output: <64-byte-sig-hex>

# Verify proof
aegis pop verify --user-pub <path|hex> --nonce <hex> --sig <hex>
# Exit code 0 if valid, 1 if invalid
```

## Security Considerations

1. **Private Key Handling**:
   - Never print private keys to stdout in normal mode
   - Warn user loudly when generating vendor keys
   - Suggest cold storage and backup

2. **File Operations**:
   - Check for existing files before writing
   - Require `--force` to overwrite
   - Validate output paths (reject `..` components)
   - Set restrictive file permissions on private keys (0600 on Unix)

3. **Input Validation**:
   - Validate hex strings (correct length, valid hex chars)
   - Validate JSON schema before processing
   - Clear error messages for invalid inputs

## Error Handling

Use `anyhow::Result` for error propagation with context:
- File I/O errors: "Failed to read vendor private key from {path}"
- Validation errors: "Invalid payload: campus plan requires expires_at"
- Crypto errors: "Certificate verification failed: invalid signature"

Exit codes:
- `0`: Success
- `1`: General error
- `2`: Validation error
- `3`: File I/O error

## Dependencies

- `clap` (v4): CLI argument parsing
- `anyhow`: Error handling with context
- `hex`: Hex encoding/decoding
- `base64`: Base64 encoding
- `serde_json`: JSON payload parsing
- `aegis-crm-core`: Core crypto library
- `chrono`: Timestamp formatting (human output only)

## Testing Strategy

1. **Unit tests**: Payload validation logic
2. **Integration tests**: End-to-end command flows
3. **Fixture-based tests**: Use existing golden vectors
