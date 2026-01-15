//! Aegis CRM CLI - Command-line tool for cryptographic license management

mod commands;
mod error;
mod utils;

use clap::{Parser, Subcommand};
use error::Result;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aegis")]
#[command(about = "Aegis CRM - Cryptographic license management", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Vendor operations
    Vendor {
        #[command(subcommand)]
        command: VendorCommands,
    },

    /// User operations
    User {
        #[command(subcommand)]
        command: UserCommands,
    },

    /// Issue a license certificate
    Issue {
        /// Path to vendor private key or hex string
        #[arg(long)]
        vendor_priv: String,

        /// Path to user public key or hex string
        #[arg(long)]
        user_pub: String,

        /// Path to JSON payload file
        #[arg(long)]
        payload: PathBuf,

        /// Output certificate path
        #[arg(long)]
        out: PathBuf,

        /// Force overwrite existing files
        #[arg(long)]
        force: bool,

        /// Output machine-readable JSON
        #[arg(long)]
        json: bool,
    },

    /// Inspect a license certificate
    Inspect {
        /// Path to certificate file
        #[arg(long)]
        cert: PathBuf,

        /// Path to vendor public key or hex string
        #[arg(long)]
        vendor_pub: String,

        /// Output machine-readable JSON
        #[arg(long)]
        json: bool,
    },

    /// Proof-of-Possession operations
    Pop {
        #[command(subcommand)]
        command: PopCommands,
    },
}

#[derive(Subcommand)]
enum VendorCommands {
    /// Generate vendor keypair
    Keygen {
        /// Output directory (default: ./vendor_keys)
        #[arg(long)]
        out: Option<PathBuf>,

        /// Force overwrite existing files
        #[arg(long)]
        force: bool,

        /// Output machine-readable JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum UserCommands {
    /// Generate user keypair
    Keygen {
        /// Output directory (default: ./user_keys)
        #[arg(long)]
        out: Option<PathBuf>,

        /// Force overwrite existing files
        #[arg(long)]
        force: bool,

        /// Output machine-readable JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum PopCommands {
    /// Generate a challenge nonce
    Challenge,

    /// Prove ownership of user private key
    Prove {
        /// Path to user private key or hex string
        #[arg(long)]
        user_priv: String,

        /// Challenge nonce (hex)
        #[arg(long)]
        nonce: String,
    },

    /// Verify proof-of-possession signature
    Verify {
        /// Path to user public key or hex string
        #[arg(long)]
        user_pub: String,

        /// Challenge nonce (hex)
        #[arg(long)]
        nonce: String,

        /// Signature (hex)
        #[arg(long)]
        sig: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Vendor { command } => match command {
            VendorCommands::Keygen { out, force, json } => {
                commands::vendor::run(out, force, json)?;
            }
        },

        Commands::User { command } => match command {
            UserCommands::Keygen { out, force, json } => {
                commands::user::run(out, force, json)?;
            }
        },

        Commands::Issue {
            vendor_priv,
            user_pub,
            payload,
            out,
            force,
            json,
        } => {
            commands::issue::run(vendor_priv, user_pub, payload, out, force, json)?;
        }

        Commands::Inspect {
            cert,
            vendor_pub,
            json,
        } => {
            commands::inspect::run(cert, vendor_pub, json)?;
        }

        Commands::Pop { command } => match command {
            PopCommands::Challenge => {
                commands::pop::challenge()?;
            }
            PopCommands::Prove { user_priv, nonce } => {
                commands::pop::prove(user_priv, nonce)?;
            }
            PopCommands::Verify {
                user_pub,
                nonce,
                sig,
            } => {
                commands::pop::verify(user_pub, nonce, sig)?;
            }
        },
    }

    Ok(())
}
