//! Blockchain domain models

use serde::{Deserialize, Serialize};

/// Transaction status on Solana
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Finalized,
    Failed,
}

/// Vote commitment transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentTransaction {
    pub signature: String,
    pub commitment_hash: String,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub status: TransactionStatus,
}
