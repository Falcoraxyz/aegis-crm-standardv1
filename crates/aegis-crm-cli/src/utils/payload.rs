//! Payload validation and processing

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// License tier types
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LicenseTier {
    LifetimePro,
    Campus,
}

/// License limits from JSON
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LimitsJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_max: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_grace_days: Option<u32>,
}

/// License metadata from JSON
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetadataJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub university: Option<String>,
}

/// License payload from JSON
#[derive(Debug, Deserialize, Serialize)]
pub struct LicensePayloadJson {
    pub tier: LicenseTier,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<LimitsJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataJson>,
}

impl LicensePayloadJson {
    /// Validate payload according to business rules
    pub fn validate(&self) -> Result<()> {
        // Campus tier MUST have expiry
        if self.tier == LicenseTier::Campus && self.expires_at.is_none() {
            anyhow::bail!(
                "Campus tier requires expires_at field (must be a future Unix timestamp)"
            );
        }

        // Validate campus expiry is in the future
        if self.tier == LicenseTier::Campus {
            if let Some(expiry) = self.expires_at {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();
                if expiry <= now {
                    anyhow::bail!(
                        "Campus tier expires_at must be in the future (got: {}, now: {})",
                        expiry,
                        now
                    );
                }
            }
        }

        // Validate features
        if self.features.is_empty() {
            eprintln!("⚠️  Warning: No features specified. License will have empty feature set.");
        }

        Ok(())
    }

    /// Get tier as string for core library
    pub fn tier_string(&self) -> String {
        match self.tier {
            LicenseTier::LifetimePro => "lifetime_pro".to_string(),
            LicenseTier::Campus => "campus".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campus_requires_expiry() {
        let payload = LicensePayloadJson {
            tier: LicenseTier::Campus,
            expires_at: None,
            features: vec![],
            limits: None,
            metadata: None,
        };

        assert!(payload.validate().is_err());
    }

    #[test]
    fn test_lifetime_pro_optional_expiry() {
        let payload = LicensePayloadJson {
            tier: LicenseTier::LifetimePro,
            expires_at: None,
            features: vec![],
            limits: None,
            metadata: None,
        };

        assert!(payload.validate().is_ok());
    }

    #[test]
    fn test_features_all_wildcard() {
        let payload = LicensePayloadJson {
            tier: LicenseTier::LifetimePro,
            expires_at: None,
            features: vec!["ALL".to_string()],
            limits: None,
            metadata: None,
        };

        assert!(payload.validate().is_ok());
        assert!(payload.features.contains(&"ALL".to_string()));
    }

    #[test]
    fn test_campus_with_future_expiry() {
        let future = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 86400; // +1 day

        let payload = LicensePayloadJson {
            tier: LicenseTier::Campus,
            expires_at: Some(future),
            features: vec!["education".to_string()],
            limits: None,
            metadata: None,
        };

        assert!(payload.validate().is_ok());
    }
}
