//! File I/O utilities

use crate::error::Result;
use anyhow::Context;
use std::fs;
use std::path::Path;

/// Read hex string either directly or from a file path
pub fn read_hex_or_file(input: &str) -> Result<Vec<u8>> {
    // If it looks like a valid hex string, decode it directly
    if (input.len() & 1) == 0 && input.chars().all(|c| c.is_ascii_hexdigit()) {
        hex::decode(input).context("Invalid hex string")
    } else {
        // Otherwise treat as file path
        let contents =
            fs::read_to_string(input).with_context(|| format!("Failed to read file: {}", input))?;

        let hex_str = contents.trim();
        hex::decode(hex_str).with_context(|| format!("Invalid hex content in file: {}", input))
    }
}

/// Write hex-encoded data to file
pub fn write_hex_file(path: &Path, data: &[u8]) -> Result<()> {
    let hex_string = hex::encode(data);
    fs::write(path, hex_string)
        .with_context(|| format!("Failed to write hex file: {}", path.display()))?;

    // Set restrictive permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

/// Validate output path doesn't contain path traversal
pub fn validate_output_path(path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy();

    if path_str.contains("..") {
        anyhow::bail!(
            "Path contains '..' component (potential path traversal): {}",
            path_str
        );
    }

    Ok(())
}

/// Check for file overwrite and enforce --force flag
pub fn check_overwrite(path: &Path, force: bool) -> Result<()> {
    if path.exists() && !force {
        anyhow::bail!(
            "File already exists: {}\nUse --force to overwrite",
            path.display()
        );
    }
    Ok(())
}

/// Create directory if it doesn't exist
pub fn ensure_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}
