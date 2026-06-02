# Review Brief: DB Migration (V4.1)

## Review Target

- The proposed zero-downtime database migration strategy for splitting `user_accounts` into `users` and `credentials`.

## Evidence Basis

- `db/migrations/2026_v4_1_split.sql`
- `db/migrations/2026_v4_1_split_down.sql`
- `tests/auth_migration_test.rs`
- the architectural decision record on schema dual-writes

## Boundary Findings

- Severity: high
	Location: `db/migrations/2026_v4_1_split.sql` backfill and cutover path
	Rationale: The identity service cannot tolerate data loss or downtime during the backfill phase.
	Recommended Change: Prove rollback and backfill timing before production acceptance.
- Severity: high
	Location: downstream enterprise login continuity
	Rationale: A missed record would lock out enterprise customers immediately.
	Recommended Change: Keep acceptance gated until the rehearsal evidence covers record completeness.

## Ownership Notes

- `platform-dba` owns production acceptance together with the identity service owner.

## Missing Evidence

Status: missing-evidence-open

- The backfill query speed against 10M rows is still unproven in production-like conditions.
- The rollback rehearsal has not yet demonstrated a clean return to the V4.0 schema without data loss.

## Collection Priorities

- Run a production-like backfill load test with the current chunking strategy.
- Capture a rollback rehearsal proving the down migration restores V4.0 exactly.

## Decision Impact

- If the packet is accepted, tonight's production migration can proceed under the documented dual-write plan.
- If the packet remains open, the release should hold until rollback credibility and chunk sizing evidence exist.

## Reversibility Concerns

- The migration is only reversible if the rollback script is proven against a populated dataset.
- Metric drift during dual-write is acceptable only while the rollback window remains credible.

## Final Disposition

Status: awaiting-disposition

Rationale: the packet is close to acceptable, but explicit disposition should wait for rollback rehearsal evidence and production-like backfill proof.

## Accepted Risks

- Temporary metrics skew during the dual-write window may be accepted if the rollback rehearsal passes.