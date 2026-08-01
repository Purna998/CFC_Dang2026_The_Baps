//! Result Service for EEMP
//!
//! Provides:
//! - Vote counting and tallying
//! - Result calculation
//! - Winner determination
//! - Result publication

pub mod dto;
pub mod models;
pub mod repository;
pub mod service;
pub mod calculator;

pub use service::ResultService;
