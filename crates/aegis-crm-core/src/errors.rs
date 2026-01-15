//! Error types for Aegis CRM Standard.

use thiserror::Error;

/// Standard error codes for Aegis CRM operations.
#[derive(Error, Debug)]
pub enum AegisError {
    /// CBOR parsing failed or invalid certificate structure.
    #[error("Certificate parsing error: invalid CBOR structure")]
    CertParse,

    /// Protocol version not supported.
    #[error("Unsupported protocol version")]
    UnsupportedVersion,

    /// Vendor signature verification failed.
    #[error("Invalid certificate signature")]
    CertSignature,

    /// Certificate has expired.
    #[error("Certificate expired")]
    CertExpired,

    /// Proof-of-Possession signature verification failed.
    #[error("Invalid proof-of-possession signature")]
    PopSignature,

    /// Key operation failed (invalid format or derivation error).
    #[error("Key operation error: {0}")]
    Key(String),

    /// Cryptographic operation failed.
    #[error("Cryptographic operation error: {0}")]
    Crypto(String),
}
