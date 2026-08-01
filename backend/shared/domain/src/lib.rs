//! Domain primitives and value objects shared across all services
//!
//! This library provides the foundational domain types used throughout EEMP:
//! - Value objects (TenantId, UserId, ElectionId, etc.)
//! - Domain entities (User, Election, Candidate, etc.)
//! - Common enums (UserRole, ElectionStatus, etc.)

pub mod value_objects;
pub mod entities;
pub mod enums;

// Re-export commonly used types
pub use value_objects::*;
pub use entities::*;
pub use enums::*;
