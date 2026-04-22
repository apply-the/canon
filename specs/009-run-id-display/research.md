# Research: Run Identity, Display Id, and Authored-Input Refactor

**Feature**: 009-run-id-display  
**Date**: 2026-04-22

This document resolves design questions raised by [spec.md](spec.md) and
[plan.md](plan.md) before Phase 1 design. No `[NEEDS CLARIFICATION]`
markers remain.

## R-001: UUID variant for canonical identity

**Decision**: Keep UUIDv7 (`uuid::Uuid::now_v7`), already used by the
engine across `crates/canon-engine/src/orchestrator/service.rs`.

**Rationale**:

- The engine already generates UUIDv7 in seven separate code paths today.
  Switching to v4 would be a regression and would lose v7's monotonic
  embedded-timestamp ordering.
- v7's time-ordering helps lexicographic listing and debugging without
  changing any identity semantics.
- The `uuid` dependency is already pinned with the `v7` feature in
  `Cargo.toml`.

**Alternatives considered**:

- UUIDv4: rejected — no time ordering, would require re-tooling existing
  generation sites for no benefit.
- ULID / KSUID: rejected — introduces a new dependency and identity
  vocabulary; out of scope.

## R-002: `short_id` derivation

**Decision**: `short_id = first 8 hex characters of the canonical
lowercase UUID string`.

**Rationale**:

- 8 hex chars = 32 bits ≈ 4.3 billion values → adequate uniqueness within
  a single repository's run set, where collisions are reported clearly
  rather than silently resolved.
- Trivial to compute (`uuid.to_string()[..8]`) and trivial to recognize
  visually.
- Matches the leading bytes of the UUID canonical form, so substring
  matching against full UUIDs in lookup is also straightforward.

**Alternatives considered**:

- Base32 / Crockford encoding: rejected — adds an encoder, hides the
  relationship to the underlying UUID.
- 6 hex chars: rejected — collision probability noticeably higher, less
  comfortable margin.
- 12 hex chars: rejected — longer than necessary for the human-friendly
  use case; 8 chars is the long-standing git short-hash convention.

## R-003: Date component and timezone

**Decision**: `YYYYMMDD` portion of `run_id` and the `YYYY/MM/` filesystem
buckets are derived from the UTC date of `created_at`. `created_at` is
stored in canonical RFC 3339 with `Z` suffix.

**Rationale**:

- Eliminates ambiguity across machines, CI runners, and contributors in
  different timezones.
- Aligns with existing time handling (`time` crate with `serde` /
  `formatting` features).
- Filesystem buckets remain stable across reloads and across operators.

**Alternatives considered**:

- Local time: rejected — would produce different paths on different
  machines for the same logical run instant.
- ISO week-numbered buckets: rejected — harder for operators to navigate
  by eye.

## R-004: Slug derivation source and sanitization

**Decision**:

- Source priority: (1) explicit `--slug` / `--title` CLI argument when
  present, (2) first H1 heading of the authored input file when
  available, (3) mode summary fallback (e.g. `mode-default`), (4) empty
  → no slug suffix on the directory name.
- Sanitization rules (all applied in order):
  1. Lowercase.
  2. ASCII-fold; drop non-ASCII characters that have no fold.
  3. Replace any run of non-`[a-z0-9]` with a single `-`.
  4. Trim leading/trailing `-`.
  5. Cap length at 60 characters; if truncation lands mid-word, trim
     trailing `-` again.
  6. If the result is empty, slug is `None`.

**Rationale**:

- Lowercase ASCII keeps directory names case-insensitive-safe (macOS,
  Windows) and grep-friendly.
- 60-character cap keeps total path length comfortably under typical
  filesystem limits when nested under `.canon/runs/YYYY/MM/`.
- A deterministic, side-effect-free pipeline is easy to test.

**Alternatives considered**:

- Allow Unicode slugs: rejected — invites filesystem and grep
  inconsistencies across platforms.
- Title-case slugs: rejected — case-insensitive filesystems would create
  apparent collisions.

## R-005: On-disk layout and parsing

**Decision**:

- New layout: `.canon/runs/YYYY/MM/R-YYYYMMDD-SHORTID[--slug]/`.
- Directory-name parser: split on the **first** `--` only.
  - Token 0 = `R-YYYYMMDD-SHORTID` (the display id)
  - Token 1 (if present) = slug payload (which itself may contain `--`)
