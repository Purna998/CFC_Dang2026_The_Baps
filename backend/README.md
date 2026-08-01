# EEMP Backend

Production-ready Rust backend for the Enterprise Election Management Platform.

## Quick Start

```bash
# 1. Install dependencies
make install

# 2. Setup environment and database
make setup

# 3. Start development server
make dev
```

The API will be available at `http://localhost:8000`

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Development](#development)
- [Testing](#testing)
- [Deployment](#deployment)
- [API Documentation](#api-documentation)
- [Project Structure](#project-structure)

## Architecture

This is a **modular monolith** with microservice-ready bounded contexts (DDD).

### Workspace Structure

```
backend/
├── services/          # Bounded context services (10 services)
│   ├── api-gateway/           # HTTP API gateway (Axum)
│   ├── organization-service/   # Multi-tenant foundation
│   ├── auth-service/          # Authentication & authorization
│   ├── election-service/      # Election management
│   ├── voting-service/        # Vote casting
│   ├── candidate-service/     # Candidate management
│   ├── result-service/        # Result calculation
│   ├── audit-service/         # Audit logging
│   ├── crypto-service/        # Cryptography operations
│   └── blockchain-service/    # Solana integration
│
├── shared/           # Shared libraries
│   ├── domain/      # Domain primitives (TenantId, UserId, Email, etc.)
│   ├── error/       # Error handling and API error responses
│   ├── config/      # Configuration management
│   ├── database/    # Database utilities (SQLx pool, RLS)
│   └── observability/ # Logging and tracing
│
└── migrations/       # SQLx database migrations
```

## Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- PostgreSQL 16+
- Redis 7+

### Setup

1. **Clone the repository**
   ```bash
   git clone <repo-url>
   cd e-voting-system/backend
   ```

2. **Copy environment configuration**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Start dependencies (Docker Compose)**
   ```bash
   cd ../infrastructure/docker
   docker-compose up -d postgres redis
   ```

4. **Run database migrations**
   ```bash
   cargo run --bin migrate
   ```

5. **Run the API server**
   ```bash
   cargo run --bin api-gateway
   ```

   The API will be available at `http://localhost:8000`

## Development

### Build all services
```bash
cargo build --workspace
```

### Run tests
```bash
cargo test --workspace
```

### Check code (fast compile check)
```bash
cargo check --workspace
```

### Format code
```bash
cargo fmt --all
```

### Lint code
```bash
cargo clippy --workspace -- -D warnings
```

## Shared Libraries

### eemp-domain
Domain primitives and value objects used across all services.

- **Value Objects:** `TenantId`, `UserId`, `ElectionId`, `Email`, `Password`, `PasswordHash`
- **Enums:** `UserRole`, `ElectionStatus`, `ElectionType`
- **Entities:** `User`, `Election`, `Candidate` (to be implemented)

### eemp-error
Centralized error handling with standardized API error responses.

- **AppError:** Application error types (authentication, authorization, validation, business logic)
- **ErrorResponse:** Standard JSON error response format
- **Axum Integration:** `IntoResponse` implementation for error handling

### eemp-config
Configuration management from environment variables.

- **Config:** Application configuration struct
- **Environment:** Development, Staging, Production
- **Twelve-Factor:** Environment-based configuration

### eemp-database
Database utilities for PostgreSQL with SQLx.

- **Database:** Connection pool wrapper
- **Multi-Tenancy:** Row-Level Security (RLS) helper
- **Migrations:** SQLx migration runner
- **Health Check:** Database connection verification

### eemp-observability
Structured logging with tracing and OpenTelemetry.

- **Tracing:** JSON-formatted structured logging
- **Environment-Aware:** Log levels per environment
- **OpenTelemetry:** Future integration with distributed tracing

## Multi-Tenancy

Every request must set the tenant context:

```rust
use eemp_database::Database;
use eemp_domain::TenantId;

// At the start of each request
db.set_tenant_context(tenant_id).await?;

// Now all queries automatically filtered by tenant_id via Row-Level Security
let users = sqlx::query_as!(User, "SELECT * FROM users")
    .fetch_all(db.pool())
    .await?;
```

PostgreSQL Row-Level Security (RLS) enforces tenant isolation at the database level.

## Security

- **Password Hashing:** Argon2id (19 MiB memory, 2 iterations) - OWASP recommended
- **JWT:** RS256 (RSA 2048-bit), 15-minute access tokens, 7-day refresh tokens
- **Multi-Tenancy:** Database-enforced via PostgreSQL RLS
- **Input Validation:** Validator crate for all inputs
- **Error Handling:** No sensitive data in error responses

## License

Proprietary - EEMP Team
