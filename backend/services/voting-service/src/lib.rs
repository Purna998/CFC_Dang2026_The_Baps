//! Voting Service for EEMP
//!
//! Provides:
//! - Ballot casting workflow
//! - Vote encryption
//! - Eligibility verification
//! - Receipt generation
//! - Duplicate vote prevention

pub mod dto;
pub mod models;
pub mod repository;
pub mod service;
pub mod eligibility;

pub use service::VotingService;
