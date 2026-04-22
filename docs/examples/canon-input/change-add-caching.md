# Change Brief: Add Read-Through Caching to Profiles

System Slice: The unbounded load on the `user_profiles_db` from repeated API reads globally. We need to introduce distributed caching in front of `ProfileService`.
Intended Change: Add a Redis read-through layer into `crates/profile-engine/src/repository.rs`.
Legacy Invariants: Writing to a user profile must synchronously persist to PostgreSQL. The cache invalidation must not mask a database failure and must serve reads at 5ms latency consistently.
Change Surface: Only the read/write paths in `crates/profile-engine/`, and the API container environment configurations handling REDIS_URL.
Implementation Plan: Step 1: Add Redis connection pool to process startup. Step 2: Decorate `get_profile_by_id` with a Redis check and failover to PG. Step 3: Decorate `update_profile` with Redis `DEL` invalidation after PG commit.
Validation Strategy: Verify via load tests that database connection limits are no longer hit under 500 RPS load. Confirm stale reads do not exceed 10ms after an update using synthetic test clients.
Decision Record: Chosen an explicit cache-aside and invalidation over TTL-only expiration to guarantee profile consistency immediately on user-driven edits.
Owner: backend-platform
Risk Level: bounded-impact
Zone: yellow