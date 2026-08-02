use eemp_error::{AppError, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// HTTP client for blockchain microservice
pub struct BlockchainClient {
    base_url: String,
    client: reqwest::Client,
}

/// Request to submit vote commitment to blockchain
#[derive(Debug, Serialize)]
pub struct BlockchainCommitmentRequest {
    pub election_id: String,
    pub voter_id: String,
    pub commitment_hash: String, // hex encoded SHA-256
    pub signature: String,        // hex encoded Ed25519 signature
    pub tenant_id: Option<String>,
}

/// Response from blockchain service
#[derive(Debug, Deserialize)]
pub struct BlockchainCommitmentResponse {
    pub transaction_id: String,
    pub slot: u64,
    pub timestamp: i64,
    pub status: String,
}

/// Error response from blockchain service
#[derive(Debug, Deserialize)]
pub struct BlockchainErrorResponse {
    pub error: String,
    pub message: String,
}

impl BlockchainClient {
    /// Create a new blockchain client
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Submit vote commitment to blockchain
    /// This is the CRITICAL dual-vote prevention call:
    /// - If voter already voted, blockchain returns 409 Conflict
    /// - On-chain PDA prevents duplicate commitments
    /// - Returns transaction ID if successful
    pub async fn submit_commitment(
        &self,
        election_id: Uuid,
        voter_id: Uuid,
        commitment_hash: &[u8; 32],
        signature: &[u8; 64],
        tenant_id: Uuid,
    ) -> Result<BlockchainCommitmentResponse> {
        let request = BlockchainCommitmentRequest {
            election_id: election_id.to_string(),
            voter_id: voter_id.to_string(),
            commitment_hash: hex::encode(commitment_hash),
            signature: hex::encode(signature),
            tenant_id: Some(tenant_id.to_string()),
        };

        tracing::info!(
            "Submitting vote commitment to blockchain - Election: {}, Voter: {}",
            election_id,
            voter_id
        );

        let response = self
            .client
            .post(format!("{}/api/v1/commitments", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to connect to blockchain service: {}", e);
                AppError::InternalError(format!("Blockchain service unavailable: {}", e))
            })?;

        match response.status() {
            status if status.is_success() => {
                let blockchain_response = response
                    .json::<BlockchainCommitmentResponse>()
                    .await
                    .map_err(|e| {
                        AppError::InternalError(format!(
                            "Failed to parse blockchain response: {}",
                            e
                        ))
                    })?;

                tracing::info!(
                    "Vote commitment submitted to blockchain - TX: {}",
                    blockchain_response.transaction_id
                );

                Ok(blockchain_response)
            }
            reqwest::StatusCode::CONFLICT => {
                // Blockchain detected duplicate vote!
                let error_response = response
                    .json::<BlockchainErrorResponse>()
                    .await
                    .ok();

                let message = error_response
                    .map(|e| e.message)
                    .unwrap_or_else(|| "Blockchain detected duplicate vote".to_string());

                tracing::warn!(
                    "Blockchain rejected duplicate vote - Election: {}, Voter: {}",
                    election_id,
                    voter_id
                );

                Err(AppError::AlreadyVoted)
            }
            status => {
                let error_text = response.text().await.unwrap_or_default();
                tracing::error!(
                    "Blockchain service error - Status: {}, Error: {}",
                    status,
                    error_text
                );

                Err(AppError::InternalError(format!(
                    "Blockchain service error ({}): {}",
                    status, error_text
                )))
            }
        }
    }

    /// Health check for blockchain service
    pub async fn health_check(&self) -> Result<bool> {
        match self
            .client
            .get(format!("{}/health", self.base_url))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_blockchain_client_creation() {
        let client = BlockchainClient::new("http://localhost:8001".to_string());
        assert_eq!(client.base_url, "http://localhost:8001");
    }
}
