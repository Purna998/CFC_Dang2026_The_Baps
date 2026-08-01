//! Password hashing with Argon2id
//!
//! Uses OWASP recommended parameters:
//! - Memory cost: 19 MiB (19456 KiB)
//! - Time cost: 2 iterations
//! - Parallelism: 1 thread

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, ParamsBuilder,
};
use eemp_config::SecurityConfig;
use eemp_domain::{Password, PasswordHash as DomainPasswordHash};
use eemp_error::{AppError, Result};

pub struct PasswordHasher {
    argon2: Argon2<'static>,
}

impl PasswordHasher {
    /// Create a new password hasher with security config
    pub fn new(config: &SecurityConfig) -> Result<Self> {
        let mut params_builder = ParamsBuilder::new();
        params_builder
            .m_cost(config.argon2_memory_cost)
            .t_cost(config.argon2_time_cost)
            .p_cost(config.argon2_parallelism)
            .map_err(|e| AppError::InternalError(format!("Invalid Argon2 parameters: {}", e)))?;

        let params = params_builder
            .params()
            .map_err(|e| AppError::InternalError(format!("Failed to build Argon2 params: {}", e)))?;

        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        Ok(Self { argon2 })
    }

    /// Hash a password using Argon2id
    pub fn hash_password(&self, password: &Password) -> Result<DomainPasswordHash> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(password.as_str().as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;

        Ok(DomainPasswordHash::new(password_hash.to_string())?)
    }

    /// Verify a password against its hash
    pub fn verify_password(&self, password: &Password, hash: &DomainPasswordHash) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash.as_str())
            .map_err(|e| AppError::InternalError(format!("Invalid password hash: {}", e)))?;

        match self.argon2.verify_password(password.as_str().as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(AppError::InternalError(format!(
                "Password verification failed: {}",
                e
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> SecurityConfig {
        SecurityConfig {
            argon2_memory_cost: 19456,
            argon2_time_cost: 2,
            argon2_parallelism: 1,
        }
    }

    #[test]
    fn test_hash_and_verify_password() {
        let hasher = PasswordHasher::new(&test_config()).unwrap();
        let password = Password::new("TestPassword123!".to_string()).unwrap();

        let hash = hasher.hash_password(&password).unwrap();
        assert!(hasher.verify_password(&password, &hash).unwrap());
    }

    #[test]
    fn test_verify_wrong_password() {
        let hasher = PasswordHasher::new(&test_config()).unwrap();
        let password = Password::new("TestPassword123!".to_string()).unwrap();
        let wrong_password = Password::new("WrongPassword456!".to_string()).unwrap();

        let hash = hasher.hash_password(&password).unwrap();
        assert!(!hasher.verify_password(&wrong_password, &hash).unwrap());
    }

    #[test]
    fn test_different_hashes_for_same_password() {
        let hasher = PasswordHasher::new(&test_config()).unwrap();
        let password = Password::new("TestPassword123!".to_string()).unwrap();

        let hash1 = hasher.hash_password(&password).unwrap();
        let hash2 = hasher.hash_password(&password).unwrap();

        // Hashes should be different due to random salt
        assert_ne!(hash1.as_str(), hash2.as_str());

        // But both should verify correctly
        assert!(hasher.verify_password(&password, &hash1).unwrap());
        assert!(hasher.verify_password(&password, &hash2).unwrap());
    }
}
