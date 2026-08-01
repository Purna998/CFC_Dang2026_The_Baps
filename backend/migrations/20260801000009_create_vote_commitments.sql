-- Migration: Create vote commitments table
-- Description: Blockchain vote commitments for immutable verification

CREATE TABLE IF NOT EXISTS vote_commitments (
    commitment_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    election_id UUID NOT NULL REFERENCES elections(election_id) ON DELETE CASCADE,
    ballot_id UUID NOT NULL REFERENCES ballots(ballot_id) ON DELETE CASCADE,
    commitment_hash TEXT NOT NULL UNIQUE,
    blockchain_transaction_id TEXT,
    blockchain_network VARCHAR(50) NOT NULL DEFAULT 'solana',
    blockchain_block_height BIGINT,
    blockchain_timestamp TIMESTAMPTZ,
    signature TEXT NOT NULL,
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ
);

-- Indexes
CREATE INDEX idx_vote_commitments_tenant_id ON vote_commitments(tenant_id);
CREATE INDEX idx_vote_commitments_election_id ON vote_commitments(election_id);
CREATE INDEX idx_vote_commitments_ballot_id ON vote_commitments(ballot_id);
CREATE INDEX idx_vote_commitments_commitment_hash ON vote_commitments(commitment_hash);
CREATE INDEX idx_vote_commitments_tx_id ON vote_commitments(blockchain_transaction_id);
CREATE INDEX idx_vote_commitments_submitted_at ON vote_commitments(submitted_at DESC);

-- Row-Level Security
ALTER TABLE vote_commitments ENABLE ROW LEVEL SECURITY;

-- Policy: Vote commitments isolated by tenant
CREATE POLICY vote_commitments_tenant_isolation ON vote_commitments
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE vote_commitments IS 'Blockchain vote commitments for immutable verification (Solana)';
COMMENT ON COLUMN vote_commitments.commitment_hash IS 'SHA-256 hash of vote data submitted to blockchain';
COMMENT ON COLUMN vote_commitments.blockchain_transaction_id IS 'Solana transaction signature';
COMMENT ON COLUMN vote_commitments.blockchain_block_height IS 'Solana block height/slot number';
COMMENT ON COLUMN vote_commitments.signature IS 'Ed25519 signature of commitment';
COMMENT ON COLUMN vote_commitments.confirmed_at IS 'When blockchain transaction was confirmed';
