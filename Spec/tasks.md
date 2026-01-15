# Aegis CRM CLI - Implementation Tasks

## Phase 1: Project Setup
- [ ] Create `crates/aegis-crm-cli/` directory
- [ ] Create `Cargo.toml` with dependencies
- [ ] Update workspace `Cargo.toml` to include CLI crate
- [ ] Create basic `main.rs` with clap setup

## Phase 2: Core Infrastructure
- [ ] Implement `error.rs` with custom error types
- [ ] Implement `utils/io.rs` - file I/O helpers
  - [ ] `read_hex_or_file()` - read hex string or file path
  - [ ] `write_hex_file()` - write hex to file
  - [ ] `validate_output_path()` - check for path traversal
  - [ ] `set_restrictive_permissions()` - Unix 0600
- [ ] Implement `utils/hex.rs` - hex encoding utilities
- [ ] Implement `utils/payload.rs` - JSON payload validation

## Phase 3: Command Implementation

### Vendor Keygen
- [ ] Implement `commands/vendor.rs`
- [ ] Add `keygen` subcommand handler
- [ ] Implement output directory creation
- [ ] Add overwrite protection with `--force`
- [ ] Add security warning display
- [ ] Add `--json` output format
- [ ] Test: successful keygen
- [ ] Test: refuse overwrite without force

### User Keygen
- [ ] Implement `commands/user.rs`
- [ ] Copy vendor keygen logic adapted for users
- [ ] Test: successful keygen
- [ ] Test: refuse overwrite without force

### Certificate Issuance
- [ ] Implement `commands/issue.rs`
- [ ] Parse JSON payload
- [ ] Validate payload schema
  - [ ] Check plan is lifetime_pro or campus
  - [ ] Validate campus has expires_at
  - [ ] Parse features array
- [ ] Convert features to bitmask (first 8 → bits 0-7)
- [ ] Generate random license_id
- [ ] Load vendor private key
- [ ] Load/parse user public key
- [ ] Issue certificate using core library
- [ ] Write binary CBOR output
- [ ] Write base64 output
- [ ] Add `--json` summary output
- [ ] Test: successful issuance
- [ ] Test: campus validation
- [ ] Test: feature bitmask conversion

### Certificate Inspection
- [ ] Implement `commands/inspect.rs`
- [ ] Load and decode certificate
- [ ] Verify signature
- [ ] Check expiry status
- [ ] Format human-readable output
- [ ] Add `--json` output
- [ ] Test: inspect valid cert
- [ ] Test: inspect expired cert
- [ ] Test: inspect tampered cert

### PoP Operations
- [ ] Implement `commands/pop.rs`
- [ ] Implement `challenge` subcommand
- [ ] Implement `prove` subcommand
- [ ] Implement `verify` subcommand
- [ ] Test: challenge generates valid nonce
- [ ] Test: prove/verify roundtrip
- [ ] Test: invalid signature fails

## Phase 4: Example Payloads
- [ ] Create `examples/payloads/lifetime_pro.json`
- [ ] Create `examples/payloads/campus.json`

## Phase 5: Documentation
- [ ] Create `docs/CLI.md`
  - [ ] Installation instructions
  - [ ] Command reference
  - [ ] Usage examples
  - [ ] Payload schema
- [ ] Add CLI section to main README.md
- [ ] Update CHANGELOG.md

## Phase 6: Integration Tests
- [ ] Test: payload validation (campus without expires_at)
- [ ] Test: issue + inspect roundtrip
- [ ] Test: pop prove + verify flow
- [ ] Test: file overwrite protection
- [ ] Test: invalid hex input handling

## Phase 7: Quality Assurance
- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy -- -D warnings`
- [ ] Run `cargo test --all`
- [ ] Verify all tests pass
- [ ] Check code coverage
- [ ] Manual testing on Windows
- [ ] Manual testing on Linux (if available)

## Phase 8: Final Validation
- [ ] All commands work as expected
- [ ] Help text is clear
- [ ] Error messages are actionable
- [ ] Security warnings are prominent
- [ ] Documentation is complete
- [ ] Examples work correctly
- [ ] CI passes

## Success Criteria
- [ ] All 5 commands implemented and tested
- [ ] ≥3 integration tests passing
- [ ] Zero clippy warnings
- [ ] All tests pass
- [ ] Documentation complete with examples
- [ ] Binary builds on target platforms
