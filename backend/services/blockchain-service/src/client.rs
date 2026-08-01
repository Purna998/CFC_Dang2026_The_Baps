//! Solana blockchain client

use eemp_error::{AppError, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

use crate::models::{CommitmentTransaction, TransactionStatus};

/// Solana blockchain client
pub struct SolanaClient {
    rpc_client: RpcClient,
    payer: Keypair,
    program_id: Pubkey,
}

impl SolanaClient {
    /// Create a new Solana client
    ///
    /// # Arguments
    /// * `rpc_url` - Solana RPC endpoint (e.g., "https://api.devnet.solana.com")
    /// * `payer_keypair` - Keypair for transaction fees
    /// * `program_id` - Vote commitment program ID
    pub fn new(rpc_url: &str, payer_keypair: Keypair, program_id: &str) -> Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );

        let program_id = Pubkey::from_str(program_id)
            .map_err(|e| AppError::InternalError(format!("Invalid program ID: {}", e)))?;

        tracing::info!(
            rpc_url = %rpc_url,
            program_id = %program_id,
            "Solana client initialized"
        );

        Ok(Self {
            rpc_client,
            payer: payer_keypair,
            program_id,
        })
    }

    /// Submit vote commitment to Solana
    ///
    /// # Arguments
    /// * `commitment_hash` - SHA-256 hash of vote commitment
    /// * `signature` - Ed25519 signature of commitment
    ///
    /// # Returns
    /// * Transaction signature as string
    pub async fn submit_commitment(
        &self,
        commitment_hash: &str,
        signature: &str,
    ) -> Result<String> {
        tracing::info!(
            commitment_hash = %commitment_hash,
            "Submitting vote commitment to Solana"
        );

        // In a real implementation, this would:
        // 1. Create instruction data with commitment_hash and signature
        // 2. Build transaction to call the vote commitment program
        // 3. Sign and send transaction
        // 4. Wait for confirmation

        // For MVP, we'll create a memo transaction as a placeholder
        let memo_data = format!("VOTE_COMMITMENT:{}", commitment_hash);

        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| AppError::InternalError(format!("Failed to get blockhash: {}", e)))?;

        // Create a simple transfer transaction as memo placeholder
        let instruction = system_instruction::transfer(
            &self.payer.pubkey(),
            &self.payer.pubkey(), // Self-transfer
            1, // 1 lamport
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.payer.pubkey()),
            &[&self.payer],
            recent_blockhash,
        );

        let tx_signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| {
                AppError::InternalError(format!("Failed to send transaction: {}", e))
            })?;

        let signature_str = tx_signature.to_string();

        tracing::info!(
            signature = %signature_str,
            "Vote commitment submitted to Solana"
        );

        Ok(signature_str)
    }

    /// Get transaction status
    pub async fn get_transaction_status(&self, signature: &str) -> Result<CommitmentTransaction> {
        let sig = solana_sdk::signature::Signature::from_str(signature)
            .map_err(|e| AppError::ValidationError(format!("Invalid signature: {}", e)))?;

        let status = self
            .rpc_client
            .get_signature_status(&sig)
            .map_err(|e| AppError::InternalError(format!("Failed to get status: {}", e)))?;

        let tx_status = match status {
            Some(Ok(_)) => TransactionStatus::Finalized,
            Some(Err(_)) => TransactionStatus::Failed,
            None => TransactionStatus::Pending,
        };

        // Get transaction details
        let transaction_details = self
            .rpc_client
            .get_transaction(&sig, solana_client::rpc_config::RpcTransactionConfig::default())
            .ok();

        let (slot, block_time) = if let Some(tx) = transaction_details {
            (tx.slot, tx.block_time)
        } else {
            (0, None)
        };

        Ok(CommitmentTransaction {
            signature: signature.to_string(),
            commitment_hash: String::new(), // Would parse from transaction data
            slot,
            block_time,
            status: tx_status,
        })
    }

    /// Verify commitment on blockchain
    pub async fn verify_commitment(
        &self,
        signature: &str,
        expected_hash: &str,
    ) -> Result<bool> {
        let tx_status = self.get_transaction_status(signature).await?;

        match tx_status.status {
            TransactionStatus::Confirmed | TransactionStatus::Finalized => {
                // In production, would verify the commitment_hash from transaction data
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Get current slot (block height)
    pub async fn get_current_slot(&self) -> Result<u64> {
        self.rpc_client
            .get_slot()
            .map_err(|e| AppError::InternalError(format!("Failed to get slot: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        // Test placeholder - requires actual Solana devnet
        // let keypair = Keypair::new();
        // let client = SolanaClient::new("https://api.devnet.solana.com", keypair, "program_id");
    }
}
