# Microservices Architecture Implementation

This project implements all 22 microservice patterns described in `MICROSERVICE-PATTERNS.MD` using Rust and modern cloud-native technologies.

## Project Structure

```
MICROSERVICES-ARCH/
├── gateway/              # API Gateway service
├── bff/                  # Backend-for-Frontend services
│   └── web-bff/          # Web-specific BFF
├── services/             # Core domain services
│   ├── user-service/     # User management service
│   └── order-service/    # Order processing service
├── shared/               # Shared libraries and models
├── messaging/            # Messaging infrastructure
├── observability/        # Tracing and metrics
├── security/             # Security utilities
├── config/               # Configuration management
└── docs/                 # Documentation
    └── architecture/     # Architecture documentation
```

## Implemented Patterns

1. **Service-per-Bounded-Context** - Separate services for User and Order domains
2. **API Style & Versioning** - RESTful APIs with versioning
3. **Service Discovery & Naming** - Configuration-based service discovery
4. **API Gateway (Edge)** - Centralized API gateway
5. **Backends-for-Frontends (BFF)** - Web BFF for frontend integration
6. **Async Messaging Basics** - NATS-based messaging system
7. **Idempotent Endpoints & Dedup** - Idempotency patterns
8. **Data Ownership (Private Schema)** - Database per service
9. **Outbox Pattern (Transactional Events)** - Consistent event publishing
10. **Change Data Capture (CDC) for Integration** - Event-driven integration
11. **Event Schema Governance** - Versioned event schemas
12. **Consumer-Driven Contracts (CDCt) Testing** - Contract testing
13. **Anti-Corruption Layer (ACL)** - Data transformation layers
14. **Read Models & CQRS-Lite** - Read/write model separation
15. **Caching Tiers (Service & Shared)** - Multi-level caching
16. **Rate Limits, Quotas, and Shaping** - Rate limiting middleware
17. **Security: mTLS + AuthN/Z Between Services** - JWT-based security
18. **Secrets & Config Management (Per Service)** - Environment-based config
19. **Health, Readiness, and Graceful Shutdown** - Health check endpoints
20. **Observability: Tracing, Metrics, Correlation** - OpenTelemetry integration
21. **CI/CD Per Service (Pipelines & Templates)** - Workspace-based builds
22. **Deployment Topology (Single Region, Many Services)** - Kubernetes-ready

## Technologies Used

- **Rust** - Core language
- **Axum** - Web framework
- **SQLx** - Database toolkit
- **NATS** - Messaging system
- **OpenTelemetry** - Observability
- **Tower/Tokio** - Async runtime and middleware
- **Serde** - Serialization
- **Moka** - Caching

## Getting Started

1. Install Rust toolchain
2. Navigate to the project root
3. Run services individually or use workspace commands:

```bash
# Build all services
cargo build

# Run a specific service
cargo run -p user-service

# Run the gateway
cargo run -p gateway
```

## Services

- **Gateway** - Runs on port 3000
- **User Service** - Runs on port 3001
- **Order Service** - Runs on port 3002
- **Web BFF** - Runs on port 3003

## Documentation

See `docs/architecture/l2-patterns.md` for detailed implementation of each pattern.