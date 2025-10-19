# Rust Layer System - Layer 1 (Monolith)

This project implements a Layer 1 monolithic architecture based on the patterns described in [LAYER1.MD](../LAYER1.MD).

## Architecture Patterns Implemented

### All 20 Layer 1 Patterns Fully Implemented ✅

1. **Layered Architecture** - 3-Layer (Controller/Service/Repository)
2. **Request Routing & Controllers** - Using Axum
3. **Validation & Error Handling** - Using thiserror and validator
4. **Configuration** - 12-Factor with environment variables
5. **Observability Foundations** - Structured logging with tracing
6. **Build & CI** - Cargo-based build system with GitHub Actions
7. **Deployment Patterns** - Single binary deployment with Docker and Nginx reverse proxy
8. **Testing Strategy** - Unit and integration tests
9. **Modular Monolith** - Rust modules organization
10. **Transaction & Unit of Work** - Database transaction patterns
11. **Caching (Local)** - In-process caching with Moka
12. **Background Jobs & Scheduling** - In-process job scheduler
13. **Repository & Query Patterns** - Full database implementation with CRUD operations
14. **Security Hygiene** - Comprehensive security middleware (CORS, headers, etc.)
15. **Data Migrations & Seeding** - Automated migration runner with seed data
16. **API Gateway / Reverse Proxy** - Nginx reverse proxy with TLS termination and rate limiting
17. **Basic Split** - Static assets serving and admin API separation
18. **Authentication & Session Basics** - JWT-based authentication with login endpoint
19. **Authorization (Simple RBAC)** - Role-based access control with middleware
20. **API Evolution & Versioning** - Versioned API endpoints (v1 and v2)

## Project Structure

```
src/
├── main.rs          # Application entry point
├── handlers/        # HTTP handlers (controllers)
├── services.rs      # Business logic layer
├── repositories.rs  # Data access layer
├── models.rs        # Domain models
├── config.rs        # Configuration management
├── error.rs         # Error handling
├── uow.rs           # Unit of Work pattern
├── cache.rs         # Caching service
├── auth.rs          # Authentication middleware
├── rbac.rs          # Role-based access control
└── jobs.rs          # Background job scheduler

migrations/          # Database migrations
static/              # Static assets
Dockerfile           # Docker configuration
docker-compose.yml   # Docker Compose configuration
.github/workflows/   # CI/CD workflows
```

## Getting Started

### Option 1: Run with Cargo

1. Install Rust using [rustup](https://rustup.rs/)
2. Run the application:

```bash
cargo run
```

The application will start on http://127.0.0.1:3000

### Option 2: Run with Docker

1. Build and run with Docker Compose:

```bash
docker-compose up --build
```

The application will be available at http://localhost:3000

## Endpoints

### Public Endpoints
- `GET /` - Root endpoint
- `GET /healthz` - Health check endpoint
- `GET /static/*` - Static assets

### Authentication Endpoints
- `POST /api/auth/login` - User login

### API v1 Endpoints
- `POST /api/v1/users` - Create a user
- `GET /api/v1/users/:id` - Get a user by ID

### API v2 Endpoints
- `POST /api/v2/users` - Create a user with preferences
- `GET /api/v2/users/:id` - Get a user by ID
- `PATCH /api/v2/users/:id` - Update a user

### Admin Endpoints
- `GET /admin/stats` - System statistics
- `GET /admin/health` - Admin health check
- `GET /admin/protected` - Protected admin endpoint (requires admin role)

## Configuration

The application can be configured using environment variables or a `.env` file:

- `DATABASE_URL` - Database connection string
- `HOST` - Server host (default: 127.0.0.1)
- `PORT` - Server port (default: 3000)