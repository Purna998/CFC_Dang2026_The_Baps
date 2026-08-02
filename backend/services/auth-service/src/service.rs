//! Authentication service implementation

use crate::{
    dto::*,
    jwt::JwtManager,
    mfa::MfaManager,
    password::PasswordHasher,
    session::{SessionManager, Session},
};
use eemp_config::Config;
use eemp_database::Database;
use eemp_domain::{Email, Password, TenantId, UserId, UserRole};
use eemp_error::{AppError, Result};
use std::sync::Arc;

pub struct AuthService {
    db: Database,
    jwt_manager: Arc<JwtManager>,
    password_hasher: Arc<PasswordHasher>,
    session_manager: Arc<SessionManager>,
    config: Config,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(db: Database, config: Config) -> Result<Self> {
        let jwt_manager = Arc::new(JwtManager::new(config.jwt.clone())?);
        let password_hasher = Arc::new(PasswordHasher::new(&config.security)?);
        let session_manager = Arc::new(SessionManager::new(
            &config.redis.url,
            db.pool().clone(),
        )?);

        Ok(Self {
            db,
            jwt_manager,
            password_hasher,
            session_manager,
            config,
        })
    }

    /// Login a user
    pub async fn login(
        &self,
        req: LoginRequest,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<LoginResponse> {
        let email = Email::new(req.email)?;
        let password = Password::new(req.password)?;

        // Get user from database
        let user_row = sqlx::query!(
            r#"
            SELECT user_id, tenant_id, email, password_hash, role, mfa_enabled, mfa_secret
            FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
            email.as_str()
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Database query failed: {}", e)))?
        .ok_or_else(|| AppError::InvalidToken)?;

        // Verify password
        let password_hash = eemp_domain::PasswordHash::new(user_row.password_hash);
        let is_valid = self
            .password_hasher
            .verify_password(&password, &password_hash)?;

        if !is_valid {
            tracing::warn!(email = %email.as_str(), "Failed login attempt");
            return Err(AppError::InvalidToken);
        }

        // Check if MFA is required
        if user_row.mfa_enabled {
            if let Some(totp_code) = req.totp_code {
                let mfa_secret = user_row
                    .mfa_secret
                    .ok_or_else(|| AppError::InternalError("MFA secret not found".to_string()))?;

                let is_valid = MfaManager::verify_totp(&mfa_secret, &totp_code, email.as_str())?;

                if !is_valid {
                    tracing::warn!(email = %email.as_str(), "Invalid TOTP code");
                    return Err(AppError::InvalidToken);
                }
            } else {
                // MFA required but not provided
                return Ok(LoginResponse {
                    access_token: String::new(),
                    refresh_token: String::new(),
                    expires_in: 0,
                    token_type: "Bearer".to_string(),
                    user: UserInfo {
                        user_id: UserId::from_uuid(user_row.user_id),
                        tenant_id: TenantId::from_uuid(user_row.tenant_id),
                        email: email.as_str().to_string(),
                        role: user_row.role,
                    },
                    requires_mfa: true,
                });
            }
        }

        let user_id = UserId::from_uuid(user_row.user_id);
        let tenant_id = TenantId::from_uuid(user_row.tenant_id);
        let role = user_row.role.parse::<UserRole>()
            .map_err(|e| AppError::ValidationError(format!("Invalid role: {}", e)))?;

        // Generate tokens
        let access_token = self.jwt_manager.generate_access_token(user_id, tenant_id, role.clone())?;
        let refresh_token = self.jwt_manager.generate_refresh_token(user_id, tenant_id)?;

        // Create session
        let refresh_token_hash = Self::hash_token(&refresh_token);
        self.session_manager
            .create_session(
                user_id,
                tenant_id,
                refresh_token_hash,
                ip_address,
                user_agent,
                self.config.jwt.refresh_token_expiry_seconds,
            )
            .await?;

        tracing::info!(user_id = %user_id, email = %email.as_str(), "User logged in");

        Ok(LoginResponse {
            access_token,
            refresh_token,
            expires_in: self.config.jwt.access_token_expiry_seconds,
            token_type: "Bearer".to_string(),
            user: UserInfo {
                user_id,
                tenant_id,
                email: email.as_str().to_string(),
                role: role.to_string(),
            },
            requires_mfa: false,
        })
    }

    /// Refresh access token
    pub async fn refresh_token(&self, req: RefreshTokenRequest) -> Result<RefreshTokenResponse> {
        // Verify refresh token
        let claims = self.jwt_manager.verify_refresh_token(&req.refresh_token)?;

        let user_id = UserId::from_uuid(
            uuid::Uuid::parse_str(&claims.sub)
                .map_err(|_e| AppError::InvalidToken)?
        );
        let tenant_id = TenantId::from_uuid(
            uuid::Uuid::parse_str(&claims.tenant_id)
                .map_err(|_e| AppError::InvalidToken)?
        );

        // Verify session exists
        let refresh_token_hash = Self::hash_token(&req.refresh_token);
        let session_row = sqlx::query!(
            r#"
            SELECT session_id, user_id, tenant_id, refresh_token_hash,
                   created_at, expires_at, last_activity, ip_address, user_agent
            FROM sessions
            WHERE refresh_token_hash = $1 AND expires_at > NOW()
            "#,
            refresh_token_hash
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get session: {}", e)))?
        .ok_or_else(|| AppError::InvalidToken)?;

        let session = Session {
            session_id: session_row.session_id.to_string(),
            user_id: UserId::from_uuid(session_row.user_id),
            tenant_id: TenantId::from_uuid(session_row.tenant_id),
            refresh_token_hash: session_row.refresh_token_hash,
            created_at: session_row.created_at,
            expires_at: session_row.expires_at,
            last_activity: session_row.last_activity,
            ip_address: session_row.ip_address,
            user_agent: session_row.user_agent,
        };

        // Get user role
        let user_row = sqlx::query!(
            "SELECT role FROM users WHERE user_id = $1",
            user_id.as_uuid()
        )
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user: {}", e)))?;

        let role = user_row.role.parse::<UserRole>()
            .map_err(|e| AppError::ValidationError(format!("Invalid role: {}", e)))?;

        // Generate new tokens
        let new_access_token = self.jwt_manager.generate_access_token(user_id, tenant_id, role)?;
        let new_refresh_token = self.jwt_manager.generate_refresh_token(user_id, tenant_id)?;

        // Update session
        self.session_manager
            .update_last_activity(&session.session_id)
            .await?;

        tracing::info!(user_id = %user_id, "Token refreshed");

        Ok(RefreshTokenResponse {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
            expires_in: self.config.jwt.access_token_expiry_seconds,
            token_type: "Bearer".to_string(),
        })
    }

    /// Logout a user
    pub async fn logout(&self, req: LogoutRequest) -> Result<()> {
        let refresh_token_hash = Self::hash_token(&req.refresh_token);

        // Get session
        let session = sqlx::query!(
            "SELECT session_id FROM sessions WHERE refresh_token_hash = $1",
            refresh_token_hash
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get session: {}", e)))?
        .ok_or_else(|| AppError::InvalidToken)?;

        // Delete session
        self.session_manager.delete_session(&session.session_id.to_string()).await?;

        tracing::info!(session_id = %session.session_id, "User logged out");

        Ok(())
    }

    /// Register a new user
    pub async fn register(&self, req: RegisterRequest) -> Result<RegisterResponse> {
        let email = Email::new(req.email)?;
        let password = Password::new(req.password)?;

        // Check if user already exists
        let existing = sqlx::query!(
            "SELECT user_id FROM users WHERE email = $1",
            email.as_str()
        )
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Database query failed: {}", e)))?;

        if existing.is_some() {
            return Err(AppError::DuplicateResource("Email already registered".to_string()));
        }

        // Hash password
        let password_hash = self.password_hasher.hash_password(&password)?;

        // Create user
        let user_id = UserId::new();
        sqlx::query!(
            r#"
            INSERT INTO users (user_id, tenant_id, email, password_hash, full_name, role)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user_id.as_uuid(),
            req.tenant_id.as_uuid(),
            email.as_str(),
            password_hash.as_str(),
            req.full_name,
            UserRole::Voter.to_string(),
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create user: {}", e)))?;

        tracing::info!(user_id = %user_id, email = %email.as_str(), "User registered");

        Ok(RegisterResponse {
            user_id,
            email: email.as_str().to_string(),
        })
    }

    /// Enable MFA for a user
    pub async fn enable_mfa(&self, user_id: UserId, email: &str) -> Result<EnableMfaResponse> {
        let secret = MfaManager::generate_secret()?;
        let qr_code_url = MfaManager::generate_qr_code_url(&secret, email, "EEMP")?;
        let backup_codes = MfaManager::generate_backup_codes();

        // Store MFA secret (not yet enabled until verified)
        sqlx::query!(
            "UPDATE users SET mfa_secret = $1 WHERE user_id = $2",
            secret,
            user_id.as_uuid()
        )
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to store MFA secret: {}", e)))?;

        // Store backup codes (hashed)
        for code in &backup_codes {
            let code_hash = Self::hash_token(code);
            sqlx::query!(
                "INSERT INTO mfa_backup_codes (user_id, code_hash) VALUES ($1, $2)",
                user_id.as_uuid(),
                code_hash
            )
            .execute(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to store backup code: {}", e)))?;
        }

        tracing::info!(user_id = %user_id, "MFA setup initiated");

        Ok(EnableMfaResponse {
            secret,
            qr_code_url,
            backup_codes,
        })
    }

    /// Verify MFA setup
    pub async fn verify_mfa(
        &self,
        user_id: UserId,
        email: &str,
        totp_code: &str,
    ) -> Result<VerifyMfaResponse> {
        // Get MFA secret
        let user_row = sqlx::query!(
            "SELECT mfa_secret FROM users WHERE user_id = $1",
            user_id.as_uuid()
        )
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user: {}", e)))?;

        let mfa_secret = user_row
            .mfa_secret
            .ok_or_else(|| AppError::ValidationError("MFA not set up".to_string()))?;

        // Verify TOTP code
        let is_valid = MfaManager::verify_totp(&mfa_secret, totp_code, email)?;

        if is_valid {
            // Enable MFA
            sqlx::query!(
                "UPDATE users SET mfa_enabled = TRUE WHERE user_id = $1",
                user_id.as_uuid()
            )
            .execute(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to enable MFA: {}", e)))?;

            tracing::info!(user_id = %user_id, "MFA enabled");
        }

        Ok(VerifyMfaResponse { verified: is_valid })
    }

    /// Hash a token using SHA-256
    fn hash_token(token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
