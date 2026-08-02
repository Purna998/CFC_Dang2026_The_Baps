//! Multi-Factor Authentication (MFA) with TOTP
//!
//! Implements Time-based One-Time Password (TOTP) per RFC 6238.

use eemp_error::{AppError, Result};
use rand::Rng;
use totp_rs::{Algorithm, Secret, TOTP};

pub struct MfaManager;

impl MfaManager {
    /// Generate a new TOTP secret for a user
    pub fn generate_secret() -> Result<String> {
        let secret = Secret::generate_secret();
        Ok(secret.to_encoded().to_string())
    }

    /// Generate a QR code URL for TOTP setup
    pub fn generate_qr_code_url(secret: &str, email: &str, issuer: &str) -> Result<String> {
        let totp = Self::create_totp(secret, email, issuer)?;
        Ok(totp.get_url())
    }

    /// Verify a TOTP code
    pub fn verify_totp(secret: &str, code: &str, email: &str) -> Result<bool> {
        let totp = Self::create_totp(secret, email, "EEMP")?;
        Ok(totp.check_current(code).unwrap_or(false))
    }

    /// Generate backup codes (8 codes, 10 characters each)
    pub fn generate_backup_codes() -> Vec<String> {
        let mut rng = rand::thread_rng();
        (0..8)
            .map(|_| {
                (0..10)
                    .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                    .collect::<String>()
                    .to_uppercase()
            })
            .collect()
    }

    /// Create a TOTP instance
    fn create_totp(secret: &str, email: &str, issuer: &str) -> Result<TOTP> {
        TOTP::new(
            Algorithm::SHA1,
            6,      // 6-digit codes
            1,      // 1 step (30 seconds)
            30,     // 30 second time step
            Secret::Encoded(secret.to_string())
                .to_bytes()
                .map_err(|e| AppError::ValidationError(format!("Invalid secret: {}", e)))?,
            Some(issuer.to_string()),
            email.to_string(),
        )
        .map_err(|e| AppError::InternalError(format!("Failed to create TOTP: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        let secret = MfaManager::generate_secret().unwrap();
        assert!(!secret.is_empty());
    }

    #[test]
    fn test_generate_qr_code_url() {
        let secret = MfaManager::generate_secret().unwrap();
        let url = MfaManager::generate_qr_code_url(&secret, "test@example.com", "EEMP").unwrap();
        assert!(url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_generate_backup_codes() {
        let codes = MfaManager::generate_backup_codes();
        assert_eq!(codes.len(), 8);
        for code in codes {
            assert_eq!(code.len(), 10);
            assert!(code.chars().all(|c| c.is_ascii_alphanumeric()));
        }
    }

    #[test]
    fn test_verify_totp_with_current_time() {
        let secret = MfaManager::generate_secret().unwrap();
        let totp = MfaManager::create_totp(&secret, "test@example.com", "EEMP").unwrap();
        let code = totp.generate_current().unwrap();

        // Verify the current code
        assert!(MfaManager::verify_totp(&secret, &code, "test@example.com").unwrap());
    }

    #[test]
    fn test_verify_invalid_totp() {
        let secret = MfaManager::generate_secret().unwrap();

        // Verify an invalid code
        assert!(!MfaManager::verify_totp(&secret, "000000", "test@example.com").unwrap());
    }
}
