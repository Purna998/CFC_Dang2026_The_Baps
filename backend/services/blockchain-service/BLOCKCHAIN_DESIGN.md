# Blockchain Service - Production Design

## Purpose

The blockchain service is **THE PRIMARY SOLUTION** for preventing:
- ✅ Dual voting (same person voting twice)
- ✅ Invalid paper ballots (tampered or forged votes)
- ✅ Vote manipulation after casting
- ✅ Centralized database tampering
- ✅ Lack of public verifiability

## Current Implementation vs Production

### Current (MVP/Placeholder)
```rust
// services/blockchain-service/src/client.rs:60-113
// Uses simple Solana transfer transactions as placeholders
pub async fn submit_commitment(&self, commitment_hash: &str, signature: &str) -> Result<String> {
    // Creates a memo transaction
    let memo_data = format!("VOTE_COMMITMENT:{}", commitment_hash);
    
    // Self-transfer of 1 lamport
    let instruction = system_instruction::transfer(
        &self.payer.pubkey(),
        &self.payer.pubkey(),
        1,
    );
    
    // Submit to Solana
    let tx_signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
    
    Ok(tx_signature.to_string())
}
```

**Limitations:**
- ❌ Commitment data not actually stored on-chain
- ❌ No smart contract validation
- ❌ Cannot query commitments by election
- ❌ No on-chain duplicate prevention

### Production Implementation (Required)

#### 1. Solana Anchor Smart Contract

```rust
// programs/vote-commitment/src/lib.rs
use anchor_lang::prelude::*;

declare_id!("VoteCommitProgram11111111111111111111111111");

#[program]
pub mod vote_commitment {
    use super::*;

    /// Submit a vote commitment
    /// This is called ONCE per voter per election
    pub fn submit_commitment(
        ctx: Context<SubmitCommitment>,
        election_id: [u8; 16],      // UUID as bytes
        voter_id_hash: [u8; 32],    // SHA-256 hash of voter_id (privacy)
        commitment_hash: [u8; 32],  // SHA-256 of encrypted ballot
        signature: [u8; 64],        // Ed25519 signature
        timestamp: i64,
    ) -> Result<()> {
        let commitment = &mut ctx.accounts.commitment;
        
        // Verify this voter hasn't already voted in this election
        require!(
            commitment.election_id == [0; 16],
            ErrorCode::AlreadyVoted
        );
        
        // Store commitment on-chain (permanent, immutable)
        commitment.election_id = election_id;
        commitment.voter_id_hash = voter_id_hash;
        commitment.commitment_hash = commitment_hash;
        commitment.signature = signature;
        commitment.timestamp = timestamp;
        commitment.bump = *ctx.bumps.get("commitment").unwrap();
        
        emit!(VoteCommitmentSubmitted {
            election_id,
            voter_id_hash,
            commitment_hash,
            timestamp,
        });
        
        Ok(())
    }
    
    /// Verify a commitment exists
    pub fn verify_commitment(
        ctx: Context<VerifyCommitment>,
        commitment_hash: [u8; 32],
    ) -> Result<bool> {
        let commitment = &ctx.accounts.commitment;
        Ok(commitment.commitment_hash == commitment_hash)
    }
}

#[derive(Accounts)]
#[instruction(election_id: [u8; 16], voter_id_hash: [u8; 32])]
pub struct SubmitCommitment<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + VoteCommitment::INIT_SPACE,
        seeds = [
            b"commitment",
            election_id.as_ref(),
            voter_id_hash.as_ref(),
        ],
        bump
    )]
    pub commitment: Account<'info, VoteCommitment>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyCommitment<'info> {
    pub commitment: Account<'info, VoteCommitment>,
}

/// On-chain vote commitment data
#[account]
#[derive(InitSpace)]
pub struct VoteCommitment {
    pub election_id: [u8; 16],         // 16 bytes
    pub voter_id_hash: [u8; 32],       // 32 bytes (privacy-preserving)
    pub commitment_hash: [u8; 32],     // 32 bytes (ballot hash)
    pub signature: [u8; 64],           // 64 bytes (Ed25519)
    pub timestamp: i64,                // 8 bytes
    pub bump: u8,                      // 1 byte
    // Total: 153 bytes + 8 byte discriminator = 161 bytes
}

#[event]
pub struct VoteCommitmentSubmitted {
    pub election_id: [u8; 16],
    pub voter_id_hash: [u8; 32],
    pub commitment_hash: [u8; 32],
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("This voter has already voted in this election")]
    AlreadyVoted,
}
```

