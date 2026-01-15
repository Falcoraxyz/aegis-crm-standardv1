//! Certificate inspection command

use crate::error::Result;
use crate::utils::io::read_hex_or_file;
use aegis_crm_core::cert::{decode_cert, verify_cert};
use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct InspectOutput {
    status: String,
    tier: String,
    product_id: String,
    license_id: String,
    issued_at: u64,
    issued_at_formatted: String,
    expiry: Option<u64>,
    expiry_formatted: Option<String>,
    features: Vec<String>,
    features_count: usize,
    user_pubkey: String,
}

pub fn run(cert_path: PathBuf, vendor_pub: String, json: bool) -> Result<()> {
    // Load certificate
    let cert_bytes = fs::read(&cert_path)
        .with_context(|| format!("Failed to read certificate: {}", cert_path.display()))?;

    let cert = decode_cert(&cert_bytes).context("Failed to decode certificate")?;

    // Load vendor public key
    let vendor_pub_bytes =
        read_hex_or_file(&vendor_pub).context("Failed to read vendor public key")?;
    if vendor_pub_bytes.len() != 33 {
        anyhow::bail!(
            "Vendor public key must be 33 bytes, got {}",
            vendor_pub_bytes.len()
        );
    }
    let mut vendor_pubkey = [0u8; 33];
    vendor_pubkey.copy_from_slice(&vendor_pub_bytes);

    // Get current timestamp
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    // Verify certificate
    let verification_result = verify_cert(&vendor_pubkey, &cert, now);
    let status = if verification_result.is_ok() {
        "VALID"
    } else if matches!(
        verification_result,
        Err(aegis_crm_core::AegisError::CertExpired)
    ) {
        "EXPIRED"
    } else {
        "INVALID"
    };

    // Format timestamps
    let issued_at_dt = DateTime::from_timestamp(cert.payload.issued_at as i64, 0)
        .unwrap_or(DateTime::<Utc>::MIN_UTC);
    let issued_at_formatted = issued_at_dt.format("%Y-%m-%d %H:%M:%S UTC").to_string();

    let expiry_formatted = cert.payload.expiry.map(|exp| {
        let exp_dt = DateTime::from_timestamp(exp as i64, 0).unwrap_or(DateTime::<Utc>::MIN_UTC);
        exp_dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    });

    // Output results
    if json {
        let output = InspectOutput {
            status: status.to_string(),
            tier: cert.payload.tier.clone(),
            product_id: cert.payload.product_id.clone(),
            license_id: hex::encode(cert.payload.license_id),
            issued_at: cert.payload.issued_at,
            issued_at_formatted,
            expiry: cert.payload.expiry,
            expiry_formatted,
            features: cert.payload.features.clone(),
            features_count: cert.payload.features.len(),
            user_pubkey: hex::encode(cert.payload.user_pubkey),
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("\nCertificate: {}", cert_path.display());

        // Status with emoji
        let status_icon = match status {
            "VALID" => "✅",
            "EXPIRED" => "⏰",
            _ => "❌",
        };
        println!("Status: {} {}", status_icon, status);

        println!("Tier: {}", cert.payload.tier);
        println!("Product: {}", cert.payload.product_id);
        println!(
            "License ID: {}...",
            hex::encode(&cert.payload.license_id[..8])
        );
        println!("Issued: {}", issued_at_formatted);

        if let Some(exp_str) = expiry_formatted {
            println!("Expires: {}", exp_str);
        } else {
            println!("Expires: Never (perpetual)");
        }

        let feature_count = cert.payload.features.len();
        if cert.payload.features.contains(&"ALL".to_string()) {
            println!("Features: ALL (wildcard - unlimited access)");
        } else if feature_count > 0 {
            println!(
                "Features: {} ({})",
                feature_count,
                cert.payload.features.join(", ")
            );
        } else {
            println!("Features: None");
        }

        if let Some(limits) = &cert.payload.limits {
            if let Some(seats) = limits.seat_max {
                println!("Seat Limit: {}", seats);
            }
            if let Some(grace) = limits.offline_grace_days {
                println!("Offline Grace: {} days", grace);
            }
        }

        if let Some(meta) = &cert.payload.metadata {
            println!("Metadata:");
            if let Some(product) = &meta.product {
                println!("  - Product: {}", product);
            }
            if let Some(version) = &meta.version {
                println!("  - Version: {}", version);
            }
            if let Some(university) = &meta.university {
                println!("  - University: {}", university);
            }
        }

        println!(
            "User Public Key: {}...",
            hex::encode(&cert.payload.user_pubkey[..8])
        );
        println!();
    }

    Ok(())
}
