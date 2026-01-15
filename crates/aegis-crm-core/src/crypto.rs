//! Cryptographic primitives for Aegis CRM.

use crate::errors::AegisError;
use k256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use k256::EncodedPoint;
use sha2::{Digest, Sha256};

/// Compute SHA-256 hash of input bytes.
pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

/// Sign a digest using ECDSA secp256k1, returning compact 64-byte signature (r||s).
pub fn sign_compact(privkey: &[u8; 32], digest: &[u8; 32]) -> Result<[u8; 64], AegisError> {
    let signing_key = SigningKey::from_bytes(privkey.into())
        .map_err(|e| AegisError::Crypto(format!("Invalid signing key: {}", e)))?;

    let signature: Signature = signing_key.sign(digest);
    let sig_bytes = signature.to_bytes();

    Ok(sig_bytes.into())
}

/// Verify an ECDSA secp256k1 signature.
pub fn verify_compact(
    pubkey: &[u8; 33],
    digest: &[u8; 32],
    sig: &[u8; 64],
) -> Result<(), AegisError> {
    let verifying_key = VerifyingKey::from_sec1_bytes(pubkey)
        .map_err(|e| AegisError::Crypto(format!("Invalid public key: {}", e)))?;

    let signature = Signature::from_bytes(sig.into())
        .map_err(|e| AegisError::Crypto(format!("Invalid signature format: {}", e)))?;

    verifying_key
        .verify(digest, &signature)
        .map_err(|_| AegisError::Crypto("Signature verification failed".to_string()))?;

    Ok(())
}

/// Derive compressed public key from private key.
pub fn pubkey_from_privkey_inner(privkey: &[u8; 32]) -> Result<[u8; 33], AegisError> {
    let signing_key = SigningKey::from_bytes(privkey.into())
        .map_err(|e| AegisError::Key(format!("Invalid private key: {}", e)))?;

    let verifying_key = signing_key.verifying_key();
    let encoded = EncodedPoint::from(verifying_key);
    let compressed = encoded.compress();

    let bytes = compressed.as_bytes();
    if bytes.len() != 33 {
        return Err(AegisError::Key("Public key compression failed".to_string()));
    }

    let mut result = [0u8; 33];
    result.copy_from_slice(bytes);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let data = b"hello world";
        let hash = sha256(data);
        assert_eq!(hash.len(), 32);
        let hash2 = sha256(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_sign_verify_roundtrip() {
        let privkey = [0x42u8; 32];
        let message = b"test message";
        let digest = sha256(message);

        let signature = sign_compact(&privkey, &digest).expect("Signing failed");
        assert_eq!(signature.len(), 64);

        let pubkey = pubkey_from_privkey_inner(&privkey).expect("Pubkey derivation failed");
        assert_eq!(pubkey.len(), 33);

        verify_compact(&pubkey, &digest, &signature).expect("Verification failed");
    }

    #[test]
    fn test_verify_invalid_signature() {
        let privkey = [0x42u8; 32];
        let digest = sha256(b"test");

        let signature = sign_compact(&privkey, &digest).expect("Signing failed");
        let pubkey = pubkey_from_privkey_inner(&privkey).expect("Pubkey derivation failed");

        let mut bad_sig = signature;
        bad_sig[0] ^= 0xFF;

        assert!(verify_compact(&pubkey, &digest, &bad_sig).is_err());
    }

    #[test]
    fn test_pubkey_from_privkey() {
        let privkey = [0x33u8; 32];
        let pubkey = pubkey_from_privkey_inner(&privkey).expect("Failed to derive pubkey");
        assert_eq!(pubkey.len(), 33);
        assert!(pubkey[0] == 0x02 || pubkey[0] == 0x03);
    }
}
