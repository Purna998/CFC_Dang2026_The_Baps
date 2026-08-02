//! Data Transfer Objects (DTOs) for authentication

use eemp_domain::{TenantId, UserId};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub user: UserInfo,
    pub requires_mfa: bool,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 32))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    pub tenant_id: TenantId,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 2, max = 100))]
    pub full_name: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: UserId,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyTokenResponse {
    pub valid: bool,
    pub user_id: Option<UserId>,
    pub tenant_id: Option<TenantId>,
}

#[derive(Debug, Deserialize)]
pub struct EnableMfaRequest {}

#[derive(Debug, Serialize)]
pub struct EnableMfaResponse {
    pub secret: String,
    pub qr_code_url: String,
    pub backup_codes: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyMfaRequest {
    #[validate(length(equal = 6))]
    pub totp_code: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyMfaResponse {
    pub verified: bool,
}
