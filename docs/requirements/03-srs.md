# Software Requirements Specification (SRS)
## Enterprise Election Management Platform (EEMP)

**Document Version:** 1.0  
**Last Updated:** 2026-08-01  
**Status:** Draft  
**Classification:** Internal

---

## Document Control

| Field | Value |
|-------|-------|
| **Document Type** | Software Requirements Specification |
| **Owner** | Product Management + Engineering |
| **Reviewers** | CTO, Lead Engineers, QA Team |
| **Approvers** | CTO, Product Owner |
| **Target Audience** | Engineers, QA, architects |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Overall Description](#2-overall-description)
3. [System Features](#3-system-features)
4. [External Interface Requirements](#4-external-interface-requirements)
5. [System Requirements](#5-system-requirements)
6. [Data Requirements](#6-data-requirements)
7. [Constraints](#7-constraints)
8. [Assumptions and Dependencies](#8-assumptions-and-dependencies)
9. [Acceptance Criteria](#9-acceptance-criteria)

---

## 1. Introduction

### 1.1 Purpose

This Software Requirements Specification (SRS) defines the functional and system requirements for the Enterprise Election Management Platform (EEMP). It serves as the foundation for system design, implementation, testing, and validation.

**Intended Audience:**
- Software engineers implementing the system
- QA engineers writing test plans
- Product managers validating features
- Architects ensuring alignment with HLD

### 1.2 Scope

**Product Name:** Enterprise Election Management Platform (EEMP)

**Product Description:**
EEMP is a multi-tenant SaaS platform that enables organizations to conduct secure, transparent, blockchain-backed digital elections.

**Benefits:**
- 90% cost reduction vs. manual elections
- Instant result publication (vs. hours/days for manual counting)
- Blockchain-backed transparency and auditability
- Configurable for any organizational election type
- Accessible from anywhere, anytime

**Features (High-Level):**
- Multi-tenant organization management
- Configurable election types and rules
- End-to-end encrypted ballot casting
- Blockchain vote commitment storage
- Real-time result calculation
- Comprehensive audit trail
- Analytics and reporting

**Out of Scope (B2B Phase):**
- Government election workflows
- National ID integration
- Election commission interfaces
- Government voter registry
- Government political party registration

### 1.3 Definitions and Acronyms

| Term | Definition |
|------|------------|
| **Tenant** | An organization using the platform (isolated data space) |
| **Organization** | A legal entity conducting elections (university, company, NGO, etc.) |
| **Election** | A specific voting event with defined positions, candidates, and rules |
| **Candidate** | Individual standing for election to a position |
| **Voter** | Organization member eligible to cast votes |
| **Ballot** | Encrypted vote record |
| **Vote Commitment** | Cryptographic hash of vote stored on blockchain |
| **Eligibility Rule** | Configurable criteria determining voter eligibility |
| **RBAC** | Role-Based Access Control |
| **ABAC** | Attribute-Based Access Control |
| **RLS** | Row-Level Security (PostgreSQL) |
| **MFA** | Multi-Factor Authentication |
| **JWT** | JSON Web Token |
| **HSM** | Hardware Security Module |

### 1.4 References

- [Vision Document](01-vision.md)
- [Business Requirements Document (BRD)](02-brd.md)
- [High-Level Design (HLD)](../architecture/01-hld.md)
- [Security Architecture](../security/01-security-architecture.md)
- [Database Schema](../design/01-database-schema.md)

---

## 2. Overall Description

### 2.1 Product Perspective

EEMP is a new, self-contained product in the organizational election management space. It interfaces with:

**External Systems:**
- Email services (SMTP) for notifications
- SMS gateways for MFA
- Solana blockchain for vote commitments
- Object storage (S3) for documents
- SSO providers (SAML/OAuth) - future

**System Interfaces:**
- RESTful API for all operations
- WebSocket connections for real-time updates (future)
- Blockchain RPC for Solana interaction
- SMTP for email delivery
- HTTP API for SMS gateway

### 2.2 Product Functions

**Primary Functions:**

1. **Multi-Tenant Organization Management**
   - Organization onboarding and configuration
   - Tenant isolation and data security
   - Organization branding and customization

2. **User Management and Authentication**
   - User registration and authentication
   - Role-based access control
   - Multi-factor authentication
   - Session management

3. **Election Management**
   - Election creation and configuration
   - Election lifecycle state management
   - Position and eligibility rule definition
   - Election scheduling

4. **Candidate Management**
   - Candidate registration
   - Document upload and verification
   - Profile management

5. **Vote Casting**
   - Secure ballot encryption
   - Blockchain commitment submission
   - Vote verification code generation
   - Voter participation tracking

6. **Result Management**
   - Result calculation and aggregation
   - Result publication
   - Result verification

7. **Audit and Compliance**
   - Immutable audit logging
   - Blockchain verification
   - Compliance reporting

8. **Analytics and Reporting**
   - Real-time dashboards
   - Participation metrics
   - Custom reports

### 2.3 User Classes and Characteristics

| User Class | Technical Expertise | Frequency of Use | Functions Used |
|------------|-------------------|------------------|----------------|
| **Platform Super Admin** | High | Daily | All platform management, tenant oversight, system monitoring |
| **Organization Owner** | Medium | Weekly | Organization config, user management, election oversight |
| **Election Manager** | Medium | Daily (during election) | Election creation, candidate management, result publication |
| **Election Officer** | Medium | Daily (during election) | Voter verification, candidate verification, monitoring |
| **Voter** | Low | Occasional (during elections) | Vote casting, result viewing, vote verification |
| **Candidate** | Low | Occasional | Profile management, document upload, result tracking |
| **Auditor** | High | Periodic | Audit log access, blockchain verification, compliance reporting |
| **Observer** | Medium | During elections | Real-time monitoring, verification |

### 2.4 Operating Environment

**Server-Side:**
- **Operating System:** Linux (Ubuntu 22.04+ or container runtime)
- **Runtime:** Rust compiled binaries
- **Database:** PostgreSQL 16+
- **Cache:** Redis 7+
- **Object Storage:** S3-compatible (AWS S3, MinIO, GCS)
- **Blockchain:** Solana Mainnet (or Devnet for testing)
- **Containers:** Docker 24+
- **Orchestration:** Kubernetes 1.28+ (production)

**Client-Side:**
- **Web Browsers:** Chrome 120+, Firefox 120+, Safari 16+, Edge 120+
- **Mobile:** iOS Safari 16+, Chrome Mobile 120+
- **Screen Readers:** JAWS, NVDA, VoiceOver (WCAG 2.1 AA compliance)
- **Internet Connection:** Minimum 1 Mbps (recommended 5+ Mbps)
- **Screen Resolution:** Minimum 320px width (mobile-first responsive)

### 2.5 Design and Implementation Constraints

**Technical Constraints:**
- Must use Rust for backend (memory safety requirement)
- Must use PostgreSQL (ACID compliance requirement)
- Must use Solana blockchain (architecture decision)
- Must support TLS 1.3
- Must implement Argon2id for password hashing
- Must use Ed25519 for digital signatures

**Regulatory Constraints:**
- Must comply with GDPR (EU)
- Must comply with CCPA (California)
- Must achieve WCAG 2.1 AA accessibility
- Must support data residency requirements

**Business Constraints:**
- B2B organizations only (no government elections in Phase 1)
- SaaS delivery model only (no on-premise in Phase 1)
- English language only in Phase 1 (architecture supports i18n)

### 2.6 Assumptions and Dependencies

**Assumptions:**
- Users have reliable internet connectivity
- Organizations have accurate voter/member lists
- Email delivery is reliable (99%+ delivery rate)
- Solana blockchain maintains performance (65K+ TPS)

**Dependencies:**
- PostgreSQL 16+ availability
- Redis 7+ availability
- Solana blockchain availability (99.9%+ uptime)
- SMTP service availability
- S3-compatible object storage availability

---

## 3. System Features

### 3.1 Organization Management

#### 3.1.1 Organization Registration (FR-ORG-001)

**Priority:** Must Have  
**Risk:** Low

**Description:**
Allow new organizations to self-register and create a tenant account on the platform.

**Functional Requirements:**

**FR-ORG-001.1:** Organization Self-Registration
- System SHALL provide a public registration form
- System SHALL collect: organization name, type, domain, admin email, admin name
- System SHALL validate email domain (basic format validation)
- System SHALL check for duplicate organization names
- System SHALL generate unique tenant ID (UUID v4)
- System SHALL create default roles and permissions for organization
- System SHALL send email verification to admin
- Registration SHALL complete in <10 seconds (p95)

**FR-ORG-001.2:** Email Verification
- System SHALL send verification email with time-limited token (24 hours)
- System SHALL provide verification link in email
- System SHALL activate organization upon successful verification
- System SHALL mark admin user as verified
- System SHALL send welcome email with onboarding instructions

**FR-ORG-001.3:** Organization Subdomain
- System SHALL generate unique subdomain: `{org-slug}.eemp.app`
- System SHALL validate subdomain uniqueness
- System SHALL support custom domain (future)
- Subdomain SHALL only contain alphanumeric and hyphens
- Subdomain SHALL be 3-63 characters

**Input:**
- Organization name (required, 2-255 characters)
- Organization type (required, from predefined list)
- Admin email (required, valid email format)
- Admin name (required, 2-255 characters)
- Admin password (required, min 8 characters, complexity requirements)

**Output:**
- Success: Organization created, verification email sent, tenant ID returned
- Failure: Validation errors, duplicate organization error

**Acceptance Criteria:**
- [ ] Organization can register in <3 minutes
- [ ] Email verification link works
- [ ] Duplicate organization names rejected
- [ ] Subdomain generated correctly
- [ ] Admin can log in after verification

---

#### 3.1.2 Organization Configuration (FR-ORG-002)

**Priority:** Must Have  
**Risk:** Low

**Description:**
Allow organization admins to configure organization-specific settings and branding.

**Functional Requirements:**

**FR-ORG-002.1:** Organization Profile
- System SHALL allow editing: name, description, website, contact email, phone
- System SHALL validate all inputs
- System SHALL log all profile changes in audit log
- Changes SHALL take effect immediately
- System SHALL support organization logo upload (max 2MB, PNG/JPG/SVG)

**FR-ORG-002.2:** Organization Branding
- System SHALL allow configuring: primary color, secondary color, logo
- System SHALL validate color hex codes
- System SHALL preview branding changes before saving
- System SHALL apply branding to all organization pages
- System SHALL support light/dark mode variants

**FR-ORG-002.3:** Organization Settings
- System SHALL allow configuring:
  - Default language (future)
  - Timezone (for election scheduling)
  - Date format
  - Election approval workflow (enabled/disabled)
  - Voter self-registration (enabled/disabled)
  - MFA requirement (enforced/optional)
- System SHALL validate all settings
- System SHALL apply settings to all organization users/elections

**Input:**
- Organization profile fields (optional)
- Branding configuration (optional)
- Settings toggles (optional)

**Output:**
- Success: Settings saved, preview updated
- Failure: Validation errors

**Acceptance Criteria:**
- [ ] Organization admin can update settings
- [ ] Branding changes appear immediately
- [ ] Logo upload works (PNG, JPG, SVG)
- [ ] All changes logged in audit trail
- [ ] Settings persist across sessions

---

### 3.2 User Management and Authentication

#### 3.2.1 User Registration (FR-AUTH-001)

**Priority:** Must Have  
**Risk:** Medium (security-critical)

**Description:**
Allow users to register accounts within their organization tenant.

**Functional Requirements:**

**FR-AUTH-001.1:** User Self-Registration
- System SHALL provide user registration form
- System SHALL collect: email, password, full name
- System SHALL validate email uniqueness within tenant
- System SHALL enforce password policy:
  - Minimum 8 characters
  - At least 1 uppercase letter
  - At least 1 lowercase letter
  - At least 1 number
  - At least 1 special character
- System SHALL hash password using Argon2id (OWASP recommended parameters)
- System SHALL assign default "Voter" role
- System SHALL send email verification
- System SHALL create inactive account until email verified

**FR-AUTH-001.2:** Bulk User Import
- System SHALL allow organization admin to import users via CSV
- CSV SHALL contain: email, full name, custom attributes (for eligibility)
- System SHALL validate all rows before import
- System SHALL generate temporary passwords (16 characters, cryptographically random)
- System SHALL send credentials email to each imported user
- System SHALL mark users as "password reset required" on first login
- System SHALL support up to 10,000 users per import

**FR-AUTH-001.3:** Email Verification
- System SHALL send verification email with token (valid 24 hours)
- System SHALL activate user account upon verification
- System SHALL allow resending verification email (rate limited: 1 per 5 minutes)
- System SHALL log all verification attempts

**Input:**
- Email (required, valid format, unique within tenant)
- Password (required, meets policy)
- Full name (required, 2-255 characters)

**Output:**
- Success: User created, verification email sent
- Failure: Validation errors, duplicate email

**Acceptance Criteria:**
- [ ] User can register with valid credentials
- [ ] Weak passwords rejected
- [ ] Duplicate emails rejected within tenant (allowed across tenants)
- [ ] Email verification works
- [ ] Bulk import works for 1000+ users
- [ ] Temporary passwords meet security policy

---

#### 3.2.2 User Authentication (FR-AUTH-002)

**Priority:** Must Have  
**Risk:** Critical (security-critical)

**Description:**
Authenticate users using email/password with JWT token issuance.

**Functional Requirements:**

**FR-AUTH-002.1:** Login with Email/Password
- System SHALL authenticate users via email and password
- System SHALL verify password using Argon2id
- System SHALL check user account status (active, suspended, deleted)
- System SHALL check email verification status
- System SHALL enforce tenant isolation (cannot log in to wrong tenant)
- System SHALL issue JWT access token (valid 15 minutes)
- System SHALL issue refresh token (valid 7 days, stored in database)
- System SHALL create session record in database and Redis
- System SHALL log successful login
- System SHALL return user profile and permissions

**FR-AUTH-002.2:** Failed Login Handling
- System SHALL increment failed login counter
- System SHALL enforce rate limiting:
  - After 3 failed attempts: 1-minute lockout
  - After 5 failed attempts: 5-minute lockout
  - After 10 failed attempts: 1-hour lockout
- System SHALL log all failed login attempts (audit)
- System SHALL send email notification after 5 failed attempts
- System SHALL support admin account unlock

**FR-AUTH-002.3:** JWT Token Structure
- Access Token SHALL contain:
  - `sub`: User ID (UUID)
  - `email`: User email
  - `tenant_id`: Organization ID
  - `role`: User role(s)
  - `permissions`: User permissions (array)
  - `exp`: Expiration (15 minutes from issuance)
  - `iat`: Issued at
  - `jti`: Token ID (for revocation)
- Refresh Token SHALL be opaque UUID stored in database
- System SHALL sign tokens with RS256 (RSA + SHA-256)
- System SHALL rotate signing keys every 90 days

**FR-AUTH-002.4:** Token Refresh
- System SHALL provide refresh token endpoint
- System SHALL validate refresh token from database
- System SHALL check token expiration and revocation status
- System SHALL issue new access token (15 min)
- System SHALL rotate refresh token (new refresh token issued, old invalidated)
- System SHALL enforce maximum 1 refresh per minute per user

**Input:**
- Email (required)
- Password (required)

**Output:**
- Success: access_token, refresh_token, user profile, permissions
- Failure: Invalid credentials error, account locked error

**Acceptance Criteria:**
- [ ] User can log in with valid credentials
- [ ] Invalid credentials rejected
- [ ] Suspended accounts cannot log in
- [ ] Unverified emails cannot log in
- [ ] JWT contains correct claims
- [ ] Refresh token works
- [ ] Rate limiting enforced
- [ ] Failed attempts logged
- [ ] Tenant isolation enforced (cannot access other tenant data)

---

#### 3.2.3 Multi-Factor Authentication (FR-AUTH-003)

**Priority:** Should Have  
**Risk:** Medium

**Description:**
Support time-based one-time password (TOTP) multi-factor authentication.

**Functional Requirements:**

**FR-AUTH-003.1:** MFA Enrollment
- System SHALL allow users to enable MFA
- System SHALL generate TOTP secret (base32, 32 bytes)
- System SHALL display QR code (Google Authenticator compatible)
- System SHALL display secret key (for manual entry)
- System SHALL require user to enter verification code to confirm enrollment
- System SHALL store encrypted TOTP secret in database
- System SHALL generate backup codes (10 codes, 8 characters each)
- System SHALL mark user as "MFA enabled"

**FR-AUTH-003.2:** MFA Login Flow
- System SHALL detect MFA-enabled users after password verification
- System SHALL prompt for TOTP code (6 digits)
- System SHALL validate TOTP using 30-second window
- System SHALL accept codes from ±1 time window (prevent clock skew issues)
- System SHALL enforce rate limiting (5 attempts per 5 minutes)
- System SHALL log all MFA attempts
- System SHALL issue JWT only after successful MFA verification

**FR-AUTH-003.3:** Backup Codes
- System SHALL allow users to generate new backup codes
- System SHALL invalidate old backup codes when new ones generated
- System SHALL allow using backup code in place of TOTP
- System SHALL mark backup code as used (one-time use)
- System SHALL notify user when only 2 backup codes remain

**FR-AUTH-003.4:** MFA Enforcement
- System SHALL allow organization admin to enforce MFA for all users
- System SHALL allow grace period (7 days) for users to enroll
- System SHALL block login after grace period if MFA not enrolled
- System SHALL send email reminders during grace period

**Input:**
- TOTP code (6 digits) or backup code (8 characters)

**Output:**
- Success: JWT tokens issued
- Failure: Invalid code error

**Acceptance Criteria:**
- [ ] User can enroll MFA with QR code
- [ ] TOTP codes validate correctly
- [ ] Backup codes work
- [ ] MFA enforcement works at organization level
- [ ] Grace period enforced
- [ ] Clock skew tolerance works (±30 seconds)

---

### 3.3 Authorization and Access Control

#### 3.3.1 Role-Based Access Control (FR-AUTHZ-001)

**Priority:** Must Have  
**Risk:** Critical (security-critical)

**Description:**
Implement role-based access control with configurable roles and permissions.

**Functional Requirements:**

**FR-AUTHZ-001.1:** Default Roles
- System SHALL provide default roles:
  - **Organization Owner:** Full access to organization
  - **Organization Admin:** Manage users, elections, settings (cannot delete org)
  - **Election Manager:** Create/manage elections, candidates, publish results
  - **Election Officer:** Monitor elections, verify voters/candidates
  - **Voter:** Cast votes, view results
  - **Candidate:** Manage candidate profile, upload documents
  - **Auditor:** Read-only access to audit logs, blockchain verification
  - **Observer:** Read-only access to election monitoring
- System SHALL assign default permissions to each role
- System SHALL allow organization to customize role names and permissions

**FR-AUTHZ-001.2:** Permission Model
- System SHALL implement granular permissions:
  - `organization:read`, `organization:write`, `organization:delete`
  - `user:read`, `user:write`, `user:delete`, `user:impersonate`
  - `election:read`, `election:create`, `election:update`, `election:delete`, `election:publish`
  - `candidate:read`, `candidate:create`, `candidate:verify`, `candidate:reject`
  - `vote:cast`, `vote:read` (aggregate only, no individual votes)
  - `result:read`, `result:publish`
  - `audit:read`, `audit:export`
  - `settings:read`, `settings:write`
- System SHALL enforce permissions at API layer (every endpoint checks)
- System SHALL cache user permissions in Redis (5-minute TTL)
- System SHALL invalidate permission cache on role assignment change

**FR-AUTHZ-001.3:** Role Assignment
- System SHALL allow organization admin to assign roles to users
- System SHALL allow multiple roles per user
- System SHALL support role inheritance (future)
- System SHALL log all role assignment changes
- System SHALL prevent removing last owner role (must have at least 1 owner)

**FR-AUTHZ-001.4:** Permission Checking
- System SHALL check permissions on every API request
- System SHALL return 403 Forbidden for unauthorized requests
- System SHALL log unauthorized access attempts
- System SHALL support permission checks in code: `user.has_permission("election:create")`

**Input:**
- User ID
- Requested permission or action

**Output:**
- Boolean: Authorized / Not Authorized

**Acceptance Criteria:**
- [ ] Default roles created for new organization
- [ ] Role assignment works
- [ ] Permissions enforced at API level
- [ ] Unauthorized requests return 403
- [ ] Permission cache works
- [ ] Cannot remove last owner
- [ ] All permission changes logged

---

### 3.4 Election Management

#### 3.4.1 Election Creation (FR-ELEC-001)

**Priority:** Must Have  
**Risk:** Medium

**Description:**
Allow election managers to create elections using a guided wizard.

**Functional Requirements:**

**FR-ELEC-001.1:** Election Wizard (Step 1: Basic Information)
- System SHALL provide election creation wizard
- System SHALL collect:
  - Election title (required, 3-255 characters)
  - Description (optional, max 2000 characters)
  - Election type (required, from configurable list)
  - Start date/time (required, must be future)
  - End date/time (required, must be after start)
- System SHALL validate all inputs
- System SHALL calculate election duration
- System SHALL warn if duration < 1 hour or > 30 days
- System SHALL save draft election

**FR-ELEC-001.2:** Election Types (Configurable)
- System SHALL support configurable election types:
  - **Individual Candidate:** Vote for one candidate per position
  - **Post-Wise:** Multiple positions, vote for candidate(s) per position
  - **Panel Election:** Vote for a group/slate of candidates
  - **Ranked Choice:** Rank candidates by preference (future)
  - **Approval Voting:** Approve/disapprove multiple candidates (future)
- System SHALL store election type configuration as JSON metadata
- System SHALL allow organization to define custom election types

**FR-ELEC-001.3:** Election Wizard (Step 2: Positions)
- System SHALL allow defining positions (for post-wise elections)
- System SHALL collect per position:
  - Position name (required, e.g., "President", "Secretary")
  - Description (optional)
  - Number of seats (required, default 1)
  - Voting rule (e.g., "Vote for 1", "Vote for up to 3")
- System SHALL allow adding multiple positions
- System SHALL allow reordering positions (display order)
- System SHALL save position configuration

**FR-ELEC-001.4:** Election Wizard (Step 3: Eligibility Rules)
- System SHALL allow defining voter eligibility rules
- System SHALL support rule types:
  - **All Organization Members:** Every user can vote
  - **Attribute-Based:** Filter by user attributes (e.g., department, enrollment year)
  - **Custom List:** Upload CSV of eligible voter IDs
  - **Dynamic Rule:** Boolean expression (e.g., `department == "Engineering" AND tenure > 1`)
- System SHALL validate rule syntax
- System SHALL preview eligible voter count
- System SHALL save eligibility rules

**FR-ELEC-001.5:** Election Wizard (Step 4: Settings)
- System SHALL allow configuring:
  - Anonymous voting (hide voter identity from managers, default: true)
  - Public results (visible to all org members after close, default: true)
  - Allow candidate self-nomination (default: false)
  - Voter can verify vote (blockchain verification, default: true)
  - Require candidate verification (default: true)
- System SHALL validate settings
- System SHALL save settings

**FR-ELEC-001.6:** Election Wizard (Step 5: Review & Create)
- System SHALL display summary of all configuration
- System SHALL allow editing (return to previous steps)
- System SHALL validate entire election configuration
- System SHALL create election in "Draft" state
- System SHALL generate unique election ID (UUID)
- System SHALL log election creation
- System SHALL send notification to election manager

**Input:**
- Election configuration (title, type, dates, positions, eligibility, settings)

**Output:**
- Success: Election created in Draft state, election ID returned
- Failure: Validation errors

**Acceptance Criteria:**
- [ ] Election manager can complete wizard in <10 minutes
- [ ] All steps save draft progress
- [ ] Validation works on each step
- [ ] Election created in Draft state
- [ ] Eligibility rule preview shows correct count
- [ ] Election ID generated
- [ ] Audit log entry created

---

#### 3.4.2 Election Lifecycle State Management (FR-ELEC-002)

**Priority:** Must Have  
**Risk:** High (critical business logic)

**Description:**
Manage election through defined states with automatic and manual transitions.

**Functional Requirements:**

**FR-ELEC-002.1:** Election States
- System SHALL enforce election state machine:
  ```
  Draft → Review → Scheduled → Open → Closed → Verifying → Published → Archived
  ```
- System SHALL store current state in database
- System SHALL store state transition history (who, when, why)
- System SHALL display current state to users
- System SHALL allow only valid state transitions

**FR-ELEC-002.2:** State Transitions

**Draft → Review:**
- Triggered by: Election manager clicks "Submit for Review"
- Validation: Election configuration complete, at least 1 position, at least 1 eligibility rule
- Action: Notify organization admin/election officer
- Allowed Reverse: Admin can send back to Draft with comments

**Review → Scheduled:**
- Triggered by: Organization admin approves (or auto-approve if disabled)
- Validation: Review approval
- Action: Schedule election start job, notify election manager
- Allowed Reverse: Admin can send back to Draft

**Scheduled → Open:**
- Triggered by: Scheduled start time reached (automatic) OR manual start by election manager
- Validation: Start time reached OR manual override permission
- Action: Open voting, notify all eligible voters, activate election page
- Allowed Reverse: Emergency stop (admin only, requires justification)

**Open → Closed:**
- Triggered by: End time reached (automatic) OR manual close by election manager
- Validation: End time reached OR manual override permission
- Action: Stop accepting votes, lock ballots, notify election manager
- Allowed Reverse: Reopen (admin only, requires justification, logs warning)

**Closed → Verifying:**
- Triggered by: Automatic after close OR manual by election manager
- Validation: All votes confirmed on blockchain
- Action: Begin result calculation, verify blockchain confirmations
- Duration: Usually <1 minute (can be up to 5 minutes for large elections)

**Verifying → Published:**
- Triggered by: Verification complete AND election manager publishes results
- Validation: All blockchain confirmations verified, results calculated
- Action: Publish results to eligible viewers, notify all voters, generate audit report
- Allowed Reverse: Unpublish (admin only, rare, requires justification)

**Published → Archived:**
- Triggered by: Manual by organization admin OR automatic after retention period (7 years default)
- Validation: Election completed, results published
- Action: Mark as archived, remove from active election list, retain data for compliance

**FR-ELEC-002.3:** State Transition Validation
- System SHALL validate business rules before each transition
- System SHALL check user permissions for manual transitions
- System SHALL log all transition attempts (success and failure)
- System SHALL send notifications on state changes
- System SHALL prevent invalid transitions (return error)

**FR-ELEC-002.4:** Scheduled State Transitions
- System SHALL use background job scheduler for automatic transitions
- System SHALL trigger "Scheduled → Open" at election start time (±30 seconds)
- System SHALL trigger "Open → Closed" at election end time (±30 seconds)
- System SHALL handle timezone conversions correctly
- System SHALL retry failed transitions (up to 3 attempts)

**Input:**
- Target state
- User ID (for manual transitions)
- Optional: reason/justification (for reversals)

**Output:**
- Success: State updated, transition logged, notifications sent
- Failure: Invalid transition error, validation error

**Acceptance Criteria:**
- [ ] State machine enforces valid transitions
- [ ] Invalid transitions rejected
- [ ] Scheduled transitions fire on time (±30 sec)
- [ ] All transitions logged with actor and timestamp
- [ ] Notifications sent on state change
- [ ] State history tracked
- [ ] Emergency stop works (admin only)
- [ ] Reopen requires justification and logs warning

---

#### 3.4.3 Candidate Management (FR-CAND-001)

**Priority:** Must Have  
**Risk:** Medium

**Description:**
Manage candidate registration, verification, and profile data.

**Functional Requirements:**

**FR-CAND-001.1:** Candidate Registration
- System SHALL allow candidate registration for elections
- System SHALL support two registration modes:
  - **Admin-Nominated:** Election manager adds candidate
  - **Self-Nominated:** Candidate applies (if enabled in election settings)
- System SHALL collect:
  - Full name (required)
  - Position (required, from election positions)
  - Biography (optional, max 500 words)
  - Photo (optional, max 2MB, JPG/PNG)
  - Supporting documents (optional, e.g., nomination form, citizenship proof)
- System SHALL validate all inputs
- System SHALL check duplicate candidate per position (same person cannot run twice for same position)
- System SHALL create candidate record in "Pending Verification" state (if verification required)
- System SHALL notify election manager of new candidate

**FR-CAND-001.2:** Candidate Verification Workflow
- System SHALL enforce verification workflow (if enabled):
  - Candidate uploads required documents
  - Election officer reviews documents
  - Election officer approves or rejects with reason
  - Candidate notified of decision
- System SHALL track verification status: Pending, Approved, Rejected
- System SHALL allow election officer to request additional documents
- System SHALL send email notifications at each step
- System SHALL only allow approved candidates to appear on ballot
- System SHALL deadline for candidate registration (configurable, default 24 hours before election start)

**FR-CAND-001.3:** Candidate Profile
- System SHALL provide candidate profile page (public to eligible voters)
- System SHALL display:
  - Candidate photo
  - Full name
  - Position
  - Biography
  - Uploaded documents (public or private, configurable)
- System SHALL allow candidate to edit profile (before verification)
- System SHALL lock profile after verification (prevent tampering)
- System SHALL track profile edit history (audit log)

**FR-CAND-001.4:** Candidate List
- System SHALL provide candidate list page per election
- System SHALL group candidates by position (for post-wise elections)
- System SHALL display verification status (for election managers)
- System SHALL allow filtering by position, status
- System SHALL allow searching by name
- System SHALL show candidate count per position

**Input:**
- Candidate registration data (name, position, bio, photo, documents)
- Verification decision (approve/reject with reason)

**Output:**
- Success: Candidate registered, verification request created
- Failure: Validation errors, duplicate candidate error

**Acceptance Criteria:**
- [ ] Candidate can register (self-nomination if enabled)
- [ ] Election manager can add candidate
- [ ] Document upload works (PDF, JPG, PNG)
- [ ] Verification workflow works
- [ ] Approved candidates appear on ballot
- [ ] Rejected candidates notified with reason
- [ ] Candidate profile page displays correctly
- [ ] Candidate list shows all candidates by position
- [ ] Registration deadline enforced
- [ ] Profile locked after verification

---

### 3.5 Voting

#### 3.5.1 Vote Casting (FR-VOTE-001)

**Priority:** Must Have  
**Risk:** Critical (security-critical, business-critical)

**Description:**
Allow eligible voters to cast encrypted votes with blockchain commitment.

**Functional Requirements:**

**FR-VOTE-001.1:** Voter Eligibility Check
- System SHALL verify voter eligibility before showing ballot
- System SHALL evaluate eligibility rules defined for election
- System SHALL check voter has not already voted
- System SHALL check election is in "Open" state
- System SHALL display error message if ineligible
- System SHALL log eligibility check (pass/fail)

**FR-VOTE-001.2:** Ballot Display
- System SHALL display ballot with all positions and candidates
- System SHALL group by position (for post-wise elections)
- System SHALL display candidate photo, name, biography
- System SHALL enforce voting rules (e.g., "Vote for 1", "Vote for up to 3")
- System SHALL provide clear UI for selection
- System SHALL show vote count remaining (e.g., "Select 1 more")
- System SHALL allow preview before final submission

**FR-VOTE-001.3:** Vote Encryption
- System SHALL encrypt ballot before storage:
  1. Generate election-specific public key (X25519)
  2. Voter selects candidates
  3. Create ballot JSON: `{"election_id": "...", "votes": [{"position_id": "...", "candidate_id": "..."}]}`
  4. Encrypt ballot using X25519 + AES-256-GCM
  5. Generate vote commitment hash (SHA-256 of encrypted ballot)
- System SHALL NOT store plaintext votes
- System SHALL store encrypted ballot in PostgreSQL
- System SHALL associate ballot with voter via double-hashed ID (anonymity)

**FR-VOTE-001.4:** Blockchain Commitment Submission
- System SHALL submit vote commitment to Solana blockchain:
  1. Create vote commitment payload: `{election_id, commitment_hash, timestamp, signature}`
  2. Sign payload with election authority key (Ed25519)
  3. Submit transaction to Solana via Anchor program
  4. Wait for confirmation (Solana finality ~400ms)
  5. Store transaction ID in PostgreSQL
- System SHALL retry failed submissions (up to 3 attempts)
- System SHALL not mark vote as "cast" until blockchain confirmation
- System SHALL handle blockchain errors gracefully (display user-friendly message)

**FR-VOTE-001.5:** Vote Confirmation
- System SHALL mark voter as "participated" in election (prevent double voting)
- System SHALL generate verification code (includes blockchain TX ID)
- System SHALL display confirmation page with:
  - "Your vote has been recorded"
  - Verification code (e.g., `VOTE-A1B2C3-TXID...`)
  - Blockchain transaction ID (link to Solana explorer)
  - Timestamp
- System SHALL send confirmation email with verification code
- System SHALL log vote casting event (audit log)

**FR-VOTE-001.6:** Vote Casting Time Limit
- System SHALL enforce time limit: vote must be cast while election is "Open"
- System SHALL display countdown timer on ballot page
- System SHALL auto-save partial ballot (draft) every 30 seconds (in session, not database)
- System SHALL warn if election closing soon (<5 minutes remaining)
- System SHALL reject vote submission if election closed (even if ballot loaded while open)

**Input:**
- Voter ID (from JWT)
- Election ID
- Selected candidates (one or more, depending on election rules)

**Output:**
- Success: Vote recorded, verification code, blockchain TX ID
- Failure: Eligibility error, encryption error, blockchain error, election closed error

**Acceptance Criteria:**
- [ ] Eligible voter can cast vote
- [ ] Ineligible voter blocked with clear message
- [ ] Voter cannot vote twice (idempotency check)
- [ ] Ballot encrypted before storage
- [ ] Vote commitment submitted to Solana
- [ ] Blockchain confirmation received within 5 seconds (p95)
- [ ] Verification code generated and displayed
- [ ] Confirmation email sent
- [ ] Vote casting takes <2 seconds total (p95)
- [ ] Voter cannot see other voters' votes
- [ ] Election manager cannot see individual votes (anonymity preserved)
- [ ] Auto-save works for partially filled ballots
- [ ] Countdown timer displays correctly
- [ ] Vote rejected if election closed

---

#### 3.5.2 Vote Verification (FR-VOTE-002)

**Priority:** Must Have  
**Risk:** Medium

**Description:**
Allow voters to verify their vote was recorded correctly using blockchain.

**Functional Requirements:**

**FR-VOTE-002.1:** Verification Code Lookup
- System SHALL provide verification page (public, no login required)
- System SHALL allow entering verification code
- System SHALL validate verification code format
- System SHALL retrieve vote record from database
- System SHALL retrieve blockchain transaction from Solana
- System SHALL display verification result

**FR-VOTE-002.2:** Verification Display
- System SHALL display:
  - Election title
  - Vote timestamp (when cast)
  - Blockchain transaction ID
  - Blockchain confirmation status (confirmed, pending, failed)
  - Blockchain explorer link (Solana Explorer)
  - Vote commitment hash
  - Message: "Your vote was successfully recorded and is immutable on the blockchain"
- System SHALL NOT display actual vote choices (preserves anonymity)

**FR-VOTE-002.3:** Blockchain Verification
- System SHALL verify vote commitment on blockchain:
  1. Query Solana for transaction ID
  2. Retrieve transaction data from blockchain
  3. Verify commitment hash matches database record
  4. Verify transaction signature
  5. Verify transaction finalized
- System SHALL display verification status: Verified ✓, Pending ⏳, Failed ✗
- System SHALL show error message if verification fails (e.g., "Transaction not found")

**FR-VOTE-002.4:** Independent Verification Tool
- System SHALL provide public verification API endpoint (no auth required)
- External auditors SHALL be able to verify votes using API
- API SHALL accept: verification code or transaction ID
- API SHALL return: verification status, commitment hash, timestamp
- System SHALL rate-limit public API (100 requests per minute per IP)

**Input:**
- Verification code (format: `VOTE-XXXXXX-TXID...`)

**Output:**
- Success: Verification details, blockchain confirmation status
- Failure: Invalid code error, transaction not found error

**Acceptance Criteria:**
- [ ] Voter can verify vote using verification code
- [ ] Blockchain transaction found and displayed
- [ ] Commitment hash matches database
- [ ] Verification works without login (public)
- [ ] Solana Explorer link works
- [ ] Independent API works for auditors
- [ ] Rate limiting enforced on public endpoint
- [ ] Vote choices NOT displayed (anonymity preserved)
- [ ] Verification page loads in <1 second

---

### 3.6 Result Management

#### 3.6.1 Result Calculation (FR-RESULT-001)

**Priority:** Must Have  
**Risk:** High (business-critical)

**Description:**
Calculate election results accurately and transparently.

**Functional Requirements:**

**FR-RESULT-001.1:** Result Calculation Trigger
- System SHALL trigger result calculation when election moves to "Verifying" state
- System SHALL run calculation as background job
- System SHALL verify all votes confirmed on blockchain before calculating
- System SHALL lock ballots (prevent any modifications)
- System SHALL log calculation start time

**FR-RESULT-001.2:** Vote Decryption
- System SHALL decrypt encrypted ballots:
  1. Retrieve election private key from secure storage (encrypted at rest)
  2. Decrypt each ballot using X25519 + AES-256-GCM
  3. Parse ballot JSON
  4. Validate ballot structure
  5. Extract vote choices
- System SHALL handle decryption errors (skip invalid ballots, log error)
- System SHALL never store decrypted ballots (decrypt on-the-fly)

**FR-RESULT-001.3:** Vote Aggregation
- System SHALL aggregate votes by position and candidate:
  - Count votes per candidate per position
  - Calculate vote percentage per candidate
  - Calculate total votes cast
  - Calculate voter turnout (voted / eligible)
- System SHALL support different counting methods (future):
  - First-Past-The-Post (FPTP) - highest vote count wins
  - Ranked Choice Voting (RCV) - instant runoff
  - Approval Voting - most approvals win
- System SHALL store aggregated results in database

**FR-RESULT-001.4:** Winner Determination
- System SHALL determine winner(s) based on election rules:
  - For single-seat position: candidate with most votes
  - For multi-seat position: top N candidates by vote count
  - Handle ties (system SHALL flag ties for manual resolution)
- System SHALL mark winners in results table
- System SHALL log winner determination logic

**FR-RESULT-001.5:** Result Verification
- System SHALL verify result integrity:
  - Total votes counted == total votes cast
  - All blockchain commitments verified
  - No duplicate votes counted
  - Decryption success rate > 99% (log failures)
- System SHALL generate result report with:
  - Total eligible voters
  - Total votes cast
  - Turnout percentage
  - Votes per candidate per position
  - Winners
  - Timestamp
  - Blockchain verification summary
- System SHALL store result report (immutable, signed with election authority key)

**Input:**
- Election ID

**Output:**
- Success: Results calculated, winners determined, report generated
- Failure: Verification error, decryption error

**Acceptance Criteria:**
- [ ] Results calculated correctly
- [ ] Vote counts match expected totals
- [ ] Winners determined correctly
- [ ] Ties flagged for manual resolution
- [ ] Results verified against blockchain
- [ ] Result report generated
- [ ] Calculation completes in <5 seconds for 10K votes (p95)
- [ ] Decryption errors logged and handled gracefully
- [ ] Result report signed with election authority key

---

#### 3.6.2 Result Publication (FR-RESULT-002)

**Priority:** Must Have  
**Risk:** Medium

**Description:**
Publish election results to authorized viewers.

**Functional Requirements:**

**FR-RESULT-002.1:** Result Publication Trigger
- System SHALL allow election manager to publish results manually
- System SHALL require election in "Verifying" state (results calculated)
- System SHALL verify results calculated successfully
- System SHALL transition election to "Published" state
- System SHALL log publication event (who published, when)

**FR-RESULT-002.2:** Result Display
- System SHALL display results page with:
  - Election title and description
  - Election dates (start, end)
  - Total eligible voters
  - Total votes cast
  - Turnout percentage
  - Results per position:
    - Candidate name, photo
    - Vote count
    - Vote percentage
    - Winner indicator (badge, highlight)
  - Blockchain verification status
  - Result publication timestamp
- System SHALL order candidates by vote count (descending)
- System SHALL highlight winners
- System SHALL display "Tie" indicator if applicable

**FR-RESULT-002.3:** Access Control
- System SHALL enforce result access control:
  - If "Public Results" enabled: visible to all organization members
  - If "Public Results" disabled: visible only to election managers, organization admins, auditors
  - If election in "Closed" or "Verifying" state: results not visible (only to election managers)
- System SHALL return 403 Forbidden if unauthorized access attempt
- System SHALL log all result access attempts

**FR-RESULT-002.4:** Result Export
- System SHALL allow exporting results in multiple formats:
  - CSV: candidate_name, position, vote_count, vote_percentage, winner (yes/no)
  - JSON: structured result object
  - PDF: formatted result report with organization branding
- System SHALL include blockchain verification details in export
- System SHALL sign PDF with election authority digital signature
- System SHALL log all exports (who exported, when, format)

**FR-RESULT-002.5:** Result Notifications
- System SHALL send result publication notification:
  - Email to all voters (if public results)
  - Email to election manager, organization admin
  - Email to candidates (with their individual results)
- System SHALL include result summary in email
- System SHALL include link to full results page

**Input:**
- Election ID
- User ID (for access control check)

**Output:**
- Success: Results displayed, PDF/CSV/JSON export
- Failure: Unauthorized error, results not ready error

**Acceptance Criteria:**
- [ ] Results displayed correctly on results page
- [ ] Winners highlighted
- [ ] Ties indicated
- [ ] Access control enforced (public/private results)
- [ ] CSV export works
- [ ] JSON export works
- [ ] PDF export works with branding
- [ ] PDF digitally signed
- [ ] Result notifications sent
- [ ] All result access logged
- [ ] Results page loads in <1 second

---

### 3.7 Audit and Compliance

#### 3.7.1 Audit Logging (FR-AUDIT-001)

**Priority:** Must Have  
**Risk:** Critical (compliance-critical)

**Description:**
Maintain immutable audit trail of all system operations.

**Functional Requirements:**

**FR-AUDIT-001.1:** Audit Events
- System SHALL log all security-sensitive events:
  - User authentication (login, logout, failed login)
  - Authorization failures (403 Forbidden)
  - User management (create, update, delete, role assignment)
  - Organization management (create, update settings)
  - Election lifecycle (create, state transitions, publish results)
  - Candidate management (register, verify, reject)
  - Vote casting (successful, failed, eligibility check)
  - Blockchain submissions (success, failure, retry)
  - Configuration changes (election rules, eligibility, settings)
  - Data exports (results, audit logs)
  - Admin actions (impersonation, emergency stop, reopen election)

**FR-AUDIT-001.2:** Audit Log Structure
- System SHALL store audit logs with:
  - `id`: UUID (unique log entry ID)
  - `timestamp`: DateTime (UTC, ISO 8601)
  - `tenant_id`: UUID (organization ID, for multi-tenancy)
  - `user_id`: UUID (actor who performed action, NULL for system)
  - `action`: String (e.g., "vote:cast", "election:create", "user:login")
  - `entity_type`: String (e.g., "election", "vote", "user")
  - `entity_id`: UUID (affected entity, e.g., election ID)
  - `details`: JSONB (additional context, e.g., old/new values)
  - `ip_address`: String (client IP)
  - `user_agent`: String (client browser)
  - `correlation_id`: UUID (trace requests across services)
  - `result`: Enum (success, failure, error)
  - `error_message`: String (if result = failure/error)
- System SHALL store logs in PostgreSQL (append-only table, no updates/deletes)
- System SHALL partition audit logs by month (for performance)

**FR-AUDIT-001.3:** Audit Log Retention
- System SHALL retain audit logs indefinitely (compliance requirement)
- System SHALL archive old logs (>1 year) to object storage (S3)
- System SHALL support querying archived logs (slower, but accessible)
- System SHALL never delete audit logs (immutable)

**FR-AUDIT-001.4:** Audit Log Access
- System SHALL allow authorized users to query audit logs:
  - Organization Admin: audit logs for their organization only
  - Platform Super Admin: audit logs for all organizations
  - Auditor role: read-only access to audit logs for their organization
- System SHALL provide audit log query interface:
  - Filter by date range, user, action, entity type, entity ID
  - Search by correlation ID (trace request flow)
  - Export to CSV, JSON
- System SHALL enforce access control (cannot access other tenant logs)
- System SHALL log all audit log access (meta-audit)

**Input:**
- Audit event data (action, entity, user, etc.)

**Output:**
- Success: Audit log entry created
- Failure: Database error (should never happen, but log failure to stderr)

**Acceptance Criteria:**
- [ ] All security-sensitive events logged
- [ ] Audit logs immutable (no updates/deletes)
- [ ] Audit logs include correlation ID for tracing
- [ ] Audit logs queryable with filters
- [ ] Export to CSV/JSON works
- [ ] Access control enforced (tenant isolation)
- [ ] Audit log access itself logged (meta-audit)
- [ ] Old logs archived to S3 after 1 year
- [ ] Archived logs queryable (slower but accessible)
- [ ] Partition by month for performance

---

### 3.8 Analytics and Reporting

#### 3.8.1 Election Analytics (FR-ANALYTICS-001)

**Priority:** Should Have  
**Risk:** Low

**Description:**
Provide real-time analytics dashboards for election managers.

**Functional Requirements:**

**FR-ANALYTICS-001.1:** Real-Time Voting Dashboard
- System SHALL provide real-time dashboard during election:
  - Total votes cast (live count)
  - Voter turnout percentage
  - Votes per hour (bar chart)
  - Geographic distribution (if location data available)
  - Device breakdown (desktop, mobile, tablet)
  - Browser breakdown (Chrome, Firefox, Safari, etc.)
- System SHALL update dashboard every 30 seconds (WebSocket or polling)
- System SHALL NOT display vote choices (only aggregate counts)
- System SHALL enforce access control (election managers, organization admins only)

**FR-ANALYTICS-001.2:** Historical Analytics
- System SHALL provide historical analytics after election:
  - Turnout trends over time
  - Comparison with past elections (same organization)
  - Voter participation by demographic (if data available and permitted)
  - Peak voting times
  - Average time to vote
- System SHALL store aggregated analytics data (no individual voter data)
- System SHALL allow date range filtering

**FR-ANALYTICS-001.3:** Organization-Level Analytics
- System SHALL provide organization dashboard:
  - Total elections conducted
  - Total votes cast (all time)
  - Average voter turnout
  - Active elections (currently open)
  - Upcoming elections (scheduled)
  - Past elections (completed, archived)
- System SHALL display trends over time (line charts)
- System SHALL allow exporting analytics data (CSV, JSON)

**Input:**
- Election ID (for election-specific analytics)
- Organization ID (for org-level analytics)
- Date range (optional)

**Output:**
- Analytics data (JSON), charts, dashboards

**Acceptance Criteria:**
- [ ] Real-time dashboard updates every 30 seconds
- [ ] Turnout percentage displayed correctly
- [ ] Votes per hour chart displayed
- [ ] Historical analytics available after election
- [ ] Organization dashboard shows aggregate data
- [ ] Export to CSV/JSON works
- [ ] Access control enforced
- [ ] No individual voter data exposed

---

## 4. External Interface Requirements

### 4.1 User Interfaces

#### 4.1.1 Web Application (UI-001)

**Priority:** Must Have

**Requirements:**

- System SHALL provide responsive web interface (mobile-first)
- System SHALL support screen resolutions from 320px to 3840px width
- System SHALL be accessible (WCAG 2.1 AA compliance)
- System SHALL support keyboard navigation (all features accessible via keyboard)
- System SHALL support screen readers (JAWS, NVDA, VoiceOver)
- System SHALL support dark mode and light mode
- System SHALL use organization branding (logo, colors)
- System SHALL display loading states for async operations
- System SHALL display clear error messages for validation failures
- System SHALL use consistent UI components (design system)

**Page Requirements:**

| Page | URL | Access Level |
|------|-----|--------------|
| Landing Page | `/` | Public |
| Login | `/auth/login` | Public |
| Register | `/auth/register` | Public |
| MFA Setup | `/auth/mfa` | Authenticated |
| Dashboard | `/dashboard` | Authenticated |
| Elections List | `/elections` | Authenticated |
| Election Details | `/elections/:id` | Authenticated |
| Create Election Wizard | `/elections/create` | Election Manager+ |
| Vote | `/elections/:id/vote` | Eligible Voter |
| Vote Verification | `/verify` | Public |
| Results | `/elections/:id/results` | Conditional (public/private) |
| Audit Logs | `/audit` | Auditor+ |
| Organization Settings | `/settings/organization` | Organization Admin+ |
| User Management | `/settings/users` | Organization Admin+ |

---

### 4.2 Hardware Interfaces

#### 4.2.1 Solana Blockchain (HW-001)

**Priority:** Must Have

**Requirements:**

- System SHALL interface with Solana blockchain via RPC endpoint
- System SHALL support mainnet and devnet environments
- System SHALL submit vote commitments as Solana transactions
- System SHALL use Anchor framework for smart contract interaction
- System SHALL handle transaction confirmation (finality ~400ms)
- System SHALL retry failed transactions (up to 3 attempts)
- System SHALL monitor Solana network status (uptime, TPS)
- System SHALL gracefully handle Solana outages (queue transactions, retry later)

**Interface:**
- Solana RPC endpoint (HTTPS)
- Anchor program deployed on Solana
- Transaction signing with Ed25519

---

### 4.3 Software Interfaces

#### 4.3.1 PostgreSQL Database (SW-001)

**Priority:** Must Have

**Requirements:**

- System SHALL use PostgreSQL 16+ as primary data store
- System SHALL use connection pooling (minimum 10 connections, maximum 100)
- System SHALL use prepared statements (prevent SQL injection)
- System SHALL use transactions for multi-step operations
- System SHALL enforce Row-Level Security (RLS) for multi-tenancy
- System SHALL set session variable `app.current_tenant_id` for all queries
- System SHALL use JSONB for flexible schema fields
- System SHALL use UUIDs for primary keys
- System SHALL use timestamps with timezone (UTC)

**Interface:**
- PostgreSQL wire protocol (port 5432)
- SQLx library (Rust)

---

#### 4.3.2 Redis Cache (SW-002)

**Priority:** Must Have

**Requirements:**

- System SHALL use Redis 7+ for caching and session storage
- System SHALL cache: active sessions, user permissions, hot election data
- System SHALL use appropriate TTL (Time To Live) for each cache type
- System SHALL handle cache misses gracefully (fallback to database)
- System SHALL invalidate cache on data updates
- System SHALL use Redis pub/sub for real-time updates (future)

**Interface:**
- Redis protocol (port 6379)
- Redis Rust client library

---

#### 4.3.3 Object Storage (SW-003)

**Priority:** Must Have

**Requirements:**

- System SHALL use S3-compatible object storage
- System SHALL store: candidate photos, documents, organization logos, generated reports
- System SHALL use pre-signed URLs for temporary access
- System SHALL enforce access control (tenant isolation)
- System SHALL enable encryption at rest (S3-managed keys)
- System SHALL enable versioning (for compliance)
- System SHALL organize objects by tenant: `s3://bucket/org-{tenant_id}/...`

**Interface:**
- S3 API (HTTPS)
- AWS S3 SDK or MinIO client

---

#### 4.3.4 Email Service (SW-004)

**Priority:** Must Have

**Requirements:**

- System SHALL send transactional emails via SMTP
- System SHALL support SMTP services: SendGrid, AWS SES, Mailgun, or custom SMTP
- System SHALL use email templates (HTML + plain text)
- System SHALL track email delivery status (sent, delivered, bounced, failed)
- System SHALL retry failed email sends (up to 3 attempts)
- System SHALL include organization branding in emails

**Email Types:**
- Email verification
- Password reset
- MFA enrollment
- Election notifications (created, started, closing soon, closed, results published)
- Vote confirmation
- Candidate verification status

**Interface:**
- SMTP protocol (port 587 or 465)
- Email template engine

---

#### 4.3.5 SMS Gateway (SW-005)

**Priority:** Should Have

**Requirements:**

- System SHALL send SMS for MFA one-time passwords (OTP)
- System SHALL support SMS providers: Twilio, AWS SNS, or similar
- System SHALL send OTP SMS within 10 seconds
- System SHALL rate-limit SMS sends (prevent abuse)
- System SHALL log all SMS sends (audit)

**Interface:**
- REST API (HTTPS)
- SMS provider SDK

---

### 4.4 Communication Interfaces

#### 4.4.1 RESTful API (COMM-001)

**Priority:** Must Have

**Requirements:**

- System SHALL expose RESTful API for all operations
- System SHALL use HTTPS (TLS 1.3) for all communication
- System SHALL use JSON for request and response payloads
- System SHALL use HTTP methods: GET (read), POST (create), PUT/PATCH (update), DELETE (delete)
- System SHALL use HTTP status codes correctly:
  - 200 OK (success)
  - 201 Created (resource created)
  - 204 No Content (success, no body)
  - 400 Bad Request (validation error)
  - 401 Unauthorized (not authenticated)
  - 403 Forbidden (not authorized)
  - 404 Not Found (resource not found)
  - 409 Conflict (duplicate, conflict)
  - 429 Too Many Requests (rate limit exceeded)
  - 500 Internal Server Error (server error)
- System SHALL include correlation ID in response headers (`X-Correlation-ID`)
- System SHALL version API (`/api/v1/...`)
- System SHALL support pagination for list endpoints (offset/limit or cursor-based)
- System SHALL support filtering and sorting on list endpoints
- System SHALL document API using OpenAPI 3.0 specification

**Authentication:**
- System SHALL require JWT in `Authorization: Bearer <token>` header
- Public endpoints (login, register, vote verification) do not require JWT

**Rate Limiting:**
- System SHALL enforce rate limits:
  - Authenticated users: 1000 requests per 15 minutes
  - Public endpoints: 100 requests per 15 minutes per IP
  - Login endpoint: 10 attempts per 15 minutes per IP
- System SHALL return `429 Too Many Requests` when limit exceeded
- System SHALL include rate limit headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`

---

## 5. System Requirements

### 5.1 Performance Requirements

| Requirement ID | Metric | Target | Measurement |
|----------------|--------|--------|-------------|
| **PERF-001** | API Response Time (p95) | <200ms | Application metrics |
| **PERF-002** | API Response Time (p99) | <500ms | Application metrics |
| **PERF-003** | Vote Casting Latency (end-to-end) | <2s | User-facing timer |
| **PERF-004** | Blockchain Confirmation Time | <5s | Transaction finality |
| **PERF-005** | Database Query Time (p95) | <50ms | PostgreSQL slow query log |
| **PERF-006** | Page Load Time (p95) | <3s | Browser metrics |
| **PERF-007** | Concurrent Voters (peak) | 10,000+ | Load testing |
| **PERF-008** | Votes per Second (sustained) | 1,000+ | Load testing |
| **PERF-009** | Database Connection Pool | 10-100 connections | Monitoring |
| **PERF-010** | Redis Cache Hit Rate | >80% | Redis metrics |

---

### 5.2 Safety Requirements

| Requirement ID | Requirement | Priority |
|----------------|-------------|----------|
| **SAFE-001** | System SHALL never delete audit logs | Must Have |
| **SAFE-002** | System SHALL never delete cast votes (encrypted ballots) | Must Have |
| **SAFE-003** | System SHALL never expose plaintext votes to election managers | Must Have |
| **SAFE-004** | System SHALL prevent double voting (idempotency checks) | Must Have |
| **SAFE-005** | System SHALL prevent vote tampering (immutable storage) | Must Have |
| **SAFE-006** | System SHALL backup database daily (automated) | Must Have |
| **SAFE-007** | System SHALL replicate blockchain commitments (Solana inherent) | Must Have |
| **SAFE-008** | System SHALL gracefully handle database failures (retry, fallback) | Must Have |
| **SAFE-009** | System SHALL gracefully handle blockchain failures (queue, retry) | Must Have |
| **SAFE-010** | System SHALL validate all inputs (prevent injection attacks) | Must Have |

---

### 5.3 Security Requirements

**See [Security Architecture](../security/01-security-architecture.md) for detailed requirements.**

Summary:
- Argon2id password hashing
- JWT access tokens (15 min) + refresh tokens (7 days)
- MFA support (TOTP)
- End-to-end ballot encryption (X25519 + AES-256-GCM)
- Blockchain vote commitments (Ed25519 signatures)
- Row-Level Security (PostgreSQL RLS)
- Rate limiting on all endpoints
- Audit logging (immutable)
- TLS 1.3 for all communication
- HTTPS only (no HTTP)
- CSRF protection
- XSS protection (sanitize inputs, escape outputs)
- SQL injection protection (prepared statements)

---

### 5.4 Reliability Requirements

| Requirement ID | Requirement | Target | Priority |
|----------------|-------------|--------|----------|
| **REL-001** | System Uptime | 99.9% (43.8 min downtime/month) | Must Have |
| **REL-002** | Database Uptime | 99.95% | Must Have |
| **REL-003** | Mean Time Between Failures (MTBF) | >720 hours (30 days) | Should Have |
| **REL-004** | Mean Time To Recovery (MTTR) | <1 hour | Must Have |
| **REL-005** | Backup Success Rate | 100% (daily backups) | Must Have |
| **REL-006** | Data Loss Tolerance (RPO) | <5 minutes | Must Have |
| **REL-007** | Recovery Time Objective (RTO) | <1 hour | Must Have |
| **REL-008** | Transaction Retry Success Rate | >99% (after 3 attempts) | Must Have |

---

### 5.5 Maintainability Requirements

| Requirement ID | Requirement | Priority |
|----------------|-------------|----------|
| **MAINT-001** | Code SHALL follow Rust best practices and idioms | Must Have |
| **MAINT-002** | Code SHALL be modular (bounded contexts, services) | Must Have |
| **MAINT-003** | Code SHALL include unit tests (>80% coverage) | Must Have |
| **MAINT-004** | Code SHALL include integration tests | Must Have |
| **MAINT-005** | Code SHALL be documented (doc comments) | Should Have |
| **MAINT-006** | API SHALL be documented (OpenAPI 3.0) | Must Have |
| **MAINT-007** | Database migrations SHALL be versioned and reversible | Must Have |
| **MAINT-008** | System SHALL log structured logs (JSON format) | Must Have |
| **MAINT-009** | System SHALL emit metrics (Prometheus format) | Should Have |
| **MAINT-010** | System SHALL support distributed tracing (OpenTelemetry) | Should Have |

---

## 6. Data Requirements

### 6.1 Logical Data Model

See [Database Schema Document](../design/01-database-schema.md) for full details.

**Core Entities:**
- Organization
- User
- Role, Permission
- Session, RefreshToken
- Election
- Position
- EligibilityRule
- Candidate
- Ballot (encrypted vote)
- VoteCommitment (blockchain record)
- VoterParticipation
- ElectionResult
- AuditLog

**Relationships:**
- Organization 1:N Users
- User N:M Roles
- Organization 1:N Elections
- Election 1:N Positions
- Election 1:N Candidates
- Election 1:N Ballots
- Election 1:N VoteCommitments
- Election 1:N VoterParticipation

---

### 6.2 Data Dictionary

See [Data Dictionary Document](../design/02-data-dictionary.md) for full details.

**Key Fields:**
- All tables include `id` (UUID primary key)
- All tables include `tenant_id` (for multi-tenancy) except audit logs
- All tables include `created_at`, `updated_at` timestamps (UTC)
- Sensitive fields encrypted at rest (passwords, encryption keys, MFA secrets)

---

### 6.3 Data Integrity Requirements

| Requirement ID | Requirement | Priority |
|----------------|-------------|----------|
| **DATA-001** | Database SHALL enforce foreign key constraints | Must Have |
| **DATA-002** | Database SHALL enforce unique constraints (no duplicates) | Must Have |
| **DATA-003** | Database SHALL enforce NOT NULL constraints | Must Have |
| **DATA-004** | Database SHALL use transactions for multi-step operations | Must Have |
| **DATA-005** | System SHALL validate data before database insertion | Must Have |
| **DATA-006** | System SHALL sanitize inputs (prevent XSS, SQL injection) | Must Have |
| **DATA-007** | System SHALL enforce referential integrity (cascading deletes where appropriate) | Must Have |
| **DATA-008** | System SHALL use optimistic locking (prevent concurrent update conflicts) | Should Have |

---

## 7. Constraints

### 7.1 Regulatory Constraints

- **GDPR Compliance:** Must support data subject rights (access, rectification, erasure, portability)
- **CCPA Compliance:** Must support California consumer rights
- **WCAG 2.1 AA:** Must achieve accessibility compliance
- **Data Residency:** Must support data storage in specific geographic regions (future)

### 7.2 Technical Constraints

- **Backend Language:** Rust (architectural decision, no alternatives)
- **Database:** PostgreSQL (architectural decision)
- **Blockchain:** Solana (architectural decision)
- **TLS Version:** Minimum TLS 1.3
- **Password Hashing:** Argon2id (OWASP recommendation)
- **JWT Algorithm:** RS256 (RSA + SHA-256)

### 7.3 Business Constraints

- **B2B Only:** No government elections in Phase 1
- **SaaS Only:** No on-premise deployments in Phase 1
- **English Only:** Phase 1 supports English language only

---

## 8. Assumptions and Dependencies

### 8.1 Assumptions

- Users have reliable internet (minimum 1 Mbps)
- Email delivery is reliable (99%+ delivery rate)
- Solana blockchain maintains high uptime (99.9%+)
- Organizations provide accurate voter lists
- Voters have access to modern web browsers
- Organizations comply with their own election rules (system enforces configured rules, not legal compliance)

### 8.2 Dependencies

- PostgreSQL 16+ availability
- Redis 7+ availability
- Solana blockchain availability
- SMTP email service availability
- S3-compatible object storage availability
- Internet connectivity for all users
- TLS certificate for HTTPS

---

## 9. Acceptance Criteria

### 9.1 Feature Acceptance

Each feature SHALL be considered complete when:
- All functional requirements implemented
- Unit tests written and passing (>80% coverage)
- Integration tests written and passing
- API documented (OpenAPI)
- Code reviewed by senior engineer
- Security review completed (for security-critical features)
- User acceptance testing (UAT) passed
- Documentation updated

### 9.2 System Acceptance

The system SHALL be ready for production when:
- All "Must Have" features complete
- Performance targets met (see Section 5.1)
- Security audit passed (no high/critical vulnerabilities)
- Load testing passed (10K concurrent voters)
- Disaster recovery tested (backup and restore)
- Monitoring and alerting configured
- Deployment runbook completed
- User documentation completed
- SOC 2 Type I audit initiated (Type II after 12 months)

---

## 10. Appendices

### Appendix A: Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-08-01 | EEMP Architecture Team | Initial SRS document |

---

### Appendix B: Approval

| Role | Name | Signature | Date |
|------|------|-----------|------|
| **Product Owner** | | | |
| **CTO** | | | |
| **Lead Engineer** | | | |
| **QA Lead** | | | |

---

**Document Classification:** Internal  
**Confidentiality:** Proprietary and Confidential  

---

*This Software Requirements Specification (SRS) serves as the foundation for implementation, testing, and validation of EEMP. All implementation must align with the requirements defined herein.*
