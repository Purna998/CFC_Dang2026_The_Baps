//! JWT token generation and validation
//!
//! Uses RS256 (RSA 2048-bit) for signing.
//! - Access tokens: 15 minutes
//! - Refresh tokens: 7 days

use chrono::{Duration, Utc};
use eemp_config::JwtConfig;
use eemp_domain::{TenantId, UserId, UserRole};
use eemp_error::{AppError, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,      // User ID
    pub tenant_id: String, // Tenant ID
    pub role: String,      // User role
    pub exp: i64,         // Expiration timestamp
    pub iat: i64,         // Issued at timestamp
    pub jti: String,      // JWT ID (unique token ID)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,      // User ID
    pub tenant_id: String, // Tenant ID
    pub exp: i64,         // Expiration timestamp
    pub iat: i64,         // Issued at timestamp
    pub jti: String,      // JWT ID (unique token ID)
}

pub struct JwtManager {
    config: JwtConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    /// Create a new JWT manager
    pub fn new(config: JwtConfig) -> Result<Self> {
        // For RS256, we need RSA keys
        // In production, load from secure storage (e.g., AWS Secrets Manager, HashiCorp Vault)
        // For now, use the secret as HMAC for development (will upgrade to RS256 keys later)
        let encoding_key = EncodingKey::from_secret(config.access_token_secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(config.access_token_secret.as_bytes());

        Ok(Self {
            config,
            encoding_key,
            decoding_key,
        })
    }

    /// Generate an access token
    pub fn generate_access_token(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
        role: UserRole,
    ) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.access_token_expiry_seconds);

        let claims = AccessTokenClaims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            role: role.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        // Using HS256 for now, will upgrade to RS256 with proper key management
        let header = Header::new(Algorithm::HS256);

        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Failed to generate access token: {}", e)))
    }

    /// Generate a refresh token
    pub fn generate_refresh_token(&self, user_id: UserId, tenant_id: TenantId) -> Result<String> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.config.refresh_token_expiry_seconds);

        let claims = RefreshTokenClaims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        // Using HS256 for now, will upgrade to RS256 with proper key management
        let header = Header::new(Algorithm::HS256);

        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| AppError::InternalError(format!("Failed to generate refresh token: {}", e)))
    }

    /// Verify and decode an access token
    pub fn verify_access_token(&self, token: &str) -> Result<AccessTokenClaims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        decode::<AccessTokenClaims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::Unauthorized("Token has expired".to_string())
                }
                _ => AppError::Unauthorized(format!("Invalid token: {}", e)),
            })
    }

    /// Verify and decode a refresh token
    pub fn verify_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        decode::<RefreshTokenClaims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::Unauthorized("Refresh token has expired".to_string())
                }
                _ => AppError::Unauthorized(format!("Invalid refresh token: {}", e)),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> JwtConfig {
        JwtConfig {
            access_token_secret: "test-secret-key-min-32-chars-long".to_string(),
            refresh_token_secret: "test-refresh-secret-key-min-32-chars-long".to_string(),
            access_token_expiry_seconds: 900,
            refresh_token_expiry_seconds: 604800,
        }
    }

    #[test]
    fn test_generate_and_verify_access_token() {
        let manager = JwtManager::new(test_config()).unwrap();
        let user_id = UserId::new();
        let tenant_id = TenantId::new();
        let role = UserRole::OrganizationAdmin;

        let token = manager
            .generate_access_token(user_id, tenant_id, role)
            .unwrap();
        let claims = manager.verify_access_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.tenant_id, tenant_id.to_string());
        assert_eq!(claims.role, "OrganizationAdmin");
    }

    #[test]
    fn test_generate_and_verify_refresh_token() {
        let manager = JwtManager::new(test_config()).unwrap();
        let user_id = UserId::new();
        let tenant_id = TenantId::new();

        let token = manager.generate_refresh_token(user_id, tenant_id).unwrap();
        let claims = manager.verify_refresh_token(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.tenant_id, tenant_id.to_string());
    }

    #[test]
    fn test_verify_invalid_token() {
        let manager = JwtManager::new(test_config()).unwrap();
        let result = manager.verify_access_token("invalid.token.here");

        assert!(result.is_err());
    }
}
