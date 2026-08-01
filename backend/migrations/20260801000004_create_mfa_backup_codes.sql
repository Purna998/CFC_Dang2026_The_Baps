-- Migration: Create MFA backup codes table
-- Description: Backup codes for MFA recovery

CREATE TABLE IF NOT EXISTS mfa_backup_codes (
    id BIGSERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    code_hash TEXT NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_mfa_backup_codes_user_id ON mfa_backup_codes(user_id);
CREATE INDEX idx_mfa_backup_codes_code_hash ON mfa_backup_codes(code_hash);

-- Comments
COMMENT ON TABLE mfa_backup_codes IS 'MFA backup codes for account recovery';
COMMENT ON COLUMN mfa_backup_codes.code_hash IS 'SHA-256 hash of backup code';
COMMENT ON COLUMN mfa_backup_codes.used_at IS 'Timestamp when code was used (NULL if unused)';
