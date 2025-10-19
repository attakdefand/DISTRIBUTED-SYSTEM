awesome—here’s **Level 2: Basic Microservices** fully expanded. You get **22 concrete patterns**. For each: what it solves → key features → main types & sub-types → core components → Rust tips (geared to your stack).

---

# How many patterns at L2?

**22 patterns** (catalog below). Pick the ones you need now; you don’t have to implement all at once.

---

## 1) Service-per-Bounded-Context

**Solves:** decouple domains; independent deploy/scale.
**Features:** strict ownership, single responsibility, own datastore.
**Types:** coarse-grained (Users/Orders/Risk), fine-grained (Auth/Session/Profiles).
**Components:** service crate, DB, HTTP/gRPC, background worker.
**Rust tips:** workspace with per-service crates; `axum` + `sqlx`; explicit domain traits.

---

## 2) API Style & Versioning

**Solves:** safe API evolution across services.
**Features:** REST/gRPC/GraphQL, semantic versions, deprecation windows.
**Types:**

* REST: path `/v1`, media type versioning
* gRPC: protobuf versioned packages
* GraphQL: schema registry, deprecations
  **Components:** OpenAPI, proto defs, schema lints.
  **Rust tips:** `utoipa`/`progenitor` for REST; `tonic` + `prost` for gRPC; `async-graphql` for GraphQL.

---

## 3) Service Discovery & Naming

**Solves:** locating services without hardcoding.
**Features:** logical names, health-aware endpoints.
**Types:** DNS-based, mesh-based (later), config catalog.
**Components:** discovery client, health cache.
**Rust tips:** config-driven base URLs; optional Consul/etcd client; retry on NXDOMAIN.

---

## 4) API Gateway (Edge)

**Solves:** unify ingress, auth, quotas, CORS, TLS termination.
**Features:** path routing, rate limits, request shaping.
**Types:** centralized gateway; per-client route groups.
**Components:** edge proxy, policy plug-ins.
**Rust tips:** keep gateway outside app code; pass user identity via headers (mTLS/JWT).

---

## 5) Backends-for-Frontends (BFF)

**Solves:** tailor APIs per client (web, mobile, admin).
**Features:** composition, field filtering, pagination normalization.
**Types:** 1 BFF per client; vertical domain BFFs for large apps.
**Components:** BFF services, cache, schema mappers.
**Rust tips:** small `axum` services that orchestrate calls to domain services.

---

## 6) Async Messaging Basics

**Solves:** decouple, avoid lockstep sync chains.
**Features:** pub/sub topics, at-least-once delivery, consumer groups.
**Types:**

* Event (notify)
* Command (directed)
* Request-reply (correlation id)
  **Components:** NATS/Redpanda broker, producers, consumers.
  **Rust tips:** `async-nats` or `rdkafka`; include `message_id`, `causation_id`, `correlation_id`.

---

## 7) Idempotent Endpoints & Dedup

**Solves:** safe retries, exactly-once effect for clients.
**Features:** idempotency keys, dedup store, natural keys on writes.
**Types:** client-provided key; server-generated token.
**Components:** idempo table, middleware.
**Rust tips:** wrap handlers with idempotency layer; store response hash for replays.

---

## 8) Data Ownership (Private Schema)

**Solves:** eliminate shared-DB coupling.
**Features:** each service writes only its DB; others read via APIs/events.
**Types:** relational per service; polyglot where justified.
**Components:** per-service migrations, connection pools.
**Rust tips:** `sqlx` feature-per-DB; `sqlx migrate` in service startup.

---

## 9) Outbox Pattern (Transactional Events)

**Solves:** keep DB write and event publish consistent.
**Features:** write domain row + outbox in same tx; relay worker publishes.
**Types:** polling relay; log-triggered relay.
**Components:** outbox table, relay daemon, DLQ.
**Rust tips:** wrap repo save + outbox insert in one `Transaction`; relay uses backoff + marking states.

---

## 10) Change Data Capture (CDC) for Integration

**Solves:** build read models/analytics without coupling.
**Features:** tail binlog/WAL; transform, publish.
**Types:** Debezium-style stream; triggers to Kafka/NATS.
**Components:** CDC connector, transformer, topic writer.
**Rust tips:** consumer builds ClickHouse read models; ensure ordering by `source_lsn`.

---

## 11) Event Schema Governance

**Solves:** evolvable events without breaking consumers.
**Features:** schema registry, compatibility rules (backward/forward).
**Types:** Avro/JSON/Protobuf; versioned subjects.
**Components:** registry service, CI checks.
**Rust tips:** `prost`/`serde` with `#[serde(default)]`; never remove fields—deprecate.

---

## 12) Consumer-Driven Contracts (CDCt) Testing

**Solves:** avoid breaking consumers when providers change.
**Features:** provider verifies consumer pacts in CI.
**Types:** HTTP pact; gRPC pact; message pact.
**Components:** pact broker, test runners.
**Rust tips:** `pact_consumer`/`pact_provider` crates for REST; proto-based fixtures for gRPC.

---

## 13) Anti-Corruption Layer (ACL)

