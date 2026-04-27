# Architecture Brief: State Management for Distributed Workflow

_Authored as an architect for reviewers and downstream implementers evaluating
the durable workflow-state decision._

## Decision
What structural decision are we making?
We need to choose a durable state management backend for processing long-running async background jobs in our new billing service.

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

## Decision Drivers
- The billing team needs a durable, queryable system of record for long-running jobs without inventing new operational rituals.
- The platform team needs predictable retries, worker coordination, and auditability under pod crashes and deploy churn.
- The solution must remain understandable to the current team and fit within the existing operational model.

## Options Considered
- **Option 1: PostgreSQL + Dedicated Worker Table**
  - Implement a basic state machine over standard rows with atomic `SELECT FOR UPDATE SKIP LOCKED` logic.
- **Option 2: Redis Streams + PubSub**
  - Use Redis for fast event processing, holding the job state in a cache.
- **Option 3: External Workflow Engine (Temporal/GCP Step Functions)**
  - Offload workflow pausing, retries, and timing to a dedicated external SDK/infrastructure.

## Pros
- PostgreSQL keeps the source of truth, audit trail, and coordination logic in one durable system the team already operates.
- The worker-table approach supports explicit state queries for finance support without adopting a new workflow runtime first.
- It preserves a clean migration path toward a dedicated workflow engine later if concurrency or orchestration needs outgrow the initial design.

## Cons
- PostgreSQL-backed orchestration increases pressure on row-lock behavior, vacuum tuning, and transaction design.
- The team must author and maintain its own retry and timeout mechanics instead of delegating them to a workflow engine.
- Redis remains useful only as an optimization layer, not the source of truth, which limits how much complexity it can remove.

## Recommendation
Choose **PostgreSQL + Dedicated Worker Table** as the first durable workflow state backend for the billing service.

## Why Not The Others
- **Redis Streams + PubSub** is too fragile as the primary source of truth for auditable, long-running billing workflows under memory pressure and replay scenarios.
- **External Workflow Engine** adds a new infrastructure and operational model before the team has proven the billing workflow volume or complexity warrants it.

## Consequences
- The billing service must invest in explicit worker coordination, retry policies, and audit logging on top of PostgreSQL.
- Schema design, index strategy, and lock observability become part of the runtime ownership boundary for the platform team.
- A later migration to a dedicated workflow engine stays possible, but the initial API and audit model should be designed to keep that door open.

## Bounded Contexts
- Workflow Orchestration: owns long-running job state, retries, timers, and progression rules.
- Billing Policy: owns invoice lifecycle rules, billing-state semantics, and the invariants that protect customer entitlement.
- External Payments Integration: owns translation between internal billing events and Stripe/webhook payloads.

## Context Relationships
- Workflow Orchestration coordinates Billing Policy decisions but must not redefine billing-state semantics.
- External Payments Integration is upstream for payment events but downstream of Billing Policy for entitlement consequences.

## Integration Seams
- The seam between External Payments Integration and Billing Policy must translate Stripe-specific event names into internal billing states.
- The seam between Workflow Orchestration and Billing Policy must remain command-oriented so retries do not duplicate policy decisions.

## Anti-Corruption Candidates
- A Stripe adapter boundary should shield Billing Policy from provider-specific object models and retry semantics.
- A workflow runtime adapter should shield Billing Policy from Temporal-style workflow concepts if the team later adopts an external engine.

## Ownership Boundaries
- Billing Policy is owned by the billing product team because it defines revenue and entitlement semantics.
- Workflow Orchestration is owned by the platform/runtime team because it governs execution mechanics and operational safety.
- External Payments Integration is jointly reviewed, but provider model translation remains owned by the billing team.

## Shared Invariants
- A pending billing job can be retried, but it cannot be lost without an auditable state transition.
- Only one worker can advance the same billing task at a time, regardless of which orchestration option is selected.

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