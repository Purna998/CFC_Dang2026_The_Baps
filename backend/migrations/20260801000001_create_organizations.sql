-- Migration: Create organizations table
-- Description: Multi-tenant organization management

CREATE TABLE IF NOT EXISTS organizations (
    tenant_id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    organization_type VARCHAR(50) NOT NULL,
    domain VARCHAR(100) UNIQUE,
    subdomain VARCHAR(50) UNIQUE,
    logo_url TEXT,
    website TEXT,
    contact_email VARCHAR(255) NOT NULL,
    contact_phone VARCHAR(20),
    address VARCHAR(200),
    city VARCHAR(100),
    state VARCHAR(100),
    country VARCHAR(100) NOT NULL,
    postal_code VARCHAR(20),
    settings JSONB NOT NULL DEFAULT '{
        "allow_public_registration": false,
        "require_email_verification": true,
        "require_admin_approval": true,
        "max_elections_active": 5,
        "enable_anonymous_voting": true,
        "enable_voter_receipts": true,
        "enable_result_transparency": true,
        "timezone": "UTC",
        "locale": "en-US"
    }'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_organizations_domain ON organizations(domain) WHERE domain IS NOT NULL;
CREATE INDEX idx_organizations_subdomain ON organizations(subdomain) WHERE subdomain IS NOT NULL;
CREATE INDEX idx_organizations_is_active ON organizations(is_active);
CREATE INDEX idx_organizations_created_at ON organizations(created_at DESC);

-- Comments
COMMENT ON TABLE organizations IS 'Multi-tenant organizations (B2B customers)';
COMMENT ON COLUMN organizations.tenant_id IS 'Unique identifier for the organization (tenant)';
COMMENT ON COLUMN organizations.organization_type IS 'Type: University, College, School, Company, NGO, etc.';
COMMENT ON COLUMN organizations.domain IS 'Custom domain for organization (e.g., university.edu)';
COMMENT ON COLUMN organizations.subdomain IS 'Subdomain on platform (e.g., acme.eemp.com)';
COMMENT ON COLUMN organizations.settings IS 'JSON configuration for organization preferences';
