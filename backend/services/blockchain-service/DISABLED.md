# Blockchain Service - Temporarily Disabled

**Status:** Disabled due to Solana SDK 2.0 dependency conflicts

## Issue

The Solana SDK 2.0.x has breaking changes and dependency conflicts with the broader Rust ecosystem:
- Multiple versions of `solana-program` in dependency tree (1.18.26 vs 2.0.25)
- SPL (Solana Program Library) crates haven't migrated to 2.0 yet
- Type mismatches between different `solana-program` versions

## Temporary Solution

The `blockchain-service` has been commented out of the workspace `Cargo.toml` to allow other services to compile and run.

## Mock Implementation Available

For development and testing, use the mock blockchain service:

```rust
// Mock blockchain client that logs instead of submitting to Solana
pub struct MockBlockchainClient {
    enabled: bool,
}

impl MockBlockchainClient {
    pub async fn submit_vote_commitment(
        &self,
        election_id: Uuid,
        vote_hash: [u8; 32],
        commitment: [u8; 64],
    ) -> Result<String, BlockchainError> {
        tracing::info!(
            "MOCK: Would submit vote commitment to blockchain: \
             election={}, hash={}, commitment={}",
            election_id,
            hex::encode(vote_hash),
            hex::encode(commitment)
        );
        
        // Return mock transaction signature
        Ok(format!("mock_tx_{}", Uuid::new_v4()))
    }
}
```

## Resolution Path

### Option A: Downgrade to Solana SDK 1.18.x (RECOMMENDED)

**Effort:** ~4 hours  
**Risk:** Low  

1. Update workspace `Cargo.toml`:
   ```toml
   solana-sdk = "1.18"
   solana-client = "1.18"
   anchor-client = "0.30"
   ```

2. Update blockchain-service `Cargo.toml` to match

3. Update Anchor program if needed

4. Test compilation

5. Uncomment blockchain-service in workspace

### Option B: Wait for Ecosystem

**Effort:** Unknown  
**Risk:** High (timeline unknown)

Wait for SPL crates to migrate to Solana SDK 2.0.

This could take weeks or months.

### Option C: Fork and Patch SPL Crates

**Effort:** ~16 hours  
**Risk:** Very High (maintenance burden)

Fork affected SPL crates and update them to Solana 2.0.

Not recommended for production use.

## Impact on MVP

**Vote storage still works** - encrypted ballots are stored in PostgreSQL.

**Vote verification possible** - using database receipts and Ed25519 signatures.

**Blockchain immutability missing** - no on-chain audit trail until resolved.

## Testing Without Blockchain

Use the mock client:

```bash
# Set environment variable
export BLOCKCHAIN_ENABLED=false

# Run tests
cargo test --workspace

# Start API gateway
cargo run --bin api-gateway
```

The system will log blockchain operations but not submit to Solana.

## When Re-enabling

1. Uncomment in `backend/Cargo.toml`
2. Verify compilation: `cargo check -p eemp-blockchain-service`
3. Run blockchain tests: `cargo test -p eemp-blockchain-service`
4. Deploy Anchor program to devnet
5. Update API to use real blockchain client

## References

- Solana SDK 2.0 Release Notes: https://github.com/solana-labs/solana/releases/tag/v2.0.0
- SPL Migration Tracking: https://github.com/solana-labs/solana-program-library/issues
- Anchor Framework Compatibility: https://github.com/coral-xyz/anchor

---

**Last Updated:** 2026-08-02  
**Next Review:** When SPL ecosystem migrates to 2.0 or after downgrade decision
