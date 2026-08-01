//! Ed25519 digital signatures

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use eemp_error::{AppError, Result};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

/// Ed25519 key pair for signing
#[derive(Clone)]
pub struct KeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl KeyPair {
    /// Generate a new random key pair
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();

        tracing::debug!("Generated new Ed25519 key pair");

        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Create key pair from existing seed (32 bytes)
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(seed);
        let verifying_key = signing_key.verifying_key();

        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Get the verifying (public) key
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    /// Get verifying key as bytes
    pub fn verifying_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }

    /// Get verifying key as hex string
    pub fn verifying_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_bytes())
    }
}

/// Signature result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureData {
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

/// Sign data with Ed25519
///
/// # Arguments
/// * `data` - The data to sign
/// * `key_pair` - The signing key pair
///
/// # Returns
/// * `SignatureData` containing signature and public key
pub fn sign_data(data: &[u8], key_pair: &KeyPair) -> Result<SignatureData> {
    if data.is_empty() {
        return Err(AppError::ValidationError(
            "Cannot sign empty data".to_string(),
        ));
    }

    let signature = key_pair.signing_key.sign(data);

    tracing::debug!(data_len = data.len(), "Data signed with Ed25519");

    Ok(SignatureData {
        signature: signature.to_bytes().to_vec(),
        public_key: key_pair.verifying_key_bytes().to_vec(),
    })
}

/// Verify Ed25519 signature
///
/// # Arguments
/// * `data` - The original data
/// * `signature_data` - The signature and public key
///
/// # Returns
/// * `true` if signature is valid, `false` otherwise
pub fn verify_signature(data: &[u8], signature_data: &SignatureData) -> Result<bool> {
    if data.is_empty() {
        return Err(AppError::ValidationError(
            "Cannot verify signature of empty data".to_string(),
        ));
    }

    if signature_data.signature.len() != 64 {
        return Err(AppError::ValidationError(format!(
            "Invalid signature length: expected 64 bytes, got {}",
            signature_data.signature.len()
        )));
    }

    if signature_data.public_key.len() != 32 {
        return Err(AppError::ValidationError(format!(
            "Invalid public key length: expected 32 bytes, got {}",
            signature_data.public_key.len()
        )));
    }

    // Parse public key
    let public_key_bytes: [u8; 32] = signature_data.public_key[..32]
        .try_into()
        .map_err(|_| AppError::ValidationError("Failed to parse public key".to_string()))?;

    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes)
        .map_err(|e| AppError::ValidationError(format!("Invalid public key: {}", e)))?;

    // Parse signature
    let signature_bytes: [u8; 64] = signature_data.signature[..64]
        .try_into()
        .map_err(|_| AppError::ValidationError("Failed to parse signature".to_string()))?;

    let signature = Signature::from_bytes(&signature_bytes);

    // Verify signature
    match verifying_key.verify(data, &signature) {
        Ok(_) => {
            tracing::debug!(data_len = data.len(), "Signature verified successfully");
            Ok(true)
        }
        Err(_) => {
            tracing::warn!(data_len = data.len(), "Signature verification failed");
            Ok(false)
        }
    }
}

/// Serialize signature data to hex for storage
pub fn serialize_signature(sig_data: &SignatureData) -> String {
    serde_json::json!({
        "signature": hex::encode(&sig_data.signature),
        "public_key": hex::encode(&sig_data.public_key),
    })
    .to_string()
}

/// Deserialize signature data from hex storage format
pub fn deserialize_signature(data: &str) -> Result<SignatureData> {
    let json: serde_json::Value = serde_json::from_str(data)
        .map_err(|e| AppError::ValidationError(format!("Invalid signature format: {}", e)))?;

    let signature_hex = json
        .get("signature")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing signature".to_string()))?;

    let public_key_hex = json
        .get("public_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing public_key".to_string()))?;

    let signature = hex::decode(signature_hex)
        .map_err(|e| AppError::ValidationError(format!("Invalid signature hex: {}", e)))?;

    let public_key = hex::decode(public_key_hex)
        .map_err(|e| AppError::ValidationError(format!("Invalid public_key hex: {}", e)))?;

    Ok(SignatureData {
        signature,
        public_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let key_pair = KeyPair::generate();
        let data = b"Vote commitment data";

        let signature_data = sign_data(data, &key_pair).unwrap();
        let is_valid = verify_signature(data, &signature_data).unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_verify_tampered_data() {
        let key_pair = KeyPair::generate();
        let data = b"Vote commitment data";

        let signature_data = sign_data(data, &key_pair).unwrap();

        // Tamper with data
        let tampered_data = b"Tampered vote data";
        let is_valid = verify_signature(tampered_data, &signature_data).unwrap();

        assert!(!is_valid);
    }

    #[test]
    fn test_verify_wrong_key() {
        let key_pair1 = KeyPair::generate();
        let key_pair2 = KeyPair::generate();
        let data = b"Vote commitment data";

        let signature_data = sign_data(data, &key_pair1).unwrap();

        // Use public key from different key pair
        let mut wrong_sig = signature_data.clone();
        wrong_sig.public_key = key_pair2.verifying_key_bytes().to_vec();

        let is_valid = verify_signature(data, &wrong_sig).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_from_seed() {
        let seed = [42u8; 32];
        let key_pair1 = KeyPair::from_seed(&seed);
        let key_pair2 = KeyPair::from_seed(&seed);

        // Same seed should produce same keys
        assert_eq!(
            key_pair1.verifying_key_bytes(),
            key_pair2.verifying_key_bytes()
        );
    }

    #[test]
    fn test_serialize_deserialize() {
        let key_pair = KeyPair::generate();
        let data = b"Vote commitment data";

        let signature_data = sign_data(data, &key_pair).unwrap();
        let serialized = serialize_signature(&signature_data);
        let deserialized = deserialize_signature(&serialized).unwrap();

        assert_eq!(signature_data.signature, deserialized.signature);
        assert_eq!(signature_data.public_key, deserialized.public_key);

        let is_valid = verify_signature(data, &deserialized).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_empty_data() {
        let key_pair = KeyPair::generate();
        assert!(sign_data(b"", &key_pair).is_err());
    }

    #[test]
    fn test_invalid_signature_length() {
        let data = b"test data";
        let sig_data = SignatureData {
            signature: vec![1, 2, 3], // Wrong length
            public_key: vec![1; 32],
        };

        assert!(verify_signature(data, &sig_data).is_err());
    }
}
