# Requirements Brief: API v2

## Problem
API v1 relies on a legacy XML payload structure and tight REST coupling that prevents efficient mass data operations. Partners are building their own middleware scrapers because our endpoints require 10+ round trips to assemble a single customer order profile. We need to define the V2 boundary before we begin the implementation.

## Outcome
By the time this work is fully delivered, API v2 will provide a GraphQL-based bulk data retrieval capability, serving order composites in a single round trip with sub-100ms latency. The API must be documented with an OpenAPI or GraphQL schema and versioned separately from API v1.

## Constraints
- Must not touch or rewrite the underlying data stores (PostgreSQL and Redis caches); this is strictly an API-layer enhancement.
- Latency budget is strict: 100ms per composite request for a standard order profile.
- API v2 must live on a dedicated `api.domain.com/v2` or `api.domain.com/graphql` route to avoid affecting v1 traffic.

## Tradeoffs
- Flexibility vs Speed: We accept slightly denormalized graph queries for speed over perfect normalization at the edge.
- Client Complexity: Clients must learn GraphQL, accepting a steeper learning curve for efficiency.

## Out of Scope
- Taking down API v1. It will run in parallel indefinitely.
- Real-time streaming or WebSockets; REST or simple GraphQL queries only.

## Open Questions
- Will we use Apollo Federation, or a monolith schema?
- How should pagination be mapped over existing SQL pagination cursors?
- Do existing partner-facing rate limits apply, or do we implement complexity-based query limits?