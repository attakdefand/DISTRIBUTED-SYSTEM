Great—drilling into **Level 1: Monolith / Basic Split**. Below is a complete pattern catalog for this layer. For each pattern you get: **what it solves → key features → main types & sub-types → typical components → Rust tips**. Pick and mix; you don’t need all of them to be “Level-1 compliant.”

---

# L1 Pattern Set (Monolith / Basic Split)

## 1) Layered Architecture (3–5 layers)

**Solves:** keeps code organized and testable.
**Features:** UI/API → Application → Domain → Infrastructure (DB/IO).
**Types & sub-types:**

* 3-Layer (Controller/Service/Repo)
* 4-Layer (adds Domain/Use-Case)
* Hexagonal-lite (ports in domain, adapters in infra)
  **Components:** HTTP handlers, service facades, repositories.
  **Rust tips:** `axum` handlers → services (traits) → repos (`sqlx`), trait bounds for ports.

---

## 2) Modular Monolith

**Solves:** feature isolation without microservices.
**Features:** internal crates/modules per feature; compile-time boundaries.
**Types:**

* Single crate with modules
* Cargo workspace with internal crates
  **Components:** `core/` (domain), `features/<feature>`, `infra/`.
  **Rust tips:** path deps in workspace; enforce visibility with `pub(crate)`.

---

## 3) Basic Split (Early Externalization)

**Solves:** peel off obvious concerns without network sprawl.
**Sub-types:**

* Static/asset CDN split
* Background jobs/cron using same DB
* Admin API vs Public API split
  **Components:** reverse proxy/CDN, job runner.
  **Rust tips:** `tokio::time` schedulers; `axum` separate routers; `tower::limit` on admin.

---

## 4) Request Routing & Controllers

**Solves:** clear separation of transport and business.
**Features:** route tables, middleware chains, DTO mapping.
**Types:**

* REST (OpenAPI)
* gRPC (internal only, optional at L1)
  **Components:** router, middleware stack (auth, trace, rate-limit).
  **Rust tips:** `axum` + `tower` middlewares, `utoipa` for OpenAPI.

---

## 5) Transaction & Unit of Work

**Solves:** atomicity around multiple repo calls.
**Features:** begin/commit/rollback; retries on deadlocks.
**Types:**

* Per-request UoW
* Explicit UoW for commands
  **Components:** `sqlx::Transaction`, repo methods.
  **Rust tips:** pass `&mut Transaction<'_, DB>` across repos.

---

## 6) Repository & Query Patterns

**Solves:** isolate DB/sql; swap storage later.
**Features:** typed queries, pagination, ETag, optimistic locking.
**Sub-types:**

* CRUD repos
* Specification/Query objects
  **Components:** repo traits, DTO mappers.
  **Rust tips:** `sqlx::FromRow`, `serde` models, `time`/`uuid`.

---

## 7) Validation & Error Handling

**Solves:** predictable failures; safe inputs.
**Features:** layered validation (transport → domain), error taxonomy.
**Sub-types:**

* Input schema validation
* Domain invariant checks
* Central error mapper to HTTP codes
  **Components:** error type enum, validation traits.
  **Rust tips:** `thiserror`, `validator`, map errors in `IntoResponse`.

---

## 8) Configuration (12-Factor)

**Solves:** environment-specific behavior without code changes.
**Features:** `.env`/secrets, config layering, hot-reload optional.
**Sub-types:**

* Static at startup
* Reloadable (signal-based)
  **Components:** config loader, secret provider.
  **Rust tips:** `config`, `dotenvy`, optional Vault client.

---

## 9) Authentication & Session Basics

**Solves:** identity for users/admin.
**Features:** JWT stateless sessions or server-side sessions; password/2FA.
**Sub-types:**

* Cookie session (server store)
* JWT bearer (short TTL)
  **Components:** auth middleware, user store, hasher.
  **Rust tips:** `argon2`, `jsonwebtoken`, `axum-extra::extract::CookieJar`.

---

## 10) Authorization (Simple RBAC)

**Solves:** restrict actions by role/ownership.
**Features:** role/permission tables, ownership checks.
**Sub-types:**

* Role-based (RBAC)
* Resource-owner checks
  **Components:** policy guard, route guards.
  **Rust tips:** embed guards as `tower` layers or per-handler checks.

---

## 11) Caching (Local & Shared)

**Solves:** reduce DB load, speed reads.
**Features:** key naming, TTL, stampede protection (coalescing).
**Sub-types:**

* In-process LRU
* Shared Redis
  **Components:** cache client, cache-aside helpers.
  **Rust tips:** `moka` (in-proc), `redis` crate, SWR pattern in service.

