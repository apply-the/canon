# Change Brief: Add Read-Through Caching to Profiles

## System Slice
The unbounded read load on `user_profiles_db` driven by repeated profile lookups through `ProfileService`.

## Domain Slice
Profile retrieval and cache invalidation inside the profile-read boundary; write semantics and authorization stay outside this slice.

## Excluded Areas
- Profile schema changes in PostgreSQL.
- Authentication and authorization behavior outside profile reads and writes.

## Intended Change
Add a Redis-backed read-through cache in `crates/profile-engine/src/repository.rs`.

## Legacy Invariants
- Writing to a user profile must synchronously persist to PostgreSQL.
- Cache invalidation must never mask a database failure.
- Read latency expectations must remain stable under degraded cache conditions.

## Domain Invariants
- A successful profile write remains the source of truth even when Redis is unavailable.
- Cache fallback must preserve the same profile identity and visibility rules as the direct PostgreSQL path.

## Forbidden Normalization
- Do not turn write-through failures into best-effort success.
- Do not weaken the existing profile update semantics to fit the cache layer.

## Change Surface
- `crates/profile-engine/src/repository.rs`
- `crates/profile-engine/src/bootstrap.rs`
- API container environment configuration handling `REDIS_URL`

## Ownership
- Primary owner: backend-platform
- Reviewer: data-infra

## Cross-Context Risks
- Cache invalidation can leak into deployment and configuration boundaries if Redis bootstrap is treated as application logic.
- The profile-read boundary may accidentally widen into authorization behavior if cache misses trigger unrelated side effects.

## Implementation Plan
Introduce a Redis connection pool at process startup, wrap profile reads with cache lookup plus PostgreSQL fallback, and invalidate cached entries after a successful profile update.

## Sequencing
1. Add the Redis pool and health wiring without changing profile behavior.
2. Wrap `get_profile_by_id` with cache lookup and PostgreSQL fallback.
3. Invalidate the cache after successful profile updates.
4. Verify degraded-cache behavior before rollout.

## Validation Strategy
- Verify via load tests that database connection limits are no longer hit under 500 RPS load.
- Confirm stale reads do not exceed 10ms after an update using synthetic test clients.
- Exercise degraded-cache scenarios where Redis is unavailable but PostgreSQL remains healthy.

## Independent Checks
- Have a non-implementing reviewer confirm the cache invalidation path cannot mask a PostgreSQL write failure.
- Review the rollout dashboard and alert thresholds with the SRE owner before enabling the cache in production.

## Decision Record
Choose explicit cache-aside invalidation over TTL-only expiration so user-driven profile edits remain immediately consistent and failure handling stays visible.

## Boundary Tradeoffs
- Keeping cache invalidation inside the profile-read boundary preserves ownership clarity but requires extra bootstrap wiring in the same service.
- Avoiding write-through semantics protects legacy correctness but leaves read latency partially dependent on PostgreSQL during degraded-cache periods.

## Consequences
- Startup now depends on Redis health wiring and deployment configuration.
- The rollout needs additional observability for cache hit rate, invalidation success, and degraded-cache fallback behavior.

## Unresolved Questions
- Do we need a short TTL as a secondary safety net in addition to explicit invalidation?
- Should the first rollout enable caching for all profile reads or only the highest-volume endpoints?

Owner: backend-platform
Risk Level: bounded-impact
Zone: yellow