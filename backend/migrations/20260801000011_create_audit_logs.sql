-- Migration: Create audit logs table (partitioned by month)
-- Description: Immutable audit trail for all system actions

CREATE TABLE IF NOT EXISTS audit_logs (
    log_id BIGSERIAL,
    tenant_id UUID NOT NULL,
    user_id UUID,
    action VARCHAR(100) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID,
    changes JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (log_id, created_at)
) PARTITION BY RANGE (created_at);

-- Create partitions for the next 12 months
CREATE TABLE audit_logs_2026_08 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-08-01') TO ('2026-09-01');

CREATE TABLE audit_logs_2026_09 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-09-01') TO ('2026-10-01');

CREATE TABLE audit_logs_2026_10 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-10-01') TO ('2026-11-01');

CREATE TABLE audit_logs_2026_11 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-11-01') TO ('2026-12-01');

CREATE TABLE audit_logs_2026_12 PARTITION OF audit_logs
    FOR VALUES FROM ('2026-12-01') TO ('2027-01-01');

CREATE TABLE audit_logs_2027_01 PARTITION OF audit_logs
    FOR VALUES FROM ('2027-01-01') TO ('2027-02-01');

CREATE TABLE audit_logs_2027_02 PARTITION OF audit_logs
    FOR VALUES FROM ('2027-02-01') TO ('2027-03-01');

CREATE TABLE audit_logs_2027_03 PARTITION OF audit_logs
    FOR VALUES FROM ('2027-03-01') TO ('2027-04-01');

CREATE TABLE audit_logs_2027_04 PARTITION OF audit_logs
    FOR VALUES FROM ('2027-04-01') TO ('2027-05-01');

CREATE TABLE audit_logs_2027_05 PARTITION OF audit_logs
    FOR VALUES FROM ('2027-05-01') TO ('2027-06-01');

CREATE TABLE audit_logs_2027_06 PARTITION OF audit_logs
    FOR VALUES FROM ('2027-06-01') TO ('2027-07-01');

CREATE TABLE audit_logs_2027_07 PARTITION OF audit_logs
    FOR VALUES FROM ('2027-07-01') TO ('2027-08-01');

-- Indexes
CREATE INDEX idx_audit_logs_tenant_id ON audit_logs(tenant_id, created_at DESC);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id, created_at DESC);
CREATE INDEX idx_audit_logs_action ON audit_logs(action, created_at DESC);
CREATE INDEX idx_audit_logs_entity ON audit_logs(entity_type, entity_id);

-- Comments
COMMENT ON TABLE audit_logs IS 'Immutable audit trail partitioned by month';
COMMENT ON COLUMN audit_logs.action IS 'Action performed: CREATE, UPDATE, DELETE, LOGIN, LOGOUT, VOTE_CAST, etc.';
COMMENT ON COLUMN audit_logs.entity_type IS 'Type of entity: User, Election, Candidate, Ballot, etc.';
COMMENT ON COLUMN audit_logs.entity_id IS 'ID of the entity affected';
COMMENT ON COLUMN audit_logs.changes IS 'JSON representation of changes (before/after for updates)';
