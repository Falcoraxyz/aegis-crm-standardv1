# Aegis CRM - Struktur Proyek

Dokumentasi lengkap struktur file dan direktori proyek Aegis CRM Standard v1.0.

---

## 📁 Struktur Direktori Utama

```
aegis-crm/
├── .github/               # GitHub workflows dan CI/CD
│   └── workflows/
│       └── ci.yml        # CI pipeline (fmt, clippy, test, fixtures)
├── .kiro/                 # Spesifikasi dan dokumentasi CLI
│   └── specs/
│       └── aegis-crm-cli/
├── internal/              # Internal development documentation
│   └── docs/
│       ├── README.md     # Internal docs index
│       ├── QUALITY.md    # Code quality standards
│       ├── DOD.md        # Definition of Done
│       └── TASKS.md      # Implementation task breakdown
├── crates/                # Rust workspace
│   ├── aegis-crm-core/   # Core library (SDK)
│   └── aegis-crm-cli/    # CLI tool
├── docs/                  # Public documentation
│   ├── API.md            # Frozen public API contract
│   ├── CLI.md            # CLI command reference
│   └── SECURITY.md       # Security policy
├── examples/              # Example payloads
│   └── payloads/
├── fixtures/              # Golden test vectors
├── assets/                # Branding assets
├── .gitignore            # Git ignore rules
├── .gitattributes        # Git line ending rules
├── Cargo.toml            # Workspace manifest
├── rust-toolchain.toml   # Rust toolchain config
├── SPEC.md               # Aegis CRM Standard v1.0
├── README.md             # Project overview (public-facing)
├── STRUCTURE.md          # Project structure (this file)
├── CHANGELOG.md          # Version history
├── CONTRIBUTING.md       # Contribution guidelines
├── CODE_OF_CONDUCT.md    # Code of conduct
├── SECURITY.md           # Security policy
└── LICENSE               # AGPL-3.0 license
```

---

## 📂 Directory Breakdown

### Public Documentation (`docs/`)

**For end-users and library consumers:**

| File | Audience | Purpose |
|------|----------|---------|
| `API.md` | Developers | Frozen public Rust API contract |
| `CLI.md` | Users | Complete CLI command reference |
| `SECURITY.md` | Security researchers | Security policy & reporting |

### Internal Documentation (`internal/docs/`)

**For contributors and maintainers only:**

| File | Purpose |
|------|---------|
| `README.md` | Internal docs index |
| `QUALITY.md` | Code quality standards and requirements |
| `DOD.md` | Definition of Done checklist |
| `TASKS.md` | Implementation task breakdown (6 phases) |

### Root Level

| File | Description |
|------|-------------|
| `SPEC.md` | Complete Aegis CRM Standard v1.0 specification |
| `README.md` | Marketing-friendly project overview |
| `STRUCTURE.md` | This file - canonical project structure |
| `CHANGELOG.md` | Version history and release notes |
| `CONTRIBUTING.md` | How to contribute to the project |
| `CODE_OF_CONDUCT.md` | Community guidelines |
| `SECURITY.md` | Security policy (symlink/duplicate of docs/SECURITY.md) |
| `LICENSE` | AGPL-3.0 license text |

---

## 🦀 Crate: aegis-crm-core

**Path:** `crates/aegis-crm-core/`

Library inti yang mengimplementasikan Aegis CRM Standard v1.0.

### Struktur File

```
aegis-crm-core/
├── src/
│   ├── lib.rs           # Entry point, exports publik
│   ├── errors.rs        # AegisError enum (7 variants)
│   ├── crypto.rs        # SHA-256, ECDSA, pubkey derivation
│   ├── keys.rs          # vendor_keygen, user_keygen
│   ├── cert.rs          # LicensePayload, issue/verify/encode/decode
│   ├── pop.rs           # Proof-of-Possession (challenge/prove/verify)
│   └── verify.rs        # verify_license (unified API)
├── tests/
│   └── integration_test.rs  # 6 integration tests
├── examples/
│   └── generate_fixtures.rs # Generator golden vectors
├── Cargo.toml
└── README.md
```

### Module Ownership

| File | Tanggung Jawab | Exports |
|------|---------------|---------|
| `lib.rs` | Module coordinator, docs | `pub use` semua modules |
| `errors.rs` | Error handling | `AegisError` enum |
| `crypto.rs` | Cryptographic primitives | `sha256()`, `sign_compact()`, `verify_compact()` |
| `keys.rs` | Key generation | `vendor_keygen()`, `user_keygen()`, `pubkey_from_privkey()` |
| `cert.rs` | Certificate operations | `LicensePayload`, `LicenseCert`, `Limits`, `Metadata`, `issue_cert()`, `verify_cert()`, `encode_cert()`, `decode_cert()` |
| `pop.rs` | Proof-of-Possession | `challenge()`, `prove()`, `verify()` |
| `verify.rs` | Unified verification | `verify_license()` |

