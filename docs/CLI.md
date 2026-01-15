# Aegis CLI - Command Line Reference

The `aegis` command-line tool provides vendor operations for cryptographic license management using the Aegis CRM Standard v1.0.

## Installation

### From Source

```bash
cd aegis-crm
cargo build --release --bin aegis
```

The binary will be available at `target/release/aegis` (or `aegis.exe` on Windows).

### Add to PATH (Optional)

```bash
# Linux/macOS
cp target/release/aegis /usr/local/bin/

# Windows (PowerShell as Administrator)
Copy-Item target\release\aegis.exe C:\Windows\System32\
```

---

## Commands

### 1. Vendor Key Generation

Generate a vendor keypair for signing licenses.

**Usage:**
```bash
aegis vendor keygen [--out <dir>] [--force] [--json]
```

**Options:**
- `--out <dir>` - Output directory (default: `./vendor_keys`)
- `--force` - Overwrite existing files
- `--json` - Machine-readable JSON output

**Output Files:**
- `vendor_priv.hex` - Private key (32 bytes, hex-encoded)
- `vendor_pub.hex` - Public key (33 bytes, compressed SEC1)

**Example:**
```bash
aegis vendor keygen
```

**Output:**
```
⚠️  WARNING: Vendor private key is CRITICAL!
   - Store in cold storage (offline, encrypted)
   - Never commit to version control
   - Create secure backups
   - Anyone with this key can issue licenses!

✅ Vendor keypair generated successfully!
   Private key: ./vendor_keys/vendor_priv.hex
   Public key:  ./vendor_keys/vendor_pub.hex
```

---

### 2. User Key Generation

Generate a user keypair for license ownership.

**Usage:**
```bash
aegis user keygen [--out <dir>] [--force] [--json]
```

**Options:**
- `--out <dir>` - Output directory (default: `./user_keys`)
- `--force` - Overwrite existing files
- `--json` - Machine-readable JSON output

**Output Files:**
- `user_priv.hex` - Private key (32 bytes)
- `user_pub.hex` - Public key (33 bytes)

**Example:**
```bash
aegis user keygen --out ./my_keys
```

---

### 3. Issue License Certificate

Issue a signed license certificate from a JSON payload.

**Usage:**
```bash
aegis issue \
  --vendor-priv <path|hex> \
  --user-pub <path|hex> \
  --payload <json_file> \
  --out <cert_path> \
  [--force] [--json]
```

**Options:**
- `--vendor-priv` - Path to vendor private key or hex string
- `--user-pub` - Path to user public key or hex string
- `--payload` - Path to JSON payload file
- `--out` - Output certificate path
- `--force` - Overwrite existing files
- `--json` - Machine-readable JSON summary

**Output Files:**
- `<out>` - Certificate (binary CBOR format)
- `<out>.base64` - Certificate (base64-encoded)

**Payload Schema:**
```json
{
  "plan": "lifetime_pro" | "campus",
  "expires_at": <unix_seconds> | null,
  "features": ["feature1", "feature2"],
  "seat_limit": 1,
  "metadata": { "key": "value" }
}
```

**Validation Rules:**
- `campus` plan MUST have `expires_at` (non-null)
- `lifetime_pro` may have null expiry (perpetual)
- Features array maps to bitmask (first 8 features → bits 0-7)

**Example:**
```bash
aegis issue \
  --vendor-priv ./vendor_keys/vendor_priv.hex \
  --user-pub ./user_keys/user_pub.hex \
  --payload ./examples/payloads/lifetime_pro.json \
  --out ./license.cert
```

**Output:**
```
✅ License certificate issued successfully!
   License ID: 9f33ae12...
   Plan: LifetimePro
   Features: 4
   Certificate: ./license.cert
   Base64: ./license.cert.base64
```

---

### 4. Inspect License Certificate

Decode and verify a license certificate.

**Usage:**
```bash
aegis inspect \
  --cert <path> \
  --vendor-pub <path|hex> \
  [--json]
```

**Options:**
- `--cert` - Path to certificate file
- `--vendor-pub` - Path to vendor public key or hex string
- `--json` - Machine-readable JSON output

**Example:**
```bash
aegis inspect \
  --cert ./license.cert \
  --vendor-pub ./vendor_keys/vendor_pub.hex
```

