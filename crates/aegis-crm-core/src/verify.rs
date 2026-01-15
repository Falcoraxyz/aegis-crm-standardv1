//! Unified license verification (Certificate + PoP).

use crate::cert::{verify_cert, LicenseCert};
use crate::errors::AegisError;
use crate::pop::{verify as verify_pop, Nonce32, PopSignature};

pub fn verify_license(
    vendor_pubkey: &[u8; 33],
    cert: &LicenseCert,
    now_unix: u64,
    nonce: &Nonce32,
    pop_sig: &PopSignature,
) -> Result<(), AegisError> {
    verify_cert(vendor_pubkey, cert, now_unix)?;
    verify_pop(&cert.payload.user_pubkey, nonce, pop_sig)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cert::{issue_cert, LicensePayload, PROTOCOL_VERSION};
    use crate::keys::{user_keygen, vendor_keygen};
    use crate::pop::{challenge, prove};
    use getrandom::getrandom;

    #[test]
    fn test_verify_license_valid() {
        let vendor = vendor_keygen();
        let user = user_keygen();
        let mut license_id = [0u8; 32];
        getrandom(&mut license_id).unwrap();
        let payload = LicensePayload {
            version: PROTOCOL_VERSION,
            product_id: "test".to_string(),
            license_id,
            issued_at: 1700000000,
            expiry: Some(2000000000),
            user_pubkey: user.pubkey,
            tier: "lifetime_pro".to_string(),
            features: vec!["base".to_string(), "premium".to_string()],
            limits: None,
            metadata: None,
        };
        let cert = issue_cert(&vendor.privkey, payload).unwrap();
        let nonce = challenge();
        let pop_sig = prove(&user.privkey, &nonce).unwrap();
        verify_license(&vendor.pubkey, &cert, 1800000000, &nonce, &pop_sig).unwrap();
    }

    #[test]
    fn test_verify_license_invalid_pop() {
        let vendor = vendor_keygen();
        let user = user_keygen();
        let wrong = user_keygen();
        let mut license_id = [0u8; 32];
        getrandom(&mut license_id).unwrap();
        let payload = LicensePayload {
            version: PROTOCOL_VERSION,
            product_id: "test".to_string(),
            license_id,
            issued_at: 1700000000,
            expiry: None,
            user_pubkey: user.pubkey,
            tier: "lifetime_pro".to_string(),
            features: vec!["base".to_string()],
            limits: None,
            metadata: None,
        };
        let cert = issue_cert(&vendor.privkey, payload).unwrap();
        let nonce = challenge();
        let pop_sig = prove(&wrong.privkey, &nonce).unwrap();
        assert!(matches!(
            verify_license(&vendor.pubkey, &cert, 1800000000, &nonce, &pop_sig),
            Err(AegisError::PopSignature)
        ));
    }
}
