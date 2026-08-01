//! Key generation and management

use eemp_error::{AppError, Result};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Encryption key with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionKey {
    pub key_id: Uuid,
    pub key_bytes: [u8; 32],
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl EncryptionKey {
    /// Generate a new random encryption key
    pub fn generate() -> Self {
        let mut key_bytes = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_bytes);

        Self {
            key_id: Uuid::new_v4(),
            key_bytes,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create encryption key from existing bytes and ID
    pub fn from_bytes(key_id: Uuid, key_bytes: [u8; 32]) -> Self {
        Self {
            key_id,
            key_bytes,
            created_at: chrono::Utc::now(),
        }
    }

    /// Get key as reference
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.key_bytes
    }

    /// Serialize key to base64 (for secure storage)
    pub fn to_base64(&self) -> String {
        base64::encode(&self.key_bytes)
    }

    /// Deserialize key from base64
    pub fn from_base64(key_id: Uuid, b64: &str) -> Result<Self> {
        let bytes = base64::decode(b64)
            .map_err(|e| AppError::ValidationError(format!("Invalid base64 key: {}", e)))?;

        if bytes.len() != 32 {
            return Err(AppError::ValidationError(format!(
                "Invalid key length: expected 32 bytes, got {}",
                bytes.len()
            )));
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);

        Ok(Self::from_bytes(key_id, key_bytes))
    }
}

/// Generate a random encryption key
pub fn generate_encryption_key() -> EncryptionKey {
    EncryptionKey::generate()
}

/// Generate secure random bytes
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes
}

/// Generate a voter receipt code (10 characters, alphanumeric)
pub fn generate_receipt_code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789"; // Excluding ambiguous characters
    const CODE_LENGTH: usize = 10;

    let mut code = String::with_capacity(CODE_LENGTH);
    let mut rng = rand::thread_rng();

    for _ in 0..CODE_LENGTH {
        let idx = (rng.next_u32() as usize) % CHARSET.len();
        code.push(CHARSET[idx] as char);
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        let key = generate_encryption_key();
        assert_eq!(key.key_bytes.len(), 32);
        assert_ne!(key.key_bytes, [0u8; 32]); // Should be random
    }

    #[test]
    fn test_different_keys() {
        let key1 = generate_encryption_key();
        let key2 = generate_encryption_key();

        // Different keys should have different bytes
        assert_ne!(key1.key_bytes, key2.key_bytes);
        assert_ne!(key1.key_id, key2.key_id);
    }

    #[test]
    fn test_base64_roundtrip() {
        let key = generate_encryption_key();
        let b64 = key.to_base64();
        let restored = EncryptionKey::from_base64(key.key_id, &b64).unwrap();

        assert_eq!(key.key_bytes, restored.key_bytes);
        assert_eq!(key.key_id, restored.key_id);
    }

    #[test]
    fn test_invalid_base64() {
        let key_id = Uuid::new_v4();
        assert!(EncryptionKey::from_base64(key_id, "invalid!!!").is_err());
    }

    #[test]
    fn test_invalid_key_length() {
        let key_id = Uuid::new_v4();
        let short_key = base64::encode(b"short");
        assert!(EncryptionKey::from_base64(key_id, &short_key).is_err());
    }

    #[test]
    fn test_generate_random_bytes() {
        let bytes1 = generate_random_bytes(32);
        let bytes2 = generate_random_bytes(32);

        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2); // Should be random
    }

    #[test]
    fn test_generate_receipt_code() {
        let code1 = generate_receipt_code();
        let code2 = generate_receipt_code();

        assert_eq!(code1.len(), 10);
        assert_eq!(code2.len(), 10);
        assert_ne!(code1, code2); // Should be random

        // All characters should be alphanumeric
        assert!(code1.chars().all(|c| c.is_alphanumeric()));
        assert!(code2.chars().all(|c| c.is_alphanumeric()));

        // Should not contain ambiguous characters
        assert!(!code1.contains('0'));
        assert!(!code1.contains('O'));
        assert!(!code1.contains('I'));
        assert!(!code1.contains('1'));
    }
}
