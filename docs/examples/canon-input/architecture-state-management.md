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