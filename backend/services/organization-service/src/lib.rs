//! Organization Service for EEMP
//!
//! Provides:
//! - Multi-tenant organization management
//! - Organization CRUD operations
//! - Tenant resolution
//! - Organization settings

pub mod dto;
pub mod models;
pub mod repository;
pub mod service;

pub use service::OrganizationService;