**Solves:** shield your domain from external model leaks.
**Features:** translation layer; mapping, validation, defaults.
**Types:** ingress ACL (external→domain); egress ACL (domain→external).
**Components:** mappers, validators, adapters.
**Rust tips:** separate DTOs from domain types; `TryFrom` with rich errors.

---

## 14) Read Models & CQRS-Lite

**Solves:** fast queries without polluting write model.
**Features:** async projection builders; denormalized views.
**Types:** cache-only, DB-materialized (CH/Elastic).
**Components:** projector, view store, backfill jobs.
**Rust tips:** projector subscribers on NATS topics; idempotent upserts.

---

## 15) Caching Tiers (Service & Shared)

**Solves:** latency + load reduction.
**Features:** cache-aside, TTL/TTI, tag invalidation via events.
**Types:** in-proc LRU; Redis shared; request coalescing.
**Components:** cache client, key strategy, stampede guard.
**Rust tips:** `moka` + `redis`; SWR in BFFs; cache keys include version.

---

## 16) Rate Limits, Quotas, and Shaping

**Solves:** protect services from abuse and burst traffic.
**Features:** token bucket/leaky bucket, per-key quotas, spike arrest.
**Types:** edge (gateway), per-service, per-tenant.
**Components:** limiter store (Redis), middleware.
**Rust tips:** `tower` middleware for per-route limits; counters in Redis with LUA/ACLs.

---

## 17) Security: mTLS + AuthN/Z Between Services

**Solves:** zero-trust basics across services.
**Features:** mTLS, SPIFFE IDs, JWT service tokens.
**Types:** mutual TLS only; mTLS + JWT claims for fine-grained policy.
**Components:** CA/cert manager, policy engine hooks.
**Rust tips:** `hyper-rustls`/`rustls` for TLS; verify SAN/service identity in client.

---

## 18) Secrets & Config Management (Per Service)

**Solves:** safe secret handling and env-specific configs.
**Features:** rotated secrets, sealed at rest, templated envs.
**Types:** Vault/SOPS; K8s secrets (encrypted) + sidecar.
**Components:** secret injector, config loader.
**Rust tips:** `config` crate layered with env overrides; fetch secrets once and memoize.

---

## 19) Health, Readiness, and Graceful Shutdown

**Solves:** safe deploys and reliable rollouts.
**Features:** `/healthz`, `/readyz`, signal handling, connection drain.
**Types:** liveness vs readiness; dependency-aware health.
**Components:** health endpoints, preStop hooks.
**Rust tips:** `tokio::signal` for SIGTERM; drain listeners; DB ping in readiness only.

---

## 20) Observability: Tracing, Metrics, Correlation

**Solves:** follow a request across services and brokers.
**Features:** trace propagation (W3C), RED/USE metrics, log correlation.
**Types:** tracing over HTTP/gRPC; message carriers via headers.
**Components:** OTel SDK, collectors, dashboards.
**Rust tips:** `tracing` + `opentelemetry` + `tower-http::trace`; propagate `traceparent` on NATS metadata.

---

## 21) CI/CD Per Service (Pipelines & Templates)

**Solves:** fast, independent release flow.
**Features:** per-service pipelines, versioned images, SBOM, scan.
**Types:** trunk-based with short-lived branches; service charts.
**Components:** GH Actions per service, Dockerfile, Helm chart.
**Rust tips:** cache `cargo` layers; `clippy -D warnings`; `cargo audit`; push image tags like `svc:gitsha`.

---

## 22) Deployment Topology (Single Region, Many Services)

**Solves:** run many services reliably without L3 complexity.
**Features:** K8s namespace per domain, HPA on CPU/QPS, resource quotas.
**Types:** single cluster; multi-ns; preview envs.
**Components:** deployments, services, ingress, HPA, PDBs.
**Rust tips:** configure graceful timeout lower than LB timeouts; expose Prom metrics; set resource requests/limits.

---

## Minimal “L2 Starter Pack” (if you want the shortest path)

1. Service-per-bounded-context
2. API style/versioning + OpenAPI/Protos
3. Gateway + BFF
4. Async messaging (event + command)
5. Outbox + Idempotency
6. Private schema per service
7. Health/readiness + graceful shutdown
8. mTLS + per-service secrets
9. Tracing + RED metrics
10. Per-service CI/CD + Helm chart

---

## Quick Rust wiring cheatsheet

* **HTTP:** `axum` + `tower` middlewares (auth, trace, rate-limit)
* **gRPC:** `tonic` with `prost` (health service + interceptors)
* **Messaging:** `async-nats` or `rdkafka`; include trace context headers
* **DB:** `sqlx` per service; `sqlx migrate` at boot; outbox table + relay task
* **Config:** `config` + `dotenvy`; secrets pulled once
* **Obs:** `tracing`, `opentelemetry`, `prometheus` exporter
* **CI/CD:** fmt → clippy → test → audit → build → SBOM → image → chart → deploy
* **Deploy:** K8s Deployment + Service + HPA + Ingress; readiness checks use real deps

---

want me to turn this into a **`/docs/architecture/l2-patterns.md`** plus **Axum/Tonic/NATS skeletons** (outbox, idempotency middleware, health endpoints, tracing + metrics) you can drop into your workspace?
