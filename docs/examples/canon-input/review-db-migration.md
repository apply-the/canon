# Review Brief: DB Migration (V4.1)

Review Target: The proposed zero-downtime database migration strategy for splitting `user_accounts` into `users` and `credentials`.
Evidence Basis: The migration scripts in `db/migrations/2026_v4_1_split.sql`, the integration test `tests/auth_migration_test.rs`, and the attached architectural decision record on schema dual-writes.
Owner: platform-dba
Boundary Concern: The identity service cannot afford a data integrity failure or downtime during the backfill phase. A missed record means a locked-out enterprise customer. 
Pending Decision: The decision this review is expected to accept or defer: Are the dual-write transactions race-condition proof, and does the rollback script successfully revert exactly to the V4.0 state without data loss?
Open Concern: The backfill query speed against 10M rows might lock the `user_accounts` table despite the `CONCURRENTLY` index builds and chunking logic. Performance monitoring and chunk sizing remain unproven in production.

## In Scope Artifacts
- `db/migrations/2026_v4_1_split.sql`
- `db/migrations/2026_v4_1_split_down.sql`
- `tests/auth_migration_test.rs`

## Acceptance Question
- Is this zero-downtime migration plan safe to execute in the production cluster tonight?

## Out of Scope
- Actually executing the migration or monitoring it; this is pure plan review.
- Application-level changes to the new `credentials` table schemas beyond the migration definitions.