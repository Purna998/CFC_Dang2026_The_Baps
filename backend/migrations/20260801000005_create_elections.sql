-- Migration: Create elections table
-- Description: Election management with state machine

CREATE TABLE IF NOT EXISTS elections (
    election_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    election_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'Draft',
    voting_start_time TIMESTAMPTZ NOT NULL,
    voting_end_time TIMESTAMPTZ NOT NULL,
    result_publish_time TIMESTAMPTZ,
    allow_write_in_candidates BOOLEAN NOT NULL DEFAULT false,
    allow_abstain BOOLEAN NOT NULL DEFAULT true,
    require_identity_verification BOOLEAN NOT NULL DEFAULT true,
    enable_blockchain_verification BOOLEAN NOT NULL DEFAULT true,
    max_votes_per_voter INTEGER,
    created_by UUID NOT NULL REFERENCES users(user_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    CONSTRAINT elections_voting_times_check CHECK (voting_end_time > voting_start_time)
);

-- Indexes
CREATE INDEX idx_elections_tenant_id ON elections(tenant_id);
CREATE INDEX idx_elections_status ON elections(status);
CREATE INDEX idx_elections_voting_times ON elections(voting_start_time, voting_end_time);
CREATE INDEX idx_elections_created_by ON elections(created_by);
CREATE INDEX idx_elections_created_at ON elections(created_at DESC);

-- Row-Level Security
ALTER TABLE elections ENABLE ROW LEVEL SECURITY;

-- Policy: Elections isolated by tenant
CREATE POLICY elections_tenant_isolation ON elections
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE elections IS 'Elections with state machine (Draft → Review → Scheduled → Open → Closed → Verifying → Published → Archived)';
COMMENT ON COLUMN elections.election_type IS 'Type: Individual, PostWise, Panel, RankedChoice';
COMMENT ON COLUMN elections.status IS 'Current state in election lifecycle';
COMMENT ON COLUMN elections.voting_start_time IS 'When voting opens';
COMMENT ON COLUMN elections.voting_end_time IS 'When voting closes';
COMMENT ON COLUMN elections.result_publish_time IS 'When results are published (can be delayed after voting ends)';
COMMENT ON COLUMN elections.enable_blockchain_verification IS 'Whether to store vote commitments on blockchain';