---

## 12) Background Jobs & Scheduling

**Solves:** async/offline work (emails, cleanups).
**Features:** retry with backoff; idempotence keys.
**Sub-types:**

* In-proc scheduler (best effort)
* External queue (still L1 if same DB/host)
  **Components:** job runner, registry, persistence (jobs table).
  **Rust tips:** `tokio` tasks; `apalis` or `rq`-like crates.

---

## 13) API Evolution & Versioning (L1 scope)

**Solves:** change endpoints safely.
**Features:** URL versioning or header; deprecation policy.
**Sub-types:**

* `/v1`, `/v2` routers
* Media-type versioning
  **Components:** versioned routers, compatibility mappers.
  **Rust tips:** mount multiple `Router`s; map types between versions.

---

## 14) Observability Foundations

**Solves:** know what’s happening in prod.
**Features:** structured logs, request IDs, metrics, basic tracing.
**Sub-types:**

* App logs to file/stdout
* Metrics: req/sec, p95, error%
  **Components:** logging formatter, metrics exporter.
  **Rust tips:** `tracing`, `tracing-subscriber`, `opentelemetry`, `prometheus`.

---

## 15) API Gateway / Reverse Proxy (Edge Lite)

**Solves:** TLS termination, static offload, rate limits.
**Features:** single domain, health checks, gzip, CORS.
**Types:** Nginx/Caddy/Traefik in front of monolith.
**Components:** proxy, certs, static hosting.
**Rust tips:** still run single binary; keep static under CDN when possible.

---

## 16) Security Hygiene (App Level)

**Solves:** common web vulns.
**Features:** input sanitization, CSRF (if cookies), CORS rules, headers.
**Sub-types:**

* CSRF token flow
* Content Security Policy
  **Components:** security middleware, header sets.
  **Rust tips:** `axum-extra` for headers, `tower-http::cors`, `csrf` crates.

---

## 17) Build & CI (Single Pipeline)

**Solves:** reproducible builds and fast feedback.
**Features:** fmt, clippy, test, audit, SBOM (basic).
**Sub-types:**

* Release build with strip/LTO
* Docker image with non-root user
  **Components:** GH Actions (or similar), `Dockerfile`.
  **Rust tips:** `cargo fmt`, `clippy -D warnings`, `cargo-audit`, `cyclonedx-bom`.

---

## 18) Deployment Patterns (Single Target)

**Solves:** simple, reliable shipping.
**Features:** one binary, one DB; blue/green optional.
**Sub-types:**

* Systemd service
* Docker Compose single node
  **Components:** service unit/compose file, health endpoint.
  **Rust tips:** add `/healthz` endpoint; graceful shutdown via `tokio::signal`.

---

## 19) Data Migrations & Seeding

**Solves:** schema evolution safely.
**Features:** up/down migrations; idempotent seed data.
**Types:**

* SQL migration files
* Embedded migrations
  **Components:** migrator, version table.
  **Rust tips:** `sqlx migrate`, check at boot and fail fast on drift.

---

## 20) Testing Strategy (L1 Gate)

**Solves:** confidence without overkill.
**Features:** unit tests, happy-path E2E, a few integration tests.
**Sub-types:**

* Unit (domain/services)
* Integration (repo against test DB)
* E2E (HTTP + DB)
  **Components:** test harness, fixtures, ephemeral DB.
  **Rust tips:** `cargo test -- --test-threads=1` for DB tests; `testcontainers` or `docker compose` for local DB.

---

# Typical L1 Component Map (Minimal)

* **Monolith binary**: `axum` server (+ routers, middleware).
* **Domain & services**: traits/impls, validation, UoW.
* **Persistence**: `sqlx` repos, migrations.
* **Cache**: optional (`moka` or Redis).
* **Jobs**: in-proc scheduler/worker.
* **Edge**: Nginx/Caddy, optional CDN for assets.
* **Observability**: `tracing`, Prometheus exporter.
* **CI/Build**: fmt, clippy, test, audit, Docker.

---

## “How many patterns in L1?”

You have **20** concrete patterns above. If you want the ultra-minimal viable set to ship safely: **(1) Layered Architecture, (5) Transaction/UoW, (7) Validation & Errors, (8) Config, (11) Caching (later), (14) Observability, (17) Build & CI, (18) Deployment, (19) Migrations, (20) Testing**.

---

If you’d like, I can turn this into a **`/docs/architecture/l1-patterns.md`** with code snippets (Axum routers, UoW wrapper, repo trait, error mapper, config loader, healthz), plus a **`Dockerfile`** and **GitHub Actions** workflow ready to drop into your repo.
