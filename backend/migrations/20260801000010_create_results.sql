-- Migration: Create results table
-- Description: Election results after vote counting

CREATE TABLE IF NOT EXISTS results (
    result_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    election_id UUID NOT NULL REFERENCES elections(election_id) ON DELETE CASCADE,
    position_id UUID NOT NULL REFERENCES positions(position_id) ON DELETE CASCADE,
    candidate_id UUID REFERENCES candidates(candidate_id) ON DELETE CASCADE,
    vote_count INTEGER NOT NULL DEFAULT 0,
    vote_percentage NUMERIC(5, 2),
    is_winner BOOLEAN NOT NULL DEFAULT false,
    rank INTEGER,
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    CONSTRAINT results_position_candidate_unique UNIQUE (position_id, candidate_id),
    CONSTRAINT results_vote_count_positive CHECK (vote_count >= 0),
    CONSTRAINT results_percentage_valid CHECK (vote_percentage >= 0 AND vote_percentage <= 100)
);

-- Indexes
CREATE INDEX idx_results_tenant_id ON results(tenant_id);
CREATE INDEX idx_results_election_id ON results(election_id);
CREATE INDEX idx_results_position_id ON results(position_id);
CREATE INDEX idx_results_candidate_id ON results(candidate_id);
CREATE INDEX idx_results_is_winner ON results(is_winner) WHERE is_winner = true;
CREATE INDEX idx_results_vote_count ON results(position_id, vote_count DESC);

-- Row-Level Security
ALTER TABLE results ENABLE ROW LEVEL SECURITY;

-- Policy: Results isolated by tenant
CREATE POLICY results_tenant_isolation ON results
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE results IS 'Election results after vote counting';
COMMENT ON COLUMN results.vote_count IS 'Number of votes received by candidate';
COMMENT ON COLUMN results.vote_percentage IS 'Percentage of total votes (0-100)';
COMMENT ON COLUMN results.is_winner IS 'Whether candidate won the position';
COMMENT ON COLUMN results.rank IS 'Rank in the election (1 = winner, 2 = runner-up, etc.)';
COMMENT ON COLUMN results.calculated_at IS 'When results were calculated';
COMMENT ON COLUMN results.published_at IS 'When results were published (NULL if not yet published)';
