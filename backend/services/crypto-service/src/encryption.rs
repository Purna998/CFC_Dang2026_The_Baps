//! AES-256-GCM encryption for ballot data

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use eemp_error::{AppError, Result};
use serde::{Deserialize, Serialize};

/// Encrypted data with nonce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

/// Encrypt ballot data using AES-256-GCM
///
/// # Arguments
/// * `plaintext` - The data to encrypt (serialized ballot JSON)
/// * `key` - 32-byte encryption key
///
/// # Returns
/// * `EncryptedData` containing ciphertext and nonce
pub fn encrypt_ballot(plaintext: &[u8], key: &[u8; 32]) -> Result<EncryptedData> {
    if plaintext.is_empty() {
        return Err(AppError::ValidationError(
            "Cannot encrypt empty data".to_string(),
        ));
    }

    // Create cipher instance
    let cipher = Aes256Gcm::new(key.into());

    // Generate random nonce (96 bits = 12 bytes for GCM)
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Encrypt the data
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| AppError::InternalError(format!("Encryption failed: {}", e)))?;

    tracing::debug!(
        plaintext_len = plaintext.len(),
        ciphertext_len = ciphertext.len(),
        "Ballot encrypted with AES-256-GCM"
    );

    Ok(EncryptedData {
        ciphertext,
        nonce: nonce.to_vec(),
    })
}

/// Decrypt ballot data using AES-256-GCM
///
/// # Arguments
/// * `encrypted` - The encrypted data with nonce
/// * `key` - 32-byte decryption key (same as encryption key)
///
/// # Returns
/// * Decrypted plaintext bytes
pub fn decrypt_ballot(encrypted: &EncryptedData, key: &[u8; 32]) -> Result<Vec<u8>> {
    if encrypted.ciphertext.is_empty() {
        return Err(AppError::ValidationError(
            "Cannot decrypt empty data".to_string(),
        ));
    }

    if encrypted.nonce.len() != 12 {
        return Err(AppError::ValidationError(format!(
            "Invalid nonce length: expected 12 bytes, got {}",
            encrypted.nonce.len()
        )));
    }

    // Create cipher instance
    let cipher = Aes256Gcm::new(key.into());

    // Convert nonce back to array
    let nonce = Nonce::from_slice(&encrypted.nonce);

    // Decrypt the data
    let plaintext = cipher
        .decrypt(nonce, encrypted.ciphertext.as_ref())
        .map_err(|e| AppError::InternalError(format!("Decryption failed: {}", e)))?;

    tracing::debug!(
        ciphertext_len = encrypted.ciphertext.len(),
        plaintext_len = plaintext.len(),
        "Ballot decrypted with AES-256-GCM"
    );

    Ok(plaintext)
}

/// Serialize encrypted data to base64 for storage
pub fn serialize_encrypted(encrypted: &EncryptedData) -> String {
    let json = serde_json::json!({
        "ciphertext": general_purpose::STANDARD.encode(&encrypted.ciphertext),
        "nonce": general_purpose::STANDARD.encode(&encrypted.nonce),
    });
    json.to_string()
}

/// Deserialize encrypted data from base64 storage format
pub fn deserialize_encrypted(data: &str) -> Result<EncryptedData> {
    let json: serde_json::Value = serde_json::from_str(data)
        .map_err(|e| AppError::ValidationError(format!("Invalid encrypted data format: {}", e)))?;

    let ciphertext_b64 = json
        .get("ciphertext")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing ciphertext".to_string()))?;

    let nonce_b64 = json
        .get("nonce")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing nonce".to_string()))?;

    let ciphertext = general_purpose::STANDARD.decode(ciphertext_b64)
        .map_err(|e| AppError::ValidationError(format!("Invalid ciphertext base64: {}", e)))?;

    let nonce = general_purpose::STANDARD.decode(nonce_b64)
        .map_err(|e| AppError::ValidationError(format!("Invalid nonce base64: {}", e)))?;

    Ok(EncryptedData { ciphertext, nonce })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; 32] {
        [42u8; 32] // Test key (in production, use proper key generation)
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = test_key();
        let plaintext = b"Secret ballot data";

        let encrypted = encrypt_ballot(plaintext, &key).unwrap();
        let decrypted = decrypt_ballot(&encrypted, &key).unwrap();

        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_serialize_deserialize() {
        let key = test_key();
        let plaintext = b"Secret ballot data";

        let encrypted = encrypt_ballot(plaintext, &key).unwrap();
        let serialized = serialize_encrypted(&encrypted);
        let deserialized = deserialize_encrypted(&serialized).unwrap();

        assert_eq!(encrypted.ciphertext, deserialized.ciphertext);
        assert_eq!(encrypted.nonce, deserialized.nonce);

        let decrypted = decrypt_ballot(&deserialized, &key).unwrap();
        assert_eq!(plaintext, decrypted.as_slice());
    }

    #[test]
    fn test_different_nonces() {
        let key = test_key();
        let plaintext = b"Secret ballot data";

        let encrypted1 = encrypt_ballot(plaintext, &key).unwrap();
        let encrypted2 = encrypt_ballot(plaintext, &key).unwrap();

        // Same plaintext should produce different ciphertexts (different nonces)
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
        assert_ne!(encrypted1.nonce, encrypted2.nonce);

        // Both should decrypt correctly
        let decrypted1 = decrypt_ballot(&encrypted1, &key).unwrap();
        let decrypted2 = decrypt_ballot(&encrypted2, &key).unwrap();

        assert_eq!(plaintext, decrypted1.as_slice());
        assert_eq!(plaintext, decrypted2.as_slice());
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = test_key();
        let mut key2 = test_key();
        key2[0] = 0; // Different key

        let plaintext = b"Secret ballot data";
        let encrypted = encrypt_ballot(plaintext, &key1).unwrap();

        // Decryption with wrong key should fail
        assert!(decrypt_ballot(&encrypted, &key2).is_err());
    }

    #[test]
    fn test_empty_plaintext() {
        let key = test_key();
        assert!(encrypt_ballot(b"", &key).is_err());
    }

    #[test]
    fn test_invalid_nonce_length() {
        let key = test_key();
        let encrypted = EncryptedData {
            ciphertext: vec![1, 2, 3],
            nonce: vec![1, 2, 3], // Wrong length (should be 12)
        };

        assert!(decrypt_ballot(&encrypted, &key).is_err());
    }
}
