-- Migration: Create eligibility rules table
-- Description: Rules defining who can vote in elections

CREATE TABLE IF NOT EXISTS eligibility_rules (
    rule_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    election_id UUID NOT NULL REFERENCES elections(election_id) ON DELETE CASCADE,
    rule_type VARCHAR(50) NOT NULL,
    rule_config JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_eligibility_rules_tenant_id ON eligibility_rules(tenant_id);
CREATE INDEX idx_eligibility_rules_election_id ON eligibility_rules(election_id);
CREATE INDEX idx_eligibility_rules_rule_type ON eligibility_rules(rule_type);

-- Row-Level Security
ALTER TABLE eligibility_rules ENABLE ROW LEVEL SECURITY;

-- Policy: Eligibility rules isolated by tenant
CREATE POLICY eligibility_rules_tenant_isolation ON eligibility_rules
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE eligibility_rules IS 'Rules defining voter eligibility for elections';
COMMENT ON COLUMN eligibility_rules.rule_type IS 'Type: AllUsers, RoleBasedAccess, DepartmentBased, CustomList, etc.';
COMMENT ON COLUMN eligibility_rules.rule_config IS 'JSON configuration for the rule (roles, departments, user IDs, etc.)';
