Generated: 2026-01-12 14:20 UTC

# Public Rust API (Freeze)

Goal: prevent refactors caused by mismatched function names/types. The agent MUST implement this API exactly (module names, function signatures, error types).

crate: aegis-crm-core

pub mod keys {  
pub struct VendorKeypair { pub privkey: \[u8; 32\], pub pubkey: \[u8; 33\] }  
pub struct UserKeypair { pub privkey: \[u8; 32\], pub pubkey: \[u8; 33\] }  
pub fn vendor_keygen() -\> VendorKeypair;  
pub fn user_keygen() -\> UserKeypair;  
pub fn pubkey_from_privkey(privkey: &\[u8; 32\]) -\> Result\<\[u8; 33\], AegisError\>;  
}

pub mod cert {  
pub const PROTOCOL_VERSION: u16 = 1;

pub struct LicensePayload {  
pub version: u16,  
pub product_id: String,  
pub license_id: \[u8; 32\],  
pub issued_at: u64,  
pub expiry: Option\<u64\>,  
pub user_pubkey: \[u8; 33\],  
pub features: u64,  
}

pub struct LicenseCert {  
pub payload: LicensePayload,  
pub vendor_sig: \[u8; 64\],  
}

pub fn issue_cert(vendor_privkey: &\[u8; 32\], payload: LicensePayload)  
-\> Result\<LicenseCert, AegisError\>;

pub fn verify_cert(vendor_pubkey: &\[u8; 33\], cert: &LicenseCert, now_unix: u64)  
-\> Result\<(), AegisError\>;

pub fn encode_cert(cert: &LicenseCert) -\> Result\<Vec\<u8\>, AegisError\>;  
pub fn decode_cert(cbor: &\[u8\]) -\> Result\<LicenseCert, AegisError\>;  
}

pub mod pop {  
pub type Nonce32 = \[u8; 32\];  
pub type PopSignature = \[u8; 64\];

pub fn challenge() -\> Nonce32;  
pub fn prove(user_privkey: &\[u8; 32\], nonce: &Nonce32)  
-\> Result\<PopSignature, AegisError\>;

pub fn verify(user_pubkey: &\[u8; 33\], nonce: &Nonce32, sig: &PopSignature)  
-\> Result\<(), AegisError\>;  
}

pub mod verify {  
pub fn verify_license(  
vendor_pubkey: &\[u8; 33\],  
cert: &LicenseCert,  
now_unix: u64,  
nonce: &Nonce32,  
pop_sig: &PopSignature,  
) -\> Result\<(), AegisError\>;  
}

pub mod errors {  
\#\[derive(thiserror::Error, Debug)\]  
pub enum AegisError {  
E_CERT_PARSE,  
E_UNSUPPORTED_VER,  
E_CERT_SIG,  
E_CERT_EXPIRED,  
E_POP_SIG,  
E_KEY,  
E_CRYPTO,  
}  
}

# Encoding Rules

All certificates MUST be encoded with Canonical CBOR. Signature MUST be compact 64-bytes (r\|\|s). Public keys MUST be compressed 33 bytes.

# No-Go List (Agent MUST NOT do)

Do NOT implement custom cryptography. Use audited crates.

Do NOT deviate from module names or public function signatures.

Do NOT use unsafe Rust (except if explicitly permitted later).

Do NOT serialize CERT using JSON or non-canonical CBOR.
