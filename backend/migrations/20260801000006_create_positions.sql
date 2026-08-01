-- Migration: Create positions table
-- Description: Positions/seats to be filled in elections

CREATE TABLE IF NOT EXISTS positions (
    position_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    election_id UUID NOT NULL REFERENCES elections(election_id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    display_order INTEGER NOT NULL DEFAULT 0,
    seats_available INTEGER NOT NULL DEFAULT 1,
    min_votes_required INTEGER,
    max_votes_per_voter INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT positions_seats_positive CHECK (seats_available > 0)
);

-- Indexes
CREATE INDEX idx_positions_tenant_id ON positions(tenant_id);
CREATE INDEX idx_positions_election_id ON positions(election_id);
CREATE INDEX idx_positions_display_order ON positions(election_id, display_order);

-- Row-Level Security
ALTER TABLE positions ENABLE ROW LEVEL SECURITY;

-- Policy: Positions isolated by tenant
CREATE POLICY positions_tenant_isolation ON positions
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE positions IS 'Positions/seats to be filled in elections';
COMMENT ON COLUMN positions.seats_available IS 'Number of seats to be filled (e.g., 5 for board of directors)';
COMMENT ON COLUMN positions.display_order IS 'Order in which positions are displayed to voters';
COMMENT ON COLUMN positions.max_votes_per_voter IS 'Maximum votes a voter can cast for this position (NULL = same as seats_available)';
