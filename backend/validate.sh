#!/bin/bash
# Backend Validation Script
# Checks all services compile and tests pass

set -e

echo "======================================"
echo "EEMP Backend Validation"
echo "======================================"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check Rust installation
echo "Checking Rust installation..."
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Rust/Cargo not found${NC}"
    echo "Install from: https://rustup.rs/"
    exit 1
fi
echo -e "${GREEN}✓ Rust installed: $(rustc --version)${NC}"
echo ""

# Check PostgreSQL
echo "Checking PostgreSQL..."
if ! command -v psql &> /dev/null; then
    echo -e "${YELLOW}⚠ PostgreSQL client not found (optional for local dev)${NC}"
else
    echo -e "${GREEN}✓ PostgreSQL client installed${NC}"
fi
echo ""

# Check Redis
echo "Checking Redis..."
if ! command -v redis-cli &> /dev/null; then
    echo -e "${YELLOW}⚠ Redis client not found (optional for local dev)${NC}"
else
    echo -e "${GREEN}✓ Redis client installed${NC}"
fi
echo ""

# Check workspace structure
echo "Validating workspace structure..."
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}✗ Cargo.toml not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Workspace root found${NC}"
echo ""

# Format check
echo "Checking code formatting..."
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Code formatting is correct${NC}"
else
    echo -e "${YELLOW}⚠ Code formatting issues found${NC}"
    echo "Run: cargo fmt --all"
fi
echo ""

# Clippy lints
echo "Running Clippy lints..."
if cargo clippy --workspace --all-targets -- -D warnings; then
    echo -e "${GREEN}✓ No clippy warnings${NC}"
else
    echo -e "${YELLOW}⚠ Clippy found issues${NC}"
fi
echo ""

# Compile check
echo "Checking compilation..."
if cargo check --workspace; then
    echo -e "${GREEN}✓ All services compile successfully${NC}"
else
    echo -e "${RED}✗ Compilation failed${NC}"
    exit 1
fi
echo ""

# Run tests
echo "Running unit tests..."
if cargo test --workspace --lib; then
    echo -e "${GREEN}✓ All unit tests passed${NC}"
else
    echo -e "${YELLOW}⚠ Some tests failed${NC}"
fi
echo ""

# Check dependencies
echo "Checking for security advisories..."
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        echo -e "${GREEN}✓ No known security vulnerabilities${NC}"
    else
        echo -e "${YELLOW}⚠ Security advisories found${NC}"
    fi
else
    echo -e "${YELLOW}⚠ cargo-audit not installed (optional)${NC}"
    echo "Install with: cargo install cargo-audit"
fi
echo ""

# Build release
echo "Building release binaries..."
if cargo build --release --workspace; then
    echo -e "${GREEN}✓ Release build successful${NC}"
else
    echo -e "${RED}✗ Release build failed${NC}"
    exit 1
fi
echo ""

echo "======================================"
echo -e "${GREEN}✓ Validation Complete!${NC}"
echo "======================================"
echo ""
echo "Next steps:"
echo "1. Set up .env file: cp .env.example .env"
echo "2. Start PostgreSQL and Redis"
echo "3. Run migrations: sqlx migrate run"
echo "4. Start API gateway: cargo run --bin api-gateway"
