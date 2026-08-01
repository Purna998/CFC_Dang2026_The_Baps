# Security Architecture
## Enterprise Election Management Platform (EEMP)

**Document Version:** 1.0  
**Last Updated:** 2026-08-01  
**Status:** Draft  
**Classification:** Confidential

---

## Document Control

| Field | Value |
|-------|-------|
| **Document Type** | Security Architecture |
| **Owner** | Chief Security Officer + Security Architect |
| **Reviewers** | CTO, Lead Engineers, External Security Auditor |
| **Approvers** | CSO, CTO, Compliance Officer |
| **Target Audience** | Security team, engineers, auditors, compliance officers |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Security Principles](#2-security-principles)
3. [Threat Model](#3-threat-model)
4. [Authentication Architecture](#4-authentication-architecture)
5. [Authorization Architecture](#5-authorization-architecture)
6. [Cryptographic Architecture](#6-cryptographic-architecture)
7. [Network Security](#7-network-security)
8. [Application Security](#8-application-security)
9. [Data Security](#9-data-security)
10. [Infrastructure Security](#10-infrastructure-security)
11. [Security Monitoring](#11-security-monitoring)
12. [Incident Response](#12-incident-response)

---

## 1. Introduction

### 1.1 Purpose

This document defines the security architecture for EEMP, covering all security controls, mechanisms, and procedures to protect the platform against threats and ensure data confidentiality, integrity, and availability.

**Security Criticality:** EEMP handles sensitive voting data and must maintain the highest security standards to preserve:
- **Vote Secrecy:** Voter identity must not be linkable to vote choices
- **Vote Integrity:** Votes must not be altered or deleted
- **System Availability:** Elections must proceed without disruption
- **Audit Trail:** All actions must be traceable

### 1.2 Security Requirements

**Compliance Targets:**
- SOC 2 Type II (within 12 months)
- ISO 27001 (within 18 months)
- GDPR compliance (data protection)
- CCPA compliance (California privacy)
- WCAG 2.1 AA (accessibility, no security barriers)

**Performance Targets:**
- Zero critical security incidents (production)
- Mean Time to Detect (MTTD): <5 minutes
- Mean Time to Respond (MTTR): <1 hour for critical
- Penetration test: Zero high/critical findings
- Vulnerability scan: Zero high/critical findings

### 1.3 Security Scope

**In Scope:**
- Authentication and authorization
- Cryptographic operations (encryption, signing, hashing)
- Network security (TLS, firewalls, DDoS protection)
- Application security (input validation, OWASP Top 10)
- Data security (encryption at rest, in transit)
- Infrastructure security (servers, containers, cloud)
- Security monitoring and logging
- Incident response procedures

**Out of Scope:**
- Physical security (datacenter security is cloud provider responsibility)
- User device security (user responsibility)
- Third-party service security (email, SMS providers)

---

## 2. Security Principles

### 2.1 Core Principles

#### Zero Trust Architecture
**Principle:** Never trust, always verify.

**Implementation:**
- Every request must be authenticated (JWT required)
- Every action must be authorized (permission check)
- Every operation must be audited (immutable log)
- No implicit trust based on network location
- Continuous verification (session expiry, token refresh)

**Example:**
```rust
// Every API endpoint
async fn create_election(
    State(state): State<AppState>,
    claims: Claims,  // JWT validation (authentication)
    Json(req): Json<CreateElectionRequest>
) -> Result<Json<Election>> {
    // Authorization check
    check_permission(&state, claims.user_id, "election:create").await?;
    
    // Business logic
    let election = state.election_service.create(req).await?;
    
    // Audit log
    state.audit_service.log(AuditEvent {
        action: "election:create",
        actor_id: claims.user_id,
        entity_id: election.id,
        result: "success",
    }).await?;
    
    Ok(Json(election))
}
```

---

#### Defense in Depth
**Principle:** Multiple layers of security controls.

**7 Layers:**
```
┌────────────────────────────────────────────────────────┐
│ Layer 7: Audit & Monitoring (immutable logs)          │
├────────────────────────────────────────────────────────┤
│ Layer 6: Data Security (encryption at rest, RLS)      │
├────────────────────────────────────────────────────────┤
│ Layer 5: Application Security (validation, CSRF)      │
├────────────────────────────────────────────────────────┤
│ Layer 4: Authorization (RBAC, tenant isolation)       │
├────────────────────────────────────────────────────────┤
│ Layer 3: Authentication (JWT, MFA)                    │
├────────────────────────────────────────────────────────┤
│ Layer 2: TLS/HTTPS (encrypted transport)              │
├────────────────────────────────────────────────────────┤
│ Layer 1: Network Security (firewall, DDoS protection) │
└────────────────────────────────────────────────────────┘
```

**Rationale:** If one layer is compromised, others provide protection.

---

#### Least Privilege
**Principle:** Users/services have only the minimum permissions required.

**Implementation:**
- Default role: "Voter" (minimal permissions)
- Explicit permission grants (no wildcard permissions)
- Role-Based Access Control (RBAC)
- Time-limited elevated access (temporary admin)
- Regular permission audits

---

#### Secure by Default
**Principle:** Secure configuration out-of-the-box.

**Implementation:**
- HTTPS only (HTTP redirects to HTTPS)
- Secure headers enabled (CSP, HSTS, X-Frame-Options)
- CSRF protection enabled
- Rate limiting enabled
- Audit logging enabled
- MFA available (enforceable per organization)
- Strong password policy enforced

---

#### Privacy by Design
**Principle:** Privacy built into the system, not bolted on.

**Implementation:**
- Vote anonymity (double-hashed voter IDs)
- Encrypted ballots (never store plaintext votes)
- Minimal data collection (only what's necessary)
- Data retention policies (GDPR right to erasure)
- Pseudonymization where possible
- Privacy Impact Assessment (PIA) conducted

---

### 2.2 Security Design Patterns

#### Fail Securely
**Principle:** Failures default to secure state (deny, not allow).

**Examples:**
- Authentication failure → deny access (not allow)
- Authorization check error → 403 Forbidden (not 200 OK)
- Encryption failure → reject vote (not store plaintext)
- Database connection loss → return error (not allow operation)

---

#### Complete Mediation
**Principle:** Every access checked, no bypasses.

**Implementation:**
- No "backdoor" admin access without audit
- No direct database access (all via application layer with checks)
- No cached authorization decisions (re-check on every request or short TTL)

---

#### Separation of Duties
**Principle:** No single person has complete control.

**Implementation:**
- Result decryption requires election authority key (held separately from database access)
- Multi-party verification for critical operations (future: multi-signature)
- Code review required for security-sensitive changes
- Separate roles: developer, admin, auditor

---

## 3. Threat Model

### 3.1 Threat Actors

#### External Attackers
**Motivation:** Disrupt elections, manipulate results, steal data

**Capabilities:**
- Network attacks (DDoS, man-in-the-middle)
- Application attacks (injection, XSS, CSRF)
- Social engineering (phishing)
- Brute force attacks

**Likelihood:** High  
**Impact:** Critical

---

#### Malicious Insiders
**Motivation:** Manipulate results, steal data, sabotage

**Capabilities:**
- Access to internal systems
- Knowledge of system architecture
- Potential access to credentials

**Likelihood:** Low  
**Impact:** Critical

---

#### Nation-State Actors (Future, B2G)
**Motivation:** Influence election outcomes

**Capabilities:**
- Advanced persistent threats (APT)
- Zero-day exploits
- Supply chain attacks

**Likelihood:** Low (B2B), Medium (B2G)  
**Impact:** Critical

---

#### Opportunistic Attackers
**Motivation:** Financial gain, reputation

**Capabilities:**
- Automated scanning
- Known vulnerabilities
- Credential stuffing

**Likelihood:** High  
**Impact:** Medium

---

### 3.2 STRIDE Threat Analysis

| Threat | Example | Mitigation |
|--------|---------|------------|
| **Spoofing** | Attacker impersonates legitimate user | Strong authentication (Argon2id + JWT + MFA), rate limiting on login |
| **Tampering** | Attacker modifies votes or results | Blockchain immutability, encrypted ballots, audit logs, digital signatures |
| **Repudiation** | User denies casting vote | Immutable audit logs, blockchain verification code, digital signatures |
| **Information Disclosure** | Attacker accesses plaintext votes | End-to-end encryption, vote anonymization, TLS, access controls |
| **Denial of Service** | Attacker disrupts election | DDoS protection (Cloudflare), rate limiting, auto-scaling, failover |
| **Elevation of Privilege** | Attacker gains admin access | RBAC enforcement, least privilege, audit logging, session management |

---

### 3.3 Attack Vectors

#### Vote Manipulation
**Attack:** Modify or delete votes to change results

**Mitigations:**
- Votes encrypted (attacker cannot read/modify without key)
- Blockchain commitment (immutable proof of vote)
- Result verification (recount matches blockchain commitments)
- Audit logs (detect unauthorized access attempts)

**Residual Risk:** Low (multiple controls)

---

#### Voter Coercion / Vote Buying
**Attack:** Attacker forces voter to vote specific way or buys votes

**Mitigations:**
- Receipt-freeness (voter cannot prove how they voted) - **Future enhancement**
- Anonymous voting (managers cannot see individual votes)
- Secure voting environment (ideally private)
- Observer monitoring (detect suspicious patterns)

**Residual Risk:** Medium (difficult to prevent fully)

---

#### DDoS During Election
**Attack:** Overwhelm system during active voting

**Mitigations:**
- CDN with DDoS protection (Cloudflare, AWS Shield)
- Rate limiting per IP and per user
- Auto-scaling (handle traffic spikes)
- Queued operations (buffer blockchain submissions)
- Graceful degradation (read-only mode if write failures)

**Residual Risk:** Low (multiple layers)

---

#### Credential Theft
**Attack:** Steal user credentials via phishing or data breach

**Mitigations:**
- Argon2id password hashing (cannot reverse engineer)
- MFA enforcement (credential theft insufficient)
- Rate limiting on login attempts
- Account lockout after failed attempts
- Email notifications on suspicious activity
- Password breach monitoring (check against leaked password databases)

**Residual Risk:** Medium (phishing is user responsibility)

---

#### SQL Injection
**Attack:** Inject malicious SQL to access/modify database

**Mitigations:**
- Prepared statements (SQLx enforces, no string concatenation)
- Input validation (reject malformed inputs)
- Principle of least privilege (database user has minimal permissions)
- Web Application Firewall (WAF) - future

**Residual Risk:** Very Low (Rust + SQLx prevents)

---

#### Cross-Site Scripting (XSS)
**Attack:** Inject JavaScript to steal session tokens or perform actions

**Mitigations:**
- Content Security Policy (CSP) headers
- Output encoding (escape all user input in HTML)
- HttpOnly cookies (session tokens not accessible to JavaScript)
- Sanitize user inputs (strip HTML tags where not needed)

**Residual Risk:** Low (multiple controls)

---

#### Cross-Site Request Forgery (CSRF)
**Attack:** Trick user into performing unwanted action

**Mitigations:**
- CSRF tokens (synchronizer token pattern)
- SameSite cookies (cookies not sent on cross-site requests)
- Origin header validation
- Re-authentication for sensitive actions (delete account, change password)

**Residual Risk:** Very Low (multiple controls)

---

## 4. Authentication Architecture

### 4.1 Password Management

#### Password Hashing: Argon2id

**Algorithm:** Argon2id (OWASP recommended, resistant to GPU cracking)

**Parameters:**
```rust
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, Params, Version
};

// Parameters (OWASP recommended for interactive logins)
let params = Params::new(
    19456,  // memory cost (19 MiB)
    2,      // time cost (iterations)
    1,      // parallelism
    None    // output length (default 32 bytes)
)?;

let argon2 = Argon2::new(
    argon2::Algorithm::Argon2id,
    Version::V0x13,
    params
);

// Hash password
let salt = SaltString::generate(&mut OsRng);
let password_hash = argon2
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

// Store password_hash in database
// Example output: $argon2id$v=19$m=19456,t=2,p=1$salt$hash
```

**Verification:**
```rust
use argon2::PasswordVerifier;

let parsed_hash = PasswordHash::new(&db_password_hash)?;
let is_valid = Argon2::default()
    .verify_password(input_password.as_bytes(), &parsed_hash)
    .is_ok();
```

**Storage:** Password hash stored in `users.password_hash` (VARCHAR 255)

**Security Properties:**
- Memory-hard (resistant to ASIC/GPU attacks)
- Configurable difficulty (adjust as hardware improves)
- Salted (unique salt per password, stored in hash string)
- Timing attack resistant

---

#### Password Policy

**Requirements:**
- Minimum 8 characters
- At least 1 uppercase letter
- At least 1 lowercase letter
- At least 1 number
- At least 1 special character
- Not in common password list (e.g., "Password123!")
- Not previously used (store hash of last 5 passwords, prevent reuse)

**Validation:**
```rust
fn validate_password(password: &str) -> Result<(), PasswordError> {
    if password.len() < 8 {
        return Err(PasswordError::TooShort);
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(PasswordError::NoUppercase);
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(PasswordError::NoLowercase);
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(PasswordError::NoNumber);
    }
    if !password.chars().any(|c| !c.is_alphanumeric()) {
        return Err(PasswordError::NoSpecialChar);
    }
    if is_common_password(password) {
        return Err(PasswordError::TooCommon);
    }
    Ok(())
}
```

---

### 4.2 JWT Token Architecture

#### Access Token (Short-Lived)

**Purpose:** Authenticate API requests

**Lifetime:** 15 minutes

**Algorithm:** RS256 (RSA + SHA-256)

**Structure:**
```json
{
  "header": {
    "alg": "RS256",
    "typ": "JWT",
    "kid": "key-2026-08"
  },
  "payload": {
    "sub": "550e8400-e29b-41d4-a716-446655440000",  // user_id
    "email": "user@example.com",
    "tenant_id": "123e4567-e89b-12d3-a456-426614174000",
    "roles": ["voter", "election_manager"],
    "permissions": ["election:read", "election:create", "vote:cast"],
    "iat": 1722528000,  // issued at (Unix timestamp)
    "exp": 1722528900,  // expiration (15 min later)
    "jti": "abc123"     // JWT ID (for revocation)
  },
  "signature": "..."  // RSA signature
}
```

**Key Management:**
- Private key: RSA 2048-bit (stored in HSM or AWS KMS in production)
- Public key: Distributed to API servers (for verification)
- Key rotation: Every 90 days (automated)
- Multiple active keys: Support key rotation without downtime (identify via `kid`)

**Verification:**
```rust
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

let validation = Validation::new(Algorithm::RS256);
let token_data = decode::<Claims>(
    &token,
    &DecodingKey::from_rsa_pem(public_key)?,
    &validation
)?;

let claims = token_data.claims;
// Check expiration, tenant_id, permissions
```

**Storage:** Not stored (stateless, verified by signature)

---

#### Refresh Token (Long-Lived)

**Purpose:** Obtain new access tokens without re-authenticating

**Lifetime:** 7 days

**Format:** Opaque UUID v4 (not JWT)

**Storage:** Database (`refresh_tokens` table)

**Structure:**
```sql
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    token_hash VARCHAR(255) NOT NULL UNIQUE,  -- SHA-256 hash
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    device_name VARCHAR(255),
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

**Issuance:**
```rust
use uuid::Uuid;
use sha2::{Sha256, Digest};

// Generate random token
let token = Uuid::new_v4().to_string();

// Hash token for storage (never store plaintext)
let mut hasher = Sha256::new();
hasher.update(token.as_bytes());
let token_hash = format!("{:x}", hasher.finalize());

// Store in database
sqlx::query!(
    "INSERT INTO refresh_tokens (user_id, token_hash, expires_at, device_name, ip_address)
     VALUES ($1, $2, NOW() + INTERVAL '7 days', $3, $4)",
    user_id, token_hash, device_name, ip_address
).execute(&pool).await?;

// Return plaintext token to user (only time it's visible)
return token;
```

**Refresh Flow:**
```
1. User sends refresh token
2. Hash token and look up in database
3. Validate: not expired, not revoked, user still active
4. Issue new access token (15 min)
5. Rotate refresh token (invalidate old, issue new) - optional but recommended
6. Update last_used_at
```

**Rotation Strategy:**
- **Automatic rotation:** Issue new refresh token on each use (invalidate old)
- **Refresh token reuse detection:** If revoked token used, revoke all user tokens (session hijacking attempt)

**Revocation:**
- User logout: Revoke refresh token
- Password change: Revoke all user refresh tokens
- Admin action: Revoke specific or all tokens
- Suspicious activity: Automatic revocation

---

### 4.3 Multi-Factor Authentication (MFA)

#### TOTP (Time-Based One-Time Password)

**Algorithm:** RFC 6238 (TOTP)

**Implementation:**
```rust
use totp_rs::{TOTP, Algorithm, Secret};

// 1. Enrollment: Generate secret
let secret = Secret::generate_secret();
let totp = TOTP::new(
    Algorithm::SHA1,      // Google Authenticator compatible
    6,                    // 6-digit code
    1,                    // 1 step (30-second window)
    30,                   // 30-second window
    secret.to_bytes().unwrap(),
    Some("EEMP".to_string()),
    user_email.to_string()
)?;

// 2. Generate QR code for user
let qr_code_url = totp.get_qr_base64()?;  // Display QR code

// 3. User scans QR code with authenticator app (Google Authenticator, Authy, etc.)

// 4. User enters verification code to confirm enrollment
let is_valid = totp.check_current(&user_input_code)?;
if is_valid {
    // Store encrypted secret in database
    let encrypted_secret = encrypt(&secret, &master_key)?;
    sqlx::query!(
        "UPDATE users SET mfa_enabled = TRUE, mfa_secret = $1 WHERE id = $2",
        encrypted_secret, user_id
    ).execute(&pool).await?;
}
```

**Verification:**
```rust
// Login flow with MFA
// 1. Verify password (normal login)
// 2. Check if user has MFA enabled
if user.mfa_enabled {
    // 3. Prompt for TOTP code
    let stored_secret = decrypt(&user.mfa_secret, &master_key)?;
    let totp = TOTP::from_secret(stored_secret)?;
    
    // 4. Verify code (with ±1 window for clock skew)
    let is_valid = totp.check_current(&user_input_code)?
        || totp.check(&user_input_code, totp.get_time() - 30)?
        || totp.check(&user_input_code, totp.get_time() + 30)?;
    
    if !is_valid {
        return Err(AuthError::InvalidMfaCode);
    }
}
// 5. Issue JWT tokens
```

**Security:**
- Secret stored encrypted (not plaintext)
- ±1 time window tolerance (60 seconds total window to account for clock skew)
- Rate limiting: 5 attempts per 5 minutes
- Backup codes available (in case of lost device)

---

#### Backup Codes

**Purpose:** Allow MFA bypass if device lost

**Implementation:**
```rust
use rand::Rng;

// Generate 10 backup codes (8 characters each)
fn generate_backup_codes() -> Vec<String> {
    let mut rng = rand::thread_rng();
    (0..10)
        .map(|_| {
            (0..8)
                .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
                .collect::<String>()
                .to_uppercase()
        })
        .collect()
}

// Example codes: ["A1B2C3D4", "E5F6G7H8", ...]

// Store hashed codes (NOT plaintext)
let hashed_codes: Vec<String> = codes.iter()
    .map(|code| {
        let mut hasher = Sha256::new();
        hasher.update(code.as_bytes());
        format!("{:x}", hasher.finalize())
    })
    .collect();

// Store in database
sqlx::query!(
    "UPDATE users SET mfa_backup_codes = $1 WHERE id = $2",
    &hashed_codes[..],  // PostgreSQL TEXT[] array
    user_id
).execute(&pool).await?;
```

**Usage:**
```rust
// User enters backup code instead of TOTP
let user_input_code_hash = hash_sha256(&user_input_code);

// Check if code exists in user's backup codes
if user.mfa_backup_codes.contains(&user_input_code_hash) {
    // Valid backup code
    // Remove used code (one-time use)
    let remaining_codes: Vec<String> = user.mfa_backup_codes
        .into_iter()
        .filter(|c| c != &user_input_code_hash)
        .collect();
    
    sqlx::query!(
        "UPDATE users SET mfa_backup_codes = $1 WHERE id = $2",
        &remaining_codes[..], user_id
    ).execute(&pool).await?;
    
    // Warn if only 2 codes remaining
    if remaining_codes.len() <= 2 {
        send_email("Only 2 backup codes remaining, generate new codes", user.email);
    }
    
    // Allow login
    return Ok(());
}
```

---

### 4.4 Session Management

#### Session Lifecycle

**Creation:**
```rust
// After successful authentication
let session_id = Uuid::new_v4();
let access_token_jti = Uuid::new_v4().to_string();

// Store session in PostgreSQL
sqlx::query!(
    "INSERT INTO sessions (id, user_id, token_jti, ip_address, user_agent, expires_at)
     VALUES ($1, $2, $3, $4, $5, NOW() + INTERVAL '15 minutes')",
    session_id, user_id, access_token_jti, ip_address, user_agent
).execute(&pool).await?;

// Cache session in Redis (fast lookup)
redis::cmd("SETEX")
    .arg(format!("session:{}", access_token_jti))
    .arg(900)  // 15 minutes
    .arg(serde_json::to_string(&SessionData {
        user_id,
        tenant_id,
        permissions: user.permissions,
    })?)
    .query_async(&mut redis_conn)
    .await?;
```

**Validation:**
```rust
// On every API request
async fn validate_session(token_jti: &str) -> Result<SessionData> {
    // 1. Check Redis cache (fast path)
    if let Some(session_data) = redis::cmd("GET")
        .arg(format!("session:{}", token_jti))
        .query_async(&mut redis_conn)
        .await?
    {
        return Ok(serde_json::from_str(&session_data)?);
    }
    
    // 2. Check PostgreSQL (cache miss or revoked)
    let session = sqlx::query_as!(
        Session,
        "SELECT * FROM sessions WHERE token_jti = $1 AND expires_at > NOW()",
        token_jti
    )
    .fetch_optional(&pool)
    .await?;
    
    if let Some(session) = session {
        // Update cache
        redis::cmd("SETEX")
            .arg(format!("session:{}", token_jti))
            .arg(900)
            .arg(...)
            .query_async(&mut redis_conn)
            .await?;
        
        return Ok(session_data);
    }
    
    Err(AuthError::InvalidSession)
}
```

**Expiration:**
- Access token expiry: 15 minutes (enforced by JWT `exp` claim)
- Redis TTL: 15 minutes (auto-expire)
- PostgreSQL cleanup: Daily cron job (delete expired sessions)

**Revocation:**
```rust
// Logout: Revoke single session
async fn logout(token_jti: &str) -> Result<()> {
    // Remove from Redis
    redis::cmd("DEL")
        .arg(format!("session:{}", token_jti))
        .query_async(&mut redis_conn)
        .await?;
    
    // Delete from PostgreSQL
    sqlx::query!("DELETE FROM sessions WHERE token_jti = $1", token_jti)
        .execute(&pool)
        .await?;
    
    Ok(())
}

// Logout all sessions (e.g., password change)
async fn logout_all_sessions(user_id: Uuid) -> Result<()> {
    // Get all user sessions
    let sessions = sqlx::query!(
        "SELECT token_jti FROM sessions WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await?;
    
    // Remove from Redis
    for session in &sessions {
        redis::cmd("DEL")
            .arg(format!("session:{}", session.token_jti))
            .query_async(&mut redis_conn)
            .await?;
    }
    
    // Delete from PostgreSQL
    sqlx::query!("DELETE FROM sessions WHERE user_id = $1", user_id)
        .execute(&pool)
        .await?;
    
    Ok(())
}
```

---

### 4.5 Rate Limiting

**Purpose:** Prevent brute force attacks, credential stuffing, DDoS

**Implementation:** Token bucket algorithm (Redis)

**Limits:**

| Endpoint/Action | Limit | Window | Consequence |
|----------------|-------|--------|-------------|
| Login attempts | 5 attempts | 15 minutes | Account locked for 15 min |
| Failed login (global IP) | 10 attempts | 15 minutes | IP blocked temporarily |
| Password reset | 3 requests | 1 hour | Block further requests |
| API requests (authenticated) | 1000 requests | 15 minutes | HTTP 429 response |
| API requests (unauthenticated) | 100 requests | 15 minutes | HTTP 429 response |
| MFA code attempts | 5 attempts | 5 minutes | Temporary lockout |
| Vote casting | 1 vote | Per election | Already voted error |

**Code Example (Middleware):**
```rust
use redis::AsyncCommands;

async fn rate_limit_middleware(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<Response> {
    let key = format!("ratelimit:{}:{}", endpoint, identifier);
    
    // Increment counter
    let count: i64 = redis_conn.incr(&key, 1).await?;
    
    // Set expiry on first request
    if count == 1 {
        redis_conn.expire(&key, window_seconds).await?;
    }
    
    // Check limit
    if count > limit {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({"error": "Rate limit exceeded"}))
        ));
    }
    
    // Add rate limit headers
    let mut response = next.run(req).await?;
    response.headers_mut().insert("X-RateLimit-Limit", limit.into());
    response.headers_mut().insert("X-RateLimit-Remaining", (limit - count).into());
    response.headers_mut().insert("X-RateLimit-Reset", reset_time.into());
    
    Ok(response)
}
```

---

## 5. Authorization Architecture

### 5.1 Role-Based Access Control (RBAC)

#### Permission Model

**Permission Format:** `resource:action`

**Examples:**
```
organization:read
organization:write
organization:delete
user:read
user:write
user:delete
election:read
election:create
election:update
election:delete
election:publish
candidate:read
candidate:create
candidate:verify
vote:cast
result:read
result:publish
audit:read
audit:export
settings:read
settings:write
```

**Total:** ~40 permissions

---

#### Default Roles

| Role | Permissions | Description |
|------|-------------|-------------|
| **Organization Owner** | All permissions | Full control over organization |
| **Organization Admin** | Most permissions (except org delete) | Manage users, elections, settings |
| **Election Manager** | election:*, candidate:*, result:publish | Create and manage elections |
| **Election Officer** | election:read, candidate:verify, voter:verify | Support election operations |
| **Voter** | vote:cast, election:read, result:read | Cast votes and view results |
| **Candidate** | candidate:read, candidate:update (own) | Manage candidate profile |
| **Auditor** | audit:read, audit:export, election:read | Read-only access for auditing |
| **Observer** | election:read, result:read (real-time) | Monitor elections in progress |

---

#### Permission Check

**API Middleware:**
```rust
use axum::extract::State;

#[derive(Debug)]
struct PermissionRequired(&'static str);

async fn check_permission(
    State(state): State<AppState>,
    claims: Claims,
    TypedHeader(PermissionRequired(required_perm)): TypedHeader<PermissionRequired>,
    req: Request<Body>,
    next: Next<Body>,
) -> Result<Response> {
    // Get user permissions (cached in Redis, 5-minute TTL)
    let user_permissions = get_user_permissions(&state, claims.user_id).await?;
    
    // Check if user has required permission
    if !user_permissions.contains(required_perm) {
        // Audit failed authorization
        state.audit_service.log(AuditEvent {
            action: "authorization:denied",
            actor_id: claims.user_id,
            details: json!({"required": required_perm, "endpoint": req.uri()}),
            result: "failure",
        }).await?;
        
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Insufficient permissions"}))
        ));
    }
    
    Ok(next.run(req).await?)
}
```

**Usage:**
```rust
// Endpoint with permission requirement
#[axum::debug_handler]
async fn create_election(
    State(state): State<AppState>,
    claims: Claims,
    TypedHeader(PermissionRequired("election:create")): TypedHeader<PermissionRequired>,
    Json(req): Json<CreateElectionRequest>
) -> Result<Json<Election>> {
    // Permission already checked by middleware
    let election = state.election_service.create(claims.tenant_id, req).await?;
    Ok(Json(election))
}
```

---

### 5.2 Attribute-Based Access Control (ABAC)

**Future Enhancement:** Fine-grained access control based on user attributes and resource attributes.

**Example Use Cases:**
- Voter can only access elections they're eligible for
- Candidate can only update their own candidate profile
- Election manager can only manage elections they created

**Policy Example:**
```rust
struct AbacPolicy {
    resource: String,
    action: String,
    condition: Box<dyn Fn(&User, &Resource) -> bool>,
}

// Example: Candidate can update own profile only
let policy = AbacPolicy {
    resource: "candidate".to_string(),
    action: "update".to_string(),
    condition: Box::new(|user, resource| {
        if let Resource::Candidate(candidate) = resource {
            candidate.user_id == Some(user.id)
        } else {
            false
        }
    }),
};
```

---

### 5.3 Multi-Tenancy Isolation

**Row-Level Security (PostgreSQL):**

```sql
-- Enable RLS on all tenant-scoped tables
ALTER TABLE elections ENABLE ROW LEVEL SECURITY;

-- Create tenant isolation policy
CREATE POLICY tenant_isolation ON elections
    USING (tenant_id = current_setting('app.current_tenant_id')::UUID);

-- Similar for all tenant-scoped tables
```

**Application Enforcement:**
```rust
// Set tenant context at start of every request
async fn set_tenant_context(
    State(state): State<AppState>,
    claims: Claims,
    req: Request<Body>,
    next: Next<Body>,
) -> Result<Response> {
    // Set PostgreSQL session variable
    sqlx::query("SET LOCAL app.current_tenant_id = $1")
        .bind(claims.tenant_id)
        .execute(&state.pool)
        .await?;
    
    // Continue with request
    Ok(next.run(req).await?)
}
```

**Security Properties:**
- Database-enforced (not application-only)
- Cannot be bypassed by SQL injection
- Automatic filtering (all queries)
- Audit: Any access to another tenant's data triggers RLS violation (logged)

---

## 6. Cryptographic Architecture

### 6.1 Cryptographic Primitives

**Library:** `libsodium` (via `sodiumoxide` Rust bindings)

**Rationale:**
- Audited and battle-tested
- High-level API (hard to misuse)
- Constant-time operations (timing attack resistant)
- Comprehensive suite of algorithms

---

### 6.2 Password Hashing

**Algorithm:** Argon2id

**See Section 4.1** for details.

---

### 6.3 Symmetric Encryption

**Algorithm:** AES-256-GCM (Galois/Counter Mode)

**Use Cases:**
- Ballot encryption (with X25519 key exchange)
- Database field encryption (MFA secrets, private keys)

**Key Derivation:**
```rust
use sodiumoxide::crypto::secretbox;

// Derive encryption key from master key
let master_key = get_master_key_from_kms()?;  // 32 bytes
let salt = b"EEMP-Ballot-Encryption-2026";  // Fixed salt for ballots
let encryption_key = derive_key(&master_key, salt)?;

// Encrypt data
let nonce = secretbox::gen_nonce();  // 24-byte nonce
let ciphertext = secretbox::seal(plaintext, &nonce, &secretbox::Key::from_slice(&encryption_key)?);

// Store nonce + ciphertext
let encrypted_blob = [nonce.as_ref(), &ciphertext].concat();
```

**Properties:**
- Authenticated encryption (detects tampering)
- 256-bit key (quantum-resistant for foreseeable future)
- Unique nonce per encryption (never reuse nonce with same key)

---

### 6.4 Asymmetric Encryption

**Algorithm:** X25519 (Elliptic Curve Diffie-Hellman)

**Use Case:** Ballot encryption (ephemeral sender key + election public key)

**Election Key Generation:**
```rust
use sodiumoxide::crypto::box_;

// Generate election key pair (per election)
let (election_public_key, election_private_key) = box_::gen_keypair();

// Store public key in elections table (plaintext)
// Store private key encrypted with master key (for result decryption)
let encrypted_private_key = aes_encrypt(&election_private_key.as_ref(), &master_key)?;

sqlx::query!(
    "UPDATE elections 
     SET election_public_key = $1, 
         election_private_key_encrypted = $2
     WHERE id = $3",
    base64::encode(election_public_key.as_ref()),
    encrypted_private_key,
    election_id
).execute(&pool).await?;
```

**Ballot Encryption (Voter Side):**
```rust
// Generate ephemeral key pair (used once)
let (ephemeral_public_key, ephemeral_private_key) = box_::gen_keypair();

// Encrypt ballot with election public key
let nonce = box_::gen_nonce();
let ballot_json = serde_json::to_string(&ballot)?;
let ciphertext = box_::seal(
    ballot_json.as_bytes(),
    &nonce,
    &election_public_key,
    &ephemeral_private_key
);

// Store ephemeral public key + nonce + ciphertext
let encrypted_ballot = EncryptedBallot {
    ephemeral_public_key: base64::encode(ephemeral_public_key.as_ref()),
    nonce: base64::encode(nonce.as_ref()),
    ciphertext: base64::encode(&ciphertext),
};
```

**Ballot Decryption (Result Calculation):**
```rust
// Retrieve and decrypt election private key
let encrypted_private_key = election.election_private_key_encrypted;
let election_private_key_bytes = aes_decrypt(&encrypted_private_key, &master_key)?;
let election_private_key = box_::SecretKey::from_slice(&election_private_key_bytes)?;

// Decrypt ballot
let ephemeral_public_key = box_::PublicKey::from_slice(&base64::decode(&ballot.ephemeral_public_key)?)?;
let nonce = box_::Nonce::from_slice(&base64::decode(&ballot.nonce)?)?;
let ciphertext = base64::decode(&ballot.ciphertext)?;

let plaintext = box_::open(
    &ciphertext,
    &nonce,
    &ephemeral_public_key,
    &election_private_key
)?;

let ballot: Ballot = serde_json::from_slice(&plaintext)?;
```

---

### 6.5 Digital Signatures

**Algorithm:** Ed25519 (EdDSA on Curve25519)

**Use Cases:**
- Blockchain vote commitments (election authority signature)
- Result reports (election authority signature)
- API requests (future: request signing)

**Signature Generation:**
```rust
use sodiumoxide::crypto::sign;

// Election authority key pair (per election)
let (public_key, private_key) = sign::gen_keypair();

// Sign vote commitment
let commitment_data = serde_json::to_vec(&VoteCommitment {
    election_id,
    commitment_hash,
    timestamp: Utc::now(),
})?;

let signature = sign::sign_detached(&commitment_data, &private_key);

// Store signature with commitment
let vote_commitment = VoteCommitmentRecord {
    commitment_hash,
    signature: base64::encode(signature.as_ref()),
    ...
};
```

**Signature Verification:**
```rust
let public_key = sign::PublicKey::from_slice(&base64::decode(&election.authority_public_key)?)?;
let signature = sign::Signature::from_slice(&base64::decode(&commitment.signature)?)?;

let is_valid = sign::verify_detached(
    &signature,
    &commitment_data,
    &public_key
);

if !is_valid {
    return Err(CryptoError::InvalidSignature);
}
```

---

### 6.6 Hashing

**Algorithm:** SHA-256 (general purpose), SHA-3 (future-proof)

**Use Cases:**
- Vote commitments (hash of encrypted ballot)
- Blockchain transaction integrity
- Voter anonymization (double-hashed voter IDs)
- Backup code hashing

**SHA-256:**
```rust
use sha2::{Sha256, Digest};

let mut hasher = Sha256::new();
hasher.update(data);
let hash = hasher.finalize();
let hash_hex = format!("{:x}", hash);  // 64-character hex string
```

**Double Hashing (Voter Anonymization):**
```rust
// Prevent linking voter to ballot while preventing double voting
let voter_hash = hash_sha256(&format!("{}{}{}", 
    hash_sha256(&user_id.to_string()),
    election_id.to_string(),
    hash_sha256(&tenant_salt)
));

// Store in ballots table (cannot reverse engineer user_id)
```

---

### 6.7 Key Management

#### Master Key

**Purpose:** Encrypt election private keys, MFA secrets, sensitive database fields

**Storage:**
- **Development:** Environment variable or local file
- **Staging:** AWS Secrets Manager or similar
- **Production:** AWS KMS or Hardware Security Module (HSM)

**Key Rotation:**
- **Frequency:** Annually (or after suspected compromise)
- **Process:**
  1. Generate new master key
  2. Decrypt all data with old key
  3. Re-encrypt with new key
  4. Update key reference
  5. Retire old key after grace period

**Access Control:**
- Only result calculation service can access
- API key required for KMS/HSM access
- All access logged (audit trail)

---

#### Election Keys

**Purpose:** Ballot encryption/decryption per election

**Generation:** Per election (X25519 key pair + Ed25519 signing key)

**Lifecycle:**
- **Created:** When election created (draft state)
- **Used:** Throughout election (voting and result calculation)
- **Archived:** After results published (private key retained for audits)
- **Destroyed:** After retention period (7 years default, compliance requirement)

**Storage:**
- Public keys: Plaintext in `elections` table
- Private keys: Encrypted with master key in `elections` table
- Backup: Encrypted backup in cold storage (S3 Glacier)

---

#### JWT Signing Keys

**Purpose:** Sign and verify JWT access tokens

**Algorithm:** RSA 2048-bit

**Generation:**
```rust
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};

let mut rng = rand::thread_rng();
let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
let public_key = RsaPublicKey::from(&private_key);

// Export as PEM
let private_pem = private_key.to_pkcs8_pem()?;
let public_pem = public_key.to_public_key_pem()?;

// Store private key in KMS/HSM
// Distribute public key to API servers
```

**Rotation:**
- **Frequency:** Every 90 days
- **Multiple Active Keys:** Support old keys during transition (identify via `kid` in JWT header)
- **Gradual Rollout:**
  1. Generate new key pair (kid: `key-2026-11`)
  2. Start signing new tokens with new key
  3. Accept verification with both old and new keys (grace period: 15 min = max token lifetime)
  4. After grace period, retire old key

---

### 6.8 Cryptographic Best Practices

1. **Never Roll Your Own Crypto:** Use audited libraries (libsodium)
2. **Use High-Level APIs:** Avoid low-level primitives (easy to misuse)
3. **Generate Randomness Securely:** Use cryptographically secure RNG (not `rand::random()`)
4. **Constant-Time Operations:** Prevent timing attacks (libsodium provides)
5. **Key Separation:** Different keys for different purposes
6. **Regular Key Rotation:** Limit key exposure time
7. **Encrypt Keys at Rest:** Never store keys in plaintext
8. **Audit Cryptographic Operations:** Log key generation, usage, rotation

---

## 7. Network Security

### 7.1 TLS Configuration

**Version:** TLS 1.3 (minimum), TLS 1.2 fallback (deprecated clients)

**Cipher Suites (TLS 1.3):**
```
TLS_AES_256_GCM_SHA384
TLS_CHACHA20_POLY1305_SHA256
TLS_AES_128_GCM_SHA256
```

**Certificate:**
- **Issuer:** Let's Encrypt or commercial CA
- **Type:** RSA 2048-bit or ECDSA P-256
- **Validity:** 90 days (auto-renewal via certbot or similar)
- **Wildcard:** `*.eemp.app` for subdomains

**HSTS (HTTP Strict Transport Security):**
```
Strict-Transport-Security: max-age=31536000; includeSubDomains; preload
```

**Configuration (Nginx):**
```nginx
server {
    listen 443 ssl http2;
    server_name *.eemp.app;
    
    ssl_certificate /etc/letsencrypt/live/eemp.app/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/eemp.app/privkey.pem;
    
    ssl_protocols TLSv1.3 TLSv1.2;
    ssl_ciphers 'TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256';
    ssl_prefer_server_ciphers on;
    
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always;
    
    # Other security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    
    # CSP
    add_header Content-Security-Policy "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'; base-uri 'self'; form-action 'self';" always;
}

# HTTP to HTTPS redirect
server {
    listen 80;
    server_name *.eemp.app;
    return 301 https://$host$request_uri;
}
```

---

### 7.2 Firewall Rules

**Principle:** Default deny, explicit allow

**Allowed Inbound:**
- Port 443 (HTTPS) from anywhere (public API)
- Port 80 (HTTP) from anywhere (redirect to HTTPS)
- Port 22 (SSH) from bastion host only (management)

**Allowed Outbound:**
- Port 443 (HTTPS) to Solana RPC endpoints
- Port 443 (HTTPS) to S3 (object storage)
- Port 587/465 (SMTP) to email service
- Port 443 (HTTPS) to SMS gateway
- DNS (Port 53)

**Blocked:**
- All other ports
- Direct database access from internet (PostgreSQL port 5432 internal only)

---

### 7.3 DDoS Protection

**Layers:**

1. **Network Layer (L3/L4):** Cloud provider DDoS protection (AWS Shield, Cloudflare)
2. **Application Layer (L7):** 
   - WAF (Web Application Firewall) with rate limiting
   - CDN (Cloudflare) with bot detection
   - Kubernetes auto-scaling (handle traffic spikes)

**Configuration (Cloudflare):**
- **Challenge Mode:** CAPTCHA for suspicious requests
- **Rate Limiting:** 100 req/min per IP (unauthenticated endpoints)
- **Bot Fight Mode:** Block known bad bots
- **DDoS Protection:** Automatic mitigation for volumetric attacks

---

### 7.4 Internal Network Segmentation

**Production Environment:**

```
┌─────────────────────────────────────────────────┐
│              Public Internet                     │
└───────────────────┬─────────────────────────────┘
                    │
┌───────────────────▼─────────────────────────────┐
│           Load Balancer / CDN                    │
└───────────────────┬─────────────────────────────┘
                    │
┌───────────────────▼─────────────────────────────┐
│        Application Tier (API Servers)            │
│        Private Subnet: 10.0.1.0/24               │
└───────────────────┬─────────────────────────────┘
                    │
┌───────────────────▼─────────────────────────────┐
│          Data Tier (PostgreSQL, Redis)           │
│        Private Subnet: 10.0.2.0/24               │
│        No internet access, internal only         │
└──────────────────────────────────────────────────┘
```

**Security Groups:**
- Application tier: Accept HTTPS from load balancer, outbound to data tier and internet
- Data tier: Accept connections from application tier only, no outbound internet

---

## 8. Application Security

### 8.1 Input Validation

**Principle:** Validate all inputs, reject invalid data

**Validation Layers:**
1. **Client-side:** JavaScript validation (UX, not security)
2. **API layer:** Axum middleware + serde deserialization
3. **Business logic:** Domain validation rules

**Example:**
```rust
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
struct CreateElectionRequest {
    #[validate(length(min = 3, max = 255))]
    title: String,
    
    #[validate(length(max = 2000))]
    description: Option<String>,
    
    #[validate(custom = "validate_future_date")]
    start_time: DateTime<Utc>,
    
    #[validate(custom = "validate_end_after_start")]
    end_time: DateTime<Utc>,
}

async fn create_election(
    Json(req): Json<CreateElectionRequest>
) -> Result<Json<Election>> {
    // Validate
    req.validate().map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()})))
    })?;
    
    // Business logic
    ...
}
```

**Sanitization:**
- **HTML:** Strip or escape HTML tags (prevent XSS)
- **SQL:** Use prepared statements (prevent SQL injection)
- **Command:** Never execute shell commands with user input
- **Path:** Validate file paths (prevent path traversal)

**Rejection Policy:**
- Invalid input → 400 Bad Request (with detailed error)
- Never attempt to "fix" invalid input (fail securely)
- Log suspicious inputs (potential attack)

---

### 8.2 SQL Injection Prevention

**Primary Defense:** Prepared statements (parameterized queries)

**SQLx Enforcement:**
```rust
// SAFE: Parameterized query
let user = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE email = $1 AND tenant_id = $2",
    email, tenant_id
)
.fetch_one(&pool)
.await?;

// UNSAFE: String concatenation (SQLx won't compile this)
// let query = format!("SELECT * FROM users WHERE email = '{}'", email);  // DON'T DO THIS
```

**SQLx Compile-Time Verification:**
- Queries checked against database schema at compile time
- Type safety (no runtime errors from mismatched types)
- Impossible to forget parameter binding

**Secondary Defense:** Principle of least privilege
- Application database user has minimal permissions
- No DROP, TRUNCATE, or ALTER permissions
- Read/write only on specific tables

---

### 8.3 Cross-Site Scripting (XSS) Prevention

**Stored XSS:** Malicious script stored in database and executed when displayed

**Mitigations:**
1. **Output Encoding:** Escape HTML in all user-generated content
2. **Content Security Policy (CSP):** Restrict script sources
3. **Input Sanitization:** Strip dangerous HTML tags
4. **HTTPOnly Cookies:** Session tokens not accessible to JavaScript

**Next.js (Frontend) Automatic Protection:**
- React escapes variables by default
- Dangerous HTML requires explicit `dangerouslySetInnerHTML` (auditable)

**CSP Header:**
```
Content-Security-Policy: 
  default-src 'self'; 
  script-src 'self'; 
  style-src 'self' 'unsafe-inline'; 
  img-src 'self' data: https:; 
  font-src 'self'; 
  connect-src 'self'; 
  frame-ancestors 'none'; 
  base-uri 'self'; 
  form-action 'self';
```

---

### 8.4 Cross-Site Request Forgery (CSRF) Prevention

**Mitigations:**

1. **CSRF Tokens:** Synchronizer token pattern
```rust
// Generate CSRF token on login
let csrf_token = generate_random_token();
redis::cmd("SETEX")
    .arg(format!("csrf:{}", session_id))
    .arg(3600)
    .arg(&csrf_token)
    .query_async(&mut redis_conn)
    .await?;

// Return token to client (stored in httpOnly cookie or meta tag)

// Validate on state-changing requests (POST, PUT, DELETE)
async fn validate_csrf_token(
    session_id: &str,
    provided_token: &str
) -> Result<()> {
    let stored_token: String = redis::cmd("GET")
        .arg(format!("csrf:{}", session_id))
        .query_async(&mut redis_conn)
        .await?;
    
    if provided_token != stored_token {
        return Err(SecurityError::InvalidCsrfToken);
    }
    
    Ok(())
}
```

2. **SameSite Cookies:**
```rust
Set-Cookie: session_token=...; HttpOnly; Secure; SameSite=Strict
```

3. **Origin Header Validation:**
```rust
let origin = req.headers().get("Origin");
let allowed_origins = vec!["https://eemp.app", "https://*.eemp.app"];

if !allowed_origins.contains(&origin) {
    return Err(SecurityError::InvalidOrigin);
}
```

4. **Re-authentication for Sensitive Actions:**
- Delete account → require password confirmation
- Change password → require current password
- Publish election results → require MFA code (if enabled)

---

### 8.5 Clickjacking Prevention

**Mitigation:** X-Frame-Options header

```
X-Frame-Options: DENY
```

**Alternative:** CSP `frame-ancestors` directive

```
Content-Security-Policy: frame-ancestors 'none';
```

**Effect:** Prevents embedding EEMP pages in iframes (prevents clickjacking attacks)

---

### 8.6 Security Headers

**Complete Set:**
```rust
// Axum middleware
async fn security_headers_middleware(
    req: Request<Body>,
    next: Next<Body>,
) -> Response {
    let mut response = next.run(req).await;
    
    let headers = response.headers_mut();
    headers.insert("X-Frame-Options", "DENY".parse().unwrap());
    headers.insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    headers.insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    headers.insert("Referrer-Policy", "strict-origin-when-cross-origin".parse().unwrap());
    headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains; preload".parse().unwrap());
    headers.insert("Content-Security-Policy", CSP_POLICY.parse().unwrap());
    headers.insert("Permissions-Policy", "geolocation=(), microphone=(), camera=()".parse().unwrap());
    
    response
}
```

---

## 9. Data Security

### 9.1 Encryption at Rest

**Database:**
- PostgreSQL transparent data encryption (TDE) or full-disk encryption (FDE)
- Sensitive columns encrypted at application layer (MFA secrets, private keys)

**Object Storage:**
- S3 server-side encryption (SSE-S3 or SSE-KMS)
- Encrypted before upload for extra sensitive data

**Backups:**
- Encrypted before storage (pg_dump piped to GPG encryption)
- Stored in encrypted S3 buckets

---

### 9.2 Encryption in Transit

**TLS 1.3:**
- All API requests: HTTPS
- Database connections: PostgreSQL SSL mode (require)
- Redis connections: TLS (stunnel or Redis 6+ native TLS)
- S3 connections: HTTPS
- Internal service communication: mTLS (future)

---

### 9.3 Data Classification

| Classification | Examples | Protection |
|----------------|----------|------------|
| **Public** | Published election results, public election info | No encryption, public read access |
| **Internal** | User profiles, organization settings | Access control (RBAC), tenant isolation (RLS) |
| **Confidential** | Encrypted ballots, email addresses | Access control, encryption at rest, audit logging |
| **Restricted** | Plaintext votes (never stored), private keys, MFA secrets | Never persist plaintext votes; keys encrypted at rest, strict access control |

---

### 9.4 Data Retention and Deletion

**Retention Policy:**
- Election data: 7 years (compliance requirement)
- Audit logs: Indefinite (never delete)
- User accounts: Until user requests deletion (GDPR)
- Sessions: 15 minutes (auto-expire)
- Backups: 30 days (daily), 12 weeks (weekly), 12 months (monthly)

**GDPR Right to Erasure:**
```rust
async fn delete_user_data(user_id: Uuid, tenant_id: Uuid) -> Result<()> {
    // 1. Anonymize user in audit logs (replace with pseudonym)
    sqlx::query!(
        "UPDATE audit_logs SET actor_id = NULL WHERE actor_id = $1",
        user_id
    ).execute(&pool).await?;
    
    // 2. Remove personal data from user record
    sqlx::query!(
        "UPDATE users 
         SET email = 'deleted@example.com', 
             full_name = 'Deleted User', 
             password_hash = 'deleted',
             deleted_at = NOW()
         WHERE id = $1 AND tenant_id = $2",
        user_id, tenant_id
    ).execute(&pool).await?;
    
    // 3. Keep vote records (anonymized, no link to user)
    // Ballots already anonymized (voter_hash, not user_id)
    
    // 4. Remove sessions and refresh tokens
    sqlx::query!("DELETE FROM sessions WHERE user_id = $1", user_id)
        .execute(&pool).await?;
    sqlx::query!("DELETE FROM refresh_tokens WHERE user_id = $1", user_id)
        .execute(&pool).await?;
    
    Ok(())
}
```

**Secure Deletion:**
- Database: Use `DELETE` (PostgreSQL vacuum reclaims space)
- Object storage: S3 object versioning + lifecycle policy (permanent delete after 30 days)
- Backups: Exclude from future backups after deletion

---

## 10. Infrastructure Security

### 10.1 Container Security

**Base Images:**
- Use official, minimal base images (alpine, distroless)
- Scan for vulnerabilities (Trivy, Snyk)
- Keep updated (automated rebuild weekly)

**Dockerfile Best Practices:**
```dockerfile
# Use minimal base image
FROM rust:1.70-alpine AS builder

# Non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .
RUN cargo build --release

# Runtime stage (minimal)
FROM alpine:3.18
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy binary only
COPY --from=builder /app/target/release/eemp-api /usr/local/bin/

# Run as non-root
USER appuser

EXPOSE 8000
CMD ["eemp-api"]
```

**Container Scanning:**
- CI/CD: Scan images before deployment (fail on high/critical vulnerabilities)
- Runtime: Continuous scanning in container registry

---

### 10.2 Kubernetes Security

**Pod Security:**
```yaml
apiVersion: v1
kind: Pod
metadata:
  name: eemp-api
spec:
  securityContext:
    runAsNonRoot: true
    runAsUser: 1000
    fsGroup: 1000
  containers:
  - name: api
    image: eemp-api:latest
    securityContext:
      allowPrivilegeEscalation: false
      readOnlyRootFilesystem: true
      capabilities:
        drop: ["ALL"]
    resources:
      limits:
        memory: "2Gi"
        cpu: "1000m"
      requests:
        memory: "1Gi"
        cpu: "500m"
```

**Network Policies:**
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: api-policy
spec:
  podSelector:
    matchLabels:
      app: eemp-api
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: nginx-ingress
    ports:
    - protocol: TCP
      port: 8000
  egress:
  - to:
    - podSelector:
        matchLabels:
          app: postgresql
    ports:
    - protocol: TCP
      port: 5432
  - to:
    - podSelector:
        matchLabels:
          app: redis
    ports:
    - protocol: TCP
      port: 6379
```

**Secrets Management:**
- Use Kubernetes Secrets (not environment variables in Dockerfile)
- Enable encryption at rest for Secrets
- Use external secret management (AWS Secrets Manager, HashiCorp Vault)

---

### 10.3 Cloud Security (AWS/GCP)

**IAM (Identity and Access Management):**
- Least privilege (minimal permissions per service)
- Service accounts (no long-lived credentials)
- MFA for human access
- Regular access audits

**VPC (Virtual Private Cloud):**
- Private subnets for databases (no public IP)
- Public subnets for load balancers only
- NAT gateway for outbound internet (updates, APIs)

**Logging and Monitoring:**
- CloudTrail (AWS) / Cloud Audit Logs (GCP): All API calls logged
- CloudWatch / Stackdriver: Metrics and alarms
- Security Hub / Security Command Center: Centralized security findings

---

## 11. Security Monitoring

### 11.1 Security Logging

**What to Log:**
- Authentication events (login success/failure, MFA, logout)
- Authorization failures (403 Forbidden)
- Sensitive operations (delete user, publish results)
- Security events (rate limit exceeded, CSRF failure, suspicious patterns)
- Administrative actions (all admin operations)

**Log Format (Structured JSON):**
```json
{
  "timestamp": "2026-08-01T12:34:56.789Z",
  "level": "WARN",
  "event_type": "authentication:failed",
  "user_id": null,
  "ip_address": "192.0.2.1",
  "user_agent": "Mozilla/5.0...",
  "tenant_id": "123e4567-...",
  "details": {
    "reason": "invalid_password",
    "attempt_count": 3
  },
  "correlation_id": "abc-123-def"
}
```

**Log Aggregation:**
- Ship logs to centralized logging (ELK Stack, Datadog, Splunk)
- Retention: 90 days (hot), 7 years (cold/archive)

---

### 11.2 Intrusion Detection

**Patterns to Detect:**
- Brute force login attempts (>10 failures from same IP in 15 min)
- Unusual access patterns (access from new country, VPN/Tor exit node)
- Privilege escalation attempts (non-admin trying admin endpoints)
- SQL injection attempts (pattern matching in logs)
- Large data exports (potential data exfiltration)

**Alerting:**
- Slack/email notifications for critical events
- PagerDuty for incidents requiring immediate response
- Automated response (block IP, revoke session)

---

### 11.3 Security Metrics (KPIs)

| Metric | Target | Frequency |
|--------|--------|-----------|
| **Failed Login Rate** | <1% | Daily |
| **Authorization Failures** | <0.1% | Daily |
| **Mean Time to Detect (MTTD)** | <5 minutes | Per incident |
| **Mean Time to Respond (MTTR)** | <1 hour | Per incident |
| **Vulnerability Scan Findings** | 0 high/critical | Weekly |
| **Penetration Test Findings** | 0 high/critical | Quarterly |
| **Security Training Completion** | 100% employees | Annual |

---

## 12. Incident Response

### 12.1 Incident Response Plan

**Phases:**

1. **Preparation:**
   - Incident response team identified
   - Playbooks documented
   - Tools and access prepared
   - Regular drills (tabletop exercises)

2. **Detection:**
   - Security monitoring alerts
   - User reports
   - Automated detection (IDS, SIEM)

3. **Containment:**
   - Isolate affected systems
   - Revoke compromised credentials
   - Block malicious IPs
   - Preserve evidence

4. **Eradication:**
   - Remove malware/backdoors
   - Patch vulnerabilities
   - Reset compromised accounts

5. **Recovery:**
   - Restore from clean backups
   - Verify system integrity
   - Gradual service restoration
   - Enhanced monitoring

6. **Lessons Learned:**
   - Post-incident review
   - Root cause analysis
   - Update procedures
   - Improve defenses

---

### 12.2 Incident Severity Levels

| Severity | Examples | Response Time | Response Actions |
|----------|----------|---------------|------------------|
| **Critical (P0)** | Active data breach, vote manipulation, widespread outage | <15 minutes | Page on-call, all-hands, CEO notification |
| **High (P1)** | Vulnerability exploited, targeted attack, key system down | <1 hour | Page on-call, incident commander assigned |
| **Medium (P2)** | Failed login spike, suspicious activity, non-critical service down | <4 hours | Notify security team, investigate |
| **Low (P3)** | Vulnerability disclosed (no exploit), minor misconfiguration | <1 business day | Create ticket, prioritize fix |

---

### 12.3 Communication Plan

**Internal:**
- Incident commander coordinates response
- Status updates every 30 min (critical), 2 hours (high)
- Slack war room for coordination

**External:**
- User notification (if data breach or service impact)
- Legal/compliance notification (if required by law)
- Press statement (if public incident)
- Regulatory notification (within 72 hours for GDPR breach)

**Templates:**
- Data breach notification email
- Service outage status page update
- Regulatory filing (GDPR breach notification)

---

### 12.4 Breach Notification

**GDPR Requirements:**
- Notify supervisory authority within 72 hours of becoming aware
- Notify affected individuals "without undue delay"
- Include: nature of breach, likely consequences, measures taken

**Example Notification:**
> Subject: Important Security Notice: Data Breach Affecting [Organization]
> 
> Dear [User],
> 
> We are writing to inform you of a security incident that may have affected your personal data. On [date], we discovered that unauthorized access occurred to [system]. The investigation revealed that [describe what data was accessed].
> 
> **What Information Was Involved:**
> - Email addresses
> - [Other data]
> 
> **What Information Was NOT Involved:**
> - Passwords (stored as cryptographic hashes, cannot be reversed)
> - Votes (stored encrypted, cannot be read without election private key)
> 
> **What We're Doing:**
> - Closed the vulnerability immediately
> - Reset all user sessions
> - Notified authorities as required by law
> - Enhanced monitoring and security controls
> 
> **What You Should Do:**
> - Change your password (if you use the same password elsewhere)
> - Enable Multi-Factor Authentication
> - Monitor for suspicious activity
> 
> We sincerely apologize for this incident. Your trust is our top priority.
> 
> [Contact information for questions]

---

## 13. Security Testing

### 13.1 Testing Types

**1. Static Application Security Testing (SAST):**
- Tool: `cargo clippy`, `cargo audit`
- Frequency: Every commit (CI/CD)
- Checks: Code vulnerabilities, dependency vulnerabilities

**2. Dynamic Application Security Testing (DAST):**
- Tool: OWASP ZAP, Burp Suite
- Frequency: Weekly (staging), monthly (production)
- Checks: XSS, SQL injection, CSRF, security headers

**3. Penetration Testing:**
- Vendor: External security firm
- Frequency: Quarterly
- Scope: Full application, infrastructure, social engineering

**4. Bug Bounty Program:**
- Platform: HackerOne, Bugcrowd
- Launch: After initial security hardening (6 months post-launch)
- Rewards: $100-$10,000 based on severity

**5. Chaos Engineering:**
- Tool: Chaos Monkey, Gremlin
- Frequency: Monthly
- Tests: Resilience to failures (database down, network partition)

---

### 13.2 Security Checklist (Pre-Launch)

- [ ] All passwords hashed with Argon2id
- [ ] JWT access tokens signed with RS256
- [ ] Refresh tokens stored hashed
- [ ] MFA available and tested
- [ ] Rate limiting enforced on all endpoints
- [ ] CSRF protection enabled
- [ ] XSS prevention (CSP, output encoding)
- [ ] SQL injection prevention (prepared statements)
- [ ] TLS 1.3 configured
- [ ] Security headers enabled
- [ ] Input validation on all endpoints
- [ ] Authorization checks on all endpoints
- [ ] Audit logging enabled
- [ ] Ballots encrypted (never plaintext)
- [ ] Blockchain vote commitments working
- [ ] Penetration test completed (no high/critical findings)
- [ ] Vulnerability scan completed (no high/critical findings)
- [ ] Incident response plan documented
- [ ] Security training completed (all engineers)
- [ ] GDPR compliance reviewed (legal approval)
- [ ] SOC 2 audit initiated

---

## 14. Appendices

### Appendix A: Security Glossary

| Term | Definition |
|------|------------|
| **Argon2id** | Password hashing algorithm (memory-hard, resistant to GPU cracking) |
| **CSRF** | Cross-Site Request Forgery (attack tricking user into unwanted action) |
| **CSP** | Content Security Policy (HTTP header restricting script sources) |
| **Defense in Depth** | Multiple layers of security controls |
| **Ed25519** | Digital signature algorithm (elliptic curve) |
| **HSM** | Hardware Security Module (tamper-resistant device for key storage) |
| **JWT** | JSON Web Token (stateless authentication token) |
| **MTTR** | Mean Time to Respond (incident response metric) |
| **RBAC** | Role-Based Access Control (permissions based on roles) |
| **RLS** | Row-Level Security (PostgreSQL feature for multi-tenancy) |
| **TLS** | Transport Layer Security (encrypted communication protocol) |
| **TOTP** | Time-Based One-Time Password (MFA algorithm) |
| **X25519** | Elliptic curve key exchange algorithm |
| **XSS** | Cross-Site Scripting (injecting malicious scripts) |
| **Zero Trust** | Security model (never trust, always verify) |

---

### Appendix B: Security Resources

**OWASP Top 10 (2021):**
1. Broken Access Control → Mitigated by RBAC + RLS
2. Cryptographic Failures → Mitigated by TLS + encryption at rest
3. Injection → Mitigated by prepared statements
4. Insecure Design → Mitigated by threat modeling + secure design patterns
5. Security Misconfiguration → Mitigated by secure defaults + automated scanning
6. Vulnerable Components → Mitigated by dependency scanning + updates
7. Authentication Failures → Mitigated by Argon2id + MFA + rate limiting
8. Software and Data Integrity Failures → Mitigated by blockchain + digital signatures
9. Security Logging Failures → Mitigated by comprehensive audit logging
10. Server-Side Request Forgery → Mitigated by input validation + network segmentation

---

### Appendix C: Compliance Mapping

| Requirement | GDPR | SOC 2 | ISO 27001 | EEMP Implementation |
|-------------|------|-------|-----------|---------------------|
| Data encryption at rest | Art. 32 | CC6.7 | A.10.1 | AES-256, database encryption |
| Data encryption in transit | Art. 32 | CC6.7 | A.13.1 | TLS 1.3 |
| Access control | Art. 32 | CC6.1 | A.9.2 | RBAC + RLS |
| Audit logging | Art. 30 | CC7.2 | A.12.4 | Immutable audit logs |
| Right to erasure | Art. 17 | N/A | N/A | User deletion workflow |
| Data breach notification | Art. 33 | CC7.3 | A.16.1 | Incident response plan |
| MFA | Recital 83 | CC6.1 | A.9.4 | TOTP + backup codes |

---

### Appendix D: Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-08-01 | EEMP Security Team | Initial security architecture |

---

### Appendix E: Approval

| Role | Name | Signature | Date |
|------|------|-----------|------|
| **Chief Security Officer** | | | |
| **CTO** | | | |
| **Compliance Officer** | | | |
| **External Security Auditor** | | | |

---

**Document Classification:** Confidential  
**Confidentiality:** Proprietary and Confidential - Internal Security Use Only

---

*This Security Architecture document defines the security foundation for EEMP. All security implementations must align with the principles and controls specified herein. Any deviations must be approved by the Chief Security Officer.*
