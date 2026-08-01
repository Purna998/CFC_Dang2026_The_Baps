# REST API Design Principles
## Enterprise Election Management Platform (EEMP)

**Document Version:** 1.0  
**Last Updated:** 2026-08-01  
**Status:** Draft  
**Classification:** Internal

---

## Document Control

| Field | Value |
|-------|-------|
| **Document Type** | API Design Principles |
| **Owner** | API Architect + Backend Lead |
| **Reviewers** | CTO, Frontend Lead, Mobile Lead (future) |
| **Approvers** | CTO, API Architect |
| **Target Audience** | Backend engineers, frontend engineers, API consumers |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [API Design Principles](#2-api-design-principles)
3. [URL Structure](#3-url-structure)
4. [HTTP Methods](#4-http-methods)
5. [Request/Response Format](#5-requestresponse-format)
6. [Authentication](#6-authentication)
7. [Error Handling](#7-error-handling)
8. [Versioning](#8-versioning)
9. [Pagination](#9-pagination)
10. [Rate Limiting](#10-rate-limiting)
11. [API Endpoints Summary](#11-api-endpoints-summary)

---

## 1. Introduction

### 1.1 Purpose

This document defines the REST API design principles for EEMP. It serves as a guide for designing consistent, predictable, and developer-friendly APIs.

### 1.2 API Design Goals

- **Consistency:** Uniform patterns across all endpoints
- **Predictability:** Developers can guess endpoint structure
- **Developer Experience:** Clear, self-documenting APIs
- **Security:** Authentication, authorization, rate limiting
- **Performance:** Efficient, cacheable where appropriate
- **Extensibility:** Easy to add new endpoints without breaking existing

---

## 2. API Design Principles

### 2.1 RESTful Principles

**Resources as Nouns:**
- Use nouns, not verbs: `/users` not `/getUsers`
- Plural nouns: `/elections` not `/election`
- Hierarchical: `/elections/{id}/candidates` (candidates belong to election)

**HTTP Methods as Verbs:**
- `GET`: Read resources
- `POST`: Create resources
- `PUT`: Update entire resource
- `PATCH`: Update partial resource
- `DELETE`: Delete resource

**Stateless:**
- Each request contains all necessary information (JWT token)
- No server-side session state (except authentication tokens in database)

**Cacheable:**
- `GET` requests cacheable with `Cache-Control` headers
- `POST/PUT/DELETE` not cacheable

---

### 2.2 JSON API Standard

**Content Type:**
```
Content-Type: application/json
Accept: application/json
```

**Request Body (JSON):**
```json
{
  "title": "Student Union Election 2026",
  "election_type": "post_wise",
  "start_time": "2026-09-01T09:00:00Z",
  "end_time": "2026-09-02T17:00:00Z"
}
```

**Response Body (JSON):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Student Union Election 2026",
  "status": "draft",
  "created_at": "2026-08-01T10:30:00Z"
}
```

---

### 2.3 API-First Design

**Process:**
1. Define API specification (OpenAPI) first
2. Review with frontend and backend teams
3. Mock API for frontend development
4. Implement backend against specification
5. Integration testing against specification

**Benefits:**
- Parallel development (frontend doesn't wait for backend)
- Contract-driven (frontend and backend agree on interface)
- Documentation automatic (generated from OpenAPI spec)

---

## 3. URL Structure

### 3.1 Base URL

**Production:**
```
https://api.eemp.app
```

**Staging:**
```
https://api-staging.eemp.app
```

**Development:**
```
http://localhost:8000
```

---

### 3.2 URL Pattern

**Format:**
```
https://api.eemp.app/api/{version}/{resource}[/{resource_id}][/{sub_resource}]
```

**Examples:**
```
GET    /api/v1/elections
GET    /api/v1/elections/{election_id}
GET    /api/v1/elections/{election_id}/candidates
POST   /api/v1/elections/{election_id}/vote
GET    /api/v1/users/me
```

---

### 3.3 Resource Naming

**Rules:**
- Lowercase
- Plural nouns: `/users`, `/elections`, `/candidates`
- Hyphen-separated for multi-word: `/election-results` (not underscore)
- No trailing slash: `/elections` not `/elections/`
- Hierarchical relationships: `/elections/{id}/positions`

**Examples:**
```
✅ Good: /api/v1/elections
❌ Bad:  /api/v1/election
❌ Bad:  /api/v1/getElections
❌ Bad:  /api/v1/Elections (not capitalized)
```

---

## 4. HTTP Methods

### 4.1 Method Semantics

| Method | Purpose | Idempotent | Safe | Request Body | Response Body |
|--------|---------|------------|------|--------------|---------------|
| **GET** | Read resource(s) | Yes | Yes | No | Yes |
| **POST** | Create resource | No | No | Yes | Yes (created resource) |
| **PUT** | Replace entire resource | Yes | No | Yes | Yes (updated resource) |
| **PATCH** | Update partial resource | No | No | Yes | Yes (updated resource) |
| **DELETE** | Delete resource | Yes | No | No | 204 No Content |

**Idempotent:** Multiple identical requests have same effect as single request  
**Safe:** No side effects (read-only)

---

### 4.2 Method Usage

**GET - Retrieve Resources:**
```http
GET /api/v1/elections
GET /api/v1/elections/{id}
GET /api/v1/elections/{id}/candidates
```

**POST - Create Resource:**
```http
POST /api/v1/elections
Content-Type: application/json

{
  "title": "New Election",
  "election_type": "individual",
  "start_time": "2026-09-01T09:00:00Z",
  "end_time": "2026-09-02T17:00:00Z"
}
```

**Response (201 Created):**
```http
HTTP/1.1 201 Created
Location: /api/v1/elections/550e8400-e29b-41d4-a716-446655440000
Content-Type: application/json

{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "New Election",
  "status": "draft",
  ...
}
```

**PUT - Replace Entire Resource:**
```http
PUT /api/v1/elections/{id}
Content-Type: application/json

{
  "title": "Updated Election Title",
  "description": "Updated description",
  "election_type": "individual",
  "start_time": "2026-09-01T09:00:00Z",
  "end_time": "2026-09-02T17:00:00Z"
  // Must include ALL fields
}
```

**PATCH - Update Partial Resource:**
```http
PATCH /api/v1/elections/{id}
Content-Type: application/json

{
  "title": "Updated Election Title"
  // Only fields being updated
}
```

**DELETE - Remove Resource:**
```http
DELETE /api/v1/elections/{id}
```

**Response (204 No Content):**
```http
HTTP/1.1 204 No Content
```

---

### 4.3 Special Actions (Non-CRUD)

**Use POST for actions that don't map to CRUD:**
```http
POST /api/v1/elections/{id}/publish
POST /api/v1/elections/{id}/close
POST /api/v1/elections/{id}/reopen
POST /api/v1/auth/login
POST /api/v1/auth/refresh
POST /api/v1/auth/logout
```

**Format:** `POST /{resource}/{id}/{action}`

---

## 5. Request/Response Format

### 5.1 Request Headers

**Required:**
```http
Content-Type: application/json
Accept: application/json
Authorization: Bearer {access_token}
```

**Optional:**
```http
X-Request-ID: {uuid}           # For request tracing
X-Tenant-ID: {tenant_uuid}     # For multi-tenancy (extracted from JWT, not trusted from header)
```

---

### 5.2 Response Headers

**Standard:**
```http
Content-Type: application/json
X-Request-ID: {uuid}           # Echo request ID or generate new
X-RateLimit-Limit: 1000        # Rate limit ceiling
X-RateLimit-Remaining: 998     # Requests remaining
X-RateLimit-Reset: 1722528900  # Unix timestamp when limit resets
```

**Cache Control (for GET requests):**
```http
Cache-Control: public, max-age=300          # Cache for 5 minutes
Cache-Control: private, no-cache            # Don't cache
ETag: "33a64df551425fcc55e4d42a148795d9f25f89d4"
```

---

### 5.3 Response Structure

**Single Resource:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Student Union Election 2026",
  "status": "open",
  "start_time": "2026-09-01T09:00:00Z",
  "end_time": "2026-09-02T17:00:00Z",
  "created_at": "2026-08-01T10:30:00Z",
  "updated_at": "2026-08-15T14:20:00Z"
}
```

**Collection (List):**
```json
{
  "data": [
    {
      "id": "550e8400-...",
      "title": "Election 1",
      ...
    },
    {
      "id": "6ba7b810-...",
      "title": "Election 2",
      ...
    }
  ],
  "meta": {
    "total": 42,
    "page": 1,
    "per_page": 20,
    "total_pages": 3
  },
  "links": {
    "self": "/api/v1/elections?page=1",
    "next": "/api/v1/elections?page=2",
    "last": "/api/v1/elections?page=3"
  }
}
```

**Empty Collection:**
```json
{
  "data": [],
  "meta": {
    "total": 0,
    "page": 1,
    "per_page": 20,
    "total_pages": 0
  }
}
```

---

### 5.4 Date/Time Format

**ISO 8601 with UTC timezone:**
```json
{
  "created_at": "2026-08-01T10:30:00Z",
  "start_time": "2026-09-01T09:00:00Z"
}
```

**Always use UTC (Z suffix)**, clients convert to local timezone.

---

### 5.5 UUID Format

**UUIDv4 (canonical string representation):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
}
```

**Lowercase with hyphens**, 36 characters total.

---

## 6. Authentication

### 6.1 Bearer Token (JWT)

**Header:**
```http
Authorization: Bearer eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...
```

**Public Endpoints (no authentication required):**
- `POST /api/v1/auth/login`
- `POST /api/v1/auth/register`
- `POST /api/v1/organizations` (registration)
- `GET /api/v1/verify` (vote verification, public)

**All Other Endpoints:** Require valid JWT token.

---

### 6.2 Authentication Flow

**Login:**
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePassword123!"
}
```

**Response (200 OK):**
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIs...",
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000",
  "token_type": "Bearer",
  "expires_in": 900,
  "user": {
    "id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
    "email": "user@example.com",
    "full_name": "John Doe",
    "roles": ["voter", "election_manager"]
  }
}
```

**Use Access Token:**
```http
GET /api/v1/elections
Authorization: Bearer eyJhbGciOiJSUzI1NiIs...
```

**Refresh Token (when access token expires):**
```http
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response (200 OK):**
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIs...",  // New access token
  "refresh_token": "7c9e6679-7425-40de-944b-e07fc1f90ae7",  // New refresh token (rotated)
  "token_type": "Bearer",
  "expires_in": 900
}
```

---

### 6.3 Token Expiration Handling

**Client Responsibilities:**
1. Store access token and refresh token securely
2. Include access token in `Authorization` header
3. When `401 Unauthorized` with `token_expired` code:
   - Attempt token refresh
   - If refresh succeeds: Retry original request
   - If refresh fails: Redirect to login

**401 Response:**
```json
{
  "error": {
    "code": "token_expired",
    "message": "Access token has expired",
    "details": "Please refresh your token or log in again"
  }
}
```

---

## 7. Error Handling

### 7.1 HTTP Status Codes

| Status | Meaning | When to Use |
|--------|---------|-------------|
| **200 OK** | Success | Successful GET, PUT, PATCH, DELETE |
| **201 Created** | Resource created | Successful POST (resource creation) |
| **204 No Content** | Success, no body | Successful DELETE, some PUT/PATCH |
| **400 Bad Request** | Invalid request | Validation errors, malformed JSON |
| **401 Unauthorized** | Not authenticated | Missing or invalid JWT token |
| **403 Forbidden** | Not authorized | Valid token, insufficient permissions |
| **404 Not Found** | Resource not found | Resource doesn't exist |
| **409 Conflict** | Conflict | Duplicate resource, constraint violation |
| **422 Unprocessable Entity** | Validation failed | Business logic validation errors |
| **429 Too Many Requests** | Rate limit exceeded | Too many requests in time window |
| **500 Internal Server Error** | Server error | Unexpected server errors |
| **503 Service Unavailable** | Temporarily unavailable | Maintenance, overload |

---

### 7.2 Error Response Format

**Structure:**
```json
{
  "error": {
    "code": "validation_error",
    "message": "Validation failed for one or more fields",
    "details": "Election start time must be in the future",
    "field": "start_time",
    "request_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

**Fields:**
- `code`: Machine-readable error code (snake_case)
- `message`: Human-readable error message (English, short)
- `details`: Additional context (optional)
- `field`: Field name causing error (optional, for validation errors)
- `request_id`: Request ID for debugging (always included)

---

### 7.3 Error Codes

**Authentication Errors (401):**
- `missing_token`: No Authorization header
- `invalid_token`: Token malformed or signature invalid
- `token_expired`: Token past expiration time
- `token_revoked`: Token has been revoked

**Authorization Errors (403):**
- `insufficient_permissions`: User lacks required permission
- `tenant_mismatch`: Accessing resource from different tenant

**Validation Errors (400, 422):**
- `validation_error`: General validation failure
- `invalid_format`: Field has invalid format (e.g., email, UUID)
- `required_field`: Required field missing
- `field_too_long`: Field exceeds max length
- `field_too_short`: Field below min length
- `invalid_enum_value`: Value not in allowed enum

**Resource Errors (404, 409):**
- `resource_not_found`: Resource doesn't exist
- `duplicate_resource`: Resource already exists (e.g., duplicate email)
- `constraint_violation`: Foreign key or unique constraint violated

**Business Logic Errors (422):**
- `election_already_started`: Cannot edit election after start
- `already_voted`: User has already voted in this election
- `not_eligible`: User not eligible to vote in this election
- `election_not_open`: Election not in "open" state

**Rate Limiting (429):**
- `rate_limit_exceeded`: Too many requests

**Server Errors (500, 503):**
- `internal_error`: Unexpected server error
- `database_error`: Database operation failed
- `blockchain_error`: Blockchain transaction failed
- `service_unavailable`: Service temporarily unavailable

---

### 7.4 Error Examples

**400 Bad Request (Validation):**
```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "error": {
    "code": "validation_error",
    "message": "Validation failed",
    "details": "Election end time must be after start time",
    "field": "end_time",
    "request_id": "550e8400-..."
  }
}
```

**401 Unauthorized:**
```http
HTTP/1.1 401 Unauthorized
Content-Type: application/json

{
  "error": {
    "code": "token_expired",
    "message": "Access token has expired",
    "details": "Please refresh your token or log in again",
    "request_id": "550e8400-..."
  }
}
```

**403 Forbidden:**
```http
HTTP/1.1 403 Forbidden
Content-Type: application/json

{
  "error": {
    "code": "insufficient_permissions",
    "message": "You do not have permission to perform this action",
    "details": "Required permission: election:create",
    "request_id": "550e8400-..."
  }
}
```

**404 Not Found:**
```http
HTTP/1.1 404 Not Found
Content-Type: application/json

{
  "error": {
    "code": "resource_not_found",
    "message": "Election not found",
    "details": "No election exists with ID 550e8400-e29b-41d4-a716-446655440000",
    "request_id": "550e8400-..."
  }
}
```

**409 Conflict:**
```http
HTTP/1.1 409 Conflict
Content-Type: application/json

{
  "error": {
    "code": "duplicate_resource",
    "message": "User with this email already exists",
    "field": "email",
    "request_id": "550e8400-..."
  }
}
```

**422 Unprocessable Entity:**
```http
HTTP/1.1 422 Unprocessable Entity
Content-Type: application/json

{
  "error": {
    "code": "already_voted",
    "message": "You have already voted in this election",
    "details": "Each voter can only vote once per election",
    "request_id": "550e8400-..."
  }
}
```

**429 Too Many Requests:**
```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/json
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1722528900
Retry-After: 900

{
  "error": {
    "code": "rate_limit_exceeded",
    "message": "Rate limit exceeded",
    "details": "You have exceeded 1000 requests per 15 minutes. Try again in 900 seconds.",
    "request_id": "550e8400-..."
  }
}
```

**500 Internal Server Error:**
```http
HTTP/1.1 500 Internal Server Error
Content-Type: application/json

{
  "error": {
    "code": "internal_error",
    "message": "An unexpected error occurred",
    "details": "Our team has been notified and is investigating",
    "request_id": "550e8400-..."
  }
}
```

---

## 8. Versioning

### 8.1 URL Versioning

**Format:** `/api/v{major_version}/`

**Examples:**
```
/api/v1/elections
/api/v2/elections  (future)
```

**Version in URL, not header** (easier to test, cache, and debug).

---

### 8.2 Version Policy

**Major Version (v1 → v2):**
- Breaking changes only
- Remove endpoint
- Change response structure
- Remove or rename field
- Change semantics

**Minor Version (backward-compatible):**
- Add new endpoint
- Add new optional field to request
- Add new field to response (clients must ignore unknown fields)
- No URL version change

**Version Support:**
- Support N and N-1 versions (e.g., v2 and v1)
- Deprecation notice: 6 months before removing old version
- Deprecation header: `Sunset: Sat, 01 Aug 2027 23:59:59 GMT`

---

## 9. Pagination

### 9.1 Offset-Based Pagination

**Query Parameters:**
```
page=1&per_page=20
```

**Request:**
```http
GET /api/v1/elections?page=2&per_page=20
```

**Response:**
```json
{
  "data": [...],
  "meta": {
    "total": 100,
    "page": 2,
    "per_page": 20,
    "total_pages": 5
  },
  "links": {
    "self": "/api/v1/elections?page=2&per_page=20",
    "first": "/api/v1/elections?page=1&per_page=20",
    "prev": "/api/v1/elections?page=1&per_page=20",
    "next": "/api/v1/elections?page=3&per_page=20",
    "last": "/api/v1/elections?page=5&per_page=20"
  }
}
```

**Defaults:**
- `page`: 1
- `per_page`: 20
- `max per_page`: 100

---

### 9.2 Cursor-Based Pagination (Future)

**For real-time data (e.g., audit logs):**
```http
GET /api/v1/audit-logs?cursor=eyJpZCI6IjU1MGU4NDAwIn0&limit=50
```

**Benefits:**
- No duplicate results when data inserted during pagination
- More efficient for large datasets

---

## 10. Rate Limiting

### 10.1 Rate Limits

| Tier | Limit | Window | Applied To |
|------|-------|--------|------------|
| **Authenticated** | 1000 requests | 15 minutes | Per user |
| **Unauthenticated** | 100 requests | 15 minutes | Per IP address |
| **Login Endpoint** | 10 attempts | 15 minutes | Per IP address |
| **Vote Casting** | 1 request | Per election | Per user |

---

### 10.2 Rate Limit Headers

**Every Response:**
```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 998
X-RateLimit-Reset: 1722528900
```

**When Rate Limit Exceeded (429):**
```http
HTTP/1.1 429 Too Many Requests
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1722528900
Retry-After: 900

{
  "error": {
    "code": "rate_limit_exceeded",
    "message": "Rate limit exceeded",
    "details": "Try again in 900 seconds"
  }
}
```

**Client Should:**
1. Check `X-RateLimit-Remaining`
2. When approaching limit, slow down requests
3. When `429` received, wait `Retry-After` seconds

---

## 11. API Endpoints Summary

### 11.1 Authentication

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/auth/register` | Register new user |
| POST | `/api/v1/auth/login` | Login and obtain tokens |
| POST | `/api/v1/auth/refresh` | Refresh access token |
| POST | `/api/v1/auth/logout` | Logout (revoke tokens) |
| POST | `/api/v1/auth/mfa/enroll` | Enroll in MFA |
| POST | `/api/v1/auth/mfa/verify` | Verify MFA code |
| POST | `/api/v1/auth/password-reset` | Request password reset |
| POST | `/api/v1/auth/password-reset/confirm` | Confirm password reset |

---

### 11.2 Users

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/users/me` | Get current user profile |
| PATCH | `/api/v1/users/me` | Update current user profile |
| POST | `/api/v1/users/me/change-password` | Change password |
| GET | `/api/v1/users` | List users (admin) |
| GET | `/api/v1/users/{id}` | Get user by ID (admin) |
| POST | `/api/v1/users` | Create user (admin) |
| PATCH | `/api/v1/users/{id}` | Update user (admin) |
| DELETE | `/api/v1/users/{id}` | Delete user (admin) |

---

### 11.3 Organizations

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/organizations` | Register new organization |
| GET | `/api/v1/organizations/me` | Get current organization |
| PATCH | `/api/v1/organizations/me` | Update organization |
| GET | `/api/v1/organizations/me/settings` | Get settings |
| PATCH | `/api/v1/organizations/me/settings` | Update settings |

---

### 11.4 Elections

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/elections` | List elections |
| POST | `/api/v1/elections` | Create election |
| GET | `/api/v1/elections/{id}` | Get election details |
| PATCH | `/api/v1/elections/{id}` | Update election |
| DELETE | `/api/v1/elections/{id}` | Delete election |
| POST | `/api/v1/elections/{id}/publish` | Publish election |
| POST | `/api/v1/elections/{id}/close` | Close election |
| GET | `/api/v1/elections/{id}/results` | Get results |

---

### 11.5 Candidates

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/elections/{election_id}/candidates` | List candidates |
| POST | `/api/v1/elections/{election_id}/candidates` | Add candidate |
| GET | `/api/v1/candidates/{id}` | Get candidate |
| PATCH | `/api/v1/candidates/{id}` | Update candidate |
| DELETE | `/api/v1/candidates/{id}` | Delete candidate |
| POST | `/api/v1/candidates/{id}/verify` | Verify candidate |

---

### 11.6 Voting

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/elections/{id}/vote` | Cast vote |
| GET | `/api/v1/verify` | Verify vote (public) |
| GET | `/api/v1/elections/{id}/eligibility` | Check eligibility |

---

### 11.7 Audit

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/audit-logs` | List audit logs |
| GET | `/api/v1/audit-logs/export` | Export audit logs |

---

### 11.8 Analytics

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/elections/{id}/analytics` | Election analytics |
| GET | `/api/v1/analytics/dashboard` | Organization dashboard |

---

**Total Endpoints:** 40+

**See [OpenAPI Specification](openapi.yaml) for complete API documentation with request/response schemas.**

---

## Appendices

### Appendix A: HTTP Status Code Quick Reference

| Code | Name | Common Use |
|------|------|------------|
| 200 | OK | Successful GET, PUT, PATCH |
| 201 | Created | Successful POST (creation) |
| 204 | No Content | Successful DELETE |
| 400 | Bad Request | Validation error |
| 401 | Unauthorized | Invalid/missing token |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Duplicate resource |
| 422 | Unprocessable Entity | Business logic error |
| 429 | Too Many Requests | Rate limit exceeded |
| 500 | Internal Server Error | Server error |
| 503 | Service Unavailable | Maintenance/overload |

---

### Appendix B: Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-08-01 | EEMP API Team | Initial API design principles |

---

**Document Classification:** Internal  
**Confidentiality:** Proprietary and Confidential  

---

*This REST API Design document establishes consistent patterns for all EEMP APIs. All endpoint implementations must follow these principles.*