### cert.rs - License Feature System

**LicensePayload Structure:**
```rust
pub struct LicensePayload {
    pub version: u16,                    // Protocol version (1)
    pub product_id: String,              // Product identifier
    pub license_id: [u8; 32],           // Unique license ID
    pub issued_at: u64,                  // Unix timestamp
    pub expiry: Option<u64>,             // Optional expiry
    pub user_pubkey: [u8; 33],          // Compressed SEC1
    pub tier: String,                    // "lifetime_pro" | "campus"
    pub features: Vec<String>,           // Feature list or ["ALL"]
    pub limits: Option<Limits>,          // Usage limits
    pub metadata: Option<Metadata>,      // Custom metadata
}

pub struct Limits {
    pub seat_max: Option<u32>,           // Max concurrent seats
    pub offline_grace_days: Option<u32>, // Offline grace period
}

pub struct Metadata {
    pub product: Option<String>,         // Product name
    pub version: Option<String>,         // Version string
    pub university: Option<String>,      // University (campus tier)
}
```

**Methods:**
- `has_feature(&self, feature: &str) -> bool` - Check feature access (supports "ALL" wildcard)

### Testing

**Unit Tests:** 27 tests
- `crypto.rs`: 4 tests
- `keys.rs`: 4 tests  
- `cert.rs`: 11 tests (termasuk wildcard, campus validation)
- `pop.rs`: 4 tests
- `verify.rs`: 2 tests

**Integration Tests:** 6 tests
- Complete license flow
- Tampered certificate detection
- Expiry validation
- Invalid PoP rejection
- CBOR stability
- Pubkey derivation consistency

**Total:** 33 tests passing ✅

---

## 🔧 Crate: aegis-crm-cli

**Path:** `crates/aegis-crm-cli/`

Command-line tool untuk vendor operations.

### Struktur File

```
aegis-crm-cli/
├── src/
│   ├── main.rs              # CLI entry point (clap)
│   ├── error.rs             # Error types
│   ├── commands/
│   │   ├── mod.rs          # Command exports
│   │   ├── vendor.rs       # vendor keygen
│   │   ├── user.rs         # user keygen
│   │   ├── issue.rs        # Certificate issuance
│   │   ├── inspect.rs      # Certificate inspection
│   │   └── pop.rs          # PoP operations
│   └── utils/
│       ├── mod.rs          # Utility exports
│       ├── io.rs           # File I/O, hex handling
│       └── payload.rs      # JSON payload validation
├── Cargo.toml
└── README.md
```

### Command Overview

| Command | Fungsi | Output Files |
|---------|--------|--------------|
| `vendor keygen` | Generate vendor keypair | `vendor_priv.hex`, `vendor_pub.hex` |
| `user keygen` | Generate user keypair | `user_priv.hex`, `user_pub.hex` |
| `issue` | Issue license certificate | `*.cert`, `*.cert.base64` |
| `inspect` | Decode & verify certificate | stdout (human/JSON) |
| `pop challenge` | Generate PoP nonce | stdout (hex) |
| `pop prove` | Sign PoP challenge | stdout (hex signature) |
| `pop verify` | Verify PoP signature | exit code 0/1 |

### utils/payload.rs - JSON Schema

```rust
pub struct LicensePayloadJson {
    pub tier: LicenseTier,              // LifetimePro | Campus
    pub expires_at: Option<u64>,        // Unix timestamp
    pub features: Vec<String>,          // ["feature1", "ALL", ...]
    pub limits: Option<LimitsJson>,     // seat_max, offline_grace_days
    pub metadata: Option<MetadataJson>, // product, version, university
}
```

**Validasi:**
- Campus tier WAJIB ada `expires_at` (masa depan)
- Lifetime Pro boleh `expires_at = null`
- Features default ke empty vec jika tidak ada
- Support wildcard "ALL"

**Tests:** 4 tests
- Campus requires expiry
- Lifetime Pro optional expiry
- Features ALL wildcard
- Campus with future expiry

---

## 🧪 Fixtures & Examples

### fixtures/

Golden test vectors deterministik:

