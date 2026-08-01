# EEMP API Endpoints

Complete REST API documentation for the Enterprise Election Management Platform.

## Base URL

```
http://localhost:8000
```

## Authentication

Most endpoints require JWT authentication. Include the token in the `Authorization` header:

```
Authorization: Bearer <access_token>
```

---

## Health Check

### GET /health

Check API health status.

**Response:**
```json
{
  "status": "ok",
  "database": "healthy"
}
```

---

## Authentication Endpoints

### POST /api/v1/auth/register

Register a new user.

**Request:**
```json
{
  "tenant_id": "uuid",
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "full_name": "John Doe"
}
```

**Response:**
```json
{
  "user_id": "uuid",
  "email": "user@example.com"
}
```

### POST /api/v1/auth/login

Login with email and password.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "totp_code": "123456"  // Optional, required if MFA enabled
}
```

**Response:**
```json
{
  "access_token": "jwt_token",
  "refresh_token": "refresh_token",
  "expires_in": 900,
  "token_type": "Bearer",
  "user": {
    "user_id": "uuid",
    "tenant_id": "uuid",
    "email": "user@example.com",
    "role": "Voter"
  },
  "requires_mfa": false
}
```

### POST /api/v1/auth/refresh

Refresh access token.

**Request:**
```json
{
  "refresh_token": "refresh_token"
}
```

**Response:**
```json
{
  "access_token": "new_jwt_token",
  "refresh_token": "new_refresh_token",
  "expires_in": 900,
  "token_type": "Bearer"
}
```

### POST /api/v1/auth/logout

Logout (invalidate session).

**Request:**
```json
{
  "refresh_token": "refresh_token"
}
```

**Response:** `204 No Content`

### GET /api/v1/auth/me

Get current user info. **Requires authentication.**

**Response:**
```json
{
  "user_id": "uuid",
  "tenant_id": "uuid",
  "email": "user@example.com",
  "role": "Voter"
}
```

### POST /api/v1/auth/mfa/enable

Enable MFA for current user. **Requires authentication.**

**Response:**
```json
{
  "secret": "base32_secret",
  "qr_code_url": "data:image/png;base64,...",
  "backup_codes": ["CODE1", "CODE2", ...]
}
```

### POST /api/v1/auth/mfa/verify

Verify MFA setup. **Requires authentication.**

**Request:**
```json
{
  "totp_code": "123456"
}
```

**Response:**
```json
{
  "verified": true
}
```

---

## Organization Endpoints

### POST /api/v1/organizations

Create a new organization.

**Request:**
```json
{
  "name": "Acme University",
  "organization_type": "University",
  "domain": "acme.edu",
  "subdomain": "acme",
  "contact_email": "admin@acme.edu",
  "country": "United States",
  "settings": {
    "allow_public_registration": false,
    "require_email_verification": true,
    "enable_anonymous_voting": true
  }
}
```

**Response:**
```json
{
  "tenant_id": "uuid",
  "name": "Acme University",
  "organization_type": "University"
}
```

### GET /api/v1/organizations/:tenant_id

Get organization by ID.

**Response:**
```json
{
  "tenant_id": "uuid",
  "name": "Acme University",
  "organization_type": "University",
  "domain": "acme.edu",
  "subdomain": "acme",
  "contact_email": "admin@acme.edu",
  "country": "United States",
  "settings": { ... },
  "is_active": true,
  "created_at": "2026-08-01T12:00:00Z",
  "updated_at": "2026-08-01T12:00:00Z"
}
```

### GET /api/v1/organizations

List all organizations (paginated).

**Query Parameters:**
- `limit` (default: 20)
- `offset` (default: 0)

**Response:**
```json
{
  "organizations": [ ... ],
  "total": 42
}
```

### GET /api/v1/organizations/domain/:domain

Get organization by custom domain.

### GET /api/v1/organizations/subdomain/:subdomain

Get organization by subdomain.

---

## Election Endpoints

### POST /api/v1/elections

Create a new election. **Requires authentication.**

**Request:**
```json
{
  "title": "Student Council Election 2026",
  "description": "Annual student council election",
  "election_type": "Individual",
  "voting_start_time": "2026-09-01T08:00:00Z",
  "voting_end_time": "2026-09-03T20:00:00Z",
  "allow_write_in_candidates": false,
  "allow_abstain": true,
  "enable_blockchain_verification": true
}
```

**Response:**
```json
{
  "election_id": "uuid",
  "title": "Student Council Election 2026",
  "status": "Draft"
}
```

### GET /api/v1/elections/:election_id

Get election details.

**Response:**
```json
{
  "election_id": "uuid",
  "tenant_id": "uuid",
  "title": "Student Council Election 2026",
  "description": "Annual student council election",
  "election_type": "Individual",
  "status": "Open",
  "voting_start_time": "2026-09-01T08:00:00Z",
  "voting_end_time": "2026-09-03T20:00:00Z",
  "created_at": "2026-08-01T12:00:00Z"
}
```

### GET /api/v1/elections

List elections. **Requires authentication.**

**Query Parameters:**
- `limit` (default: 20)
- `offset` (default: 0)

**Response:**
```json
{
  "elections": [ ... ],
  "total": 10
}
```

### POST /api/v1/elections/:election_id/transition

Transition election status.

**Request:**
```json
{
  "new_status": "Open"
}
```

**Response:** Election details with new status

### POST /api/v1/elections/:election_id/positions

Create a position in election. **Requires authentication.**

**Request:**
```json
{
  "title": "President",
  "description": "Student Council President",
  "display_order": 0,
  "seats_available": 1
}
```

**Response:**
```json
{
  "position_id": "uuid",
  "election_id": "uuid",
  "title": "President",
  "seats_available": 1,
  "display_order": 0
}
```

### GET /api/v1/elections/:election_id/positions

List positions for an election.

**Response:**
```json
[
  {
    "position_id": "uuid",
    "election_id": "uuid",
    "title": "President",
    "seats_available": 1
  }
]
```

---

## Voting Endpoints

### POST /api/v1/voting/cast

Cast a ballot. **Requires authentication.**

**Request:**
```json
{
  "election_id": "uuid",
  "votes": [
    {
      "position_id": "uuid",
      "candidate_ids": ["uuid1", "uuid2"],
      "is_abstain": false
    }
  ]
}
```

**Response:**
```json
{
  "ballot_id": "uuid",
  "receipt_code": "A3K7N9M4P2",
  "ballot_hash": "sha256_hash",
  "cast_at": "2026-09-02T10:30:00Z",
  "commitment_created": true
}
```

### POST /api/v1/voting/verify-receipt

Verify a receipt code (anonymous).

**Request:**
```json
{
  "receipt_code": "A3K7N9M4P2"
}
```

**Response:**
```json
{
  "valid": true,
  "ballot_id": "uuid",
  "election_id": "uuid",
  "cast_at": "2026-09-02T10:30:00Z"
}
```

### GET /api/v1/voting/status/:election_id

Check voting status for current user. **Requires authentication.**

**Response:**
```json
{
  "election_id": "uuid",
  "has_voted": true,
  "ballot_id": "uuid",
  "cast_at": "2026-09-02T10:30:00Z"
}
```

---

## Result Endpoints

### POST /api/v1/results/:election_id/calculate

Calculate election results (admin only). **Requires authentication.**

**Response:** `200 OK`

### POST /api/v1/results/:election_id/publish

Publish election results (admin only). **Requires authentication.**

**Response:** `200 OK`

### GET /api/v1/results/:election_id

Get election results.

**Response:**
```json
{
  "election_id": "uuid",
  "total_ballots": 220,
  "positions": [
    {
      "position_id": "uuid",
      "position_title": "President",
      "seats_available": 1,
      "total_votes": 220,
      "candidates": [
        {
          "candidate_id": "uuid",
          "candidate_name": "John Doe",
          "vote_count": 120,
          "vote_percentage": 54.5,
          "is_winner": true,
          "rank": 1
        },
        {
          "candidate_id": "uuid",
          "candidate_name": "Jane Smith",
          "vote_count": 100,
          "vote_percentage": 45.5,
          "is_winner": false,
          "rank": 2
        }
      ]
    }
  ],
  "calculated_at": "2026-09-03T21:00:00Z",
  "published_at": "2026-09-04T09:00:00Z"
}
```

---

## Error Responses

All errors follow a standard format:

```json
{
  "error": {
    "code": "validation_error",
    "message": "Human-readable error message",
    "details": "Additional details (optional)",
    "field": "field_name (optional)",
    "request_id": "uuid"
  }
}
```

### Error Codes

- `validation_error` - Input validation failed (400/422)
- `unauthorized` - Authentication required or invalid (401)
- `forbidden` - Permission denied (403)
- `not_found` - Resource not found (404)
- `conflict` - Resource conflict (e.g., already voted) (409)
- `rate_limit_exceeded` - Too many requests (429)
- `internal_error` - Server error (500)

---

## Rate Limiting

Default rate limits:
- 100 requests per minute per IP
- 1000 requests per hour per user

Rate limit headers:
- `X-RateLimit-Limit`
- `X-RateLimit-Remaining`
- `X-RateLimit-Reset`

---

## Pagination

List endpoints support pagination via query parameters:
- `limit` - Number of results (default: 20, max: 100)
- `offset` - Number of results to skip (default: 0)

Response includes `total` count for all results.

---

## Complete Endpoint Summary

**Total: 29 Endpoints**

- **Health**: 1 endpoint
- **Authentication**: 8 endpoints
- **Organizations**: 7 endpoints
- **Elections**: 6 endpoints
- **Voting**: 3 endpoints
- **Results**: 3 endpoints
