//! Cryptography Service for EEMP
//!
//! Provides:
//! - AES-256-GCM encryption/decryption
//! - Ed25519 digital signatures
//! - X25519 key exchange
//! - SHA-256 hashing
//! - Secure random generation

pub mod encryption;
pub mod signing;
pub mod hashing;
pub mod keys;

pub use encryption::{encrypt_ballot, decrypt_ballot, EncryptedData};
pub use signing::{sign_data, verify_signature, KeyPair};
pub use hashing::{hash_data, hash_ballot};
pub use keys::{generate_encryption_key, EncryptionKey};