#### 2. Updated Rust Client

```rust
// services/blockchain-service/src/client.rs (PRODUCTION VERSION)
use anchor_client::{Client, Cluster, Program};
use anchor_lang::prelude::*;
use solana_sdk::signature::{Keypair, Signer};

pub struct SolanaClient {
    program: Program,
    payer: Keypair,
}

impl SolanaClient {
    pub fn new(rpc_url: &str, payer_keypair: Keypair, program_id: Pubkey) -> Result<Self> {
        let cluster = Cluster::Custom(rpc_url.to_string(), "wss://...".to_string());
        let client = Client::new(cluster, payer_keypair.clone());
        let program = client.program(program_id);
        
        Ok(Self {
            program,
            payer: payer_keypair,
        })
    }
    
    /// Submit vote commitment to blockchain
    /// This will FAIL if voter already voted (on-chain duplicate check)
    pub async fn submit_commitment(
        &self,
        election_id: Uuid,
        voter_id: Uuid,
        commitment_hash: [u8; 32],
        signature: [u8; 64],
    ) -> Result<String> {
        // Hash voter_id for privacy (don't store actual user IDs on-chain)
        let voter_id_hash = sha256(voter_id.as_bytes());
        
        // Derive PDA (Program Derived Address) for this commitment
        let (commitment_pda, _bump) = Pubkey::find_program_address(
            &[
                b"commitment",
                election_id.as_bytes(),
                &voter_id_hash,
            ],
            &self.program.id(),
        );
        
        // Submit transaction
        let tx_sig = self
            .program
            .request()
            .accounts(vote_commitment::accounts::SubmitCommitment {
                commitment: commitment_pda,
                payer: self.payer.pubkey(),
                system_program: system_program::ID,
            })
            .args(vote_commitment::instruction::SubmitCommitment {
                election_id: election_id.as_bytes()[..16].try_into().unwrap(),
                voter_id_hash,
                commitment_hash,
                signature,
                timestamp: chrono::Utc::now().timestamp(),
            })
            .signer(&self.payer)
            .send()
            .await?;
        
        tracing::info!(
            election_id = %election_id,
            tx_signature = %tx_sig,
            "Vote commitment submitted to Solana"
        );
        
        Ok(tx_sig.to_string())
    }
    
    /// Check if voter has already voted (on-chain check)
    pub async fn has_voted(
        &self,
        election_id: Uuid,
        voter_id: Uuid,
    ) -> Result<bool> {
        let voter_id_hash = sha256(voter_id.as_bytes());
        
        let (commitment_pda, _) = Pubkey::find_program_address(
            &[
                b"commitment",
                election_id.as_bytes(),
                &voter_id_hash,
            ],
            &self.program.id(),
        );
        
        // Try to fetch the account
        match self.program.account::<VoteCommitment>(commitment_pda).await {
            Ok(commitment) => {
                // Account exists = already voted
                Ok(commitment.election_id != [0; 16])
            }
            Err(_) => {
                // Account doesn't exist = hasn't voted yet
                Ok(false)
            }
        }
    }
    
    /// Verify a commitment exists on-chain
    pub async fn verify_commitment(
        &self,
        election_id: Uuid,
        voter_id: Uuid,
        expected_hash: [u8; 32],
    ) -> Result<bool> {
        let voter_id_hash = sha256(voter_id.as_bytes());
        
        let (commitment_pda, _) = Pubkey::find_program_address(
            &[
                b"commitment",
                election_id.as_bytes(),
                &voter_id_hash,
            ],
            &self.program.id(),
        );
        
        let commitment: VoteCommitment = self
            .program
            .account(commitment_pda)
            .await?;
        
        Ok(commitment.commitment_hash == expected_hash)
    }
}

fn sha256(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
```

