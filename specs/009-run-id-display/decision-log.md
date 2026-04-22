# Decision Log: Run Identity, Display Id, and Authored-Input Refactor

**Feature**: 009-run-id-display  
**Date**: 2026-04-22  
**Status**: Open — entries are added as decisions are made.

Decisions seeded by [spec.md](spec.md) and refined by [research.md](research.md).
Each entry records context, alternatives, decision, rationale, and
consequences.

---

## D-001: UUIDv7 as canonical machine identity

- **Context**: The engine already calls `Uuid::now_v7()` in seven places.
  The refactor needs an immutable canonical id that is independent of
  human-readable display.
- **Alternatives considered**: UUIDv4; ULID; KSUID; numeric counter.
- **Decision**: Keep UUIDv7.
- **Rationale**: Already in use; adds embedded time ordering at no cost;
  matches `uuid = { features = ["v7"] }` in `Cargo.toml`.
- **Consequences**: `short_id` derivation is pinned to the lowercase hex
  canonical form. Implementation must not silently downgrade to v4.

## D-002: UTC for date components and `created_at`

- **Context**: Display id includes `YYYYMMDD`; filesystem layout includes
  `YYYY/MM/`. Operators run Canon across timezones.
- **Alternatives considered**: Local time; ISO week buckets.
- **Decision**: Use UTC for everything date-related; store `created_at`
  in RFC 3339 with `Z`.
- **Rationale**: Eliminates per-machine drift; matches existing time
  handling.
- **Consequences**: A run started at 23:30 local time (UTC+10) lands in
  the next UTC date's `R-…-` and `YYYY/MM/` bucket. Documented in the
  quickstart and assumptions.

## D-003: `short_id = first 8 hex chars of UUID`

- **Context**: Need a human-friendly handle that ties back to the UUID.
- **Alternatives considered**: Crockford base32; 6 hex chars; 12 hex
  chars.
- **Decision**: 8 hex chars of the canonical lowercase UUID.
- **Rationale**: ~4.3 billion values, comfortably resolves typical repo
  scale; substring-aligned with full UUID (lookup symmetry); matches
  established short-hash conventions.
- **Consequences**: Collisions are possible at large scale; resolver
  reports ambiguity rather than guessing.

## D-004: Slug is metadata; first `--` is the identity boundary

- **Context**: Directory names embed both display id and slug. Slugs may
  contain `--`. Identity parsing must remain trivial.
- **Alternatives considered**: Disallow `--` in slugs; use a different
  separator (`__`, `=`); store slug only in manifest.
- **Decision**: Slug lives in directory name after a single `--`
  separator; parsing always uses `split_once("--")`. Slug is descriptive
  metadata only.
- **Rationale**: Trivial, unambiguous parser; slug never affects
  identity; grep-friendly listings.
- **Consequences**: Slug payload may itself contain `--`; this is fine
  because no later `--` is treated as identity-bearing.

## D-005: Read-compat for legacy UUID-keyed run directories; no silent migration

- **Context**: Existing repos already contain `.canon/runs/<UUID>/` run
  directories.
- **Alternatives considered**: Bulk migrate on first read; refuse legacy
  runs; silently move on lookup.
- **Decision**: Read-compat only. Lookup, status, inspect, approve,
  resume, and list all work for legacy directories. No relocation. New
  runs always use the new dated layout.
- **Rationale**: Avoids destructive moves; keeps the change low-risk for
  current operators. An explicit, narrow migration command can be
  considered later.
- **Consequences**: Two layouts coexist on disk during transition. The
  resolver and lister must walk both. Documented in research R-005 and
  R-010.

## D-006: Centralized resolver in `persistence::lookup`

- **Context**: Five existing CLI commands (`status`, `inspect`,
  `approve`, `resume`, plus list) all need to map a query to a run.
- **Alternatives considered**: Per-command resolution; resolver inside
  CLI layer.
- **Decision**: Place the resolver in
  `crates/canon-engine/src/persistence/lookup.rs` and have CLI commands
  call it.
- **Rationale**: Single source of truth for ambiguity reporting and
  legacy compat; keeps CLI thin.
- **Consequences**: Adds one new module and one new error type. Worth
  the boundary clarity.

## D-007: `@last` is in-scope because the resolver gives it for free

- **Context**: Spec FR-018 makes `@last` optional, gated on a clean fit.
- **Alternatives considered**: Defer `@last` to a follow-up.
- **Decision**: Implement `@last` in the central resolver.
- **Rationale**: Marginal cost once the resolver exists; high operator
  ergonomics; tests are trivial.
- **Consequences**: Adds a `LookupQuery::Last` variant and an
  `EmptyHistory` error.

## D-008: Authored-input non-mutation enforced by store boundary, not by best
practice

- **Context**: The historical risk is that a persistence path
  accidentally writes back into `canon-input/`.
- **Alternatives considered**: Document the rule and rely on review.
- **Decision**: Add a contract test
  (`tests/integration/inputs_snapshot_immutability.rs`) that verifies
  authored files' digests are unchanged across run create / status /
  inspect / approve / resume. Persistence code paths MUST only write
  beneath `.canon/`.
- **Rationale**: Layered verification (Constitution X). Cheap, decisive
  protection of an invariant.
- **Consequences**: Any future code that needs to "fix up" authored
  files MUST do so via an explicit, user-driven command, not as a side
  effect of a run.

---

## Open questions

_None at planning time._
