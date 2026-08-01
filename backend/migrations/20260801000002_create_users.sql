-- Migration: Create users table
-- Description: User accounts with multi-factor authentication

CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    password_hash TEXT NOT NULL,
    full_name VARCHAR(100) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'Voter',
    is_active BOOLEAN NOT NULL DEFAULT true,
    email_verified BOOLEAN NOT NULL DEFAULT false,
    email_verification_token TEXT,
    email_verification_expires_at TIMESTAMPTZ,
    password_reset_token TEXT,
    password_reset_expires_at TIMESTAMPTZ,
    mfa_enabled BOOLEAN NOT NULL DEFAULT false,
    mfa_secret TEXT,
    last_login_at TIMESTAMPTZ,
    last_login_ip VARCHAR(45),
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    locked_until TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    CONSTRAINT users_email_tenant_unique UNIQUE (email, tenant_id)
);

-- Indexes
CREATE INDEX idx_users_tenant_id ON users(tenant_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_tenant_email ON users(tenant_id, email);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_is_active ON users(is_active);
CREATE INDEX idx_users_created_at ON users(created_at DESC);
CREATE INDEX idx_users_deleted_at ON users(deleted_at) WHERE deleted_at IS NOT NULL;

-- Row-Level Security
ALTER TABLE users ENABLE ROW LEVEL SECURITY;

-- Policy: Users can only see users in their own organization
CREATE POLICY users_tenant_isolation ON users
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE users IS 'User accounts with authentication and MFA';
COMMENT ON COLUMN users.role IS 'RBAC role: OrganizationOwner, OrganizationAdmin, ElectionManager, ElectionOfficer, Voter, Candidate, Auditor, Observer';
COMMENT ON COLUMN users.mfa_enabled IS 'Whether MFA (TOTP) is enabled for this user';
COMMENT ON COLUMN users.mfa_secret IS 'TOTP secret for MFA (encrypted in production)';
COMMENT ON COLUMN users.failed_login_attempts IS 'Counter for failed login attempts (rate limiting)';
COMMENT ON COLUMN users.locked_until IS 'Account locked until this timestamp (after too many failed attempts)';
