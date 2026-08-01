-- Migration: Create database functions and triggers
-- Description: Utility functions for business logic

-- Function: Update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers: Auto-update updated_at on all relevant tables
CREATE TRIGGER organizations_updated_at BEFORE UPDATE ON organizations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER elections_updated_at BEFORE UPDATE ON elections
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER positions_updated_at BEFORE UPDATE ON positions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER candidates_updated_at BEFORE UPDATE ON candidates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER eligibility_rules_updated_at BEFORE UPDATE ON eligibility_rules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at();

-- Function: Verify election dates
CREATE OR REPLACE FUNCTION verify_election_dates()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.voting_end_time <= NEW.voting_start_time THEN
        RAISE EXCEPTION 'voting_end_time must be after voting_start_time';
    END IF;

    IF NEW.result_publish_time IS NOT NULL AND NEW.result_publish_time < NEW.voting_end_time THEN
        RAISE EXCEPTION 'result_publish_time cannot be before voting_end_time';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger: Validate election dates
CREATE TRIGGER elections_validate_dates BEFORE INSERT OR UPDATE ON elections
    FOR EACH ROW EXECUTE FUNCTION verify_election_dates();

-- Function: Prevent ballot modification
CREATE OR REPLACE FUNCTION prevent_ballot_modification()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'Ballots are immutable and cannot be modified or deleted';
END;
$$ LANGUAGE plpgsql;

-- Trigger: Make ballots immutable
CREATE TRIGGER ballots_immutable BEFORE UPDATE OR DELETE ON ballots
    FOR EACH ROW EXECUTE FUNCTION prevent_ballot_modification();

-- Function: Prevent audit log modification
CREATE OR REPLACE FUNCTION prevent_audit_log_modification()
RETURNS TRIGGER AS $$
BEGIN
    RAISE EXCEPTION 'Audit logs are immutable and cannot be modified or deleted';
END;
$$ LANGUAGE plpgsql;

-- Trigger: Make audit logs immutable
CREATE TRIGGER audit_logs_immutable BEFORE UPDATE OR DELETE ON audit_logs
    FOR EACH ROW EXECUTE FUNCTION prevent_audit_log_modification();

-- Function: Get active elections count for organization
CREATE OR REPLACE FUNCTION get_active_elections_count(org_tenant_id UUID)
RETURNS INTEGER AS $$
BEGIN
    RETURN (
        SELECT COUNT(*)
        FROM elections
        WHERE tenant_id = org_tenant_id
        AND status IN ('Scheduled', 'Open')
    );
END;
$$ LANGUAGE plpgsql;

-- Comments
COMMENT ON FUNCTION update_updated_at() IS 'Automatically updates updated_at timestamp on row modification';
COMMENT ON FUNCTION verify_election_dates() IS 'Validates election date constraints';
COMMENT ON FUNCTION prevent_ballot_modification() IS 'Ensures ballots remain immutable after casting';
COMMENT ON FUNCTION prevent_audit_log_modification() IS 'Ensures audit logs remain immutable';
COMMENT ON FUNCTION get_active_elections_count(UUID) IS 'Returns count of active elections for organization';
