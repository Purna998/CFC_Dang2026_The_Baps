//! Election Service for EEMP
//!
//! Provides:
//! - Election lifecycle management
//! - State machine transitions
//! - Position and candidate management
//! - Eligibility rules

pub mod dto;
pub mod models;
pub mod repository;
pub mod service;
pub mod state_machine;

pub use service::ElectionService;
