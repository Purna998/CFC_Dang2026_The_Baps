//! SHA-256 hashing for data integrity

use eemp_error::{AppError, Result};
use sha2::{Digest, Sha256};

/// Hash data using SHA-256
///
/// # Arguments
/// * `data` - The data to hash
///
/// # Returns
/// * 32-byte hash as hex string
pub fn hash_data(data: &[u8]) -> Result<String> {
    if data.is_empty() {
        return Err(AppError::ValidationError(
            "Cannot hash empty data".to_string(),
        ));
    }

    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    let hash_hex = hex::encode(result);

    tracing::debug!(data_len = data.len(), hash = %hash_hex, "Data hashed with SHA-256");

    Ok(hash_hex)
}

/// Hash ballot data for integrity verification
///
/// Combines ballot ID, voter ID, and encrypted ballot into a single hash
pub fn hash_ballot(
    ballot_id: &str,
    voter_id: &str,
    encrypted_ballot: &[u8],
) -> Result<String> {
    let combined = format!("{}:{}:", ballot_id, voter_id);
    let mut data = combined.as_bytes().to_vec();
    data.extend_from_slice(encrypted_ballot);

    hash_data(&data)
}

/// Verify hash matches data
pub fn verify_hash(data: &[u8], expected_hash: &str) -> Result<bool> {
    let actual_hash = hash_data(data)?;
    Ok(actual_hash == expected_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_data() {
        let data = b"test data";
        let hash = hash_data(data).unwrap();

        // SHA-256 produces 64 character hex string (32 bytes)
        assert_eq!(hash.len(), 64);

        // Same data should produce same hash
        let hash2 = hash_data(data).unwrap();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_different_data_different_hash() {
        let data1 = b"test data 1";
        let data2 = b"test data 2";

        let hash1 = hash_data(data1).unwrap();
        let hash2 = hash_data(data2).unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_ballot() {
        let ballot_id = "ballot-123";
        let voter_id = "voter-456";
        let encrypted_ballot = b"encrypted ballot data";

        let hash = hash_ballot(ballot_id, voter_id, encrypted_ballot).unwrap();
        assert_eq!(hash.len(), 64);

        // Same inputs should produce same hash
        let hash2 = hash_ballot(ballot_id, voter_id, encrypted_ballot).unwrap();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_verify_hash() {
        let data = b"test data";
        let hash = hash_data(data).unwrap();

        assert!(verify_hash(data, &hash).unwrap());
        assert!(!verify_hash(b"wrong data", &hash).unwrap());
    }

    #[test]
    fn test_empty_data() {
        assert!(hash_data(b"").is_err());
    }

    #[test]
    fn test_known_hash() {
        // Test against known SHA-256 hash
        let data = b"hello world";
        let hash = hash_data(data).unwrap();

        // SHA-256("hello world") = b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
}
