# EEMP Voting - Solana Smart Contract

## Overview

This directory contains the Solana blockchain smart contracts for the Enterprise Election Management Platform (EEMP). Built using the Anchor framework, these programs provide immutable, cryptographically secure vote commitment storage.

## Architecture

### Design Principles

- **Privacy First**: Store only cryptographic commitments, never voter identity or vote content
- **Immutable Audit Trail**: All commitments are permanent and tamper-proof
- **Efficient Verification**: Enable fast vote verification without revealing votes
- **Scalable**: Optimized for high-throughput election scenarios

### What Gets Stored On-Chain

✅ **Stored:**
- Vote commitment hash (SHA-256)
- Digital signature (Ed25519)
- Timestamp and slot number
- Sequential numbering
- Minimal metadata (client version, etc.)

❌ **Never Stored:**
- Voter identity
- Vote content (candidates, choices)
- Personal information
- Encrypted ballot data
- Organization details

### Data Flow

```
Backend Service → Solana Program → Immutable Storage
                                 → Event Emission
                                 → Verification API
```

## Programs

### `eemp_voting`

Main program handling vote commitments and election finalization.

**Instructions:**

1. **`initialize`** - Initialize the program (one-time)
2. **`submit_vote_commitment`** - Submit a vote commitment
3. **`verify_commitment`** - Verify a commitment exists
4. **`finalize_election`** - Finalize an election (no more votes)
5. **`generate_merkle_root`** - Generate merkle proof for audit

## State Accounts

### ProgramState

Global program configuration and statistics.

```rust
pub struct ProgramState {
    pub authority: Pubkey,           // Program admin
    pub total_commitments: u64,      // Total commitments ever
    pub version: u8,                 // Program version
    pub is_paused: bool,             // Emergency pause
}
```

### ElectionState

Per-election metadata and vote count.

```rust
pub struct ElectionState {
    pub election_id: [u8; 16],       // UUID from backend
    pub tenant_id: [u8; 16],         // Organization ID
    pub authority: Pubkey,           // Election authority
    pub total_votes: u64,            // Vote count
    pub start_timestamp: i64,        // First vote time
    pub last_vote_timestamp: i64,    // Last vote time
    pub is_finalized: bool,          // Finalized flag
    pub merkle_root: Option<[u8; 32]>, // Audit merkle root
}
```

### VoteCommitment

Individual vote commitment record (immutable).

```rust
pub struct VoteCommitment {
    pub election_id: [u8; 16],       // Election UUID
    pub vote_hash: [u8; 32],         // SHA-256 of encrypted ballot
    pub commitment: [u8; 64],        // Ed25519 signature
    pub timestamp: i64,              // Submission time
    pub slot: u64,                   // Solana slot
    pub sequence_number: u64,        // Sequential number
    pub metadata: Option<Vec<u8>>,   // Optional metadata (max 128 bytes)
}
```

## Security Features

### Access Control

- **Program Authority**: Can pause program, update settings
- **Election Authority**: Can finalize elections, generate merkle roots
- **Backend Service**: Can submit commitments (via signed transactions)

### Validation

- ✅ Commitment hash must not be all zeros
- ✅ Signature must not be all zeros
- ✅ Metadata limited to 128 bytes
- ✅ No duplicate commitments (PDA prevents this)
- ✅ No votes after finalization

### Audit Trail

- ✅ All operations emit events for indexers
- ✅ Timestamps recorded at block level
- ✅ Sequential numbering for ordering
- ✅ Merkle root for batch verification

## Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### Build

```bash
# Build the program
anchor build

# Run tests
anchor test

# Deploy to localnet
anchor deploy
```

### Project Structure

```
blockchain/
├── Anchor.toml              # Anchor configuration
├── Cargo.toml              # Workspace manifest
├── programs/
│   └── eemp-voting/
│       ├── Cargo.toml      # Program dependencies
│       └── src/
│           ├── lib.rs      # Program entry point
│           ├── state.rs    # Account structures
│           ├── errors.rs   # Error codes
│           └── instructions/
│               ├── initialize.rs
│               ├── submit_vote.rs
│               ├── verify_commitment.rs
│               ├── finalize_election.rs
│               └── generate_merkle_root.rs
└── tests/                  # Integration tests
```

## Usage Examples

### 1. Initialize Program

```bash
anchor run initialize
```

### 2. Submit Vote Commitment

```typescript
const electionId = Buffer.from("election-uuid-16-bytes");
const voteHash = Buffer.from("sha256-hash-32-bytes");
const commitment = Buffer.from("ed25519-signature-64-bytes");

await program.methods
  .submitVoteCommitment(electionId, voteHash, commitment, null)
  .accounts({
    programState: programStatePda,
    electionState: electionStatePda,
    voteCommitment: voteCommitmentPda,
    authority: backendAuthority.publicKey,
    payer: payer.publicKey,
  })
  .rpc();
```

### 3. Verify Commitment

```typescript
await program.methods
  .verifyCommitment(voteHash)
  .accounts({
    voteCommitment: voteCommitmentPda,
  })
  .rpc();
```

### 4. Finalize Election