- Read compatibility: when scanning `.canon/runs/`, the resolver also
  recognizes a legacy run directory at `.canon/runs/<UUID>/` (UUID-shaped
  folder name) and reads it as if it were a current run.
- New runs are always written under the new dated layout. Old runs are
  never moved by the runtime.

**Rationale**:

- `split_once("--")` is trivial and unambiguous; nothing in the slug
  payload can corrupt identity parsing.
- Read-only legacy compatibility avoids destructive moves on existing
  repos and keeps the change low-risk.
- A separate, narrow, opt-in migration command can be added later if
  desired; it is intentionally out of scope here.

**Alternatives considered**:

- `_` as separator: rejected — slugs can plausibly contain `_`, and `--`
  is the established human-readable separator (used elsewhere in the
  ecosystem, e.g. shell long-options, and visually distinct).
- Bulk migration of legacy directories on first run: rejected — silent
  destructive moves violate the spec invariants.

## R-006: Authored inputs vs run snapshot

**Decision**:

- `canon-input/<mode>.md` and `canon-input/<mode>/` are user-owned. The
  runtime MUST NOT write to them after authoring.
- On run creation, the runtime snapshots the relevant authored files into
  `.canon/runs/<…>/inputs/`, computing and persisting digests as today.
- The snapshot directory and its contents are immutable for the life of
  the run. Subsequent runtime persistence (gates, invocations,
  verification, evidence) MUST NOT touch the snapshot.

**Rationale**:

- Enforces the spec invariant of editable authoring surface vs immutable
  evidence.
- Matches how `evidence.toml`, `inputs/`, and snapshot fingerprints are
  already structured in `crates/canon-engine/src/persistence/store.rs`.

**Alternatives considered**:

- Treat `canon-input/` as both authoring and snapshot surface: rejected
  — corrupts auditability and was the original problem motivating this
  refactor.

## R-007: CLI run resolution layer

**Decision**: Centralize resolution in a new `persistence::lookup` module
that takes any of: full `run_id`, full `uuid`, prefix `short_id`, or
optional `@last`, and returns either a unique `RunHandle` or a clear
ambiguity error containing matching `run_id`s.

**Rationale**:

- A single resolver keeps `status`, `inspect`, `approve`, `resume`, and
  `list` consistent and prevents per-command drift.
- Ambiguity surfaces uniformly across commands.

**Alternatives considered**:

- Per-command resolution: rejected — duplicates logic and risks divergent
  behavior.
- Database / index file: rejected — out of scope; the on-disk layout is
  fast enough for expected scale and avoids new persistence.

## R-008: `@last` resolution

**Decision**: Implement `@last` in the same `lookup` module, returning the
run with the lexicographically greatest `run_id` (which, by virtue of
UTC-date prefix and v7 short id, equals the most-recent `created_at` in
the overwhelming common case). On exact-tie at second resolution, prefer
the run with the latest `created_at` from the manifest.

**Rationale**:

- Ergonomic sugar that costs little once the central resolver exists.
- Optional per spec FR-018; gated on the resolver landing cleanly.

**Alternatives considered**:

- Skip `@last`: still acceptable per spec; adopted only because the
  resolver gives it for free.

## R-009: Listing surface

**Decision**: Extend (or add) `canon list runs` to walk
`.canon/runs/YYYY/MM/` plus legacy UUID directories, parse each manifest,
and print rows with `run_id`, `mode`, `slug`-or-`title`, `created_at`,
and `state`. Default sort: descending `created_at`.

**Rationale**:

- Closes the user story 3 acceptance scenario.
- Reuses the same lookup/parse code paths as resolution.

**Alternatives considered**:

- Rely on shell (`ls .canon/runs/**`): rejected — no manifest summary,
  no state column, no legacy unification.

## R-010: Read-compatibility scope

**Decision**: Read compatibility = lookup, status, inspect, approve,
resume, and list all work for legacy UUID-keyed run directories. Write
operations against legacy directories continue to write inside that
legacy directory (no rewrite to new layout). New runs always use new
layout.

**Rationale**:

- Avoids destructive moves while keeping operators productive on
  existing repos.
- Matches spec FR-015 / FR-020.

**Alternatives considered**:

- Read-only mode for legacy runs (no approve/resume): rejected —
  unnecessarily punishes existing users.