## How It Prevents Dual Voting

### Scenario 1: Normal Vote
```
User A votes in Election X

1. Backend checks database: has_voted = false ✓
2. Backend calls blockchain: submit_commitment(election_x, user_a_hash, ...)
3. Solana smart contract:
   - Derives PDA: seeds = [b"commitment", election_x, user_a_hash]
   - Checks if account exists: NO
   - Creates account with commitment data (PERMANENT)
   - Returns: Success
4. Backend stores in database
5. User receives receipt
```

### Scenario 2: Dual Vote Attempt (BLOCKED)
```
User A tries to vote AGAIN in Election X

1. Backend checks database: has_voted = true ✗
   → Returns: 409 Conflict "You have already voted"
   → BLOCKED at application layer

If somehow database check is bypassed:

2. Backend calls blockchain: submit_commitment(election_x, user_a_hash, ...)
3. Solana smart contract:
   - Derives PDA: seeds = [b"commitment", election_x, user_a_hash]
   - Checks if account exists: YES (from first vote)
   - init instruction FAILS (account already exists)
   - Returns: Error "AlreadyVoted"
   → BLOCKED at blockchain layer
4. Transaction fails, no second vote recorded
```

### Scenario 3: Database Tampered (Blockchain Catches It)
```
Malicious admin deletes ballot from PostgreSQL

1. Database: ballot record deleted
2. Blockchain: commitment still exists (IMMUTABLE)
3. Audit process:
   - Query blockchain for all commitments in election
   - Compare with database ballots
   - Mismatch detected: blockchain has commitment but DB doesn't
   - Evidence of tampering preserved forever
```

## Advantages Over Database-Only

| Feature | Database Only | Database + Blockchain |
|---------|--------------|----------------------|
| **Duplicate Prevention** | ✓ UNIQUE constraint | ✓✓ UNIQUE + On-chain PDA |
| **Tampering Detection** | ✗ Admin can delete | ✓ Immutable, public audit |
| **Public Verification** | ✗ Requires DB access | ✓ Anyone can query chain |
| **Single Point of Failure** | ✗ Database is SPOF | ✓ Distributed consensus |
| **Audit Trail** | ~ Can be altered | ✓ Permanent, timestamped |
| **Trust Model** | Must trust operators | ✓ Trustless verification |

## Deployment Checklist

### Development
- [ ] Write Anchor smart contract
- [ ] Write tests (Anchor test framework)
- [ ] Deploy to Solana devnet
- [ ] Update client.rs with Anchor client
- [ ] Test with API Gateway
- [ ] Run integration tests

### Production
- [ ] Security audit of smart contract
- [ ] Deploy to Solana mainnet
- [ ] Configure program ID in .env
- [ ] Fund payer wallet with SOL
- [ ] Set up monitoring (transaction failures)
- [ ] Document voter verification process
- [ ] Train staff on blockchain verification

## Cost Analysis

**Solana Transaction Costs:**
- Account creation: ~0.002 SOL (~$0.20 USD)
- Vote commitment: 161 bytes storage
- Lamports per vote: ~0.00000005 SOL per signature

**For 1,000 votes:**
- Storage: 161 KB
- Cost: ~2 SOL (~$200 USD)
- Verification: FREE (read operations)

**For 1,000,000 votes:**
- Storage: 161 MB
- Cost: ~2,000 SOL (~$200,000 USD)

## Alternative: Layer 2 Solution

For high-volume elections, consider:
- **Solana Compression** (cNFTs) - 1000x cheaper
- **Arweave** for permanent storage + Solana for indexing
- **Batch commitments** (Merkle tree of multiple votes)

## References

- Solana Anchor Framework: https://www.anchor-lang.com/
- PDAs (Program Derived Addresses): https://solanacookbook.com/core-concepts/pdas.html
- Solana Program Library: https://spl.solana.com/
- Vote Commitment Schemes: https://en.wikipedia.org/wiki/Commitment_scheme

---

**Status:** Current implementation is placeholder. Production smart contract required for full dual-vote prevention via blockchain.

**Priority:** HIGH - Blockchain is identified as THE solution for election integrity.
