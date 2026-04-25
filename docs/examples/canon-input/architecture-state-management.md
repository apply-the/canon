# Architecture Brief: State Management for Distributed Workflow 

## Decision
What structural decision are we making?
We need to choose a durable state management backend for processing long-running async background jobs in our new billing service.

## Options
- **Option 1: PostgreSQL + Dedicated Worker Table**
  - Implement a basic state machine over standard rows with atomic `SELECT FOR UPDATE SKIP LOCKED` logic.
- **Option 2: Redis Streams + PubSub**
  - Use Redis for fast event processing, holding the job state in a cache.
- **Option 3: External Workflow Engine (Temporal/GCP Step Functions)**
  - Offload workflow pausing, retries, and timing to a dedicated external SDK/infrastructure.

## Constraints
- Must guarantee execution at-least-once for billing actions.
- Avoid introducing entirely new infrastructure management layers unless absolutely necessary.
- Total latency for state transition must be < 50ms, but total job duration can be up to 7 days (e.g. invoice retries).

## Candidate Boundaries
- Bounded Context: `billing-service` and its internal `job-queue`.
- Everything outside (e.g., Stripe Webhooks, Email Notification services) must only emit side-effects, not run business logic related to state transitions.

## Invariants
- Events must be auditable (history of state changes is kept forever).
- A pending job must never be lost if a worker pod crashes mid-execution.
- Only one worker can process an active task at a time (idempotency token pattern required or strict locks).

## Evaluation Criteria
- **Operational Complexity**: Can the current devops team maintain it?
- **Correctness/Safety**: Can it drop a message (i.e. double bill or forget to invoice)?
- **Integration Effort**: Hours needed to rebuild our core workflow vs standard libraries.
- **Observability**: Can we easily query the state of a workflow (e.g., why is user `X` stuck in `INVOICING`)?

## Risks
- Redis could drop data under memory pressure, causing lost jobs.
- PostgreSQL row-level locks limit overall concurrency at 10k transactions a second.
- Using an external engine creates tight vendor lock-in for critical business processes.

## System Context
- System: `billing-service` orchestrates long-running async billing jobs (invoicing, retries, dunning) for paying customers.
- External actors:
  - finance-ops-engineer: monitors job dashboards and inspects stuck workflows.
  - stripe-webhook-receiver: emits payment-state changes that the billing service consumes.
  - notification-service: receives bounded events to dispatch billing emails.
  - billing-frontend: triggers manual retries and reads current job state for support flows.

## Containers
- `billing-service` (Rust async worker pool): owns the job state machine and executes billing transitions.
- `postgres-jobs` (managed Postgres 15): durable store for job rows, audit log, and idempotency tokens.
- `redis-cache` (managed Redis 7): hot path for short-lived locks and worker liveness signals only — never the source of truth.
- `billing-admin` (internal SPA): observability surface for finance-ops-engineer, served from the existing admin gateway.

## Components
- `job-queue-repository`: encapsulates `SELECT FOR UPDATE SKIP LOCKED` access against `postgres-jobs`.
- `state-machine`: pure function module that validates and produces the next valid job state.
- `worker-runtime`: bounded async runtime that pulls jobs, applies the state machine, and persists transitions.
- `audit-log-writer`: append-only writer that records every state transition for compliance.
- `idempotency-guard`: enforces single-active-worker invariant via tokens before any billing side-effect fires.
- `notification-emitter`: bounded outbound adapter that publishes domain events for `notification-service`.