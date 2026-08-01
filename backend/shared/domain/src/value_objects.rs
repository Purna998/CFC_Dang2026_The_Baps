//! Value objects - immutable domain primitives
//!
//! Following DDD principles, value objects are immutable and identified by their attributes.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

/// Tenant identifier (Organization ID)
///
/// Every request must include a tenant context for multi-tenancy isolation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(Uuid);

impl TenantId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for TenantId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TenantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for TenantId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl FromStr for TenantId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s)?))
    }
}

impl From<TenantId> for Uuid {
    fn from(tenant_id: TenantId) -> Self {
        tenant_id.0
    }
}

/// User identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s)?))
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

/// Election identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ElectionId(Uuid);

impl ElectionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for ElectionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ElectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ElectionId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ElectionId> for Uuid {
    fn from(election_id: ElectionId) -> Self {
        election_id.0
    }
}

/// Candidate identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CandidateId(Uuid);

impl CandidateId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for CandidateId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CandidateId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Email address value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
pub struct Email(#[validate(email)] String);

impl Email {
    pub fn new(email: String) -> Result<Self, validator::ValidationErrors> {
        let email_obj = Self(email);
        email_obj.validate()?;
        Ok(email_obj)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Email {
    type Error = validator::ValidationErrors;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Email::new(s)
    }
}

/// Password value object (not stored, used for validation only)
///
/// NEVER store this type - always hash with Argon2id before storage.
#[derive(Debug, Clone, Validate)]
pub struct Password(#[validate(length(min = 8))] String);

impl Password {
    pub fn new(password: String) -> Result<Self, validator::ValidationErrors> {
        let password_obj = Self(password);
        password_obj.validate()?;

        // Additional password policy validation
        // TODO: Implement full password policy from security architecture

        Ok(password_obj)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

// Don't implement Display or Serialize for Password (security)

/// Password hash (Argon2id output)
///
/// This is what gets stored in the database, never the plaintext password.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for PasswordHash {
    fn from(hash: String) -> Self {
        Self(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_id_creation() {
        let tenant_id = TenantId::new();
        assert_ne!(tenant_id.as_uuid(), &Uuid::nil());
    }

    #[test]
    fn test_email_validation() {
        // Valid email
        let email = Email::new("user@example.com".to_string());
        assert!(email.is_ok());

        // Invalid email
        let email = Email::new("not-an-email".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_password_validation() {
        // Valid password (8+ characters)
        let password = Password::new("SecurePassword123!".to_string());
        assert!(password.is_ok());

        // Invalid password (too short)
        let password = Password::new("short".to_string());
        assert!(password.is_err());
    }
}
