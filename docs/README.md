# EEMP Documentation Index

## Document Organization

This documentation follows enterprise software engineering standards used by Microsoft, AWS, and government-grade digital platforms.

## Phase 0: Foundation Documents

### Requirements (Must Read First)
1. [Vision Document](requirements/01-vision.md) - Project vision, objectives, and success criteria
2. [Business Requirements Document (BRD)](requirements/02-brd.md) - Business goals and stakeholder needs
3. [Software Requirements Specification (SRS)](requirements/03-srs.md) - Functional and system requirements
4. [Functional Requirements Specification (FRS)](requirements/04-frs.md) - Detailed feature specifications
5. [Non-Functional Requirements (NFR)](requirements/05-nfr.md) - Performance, security, scalability

### Architecture
1. [High-Level Design (HLD)](architecture/01-hld.md) - System architecture overview
2. [Low-Level Design (LLD)](architecture/02-lld.md) - Detailed component design
3. [Context Diagram](architecture/03-context-diagram.md) - System boundaries and actors
4. [Component Diagram](architecture/04-component-diagram.md) - Internal structure
5. [Deployment Architecture](architecture/05-deployment-architecture.md) - Infrastructure design
6. [Multi-Tenancy Architecture](architecture/06-multi-tenancy.md) - Tenant isolation strategy
7. [Event-Driven Architecture](architecture/07-event-architecture.md) - Event flows and messaging

### Design
1. [Database Schema](design/01-database-schema.md) - PostgreSQL schema with ER diagrams
2. [Data Dictionary](design/02-data-dictionary.md) - Table and column specifications
3. [Redis Design](design/03-redis-design.md) - Caching strategy
4. [Object Storage Design](design/04-object-storage.md) - File storage architecture
5. [Blockchain Data Model](design/05-blockchain-model.md) - Solana program design
6. [UML Diagrams](design/06-uml-diagrams.md) - Class, sequence, activity diagrams

### Security
1. [Security Architecture](security/01-security-architecture.md) - Overall security design
2. [Authentication Architecture](security/02-authentication.md) - AuthN flows and mechanisms
3. [Authorization Architecture](security/03-authorization.md) - RBAC + ABAC design
4. [Cryptographic Architecture](security/04-cryptography.md) - Crypto stack specification
5. [Threat Model (STRIDE)](security/05-threat-model.md) - Security threat analysis
6. [Attack Tree Analysis](security/06-attack-trees.md) - Attack vector mapping
7. [Security Testing Plan](security/07-security-testing.md) - Penetration testing strategy

### API
1. [REST API Design](api/01-rest-api-design.md) - API design principles
2. [OpenAPI Specification](api/openapi.yaml) - Machine-readable API spec
3. [API Authentication](api/02-api-authentication.md) - JWT implementation
4. [API Rate Limiting](api/03-rate-limiting.md) - Rate limit design
5. [Webhook Architecture](api/04-webhooks.md) - Event notifications

### Deployment
1. [DevOps Strategy](deployment/01-devops-strategy.md) - CI/CD approach
2. [Docker Architecture](deployment/02-docker.md) - Container design
3. [Kubernetes Architecture](deployment/03-kubernetes.md) - Orchestration design
4. [Monitoring & Observability](deployment/04-monitoring.md) - Metrics, logs, traces
5. [Disaster Recovery](deployment/05-disaster-recovery.md) - Backup and recovery
6. [Deployment Guide](deployment/06-deployment-guide.md) - Production deployment

## Reading Order for Different Roles

### For Business Stakeholders
1. Vision Document
2. Business Requirements Document (BRD)
3. High-Level Design (HLD)
4. Security Architecture (executive summary)

### For Product Managers
1. Vision Document
2. BRD + SRS
3. Functional Requirements Specification (FRS)
4. High-Level Design (HLD)
5. API Design

### For Architects
1. All requirements documents (Vision → NFR)
2. All architecture documents (HLD → Event Architecture)
3. Security Architecture suite
4. Database Schema + Blockchain Model

### For Backend Engineers
1. SRS + FRS + NFR
2. High-Level Design + Low-Level Design
3. Database Schema + Data Dictionary
4. API Specification
5. Security Architecture (Auth + Crypto)

### For Frontend Engineers
1. SRS + FRS
2. Component Diagram
3. API Specification
4. Authentication Architecture

### For Security Engineers
1. All security documents
2. Threat Model + Attack Trees
3. Cryptographic Architecture
4. Database Schema (for data protection)

### For DevOps Engineers
1. Deployment Architecture
2. All deployment documents
3. Monitoring & Observability
4. Disaster Recovery

## Document Status Tracking

| Document | Status | Last Updated | Reviewer |
|----------|--------|--------------|----------|
| Vision | ✅ Complete | 2026-08-01 | - |
| BRD | 🔄 In Progress | - | - |
| SRS | 📋 Planned | - | - |
| FRS | 📋 Planned | - | - |
| NFR | 📋 Planned | - | - |
| HLD | 🔄 In Progress | - | - |
| LLD | 📋 Planned | - | - |
| Security Architecture | 🔄 In Progress | - | - |
| Database Schema | 🔄 In Progress | - | - |
| API Specification | 📋 Planned | - | - |

**Legend:**
- ✅ Complete and reviewed
- 🔄 In progress
- 📋 Planned
- ⏸️ On hold
- ❌ Blocked

## Contributing to Documentation

All documentation follows these standards:
- **Format:** Markdown with Mermaid diagrams
- **Version Control:** All changes tracked in git
- **Review Process:** All documents require technical review
- **Updates:** Documents are living artifacts, update when architecture changes
- **Clarity:** Write for the intended audience (see Reading Order above)

## Document Templates

See `templates/` directory for document templates following enterprise standards.

---

**Last Updated:** 2026-08-01  
**Maintained By:** EEMP Architecture Team
