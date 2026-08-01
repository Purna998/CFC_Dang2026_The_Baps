-- Migration: Create ballots table
-- Description: Encrypted ballots (votes) - the core voting data

CREATE TABLE IF NOT EXISTS ballots (
    ballot_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    election_id UUID NOT NULL REFERENCES elections(election_id) ON DELETE CASCADE,
    voter_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    encrypted_ballot TEXT NOT NULL,
    encryption_key_id UUID NOT NULL,
    ballot_hash TEXT NOT NULL,
    voter_receipt_code TEXT NOT NULL,
    ip_address VARCHAR(45),
    user_agent TEXT,
    cast_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    verified_at TIMESTAMPTZ,
    CONSTRAINT ballots_voter_election_unique UNIQUE (voter_id, election_id)
);

-- Indexes
CREATE INDEX idx_ballots_tenant_id ON ballots(tenant_id);
CREATE INDEX idx_ballots_election_id ON ballots(election_id);
CREATE INDEX idx_ballots_voter_id ON ballots(voter_id);
CREATE INDEX idx_ballots_ballot_hash ON ballots(ballot_hash);
CREATE INDEX idx_ballots_receipt_code ON ballots(voter_receipt_code);
CREATE INDEX idx_ballots_cast_at ON ballots(cast_at DESC);

-- Row-Level Security
ALTER TABLE ballots ENABLE ROW LEVEL SECURITY;

-- Policy: Ballots isolated by tenant
CREATE POLICY ballots_tenant_isolation ON ballots
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE ballots IS 'Encrypted ballots cast by voters (AES-256-GCM encrypted)';
COMMENT ON COLUMN ballots.encrypted_ballot IS 'AES-256-GCM encrypted ballot data (JSON with candidate selections)';
COMMENT ON COLUMN ballots.encryption_key_id IS 'Reference to encryption key used (for key rotation)';
COMMENT ON COLUMN ballots.ballot_hash IS 'SHA-256 hash of ballot for integrity verification';
COMMENT ON COLUMN ballots.voter_receipt_code IS 'Receipt code given to voter for verification';
COMMENT ON COLUMN ballots.verified_at IS 'When ballot was verified on blockchain';
