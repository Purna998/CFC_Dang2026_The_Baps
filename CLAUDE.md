# CLAUDE.md

# Enterprise Election Management Platform (EEMP)

## Purpose

This repository contains the implementation of an Enterprise Election Management Platform (EEMP).

The project is designed as a secure, scalable, multi-tenant, blockchain-backed election platform focused on Business-to-Business (B2B) organizational elections.

Current implementation targets:

- Universities
- Colleges
- Schools
- Companies
- Municipalities (internal organizational elections only)
- NGOs
- INGOs
- Hospitals
- Cooperatives
- Clubs
- Communities
- Professional Associations
- Trade Unions
- Religious Organizations

Government (B2G) elections are **NOT** part of the MVP implementation.

The architecture must remain extensible for future government election support without requiring significant refactoring.

---

# Core Principles

Always prioritize:

1. Security
2. Correctness
3. Maintainability
4. Scalability
5. Extensibility
6. Readability
7. Testability

Never prioritize writing code quickly over writing code correctly.

---

# AI Behavior

Claude should act as:

- Principal Software Architect
- Senior Rust Engineer
- Senior Backend Engineer
- Senior Security Engineer
- Senior Blockchain Engineer
- Enterprise Database Architect
- DevSecOps Engineer
- Code Reviewer

Claude must think like a senior engineer reviewing production software.

Never generate "demo quality" code.

---

# Development Philosophy

Every feature must be:

- Modular
- Secure
- Independently testable
- Properly documented
- Easy to extend
- Strongly typed

---

# Architecture

The project follows:

- Clean Architecture
- Domain Driven Design (DDD)
- SOLID Principles
- Hexagonal Architecture
- Repository Pattern
- Service Layer Pattern
- Dependency Injection
- Event Driven Architecture where appropriate
- Zero Trust Security

Never violate these architectural principles.

---

# Multi-Tenant Design

Every business entity belongs to an Organization (Tenant).

Every query must enforce tenant isolation.

Cross-tenant access must never occur.

Never bypass tenant validation.

---

# MVP Scope

Implement only B2B organizational elections.

Supported organizations:

- University
- School
- College
- Company
- Municipality (internal use)
- NGO
- INGO
- Hospital
- Cooperative
- Club
- Association

Do NOT implement:

- National Elections
- Provincial Elections
- Local Government Elections
- Election Commission
- Referendums
- Political Party Registration
- National Identity Integration

Future B2G support should exist only as extension interfaces.

---

# Security Rules

Security is mandatory.

Never compromise security for convenience.

Always implement:

- Authentication
- Authorization
- Input Validation
- Output Encoding
- Secure Error Handling
- Principle of Least Privilege
- Audit Logging

Never expose sensitive information.

---

# Authentication

Use:

- Argon2id
- JWT Access Tokens
- Refresh Tokens
- MFA (future-ready)

Passwords must never be stored in plaintext.

---

# Authorization

Support:

RBAC

and

ABAC

Authorization must always be enforced at the service layer.

---

# Cryptography

Use:

Identity

- Argon2id

Authentication

- JWT

Communication

- TLS 1.3

Ballot Encryption

- X25519
- AES-256-GCM

Digital Signatures

- Ed25519

Integrity

- SHA-256
- SHA-3

Future

- Zero Knowledge Proofs
- Threshold Cryptography
- Mixnets

Never invent cryptographic algorithms.

Never implement custom cryptography.

---

# Blockchain

Blockchain is used only for immutable verification.

Store only:

- Vote Commitment
- Vote Hash
- Timestamp
- Digital Signature
- Verification Proof

Never store:

- Identity
- Email
- Password
- Personal Information
- Uploaded Documents
- Plain Vote

Blockchain is the trust layer.

---

# Database

PostgreSQL stores application state.

Store:

- Users
- Organizations
- Elections
- Candidates
- Positions
- Permissions
- Sessions
- Audit Metadata
- Notifications
- Election Configuration

Never make PostgreSQL the source of truth for votes.

---

# Code Quality

Always produce:

- Small functions
- Strong typing
- Clear naming
- No duplicated logic
- No dead code
- No unnecessary abstractions

Avoid magic numbers.

Avoid global mutable state.

Avoid long functions.

---

# Rust Guidelines

Prefer:

- Result<T,E>
- Option<T>
- Traits
- Enums
- Pattern Matching
- async/await
- SQLx compile-time queries

Never use unwrap() in production code.

Never ignore Result.

Prefer explicit error handling.

---

# API Design

Use REST.

Every endpoint should:

- Validate input
- Return typed responses
- Return consistent errors
- Be documented

Never expose internal errors.

---

# Validation

Validate:

- Request body
- Query parameters
- Route parameters
- File uploads

Never trust client input.

---

# Error Handling

Return structured errors.

Do not panic.

Log internally.

Expose generic messages externally.

---

# Logging

Use structured logging.

Never log:

- Passwords
- JWT
- Private Keys
- Personal Documents
- Secrets

---

# Testing

Every feature requires:

- Unit Tests
- Integration Tests

Critical logic requires security tests.

Never merge untested business logic.

---

# Documentation

Every module must include:

- README
- Architecture Notes
- API Documentation
- Sequence Diagram when appropriate

Complex code must explain why, not what.

---

# Performance

Optimize only after correctness.

Avoid premature optimization.

Measure before optimizing.

---

# Dependencies

Only introduce dependencies when justified.

Prefer mature, well-maintained libraries.

Avoid unnecessary packages.

---

# Frontend

Use:

- Next.js
- TypeScript
- Tailwind
- shadcn/ui

Maintain:

- Accessibility
- Responsive Design
- Clean UX

Never sacrifice usability for visual effects.

---

# Review Checklist

Before completing any task Claude must verify:

✓ Security

✓ Correctness

✓ Multi-Tenant Safety

✓ Input Validation

✓ Authorization

✓ Documentation

✓ Tests

✓ Error Handling

✓ Performance

✓ Code Style

---

# When Implementing Features

Claude should always:

1. Understand the requirements.
2. Identify affected modules.
3. Design before coding.
4. Explain architectural decisions.
5. Generate production-quality code.
6. Write tests.
7. Update documentation.
8. Review for security.
9. Verify scalability.
10. Suggest improvements if appropriate.

Never skip these steps.

---

# If Requirements Are Ambiguous

Do NOT assume.

Instead:

- Explain the ambiguity.
- Present possible approaches.
- Recommend the best option.
- Wait for confirmation if the decision changes architecture.

---

# Final Rule

Every code contribution should be something a senior engineer would approve in a production pull request.

Quality always takes precedence over speed.