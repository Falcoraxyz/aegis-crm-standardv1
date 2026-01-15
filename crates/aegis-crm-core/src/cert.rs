//! License certificate issuance and verification.

use crate::crypto::{sha256, sign_compact, verify_compact};
use crate::errors::AegisError;
use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u16 = 1;

/// License tier limits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Limits {
    #[serde(rename = "seats", skip_serializing_if = "Option::is_none")]
    pub seat_max: Option<u32>,
    #[serde(rename = "grace", skip_serializing_if = "Option::is_none")]
    pub offline_grace_days: Option<u32>,
}

/// License metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub university: Option<String>,
}

fn default_tier() -> String {
    "lifetime_pro".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LicensePayload {
    #[serde(rename = "v")]
    pub version: u16,
    #[serde(rename = "pid")]
    pub product_id: String,
    #[serde(rename = "lid", with = "serde_bytes")]
    pub license_id: [u8; 32],
    #[serde(rename = "iat")]
    pub issued_at: u64,
    #[serde(rename = "exp")]
    pub expiry: Option<u64>,
    #[serde(rename = "upk", with = "serde_bytes")]
    pub user_pubkey: [u8; 33],
    #[serde(rename = "tier", default = "default_tier")]
    pub tier: String,
    #[serde(rename = "feat", default)]
    pub features: Vec<String>,
    #[serde(rename = "lim", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Limits>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl LicensePayload {
    /// Check if payload has a specific feature (supports "ALL" wildcard)
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.iter().any(|f| f == "ALL" || f == feature)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseCert {
    #[serde(flatten)]
    pub payload: LicensePayload,
    #[serde(rename = "sig", with = "serde_bytes")]
    pub vendor_sig: [u8; 64],
}

pub fn issue_cert(
    vendor_privkey: &[u8; 32],
    payload: LicensePayload,
) -> Result<LicenseCert, AegisError> {
    // Validate campus tier has expiry
    if payload.tier == "campus" && payload.expiry.is_none() {
        return Err(AegisError::CertParse);
    }

    let mut payload_bytes = Vec::new();
    ciborium::into_writer(&payload, &mut payload_bytes).map_err(|_| AegisError::CertParse)?;
    let digest = sha256(&payload_bytes);
    let vendor_sig = sign_compact(vendor_privkey, &digest)?;
    Ok(LicenseCert {
        payload,
        vendor_sig,
    })
}

pub fn verify_cert(
    vendor_pubkey: &[u8; 33],
    cert: &LicenseCert,
    now_unix: u64,
) -> Result<(), AegisError> {
    if cert.payload.version != PROTOCOL_VERSION {
        return Err(AegisError::UnsupportedVersion);
    }
    let mut payload_bytes = Vec::new();
    ciborium::into_writer(&cert.payload, &mut payload_bytes).map_err(|_| AegisError::CertParse)?;
    let digest = sha256(&payload_bytes);
    verify_compact(vendor_pubkey, &digest, &cert.vendor_sig)
        .map_err(|_| AegisError::CertSignature)?;
    if let Some(exp) = cert.payload.expiry {
        if now_unix > exp {
            return Err(AegisError::CertExpired);
        }
    }
    Ok(())
}

pub fn encode_cert(cert: &LicenseCert) -> Result<Vec<u8>, AegisError> {
    let mut cbor = Vec::new();
    ciborium::into_writer(cert, &mut cbor).map_err(|_| AegisError::CertParse)?;
    Ok(cbor)
}

pub fn decode_cert(cbor: &[u8]) -> Result<LicenseCert, AegisError> {
    ciborium::from_reader(cbor).map_err(|_| AegisError::CertParse)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::vendor_keygen;
    use getrandom::getrandom;

    fn create_test_payload() -> LicensePayload {
        let mut license_id = [0u8; 32];
        getrandom(&mut license_id).unwrap();
        LicensePayload {
            version: PROTOCOL_VERSION,
            product_id: "test".to_string(),
            license_id,
            issued_at: 1700000000,
            expiry: Some(2000000000),
            user_pubkey: [0x02; 33],
            tier: "lifetime_pro".to_string(),
            features: vec!["base".to_string(), "premium".to_string()],
            limits: None,
            metadata: None,
        }
    }

    #[test]
    fn test_issue_cert() {
        let vendor = vendor_keygen();
        let payload = create_test_payload();
        let cert = issue_cert(&vendor.privkey, payload).unwrap();
        assert_eq!(cert.vendor_sig.len(), 64);
    }

    #[test]
    fn test_verify_cert_valid() {
        let vendor = vendor_keygen();
        let cert = issue_cert(&vendor.privkey, create_test_payload()).unwrap();
        verify_cert(&vendor.pubkey, &cert, 1800000000).unwrap();
    }

    #[test]
    fn test_verify_cert_expired() {
        let vendor = vendor_keygen();
        let cert = issue_cert(&vendor.privkey, create_test_payload()).unwrap();
        assert!(matches!(
            verify_cert(&vendor.pubkey, &cert, 2100000000),
            Err(AegisError::CertExpired)
        ));
    }

    #[test]
    fn test_verify_cert_tampered() {
        let vendor = vendor_keygen();
        let mut cert = issue_cert(&vendor.privkey, create_test_payload()).unwrap();
        cert.vendor_sig[0] ^= 0xFF;
        assert!(matches!(
            verify_cert(&vendor.pubkey, &cert, 1800000000),
            Err(AegisError::CertSignature)
        ));
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let vendor = vendor_keygen();
        let cert = issue_cert(&vendor.privkey, create_test_payload()).unwrap();
        let cbor = encode_cert(&cert).unwrap();
        let decoded = decode_cert(&cbor).unwrap();
        assert_eq!(decoded.payload.product_id, cert.payload.product_id);
        assert_eq!(decoded.payload.tier, cert.payload.tier);
        assert_eq!(decoded.payload.features, cert.payload.features);
    }

    #[test]
    fn test_cbor_stability() {
        let vendor = vendor_keygen();
        let cert = issue_cert(&vendor.privkey, create_test_payload()).unwrap();
        let cbor1 = encode_cert(&cert).unwrap();
        let cbor2 = encode_cert(&cert).unwrap();
        assert_eq!(cbor1, cbor2);
    }

    #[test]
    fn test_has_feature_all_wildcard() {
        let mut payload = create_test_payload();
        payload.features = vec!["ALL".to_string()];
        assert!(payload.has_feature("any_feature"));
        assert!(payload.has_feature("another_feature"));
    }

    #[test]
    fn test_has_feature_explicit() {
        let payload = create_test_payload(); // has "base" and "premium"
        assert!(payload.has_feature("base"));
        assert!(payload.has_feature("premium"));
        assert!(!payload.has_feature("enterprise"));
    }

    #[test]
    fn test_campus_requires_expiry() {
        let vendor = vendor_keygen();
        let mut payload = create_test_payload();
        payload.tier = "campus".to_string();
        payload.expiry = None;
        assert!(matches!(
            issue_cert(&vendor.privkey, payload),
            Err(AegisError::CertParse)
        ));
    }

    #[test]
    fn test_campus_with_expiry_ok() {
        let vendor = vendor_keygen();
        let mut payload = create_test_payload();
        payload.tier = "campus".to_string();
        payload.expiry = Some(2000000000);
        assert!(issue_cert(&vendor.privkey, payload).is_ok());
    }
}
