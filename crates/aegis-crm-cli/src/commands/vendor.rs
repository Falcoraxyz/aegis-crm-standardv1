//! Vendor key generation command

use crate::error::Result;
use crate::utils::io::{check_overwrite, ensure_directory, validate_output_path, write_hex_file};
use aegis_crm_core::keys::vendor_keygen;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
pub struct VendorKeygenOutput {
    private_key_path: String,
    public_key_path: String,
}

pub fn run(out_dir: Option<PathBuf>, force: bool, json: bool) -> Result<()> {
    let out_dir = out_dir.unwrap_or(PathBuf::from("./vendor_keys"));

    // Validate output path
    validate_output_path(&out_dir)?;

    // Check paths before proceeding
    let priv_path = out_dir.join("vendor_priv.hex");
    let pub_path = out_dir.join("vendor_pub.hex");

    check_overwrite(&priv_path, force)?;
    check_overwrite(&pub_path, force)?;

    // Display security warning (unless JSON mode)
    if !json {
        eprintln!("\n⚠️  WARNING: Vendor private key is CRITICAL!");
        eprintln!("   - Store in cold storage (offline, encrypted)");
        eprintln!("   - Never commit to version control");
        eprintln!("   - Create secure backups");
        eprintln!("   - Anyone with this key can issue licenses!\n");
    }

    // Generate keypair
    let keypair = vendor_keygen();

    // Create output directory
    ensure_directory(&out_dir)?;

    // Write keys
    write_hex_file(&priv_path, &keypair.privkey)?;
    write_hex_file(&pub_path, &keypair.pubkey)?;

    // Output results
    if json {
        let output = VendorKeygenOutput {
            private_key_path: priv_path.display().to_string(),
            public_key_path: pub_path.display().to_string(),
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        println!("✅ Vendor keypair generated successfully!");
        println!("   Private key: {}", priv_path.display());
        println!("   Public key:  {}", pub_path.display());
    }

    Ok(())
}
