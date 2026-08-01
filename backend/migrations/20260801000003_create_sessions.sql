-- Migration: Create sessions table
-- Description: User session management for authentication

CREATE TABLE IF NOT EXISTS sessions (
    session_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES organizations(tenant_id) ON DELETE CASCADE,
    refresh_token_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    last_activity TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ip_address VARCHAR(45),
    user_agent TEXT,
    CONSTRAINT sessions_refresh_token_unique UNIQUE (refresh_token_hash)
);

-- Indexes
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_tenant_id ON sessions(tenant_id);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX idx_sessions_refresh_token_hash ON sessions(refresh_token_hash);

-- Row-Level Security
ALTER TABLE sessions ENABLE ROW LEVEL SECURITY;

-- Policy: Sessions isolated by tenant
CREATE POLICY sessions_tenant_isolation ON sessions
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

-- Comments
COMMENT ON TABLE sessions IS 'User sessions with refresh token tracking';
COMMENT ON COLUMN sessions.refresh_token_hash IS 'SHA-256 hash of refresh token (never store plaintext)';
COMMENT ON COLUMN sessions.expires_at IS 'Session expiration (typically 7 days)';
COMMENT ON COLUMN sessions.last_activity IS 'Last activity timestamp for session management';
