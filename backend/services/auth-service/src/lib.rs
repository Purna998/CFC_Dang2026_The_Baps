//! Authentication Service for EEMP
//!
//! Provides:
//! - User authentication (login, logout)
//! - Password hashing with Argon2id
//! - JWT token generation and validation
//! - Multi-factor authentication (TOTP)
//! - Session management

pub mod dto;
pub mod password;
pub mod jwt;
pub mod session;
pub mod mfa;
pub mod service;

pub use service::AuthService;
