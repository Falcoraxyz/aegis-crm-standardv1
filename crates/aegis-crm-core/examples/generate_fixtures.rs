//! Generate deterministic fixtures for golden vector tests

use aegis_crm_core::{
    cert::{encode_cert, issue_cert, LicensePayload, Limits, Metadata, PROTOCOL_VERSION},
    crypto::pubkey_from_privkey_inner,
    pop::{prove, PopSignature},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use std::fs;
use std::path::Path;

fn main() {
    println!("Generating golden vector fixtures...");

    let vendor_priv: [u8; 32] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20,
    ];

    let user_priv: [u8; 32] = [
        0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
        0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e,
        0x3f, 0x40,
    ];

    let vendor_pub = pubkey_from_privkey_inner(&vendor_priv).expect("Failed to derive vendor pub");
    let user_pub = pubkey_from_privkey_inner(&user_priv).expect("Failed to derive user pub");

    let license_id: [u8; 32] = [
        0x9f, 0x33, 0xae, 0x12, 0x6e, 0xcd, 0x09, 0x8d, 0xbc, 0x3e, 0xcf, 0x02, 0x06, 0x8c, 0x9f,
        0x2f, 0x06, 0xa0, 0xb3, 0x58, 0x0bd, 0x44, 0x88, 0x60, 0x49, 0x48, 0xa2, 0x93, 0x13, 0x06,
        0x67, 0x86,
    ];

    // Lifetime Pro payload
    let payload_lifetime = LicensePayload {
        version: PROTOCOL_VERSION,
        product_id: "example_product".to_string(),
        license_id,
        issued_at: 1768227654,
        expiry: None,
        user_pubkey: user_pub,
        tier: "lifetime_pro".to_string(),
        features: vec!["base_access".to_string(), "premium_features".to_string()],
        limits: Some(Limits {
            seat_max: Some(1),
            offline_grace_days: None,
        }),
        metadata: Some(Metadata {
            product: Some("Example Product".to_string()),
            version: Some("1.0".to_string()),
            university: None,
        }),
    };

    let cert = issue_cert(&vendor_priv, payload_lifetime).expect("Failed to issue cert");
    let nonce: [u8; 32] = [0xaa; 32];
    let pop_sig: PopSignature = prove(&user_priv, &nonce).expect("Failed to prove");
    let cert_cbor = encode_cert(&cert).expect("Failed to encode cert");

    let fixtures_dir = Path::new("fixtures");

    fs::write(
        fixtures_dir.join("vendor_priv.hex"),
        hex::encode(vendor_priv),
    )
    .expect("Failed to write vendor_priv.hex");
    fs::write(fixtures_dir.join("vendor_pub.hex"), hex::encode(vendor_pub))
        .expect("Failed to write vendor_pub.hex");
    fs::write(fixtures_dir.join("user_priv.hex"), hex::encode(user_priv))
        .expect("Failed to write user_priv.hex");
    fs::write(fixtures_dir.join("user_pub.hex"), hex::encode(user_pub))
        .expect("Failed to write user_pub.hex");
    fs::write(fixtures_dir.join("pop_nonce.hex"), hex::encode(nonce))
        .expect("Failed to write pop_nonce.hex");
    fs::write(fixtures_dir.join("pop_sig.hex"), hex::encode(pop_sig))
        .expect("Failed to write pop_sig.hex");
    fs::write(fixtures_dir.join("license.cert"), &cert_cbor).expect("Failed to write license.cert");
    fs::write(
        fixtures_dir.join("license.cert.base64"),
        STANDARD.encode(&cert_cbor),
    )
    .expect("Failed to write license.cert.base64");

    let payload_json = serde_json::json!({
        "v": 1,
        "pid": "example_product",
        "lid": hex::encode(license_id),
        "iat": 1768227654u64,
        "exp": null,
        "upk": hex::encode(user_pub),
        "tier": "lifetime_pro",
        "feat": ["base_access", "premium_features"],
        "lim": {"seats": 1},
        "meta": {"product": "Example Product", "version": "1.0"}
    });

    fs::write(
        fixtures_dir.join("license_payload.json"),
        serde_json::to_string_pretty(&payload_json).unwrap(),
    )
    .expect("Failed to write license_payload.json");

    println!("✅ Fixtures generated successfully!");
    println!("  - vendor_priv.hex");
    println!("  - vendor_pub.hex");
    println!("  - user_priv.hex");
    println!("  - user_pub.hex");
    println!("  - license_payload.json");
    println!("  - license.cert");
    println!("  - license.cert.base64");
    println!("  - pop_nonce.hex");
    println!("  - pop_sig.hex");
}
