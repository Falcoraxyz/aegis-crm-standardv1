//! Certificate issuance command

use crate::error::Result;
use crate::utils::io::{check_overwrite, read_hex_or_file};
use crate::utils::payload::{LicensePayloadJson, LimitsJson, MetadataJson};
use aegis_crm_core::cert::{
    encode_cert, issue_cert, LicensePayload, Limits, Metadata, PROTOCOL_VERSION,
};
use anyhow::Context;
use base64::{engine::general_purpose::STANDARD, Engine};
use getrandom::getrandom;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct IssueOutput {
    license_id: String,
    tier: String,
    expires_at: Option<u64>,
    features_count: usize,
    cert_path: String,
    cert_base64_path: String,
}

pub fn run(
    vendor_priv_path: String,
    user_pub: String,
    payload_path: PathBuf,
    out_path: PathBuf,
    force: bool,
    json: bool,
) -> Result<()> {
    // Check output paths
    check_overwrite(&out_path, force)?;
    let base64_path = out_path.with_extension("cert.base64");
    check_overwrite(&base64_path, force)?;

    // Load vendor private key
    let vendor_priv_bytes =
        read_hex_or_file(&vendor_priv_path).context("Failed to read vendor private key")?;
    if vendor_priv_bytes.len() != 32 {
        anyhow::bail!(
            "Vendor private key must be 32 bytes, got {}",
            vendor_priv_bytes.len()
        );
    }
    let mut vendor_priv = [0u8; 32];
    vendor_priv.copy_from_slice(&vendor_priv_bytes);

    // Load user public key
    let user_pub_bytes = read_hex_or_file(&user_pub).context("Failed to read user public key")?;
    if user_pub_bytes.len() != 33 {
        anyhow::bail!(
            "User public key must be 33 bytes, got {}",
            user_pub_bytes.len()
        );
    }
    let mut user_pubkey = [0u8; 33];
    user_pubkey.copy_from_slice(&user_pub_bytes);

    // Load and parse payload
    let payload_json_str = fs::read_to_string(&payload_path)
        .with_context(|| format!("Failed to read payload file: {}", payload_path.display()))?;
    let payload_json: LicensePayloadJson =
        serde_json::from_str(&payload_json_str).context("Failed to parse payload JSON")?;

    // Validate payload
    payload_json.validate()?;

    // Generate random license ID
    let mut license_id = [0u8; 32];
    getrandom(&mut license_id).context("Failed to generate random license ID")?;

    // Get current timestamp
    let issued_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    // Convert limits to core format
    let limits = payload_json.limits.as_ref().map(|l: &LimitsJson| Limits {
        seat_max: l.seat_max,
        offline_grace_days: l.offline_grace_days,
    });

    // Convert metadata to core format
    let metadata = payload_json
        .metadata
        .as_ref()
        .map(|m: &MetadataJson| Metadata {
            product: m.product.clone(),
            version: m.version.clone(),
            university: m.university.clone(),
        });

    // Create core library payload
    let payload = LicensePayload {
        version: PROTOCOL_VERSION,
        product_id: payload_json.tier_string(),
        license_id,
        issued_at,
        expiry: payload_json.expires_at,
        user_pubkey,
        tier: payload_json.tier_string(),
        features: payload_json.features.clone(),
        limits,
        metadata,
    };

    // Issue certificate
    let cert = issue_cert(&vendor_priv, payload).context("Failed to issue certificate")?;

    // Encode to CBOR
    let cert_cbor = encode_cert(&cert).context("Failed to encode certificate")?;

    // Write binary CBOR
    fs::write(&out_path, &cert_cbor)
        .with_context(|| format!("Failed to write certificate: {}", out_path.display()))?;

    // Write base64 variant
    let cert_base64 = STANDARD.encode(&cert_cbor);
    fs::write(&base64_path, cert_base64).with_context(|| {
        format!(
            "Failed to write base64 certificate: {}",
            base64_path.display()
        )
    })?;

    // Output results
    if json {
        let output = IssueOutput {
            license_id: hex::encode(license_id),
            tier: payload_json.tier_string(),
            expires_at: payload_json.expires_at,
            features_count: payload_json.features.len(),
            cert_path: out_path.display().to_string(),
            cert_base64_path: base64_path.display().to_string(),
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("✅ License certificate issued successfully!");
        println!("   License ID: {}", hex::encode(&license_id[..8]));
        println!("   Tier: {}", payload_json.tier_string());
        if let Some(exp) = payload_json.expires_at {
            println!("   Expires: {}", exp);
        } else {
            println!("   Expires: Never (perpetual)");
        }
        println!(
            "   Features: {} ({})",
            payload_json.features.len(),
            payload_json.features.join(", ")
        );
        if let Some(lim) = &payload_json.limits {
            if let Some(seats) = lim.seat_max {
                println!("   Seat Limit: {}", seats);
            }
        }
        println!("   Certificate: {}", out_path.display());
        println!("   Base64: {}", base64_path.display());
    }

    Ok(())
}