| File | Deskripsi | Format |
|------|-----------|--------|
| `vendor_priv.hex` | Vendor private key | 32 bytes, hex |
| `vendor_pub.hex` | Vendor public key | 33 bytes, compressed SEC1 |
| `user_priv.hex` | User private key | 32 bytes, hex |
| `user_pub.hex` | User public key | 33 bytes, compressed SEC1 |
| `license_payload.json` | Sample payload | JSON (new schema) |
| `license.cert` | Valid certificate | Binary CBOR |
| `license.cert.base64` | Valid certificate | Base64-encoded |
| `pop_nonce.hex` | PoP challenge | 32 bytes, hex |
| `pop_sig.hex` | PoP signature | 64 bytes, compact ECDSA |
| `README.md` | Fixture documentation | Markdown |

**Regenerasi:**
```bash
cargo run -p aegis-crm-core --example generate_fixtures
```

### examples/payloads/

Contoh payload untuk CLI:

**lifetime_pro.json:**
```json
{
  "tier": "lifetime_pro",
  "expires_at": null,
  "features": ["ALL"],
  "limits": null,
  "metadata": {
    "product": "Falcora Terminal",
    "version": "2.0"
  }
}
```

**campus.json:**
```json
{
  "tier": "campus",
  "expires_at": 1767225600,
  "features": ["base_access", "education_features", "collaboration_tools"],
  "limits": {
    "seat_max": 100,
    "offline_grace_days": 14
  },
  "metadata": {
    "product": "Falcora Terminal",
    "version": "2.0",
    "university": "Massachusetts Institute of Technology"
  }
}
```

---

## ⚙️ Configuration Files

### Root Level Config

| File | Fungsi |
|------|--------|
| `Cargo.toml` | Workspace config (resolver v2, members, profile) |
| `rust-toolchain.toml` | Rust stable + components (rustfmt, clippy) |
| `.gitignore` | Git ignore rules (target/, IDE files, logs) |
| `.gitattributes` | Line endings (LF for code, binary handling) |

### .gitattributes

Enforces consistent line endings dan binary handling:
- **Source code/docs:** `eol=lf` (Rust, TOML, MD, YAML, JSON)
- **Windows scripts:** `eol=crlf` (PS1, BAT)
- **Certificates:** `-text -diff` (binary, no EOL conversion)
- **Images:** Binary treatment
- **Markdown:** `diff=markdown` for cleaner diffs

### .github/workflows/ci.yml

**CI Pipeline:**
```yaml
on: [push, pull_request]

steps:
  - cargo fmt --check
  - cargo clippy --workspace --all-targets --all-features -- -D warnings
  - cargo test --workspace --all-features
  - cargo run -p aegis-crm-core --example generate_fixtures
  - git diff --exit-code  # Ensure fixtures are up-to-date
```

**Validasi:**
- Format compliance ✅
- Zero clippy warnings (workspace-wide) ✅
- All tests passing ✅
- Fixtures determinism ✅

---

## 🔐 Cryptography Stack

| Komponen | Library | Spesifikasi |
|----------|---------|-------------|
| ECC Curve | `k256` v0.13 | secp256k1 |
| Hashing | `sha2` v0.10 | SHA-256 |
| Signature | `k256::ecdsa` | Compact 64-byte (r\\|\\|s) |
| Pubkey Format | SEC1 compressed | 33 bytes (0x02/0x03 prefix) |
| Encoding | `ciborium` v0.2 | Canonical CBOR |
| RNG | `getrandom` v0.2 | CSPRNG |

---

## 📊 Test Coverage

### Workspace Tests

```bash
cargo test --workspace
```

**Hasil:**
- **Total:** 37 tests passing ✅
  - Core library: 33 tests (27 unit + 6 integration)
  - CLI: 4 tests (payload validation)

### CI Checks

```bash
cargo fmt --check        # ✅ Formatting
cargo clippy --workspace --all-targets --all-features -- -D warnings  # ✅ Lints
cargo test --workspace --all-features   # ✅ Tests
cargo run -p aegis-crm-core --example generate_fixtures  # ✅ Fixtures
```

---

## 🚀 Penggunaan

### Core Library

```rust
use aegis_crm_core::{
    keys::{vendor_keygen, user_keygen},
    cert::{issue_cert, verify_cert, LicensePayload, Limits, Metadata, PROTOCOL_VERSION},
    pop::{challenge, prove, verify},
};

// Generate keys
let vendor = vendor_keygen();
let user = user_keygen();

// Create payload with new schema
let mut license_id = [0u8; 32];
getrandom::getrandom(&mut license_id)?;

let payload = LicensePayload {
    version: PROTOCOL_VERSION,
    product_id: "my_app".to_string(),
    license_id,
    issued_at: now(),
    expiry: None,
    user_pubkey: user.pubkey,
    tier: "lifetime_pro".to_string(),
    features: vec!["ALL".to_string()],
    limits: Some(Limits {
        seat_max: Some(1),
        offline_grace_days: None,
    }),
    metadata: Some(Metadata {
        product: Some("My App".to_string()),
        version: Some("1.0".to_string()),
        university: None,
    }),
};

// Issue certificate
let cert = issue_cert(&vendor.privkey, payload)?;

// Verify
verify_cert(&vendor.pubkey, &cert, now())?;

// PoP
let nonce = challenge();
let sig = prove(&user.privkey, &nonce)?;
verify(&user.pubkey, &nonce, &sig)?;

// Check features
assert!(cert.payload.has_feature("any_feature")); // true with "ALL"
```

