# Aegis CRM CLI - Requirements

## Functional Requirements

### FR1: Vendor Key Generation
- **Priority**: P0 (Critical)
- **Description**: Generate vendor keypair for license signing
- **Acceptance Criteria**:
  - Generates secp256k1 keypair using CSPRNG
  - Outputs to configurable directory (default: `./vendor_keys/`)
  - Writes `vendor_priv.hex` (32 bytes) and `vendor_pub.hex` (33 bytes)
  - Displays security warning about private key storage
  - Refuses to overwrite existing files without `--force`
  - Supports `--json` machine-readable output

### FR2: User Key Generation
- **Priority**: P0 (Critical)
- **Description**: Generate user keypair for license ownership
- **Acceptance Criteria**:
  - Same as FR1 but for user keys
  - Default output: `./user_keys/`

### FR3: Certificate Issuance
- **Priority**: P0 (Critical)
- **Description**: Issue signed license certificates from payload JSON
- **Acceptance Criteria**:
  - Accepts JSON payload with plan, features, expiry, metadata
  - Validates `campus` plan has non-null `expires_at`
  - Generates random 32-byte `license_id`
  - Maps feature strings to bitmask (first 8 features → bits 0-7)
  - Outputs binary CBOR certificate + base64 variant
  - Refuses to overwrite without `--force`

### FR4: Certificate Inspection
- **Priority**: P0 (Critical)
- **Description**: Decode and verify existing certificates
- **Acceptance Criteria**:
  - Decodes CBOR certificate
  - Verifies vendor signature
  - Displays all payload fields (except no private keys)
  - Shows validity status and time-to-expiry
  - Supports`--json` output

### FR5: Proof-of-Possession Operations
- **Priority**: P1 (High)
- **Description**: PoP challenge/prove/verify for testing
- **Acceptance Criteria**:
  - `challenge`: Generates 32-byte random nonce
  - `prove`: Signs nonce with user private key
  - `verify`: Validates PoP signature, exits 0/1

## Non-Functional Requirements

### NFR1: Security
- **Priority**: P0 (Critical)
- Never expose private keys in stdout (unless `--json` excludes them)
- Warn user about vendor key security
- Validate all file paths (reject `..` components)
- Set restrictive permissions on private key files (0600 on Unix)

### NFR2: Usability
- **Priority**: P0 (Critical)
- Clear, actionable error messages
- Human-readable default output
- Machine-readable `--json` option
- Help text for all commands (`--help`)

### NFR3: Reliability
- **Priority**: P0 (Critical)
- Deterministic outputs (except keygen, license_id)
- Robust error handling with proper exit codes
- No panics in production code

### NFR4: Performance
- **Priority**: P2 (Medium)
- All operations complete in <1 second
- No unnecessary allocations

### NFR5: Portability
- **Priority**: P0 (Critical)
- Cross-platform (Windows, Linux, macOS)
- No platform-specific code except file permissions

## Constraints

### Hard Constraints
1. **NO network calls**: Entirely offline
2. **NO database**: File-based only
3. **NO custom crypto**: Use `aegis-crm-core` exclusively
4. **NO breaking changes**: Cannot modify core library API

### Soft Constraints
1. Minimal dependencies
2. Fast compile times
3. Small binary size

## Edge Cases

1. **Invalid hex input**: Clear error, suggest format
2. **Corrupted files**: Graceful failure with context
3. **Expired certificates**: Mark as EXPIRED in inspect
4. **Missing files**: Check before reading, clear error
5. **Invalid UTF-8 in paths**: Handle gracefully
6. **Very long feature lists**: Warn if >64 features (bitmask limit)

## Success Metrics

- All commands pass integration tests
- CI passes (fmt, clippy, test)
- Clear documentation with examples
- Zero clippy warnings
- Code coverage >80%
