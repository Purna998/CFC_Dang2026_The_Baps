-- Migration: Create candidates table
-- Description: Candidates running for positions

CREATE TABLE IF NOT EXISTS candidates (
    candidate_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    election_id UUID NOT NULL REFERENCES elections(election_id) ON DELETE CASCADE,
    position_id UUID NOT NULL REFERENCES positions(position_id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(user_id) ON DELETE SET NULL,
    full_name VARCHAR(100) NOT NULL,
    bio TEXT,
    photo_url TEXT,
    manifesto_url TEXT,
    display_order INTEGER NOT NULL DEFAULT 0,
    is_approved BOOLEAN NOT NULL DEFAULT false,
    is_write_in BOOLEAN NOT NULL DEFAULT false,
    approved_by UUID REFERENCES users(user_id),
    approved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    withdrawn_at TIMESTAMPTZ
);

-- Indexes
CREATE INDEX idx_candidates_tenant_id ON candidates(tenant_id);
CREATE INDEX idx_candidates_election_id ON candidates(election_id);
CREATE INDEX idx_candidates_position_id ON candidates(position_id);
CREATE INDEX idx_candidates_user_id ON candidates(user_id);
CREATE INDEX idx_candidates_is_approved ON candidates(is_approved);
CREATE INDEX idx_candidates_display_order ON candidates(position_id, display_order);

-- Row-Level Security
ALTER TABLE candidates ENABLE ROW LEVEL SECURITY;

-- Policy: Candidates isolated by tenant
CREATE POLICY candidates_tenant_isolation ON candidates
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE candidates IS 'Candidates running for positions in elections';
COMMENT ON COLUMN candidates.user_id IS 'Reference to user account (NULL for external/write-in candidates)';
COMMENT ON COLUMN candidates.is_approved IS 'Whether candidate has been approved by election officials';
COMMENT ON COLUMN candidates.is_write_in IS 'Whether this is a write-in candidate';
COMMENT ON COLUMN candidates.withdrawn_at IS 'When candidate withdrew from election (NULL if active)';