### CLI Usage

```bash
# Generate vendor keys
aegis vendor keygen

# Generate user keys  
aegis user keygen

# Issue lifetime pro license
aegis issue \
  --vendor-priv ./vendor_keys/vendor_priv.hex \
  --user-pub ./user_keys/user_pub.hex \
  --payload ./examples/payloads/lifetime_pro.json \
  --out ./license.cert

# Inspect certificate
aegis inspect \
  --cert ./license.cert \
  --vendor-pub ./vendor_keys/vendor_pub.hex

# PoP workflow
NONCE=$(aegis pop challenge)
SIG=$(aegis pop prove --user-priv ./user_keys/user_priv.hex --nonce $NONCE)
aegis pop verify --user-pub ./user_keys/user_pub.hex --nonce $NONCE --sig $SIG
```

---

## 📦 Dependencies

### Core Library (`aegis-crm-core`)

```toml
[dependencies]
k256 = { version = "0.13", features = ["ecdsa", "sha256"] }
sha2 = "0.10"
rand_core = "0.6"
getrandom = "0.2"
serde = { version = "1", features = ["derive"] }
serde_bytes = "0.11"
ciborium = "0.2"
thiserror = "2"
```

### CLI Tool (`aegis-crm-cli`)

```toml
[dependencies]
aegis-crm-core = { path = "../aegis-crm-core" }
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
hex = "0.4"
base64 = "0.22"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
getrandom = "0.2"
```

---

## 🎯 File Formats

### Hex Files (`.hex`)
- Private keys: 64 karakter hex (32 bytes)
- Public keys: 66 karakter hex (33 bytes)
- Signatures: 128 karakter hex (64 bytes)

### Certificates
- `.cert` - Binary CBOR (canonical encoding)
- `.cert.base64` - Base64-encoded CBOR (untuk embedding)

### Payloads
- `.json` - JSON schema untuk CLI
- CBOR fields menggunakan short names: `v`, `pid`, `lid`, `iat`, `exp`, `upk`, `tier`, `feat`, `lim`, `meta`

---

## 🔄 Changelog

### v1.0.0 (2026-01-15)

**Documentation Reorganization:**
- 📁 Moved dev docs to `internal/docs/` (QUALITY, DOD, TASKS)
- 📝 Rewrote README.md to be concise and marketing-friendly
- 🗂️ Separated public docs (`docs/`) from internal docs
- 🔗 Updated all documentation links

**License Feature System:**
- ✨ Added tier-based licensing (lifetime_pro, campus)
- ✨ Changed features from u64 bitmask to Vec<String>
- ✨ Added `Limits` struct (seat_max, offline_grace_days)
- ✨ Added `Metadata` struct (product, version, university)
- ✨ Implemented `has_feature()` with "ALL" wildcard
- ✅ Campus tier expiry validation
- 🧪 4 new tests for feature system

**CLI Updates:**
- 🔧 Updated payload validation for new schema
- 🔧 Enhanced issue command output
- 🔧 Enhanced inspect command (displays tier, features, limits, metadata)
- 📝 Updated example payloads

**CI/CD:**
- 🚀 Added fixture validation to CI pipeline
- 🚀 Workspace-wide clippy with all targets
- 🚀 All features testing

**Configuration:**
- ⚙️ Added `.gitattributes` for consistent line endings
- ⚙️ Enhanced `.gitignore` for Rust + IDE
- ⚙️ Updated repository URL
- ⚙️ Set minimum Rust version (1.70)

**Tests:** 37/37 passing ✅

---

## 📚 Additional Resources

- **Repository:** https://github.com/Falcoraxyz/Aegis-CRM-Standard
- **License:** AGPL-3.0-or-later
- **Rust Version:** 1.70+
- **Security Policy:** [`SECURITY.md`](SECURITY.md)
- **Contributing Guide:** [`CONTRIBUTING.md`](CONTRIBUTING.md)
- **Code of Conduct:** [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)

---

**Terakhir diupdate:** 2026-01-15  
**Versi:** 1.0.0  
**Status:** Production Ready ✅