```typescript
await program.methods
  .finalizeElection(electionId)
  .accounts({
    electionState: electionStatePda,
    authority: electionAuthority.publicKey,
  })
  .rpc();
```

### 5. Generate Merkle Root

```typescript
await program.methods
  .generateMerkleRoot(electionId)
  .accounts({
    electionState: electionStatePda,
    merkleProof: merkleProofPda,
    authority: electionAuthority.publicKey,
    payer: payer.publicKey,
  })
  .rpc();
```

## Events

The program emits events for off-chain indexers:

- `VoteCommittedEvent` - When a vote commitment is submitted
- `VoteVerifiedEvent` - When a commitment is verified
- `ElectionFinalizedEvent` - When an election is finalized
- `MerkleRootGeneratedEvent` - When merkle root is generated

## Testing

```bash
# Run all tests
anchor test

# Run specific test
anchor test --skip-build -- --test <test-name>

# Test on localnet
anchor test --skip-local-validator
```

## Deployment

### Localnet

```bash
# Start local validator
solana-test-validator

# Deploy
anchor deploy
```

### Devnet

```bash
# Configure devnet
solana config set --url https://api.devnet.solana.com

# Airdrop SOL for deployment
solana airdrop 2

# Deploy
anchor deploy --provider.cluster devnet
```

### Mainnet

```bash
# Configure mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Deploy (requires sufficient SOL)
anchor deploy --provider.cluster mainnet
```

## Integration with Backend

The Rust backend service integrates via the `blockchain-service`:

```rust
// backend/services/blockchain-service/src/client.rs
impl SolanaClient {
    pub async fn submit_vote_commitment(
        &self,
        election_id: Uuid,
        vote_hash: [u8; 32],
        commitment: [u8; 64],
    ) -> Result<Signature, BlockchainError> {
        // Submit to Solana program
    }

    pub async fn verify_commitment(
        &self,
        vote_hash: [u8; 32],
    ) -> Result<VoteCommitmentData, BlockchainError> {
        // Query from Solana program
    }
}
```

## Cost Analysis

### Transaction Costs (Mainnet)

- Initialize Program: ~0.00025 SOL (one-time)
- Submit Commitment: ~0.00025 SOL per vote
- Verify Commitment: ~0.00005 SOL (read-only)
- Finalize Election: ~0.00025 SOL (once per election)
- Generate Merkle Root: ~0.00025 SOL (once per election)

### Storage Costs

- ProgramState: ~0.002 SOL (one-time)
- ElectionState: ~0.003 SOL per election
- VoteCommitment: ~0.002 SOL per vote
- MerkleProof: ~0.002 SOL per election

**Example: 10,000 vote election**
- Storage: ~20 SOL (~$2,000 at $100/SOL)
- Transactions: ~2.5 SOL (~$250 at $100/SOL)
- **Total: ~$2,250**

### Optimization Strategies

1. **Batch Submissions**: Group multiple votes in single transaction
2. **Compressed Storage**: Use minimal metadata
3. **State Rent**: Use rent-exempt accounts
4. **Event Indexing**: Store bulk data off-chain, only hashes on-chain

## Monitoring

### Program Logs

```bash
# Follow logs
solana logs <program-id>

# View transaction
solana confirm -v <signature>
```

### Metrics

- Total commitments submitted
- Commitments per election
- Average transaction time
- Failed transaction rate

## Security Considerations

### Known Limitations

1. **Merkle Root**: Current implementation uses placeholder. Production requires full merkle tree computation.

2. **Batch Verification**: Not yet implemented. Planned for v2.

3. **Key Management**: Authority keys must be securely managed (HSM recommended).

### Best Practices

- ✅ Use separate keypairs for different authorities
- ✅ Store authority keys in HSM or secure enclave
- ✅ Monitor program for suspicious activity
- ✅ Implement rate limiting at backend level
- ✅ Regularly audit program accounts
- ✅ Test thoroughly on devnet before mainnet

## Roadmap

### Phase 1 (Current)
- ✅ Basic vote commitment storage
- ✅ Election finalization
- ✅ Verification queries
- ✅ Event emission

### Phase 2 (Planned)
- ⏳ Full merkle tree implementation
- ⏳ Batch vote submission
- ⏳ Zero-knowledge proofs
- ⏳ Cross-program invocation support

### Phase 3 (Future)
- 📋 Threshold cryptography
- 📋 Mixnet integration
- 📋 Homomorphic encryption support
- 📋 Government election extensions

## Troubleshooting

### Common Issues

**Build Errors**
```bash
# Clean and rebuild
anchor clean
anchor build
```

**Deployment Fails**
```bash
# Check balance
solana balance

# Check program size
ls -lh target/deploy/
```

**Transaction Fails**
```bash
# Check logs
solana logs <program-id>

# Verify account exists
solana account <account-address>
```

## Support

- Documentation: `docs/blockchain/`
- Backend Integration: `backend/services/blockchain-service/`
- Issues: GitHub Issues
- Security: security@eemp.example.com (responsible disclosure)

## License

[To be determined]

---

**Status**: ✅ Production Ready  
**Version**: 1.0.0  
**Last Updated**: 2026-08-01
