//! Integration tests for Aegis CRM Standard

use aegis_crm_core::{
    cert::{decode_cert, encode_cert, issue_cert, verify_cert, LicensePayload, PROTOCOL_VERSION},
    keys::{pubkey_from_privkey, user_keygen, vendor_keygen},
    pop::{challenge, prove, verify as verify_pop},
    verify::verify_license,
    AegisError,
};
use getrandom::getrandom;

#[test]
fn test_complete_license_flow() {
    let vendor = vendor_keygen();
    let user = user_keygen();
    let mut license_id = [0u8; 32];
    getrandom(&mut license_id).unwrap();

    let payload = LicensePayload {
        version: PROTOCOL_VERSION,
        product_id: "integration_test_app".to_string(),
        license_id,
        issued_at: 1700000000,
        expiry: Some(2000000000),
        user_pubkey: user.pubkey,
        tier: "lifetime_pro".to_string(),
        features: vec![
            "base".to_string(),
            "premium".to_string(),
            "advanced".to_string(),
        ],
        limits: None,
        metadata: None,
    };

    let cert = issue_cert(&vendor.privkey, payload).expect("Failed to issue cert");
    let cbor = encode_cert(&cert).expect("Failed to encode");
    let decoded = decode_cert(&cbor).expect("Failed to decode");

    assert_eq!(decoded.payload.product_id, cert.payload.product_id);
    verify_cert(&vendor.pubkey, &decoded, 1800000000).expect("Cert verification failed");

    let nonce = challenge();
    let pop_sig = prove(&user.privkey, &nonce).expect("Failed to prove");
    verify_pop(&user.pubkey, &nonce, &pop_sig).expect("PoP verification failed");

    verify_license(&vendor.pubkey, &decoded, 1800000000, &nonce, &pop_sig)
        .expect("License verification failed");
}

#[test]
fn test_tampered_certificate_fails() {
    let vendor = vendor_keygen();
    let user = user_keygen();
    let mut license_id = [0u8; 32];
    getrandom(&mut license_id).unwrap();

    let payload = LicensePayload {
        version: PROTOCOL_VERSION,
        product_id: "test_app".to_string(),
        license_id,
        issued_at: 1700000000,
        expiry: None,
        user_pubkey: user.pubkey,
        tier: "lifetime_pro".to_string(),
        features: vec!["base".to_string()],
        limits: None,
        metadata: None,
    };

    let mut cert = issue_cert(&vendor.privkey, payload).expect("Failed to issue cert");
    cert.vendor_sig[0] ^= 0xFF;

    let result = verify_cert(&vendor.pubkey, &cert, 1800000000);
    assert!(matches!(result, Err(AegisError::CertSignature)));
}

#[test]
fn test_expired_certificate_fails() {
    let vendor = vendor_keygen();
    let user = user_keygen();
    let mut license_id = [0u8; 32];
    getrandom(&mut license_id).unwrap();

    let payload = LicensePayload {
        version: PROTOCOL_VERSION,
        product_id: "test_app".to_string(),
        license_id,
        issued_at: 1700000000,
        expiry: Some(1900000000),
        user_pubkey: user.pubkey,
        tier: "campus".to_string(),
        features: vec!["education".to_string()],
        limits: None,
        metadata: None,
    };

    let cert = issue_cert(&vendor.privkey, payload).expect("Failed to issue cert");
    let result = verify_cert(&vendor.pubkey, &cert, 2000000000);
    assert!(matches!(result, Err(AegisError::CertExpired)));
}

#[test]
fn test_invalid_pop_fails() {
    let vendor = vendor_keygen();
    let user = user_keygen();
    let wrong_user = user_keygen();
    let mut license_id = [0u8; 32];
    getrandom(&mut license_id).unwrap();

    let payload = LicensePayload {
        version: PROTOCOL_VERSION,
        product_id: "test_app".to_string(),
        license_id,
        issued_at: 1700000000,
        expiry: None,
        user_pubkey: user.pubkey,
        tier: "lifetime_pro".to_string(),
        features: vec!["base".to_string()],
        limits: None,
        metadata: None,
    };

    let cert = issue_cert(&vendor.privkey, payload).expect("Failed to issue cert");
    let nonce = challenge();
    let pop_sig = prove(&wrong_user.privkey, &nonce).expect("Failed to prove");

    let result = verify_license(&vendor.pubkey, &cert, 1800000000, &nonce, &pop_sig);
    assert!(matches!(result, Err(AegisError::PopSignature)));
}

#[test]
fn test_cbor_encode_decode_stability() {
    let vendor = vendor_keygen();
    let user = user_keygen();
    let mut license_id = [0u8; 32];
    getrandom(&mut license_id).unwrap();

    let payload = LicensePayload {
        version: PROTOCOL_VERSION,
        product_id: "stability_test".to_string(),
        license_id,
        issued_at: 1700000000,
        expiry: None,
        user_pubkey: user.pubkey,
        tier: "lifetime_pro".to_string(),
        features: vec!["ALL".to_string()],
        limits: None,
        metadata: None,
    };

    let cert = issue_cert(&vendor.privkey, payload).expect("Failed to issue cert");

    for _ in 0..5 {
        let cbor = encode_cert(&cert).expect("Encoding failed");
        let decoded = decode_cert(&cbor).expect("Decoding failed");
        let cbor2 = encode_cert(&decoded).expect("Re-encoding failed");
        assert_eq!(cbor, cbor2, "CBOR encoding not stable");
    }
}

#[test]
fn test_pubkey_derivation_consistency() {
    let user = user_keygen();
    let derived = pubkey_from_privkey(&user.privkey).expect("Failed to derive pubkey");
    assert_eq!(derived, user.pubkey);
}
