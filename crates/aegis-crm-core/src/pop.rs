//! Proof-of-Possession (PoP) for license ownership.

use crate::crypto::{sha256, sign_compact, verify_compact};
use crate::errors::AegisError;
use getrandom::getrandom;

pub type Nonce32 = [u8; 32];
pub type PopSignature = [u8; 64];

pub fn challenge() -> Nonce32 {
    let mut nonce = [0u8; 32];
    getrandom(&mut nonce).expect("Failed to generate nonce");
    nonce
}

pub fn prove(user_privkey: &[u8; 32], nonce: &Nonce32) -> Result<PopSignature, AegisError> {
    let digest = sha256(nonce);
    sign_compact(user_privkey, &digest)
}

pub fn verify(
    user_pubkey: &[u8; 33],
    nonce: &Nonce32,
    sig: &PopSignature,
) -> Result<(), AegisError> {
    let digest = sha256(nonce);
    verify_compact(user_pubkey, &digest, sig).map_err(|_| AegisError::PopSignature)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::user_keygen;

    #[test]
    fn test_challenge() {
        let n1 = challenge();
        let n2 = challenge();
        assert_eq!(n1.len(), 32);
        assert_ne!(n1, n2);
    }

    #[test]
    fn test_pop_flow() {
        let user = user_keygen();
        let nonce = challenge();
        let sig = prove(&user.privkey, &nonce).unwrap();
        verify(&user.pubkey, &nonce, &sig).unwrap();
    }

    #[test]
    fn test_pop_invalid() {
        let user = user_keygen();
        let nonce = challenge();
        let mut sig = prove(&user.privkey, &nonce).unwrap();
        sig[0] ^= 0xFF;
        assert!(matches!(
            verify(&user.pubkey, &nonce, &sig),
            Err(AegisError::PopSignature)
        ));
    }

    #[test]
    fn test_pop_wrong_nonce() {
        let user = user_keygen();
        let n1 = challenge();
        let n2 = challenge();
        let sig = prove(&user.privkey, &n1).unwrap();
        assert!(verify(&user.pubkey, &n2, &sig).is_err());
    }
}