**Output:**
```
Certificate: ./license.cert
Status: ✅ VALID
Product: falcora_terminal
License ID: 9f33ae12...
Issued: 2026-01-12 08:14:14 UTC
Expires: Never (perpetual)
Features: 0x3 (2 features)
User Public Key: 03a1b2c3...
```

**Status Values:**
- `✅ VALID` - Certificate is valid and not expired
- `⏰ EXPIRED` - Certificate signature is valid but expired
- `❌ INVALID` - Signature verification failed

---

### 5. Proof-of-Possession (PoP)

Prove ownership of a user private key using challenge-response.

#### 5a. Generate Challenge

```bash
aegis pop challenge
```

**Output:** 32-byte hex nonce
```
aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
```

#### 5b. Prove Ownership

```bash
aegis pop prove \
  --user-priv <path|hex> \
  --nonce <hex>
```

**Output:** 64-byte compact ECDSA signature (hex)

#### 5c. Verify PoP

```bash
aegis pop verify \
  --user-pub <path|hex> \
  --nonce <hex> \
  --sig <hex>
```

**Exit Codes:**
- `0` - Proof valid ✅
- `1` - Proof invalid ❌

**Complete PoP Flow Example:**
```bash
# Generate challenge
NONCE=$(aegis pop challenge)
echo "Nonce: $NONCE"

# User proves ownership
SIG=$(aegis pop prove \
  --user-priv ./user_keys/user_priv.hex \
  --nonce $NONCE)
echo "Signature: $SIG"

# Verify proof
aegis pop verify \
  --user-pub ./user_keys/user_pub.hex \
  --nonce $NONCE \
  --sig $SIG

# Output: ✅ Proof-of-Possession VALID
```

---

## File Formats

### Hex Files
Human-readable hexadecimal encoding for keys and signatures.  
Example: `0102030405...1f20`

### CBOR Binary (`.cert`)
Canonical CBOR encoding for certificates (deterministic, cross-platform).

### Base64 (`.cert.base64`)
Base64-encoded certificate for embedding in JSON/XML/etc.

---

## Complete Workflow Example

```bash
# 1. Generate vendor keypair (one-time setup)
aegis vendor keygen

# 2. Generate user keypair
aegis user keygen

# 3. Issue lifetime pro license
aegis issue \
  --vendor-priv ./vendor_keys/vendor_priv.hex \
  --user-pub ./user_keys/user_pub.hex \
  --payload ./examples/payloads/lifetime_pro.json \
  --out ./license.cert

# 4. Inspect the certificate
aegis inspect \
  --cert ./license.cert \
  --vendor-pub ./vendor_keys/vendor_pub.hex

# 5. Perform proof-of-possession
NONCE=$(aegis pop challenge)
SIG=$(aegis pop prove --user-priv ./user_keys/user_priv.hex --nonce $NONCE)
aegis pop verify --user-pub ./user_keys/user_pub.hex --nonce $NONCE --sig $SIG
```

---

## Security Best Practices

1. **Vendor Private Key:**
   - Store in offline, encrypted storage
   - Never commit to version control
   - Create multiple secure backups
   - Use hardware security modules (HSMs) for production

2. **Key Distribution:**
   - Distribute only public keys
   - Verify key fingerprints out-of-band
   - Use secure channels for sensitive operations

3. **License Issuance:**
   - Validate all payload fields before issuance
   - Generate unique license IDs for each license
   - Keep audit logs of all issued licenses

4. **Certificate Storage:**
   - Certificates can be publicly distributed
   - Base64 format is safe for embedding in websites/emails
   - CBOR format ensures cross-platform compatibility

---

## Troubleshooting

### "File already exists" Error
Use `--force` to overwrite existing files.

### "Invalid hex string" Error  
Ensure hex strings have even length and contain only 0-9, a-f characters.

### "Campus plan requires expires_at" Error
Campus licenses must have an expiry date. Set `expires_at` to a Unix timestamp.

### Certificate Verification Failed
- Check vendor public key matches the key used for signing
- Verify certificate file is not corrupted
- Ensure certificate hasn't been tampered with

---

## Exit Codes

- `0` - Success
- `1` - Error (general)

---

## See Also

- [Aegis CRM Standard v1.0](../SPEC.md)
- [API Reference](API.md)
- [Example Payloads](../examples/payloads/)
