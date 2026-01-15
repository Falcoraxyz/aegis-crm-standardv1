//! Proof-of-Possession commands

use crate::error::Result;
use crate::utils::io::read_hex_or_file;
use aegis_crm_core::pop::{challenge as pop_challenge, prove as pop_prove, verify as pop_verify};
use anyhow::Context;

/// Generate PoP challenge nonce
pub fn challenge() -> Result<()> {
    let nonce = pop_challenge();
    println!("{}", hex::encode(nonce));
    Ok(())
}

/// Prove ownership of user private key
pub fn prove(user_priv: String, nonce_hex: String) -> Result<()> {
    // Load user private key
    let user_priv_bytes =
        read_hex_or_file(&user_priv).context("Failed to read user private key")?;
    if user_priv_bytes.len() != 32 {
        anyhow::bail!(
            "User private key must be 32 bytes, got {}",
            user_priv_bytes.len()
        );
    }
    let mut user_privkey = [0u8; 32];
    user_privkey.copy_from_slice(&user_priv_bytes);

    // Parse nonce
    let nonce_bytes = hex::decode(&nonce_hex).context("Invalid nonce hex")?;
    if nonce_bytes.len() != 32 {
        anyhow::bail!("Nonce must be 32 bytes, got {}", nonce_bytes.len());
    }
    let mut nonce = [0u8; 32];
    nonce.copy_from_slice(&nonce_bytes);

    // Prove
    let signature = pop_prove(&user_privkey, &nonce).context("Failed to generate PoP signature")?;

    println!("{}", hex::encode(signature));
    Ok(())
}

/// Verify PoP signature
pub fn verify(user_pub: String, nonce_hex: String, sig_hex: String) -> Result<()> {
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

    // Parse nonce
    let nonce_bytes = hex::decode(&nonce_hex).context("Invalid nonce hex")?;
    if nonce_bytes.len() != 32 {
        anyhow::bail!("Nonce must be 32 bytes, got {}", nonce_bytes.len());
    }
    let mut nonce = [0u8; 32];
    nonce.copy_from_slice(&nonce_bytes);

    // Parse signature
    let sig_bytes = hex::decode(&sig_hex).context("Invalid signature hex")?;
    if sig_bytes.len() != 64 {
        anyhow::bail!("Signature must be 64 bytes, got {}", sig_bytes.len());
    }
    let mut signature = [0u8; 64];
    signature.copy_from_slice(&sig_bytes);

    // Verify
    match pop_verify(&user_pubkey, &nonce, &signature) {
        Ok(()) => {
            println!("✅ Proof-of-Possession VALID");
            std::process::exit(0);
        }
        Err(_) => {
            eprintln!("❌ Proof-of-Possession INVALID");
            std::process::exit(1);
        }
    }
}
