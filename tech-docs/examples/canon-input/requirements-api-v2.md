# Requirements Brief: API v2

_Authored as a product lead for stakeholders deciding whether API v2 should
proceed as a bounded slice._

## Problem
API v1 relies on a legacy XML payload structure and tight REST coupling that prevents efficient mass data operations. Partners are building their own middleware scrapers because our endpoints require 10+ round trips to assemble a single customer order profile. We need to define the V2 boundary before we begin the implementation.

## Outcome
By the time this work is fully delivered, API v2 will provide a GraphQL-based bulk data retrieval capability, serving order composites in a single round trip with sub-100ms latency. The API must be documented with an OpenAPI or GraphQL schema and versioned separately from API v1.

## Constraints
- Must not touch or rewrite the underlying data stores (PostgreSQL and Redis caches); this is strictly an API-layer enhancement.
- Latency budget is strict: 100ms per composite request for a standard order profile.
- API v2 must live on a dedicated `api.domain.com/v2` or `api.domain.com/graphql` route to avoid affecting v1 traffic.

## Non-Negotiables
- API v1 traffic and contracts remain stable throughout the rollout.
- Authentication, rate limiting, and audit logging must remain compatible with the existing partner control plane.

## Options
1. Ship a GraphQL-only API v2 endpoint for composite reads.
2. Ship a REST bulk-read endpoint family under `/v2` and defer GraphQL.
3. Keep v1 and add a middleware aggregation proxy without a formal v2 surface.

## Recommended Path
Ship a GraphQL-based API v2 on a dedicated route. It gives the cleanest contract for bulk composite reads, makes over-fetching explicit, and avoids expanding the legacy REST surface with more special-case endpoints.

## Tradeoffs
- Flexibility vs Speed: We accept slightly denormalized graph queries for speed over perfect normalization at the edge.
- Client Complexity: Clients must learn GraphQL, accepting a steeper learning curve for efficiency.

## Consequences
- Partner SDKs and onboarding docs will need a versioned API v2 update.
- Query complexity controls must ship with the first public beta to protect latency and cost budgets.

## Scope Cuts
- Taking down API v1. It will run in parallel indefinitely.
- Real-time streaming or WebSockets; REST or simple GraphQL queries only.

## Deferred Work
- API v1 deprecation planning and partner migration timelines.
- Mutation support for API v2 beyond composite read use cases.

## Decision Checklist
- [x] Keep the rollout additive to API v1.
- [ ] Confirm the owning team for schema governance.
- [ ] Confirm whether query-cost limits reuse partner rate-limit policy or require a new policy.

## Open Questions
- Will we use Apollo Federation, or a monolith schema?
- How should pagination be mapped over existing SQL pagination cursors?
- Do existing partner-facing rate limits apply, or do we implement complexity-based query limits?