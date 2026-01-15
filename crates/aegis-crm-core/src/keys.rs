//! Key generation and management for Aegis CRM.

use crate::crypto::pubkey_from_privkey_inner;
use crate::errors::AegisError;
use getrandom::getrandom;

/// Vendor keypair (private + public key).
#[derive(Debug, Clone)]
pub struct VendorKeypair {
    pub privkey: [u8; 32],
    pub pubkey: [u8; 33],
}

/// User keypair (private + public key).
#[derive(Debug, Clone)]
pub struct UserKeypair {
    pub privkey: [u8; 32],
    pub pubkey: [u8; 33],
}

/// Generate a new vendor keypair using CSPRNG.
pub fn vendor_keygen() -> VendorKeypair {
    let mut privkey = [0u8; 32];
    getrandom(&mut privkey).expect("Failed to generate random bytes");
    let pubkey = pubkey_from_privkey_inner(&privkey).expect("Failed to derive public key");
    VendorKeypair { privkey, pubkey }
}

/// Generate a new user keypair using CSPRNG.
pub fn user_keygen() -> UserKeypair {
    let mut privkey = [0u8; 32];
    getrandom(&mut privkey).expect("Failed to generate random bytes");
    let pubkey = pubkey_from_privkey_inner(&privkey).expect("Failed to derive public key");
    UserKeypair { privkey, pubkey }
}

/// Derive public key from private key.
pub fn pubkey_from_privkey(privkey: &[u8; 32]) -> Result<[u8; 33], AegisError> {
    pubkey_from_privkey_inner(privkey)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vendor_keygen() {
        let vendor = vendor_keygen();
        assert_eq!(vendor.privkey.len(), 32);
        assert_eq!(vendor.pubkey.len(), 33);
        assert!(vendor.pubkey[0] == 0x02 || vendor.pubkey[0] == 0x03);
    }

    #[test]
    fn test_user_keygen() {
        let user = user_keygen();
        assert_eq!(user.privkey.len(), 32);
        assert_eq!(user.pubkey.len(), 33);
    }

    #[test]
    fn test_pubkey_from_privkey() {
        let user = user_keygen();
        let derived = pubkey_from_privkey(&user.privkey).unwrap();
        assert_eq!(derived, user.pubkey);
    }

    #[test]
    fn test_keygen_randomness() {
        let v1 = vendor_keygen();
        let v2 = vendor_keygen();
        assert_ne!(v1.privkey, v2.privkey);
    }
}
