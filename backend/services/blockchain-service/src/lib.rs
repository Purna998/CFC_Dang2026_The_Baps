//! Blockchain Service for EEMP
//!
//! Provides Solana blockchain integration for vote commitment storage.

pub mod client;
pub mod models;

pub use client::SolanaClient;
pub use models::{CommitmentTransaction, TransactionStatus};
