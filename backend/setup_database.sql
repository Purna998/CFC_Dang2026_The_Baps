-- Setup script for EEMP database
-- Run this as postgres superuser: psql -U postgres -f setup_database.sql

-- Create user
CREATE USER eemp WITH PASSWORD 'eemp_password';

-- Create database
CREATE DATABASE eemp_dev OWNER eemp;

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE eemp_dev TO eemp;

-- Connect to the database and grant schema privileges
\c eemp_dev

-- Grant privileges on public schema
GRANT ALL ON SCHEMA public TO eemp;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO eemp;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO eemp;

-- Set default privileges for future objects
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO eemp;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO eemp;

\echo 'Database setup complete. User: eemp, Database: eemp_dev'
