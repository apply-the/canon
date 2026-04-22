# Data Model: Run Identity, Display Id, and Authored-Input Refactor

**Feature**: 009-run-id-display  
**Date**: 2026-04-22

This document describes the entities and value types introduced or
extended by this feature. Implementation lives in
`crates/canon-engine/src/domain/run.rs` and
`crates/canon-engine/src/persistence/`.

## Entities

### RunManifest (extended)

Persisted per run as TOML at the run directory's `manifest.toml` (existing
location). Fields below are required unless marked optional.

| Field         | Type                       | Notes                                                                                                     |
|---------------|----------------------------|-----------------------------------------------------------------------------------------------------------|
| `uuid`        | `Uuid` (UUIDv7)            | Canonical machine identity. Immutable. Generated once at run creation.                                    |
| `run_id`      | `String` (`R-YYYYMMDD-XX`) | Display id. Derived deterministically from `uuid` + UTC `created_at`. Immutable.                          |
| `short_id`    | `String` (8 hex)           | First 8 hex chars of `uuid`'s lowercase canonical form. Derived. Immutable.                               |
| `created_at`  | `OffsetDateTime` (UTC)     | RFC 3339 with `Z` suffix. Source of `YYYYMMDD` and `YYYY/MM/`.                                            |
| `slug`        | `Option<String>`           | Sanitized lowercase ASCII slug, ≤ 60 chars. Metadata only. May be empty/absent.                           |
| `title`       | `Option<String>`           | Human-readable title. Metadata only. Free-form string.                                                    |
| `mode`        | `Mode`                     | Existing.                                                                                                 |
| `system_context` | `SystemContext`         | Existing (from feature 008).                                                                              |
| `owner`       | `String`                   | Existing.                                                                                                 |
| `risk`        | `Risk`                     | Existing.                                                                                                 |
| `zone`        | `Zone`                     | Existing.                                                                                                 |
| _existing fields_ | …                      | All current manifest fields preserved.                                                                    |

**Validation rules**:

- `uuid` MUST be a UUIDv7. Reading a manifest with a non-v7 UUID is
  permitted (read compatibility) but warned in tracing.
- `run_id` MUST match `^R-\d{8}-[0-9a-f]{8}$`.
- `short_id` MUST equal `uuid.to_string()[..8]` (lowercase).
- `created_at.date_utc()` formatted as `YYYYMMDD` MUST equal the date
  portion of `run_id`.
- `slug`, when present, MUST satisfy the sanitization invariants in
  research R-004.
- `title`, when present, MUST be a non-empty string after trimming.

**State transitions**: identity fields are write-once at run creation;
later state transitions (gate updates, evidence, approvals, resume) MUST
NOT modify them.

### RunDirectory

Filesystem container at `.canon/runs/YYYY/MM/<dir-name>/`.

`<dir-name>` shape (canonical, new runs):

```
R-YYYYMMDD-SHORTID            # slug absent
R-YYYYMMDD-SHORTID--SLUG      # slug present
```

**Parser invariant**: split on the **first** `--` only. Token 0 is the
display id; token 1 (if present) is the slug payload, which itself may
contain `--` and is treated opaquely after extraction.

**Legacy compatibility**: a directory directly under `.canon/runs/` whose
name is a UUID is also recognized as a run directory (read compat). Such
runs are not relocated by the runtime.

### AuthoredInputSurface

User-owned files under `canon-input/`:

- File form: `canon-input/<mode>.md`
- Directory form: `canon-input/<mode>/`

The runtime MUST treat this surface as read-only. No persistence path
under `crates/canon-engine/src/persistence/` may write into it.

### RunInputSnapshot

Immutable snapshot directory at `.canon/runs/<…>/inputs/` populated at
run creation.

**Fields per snapshot entry** (existing fingerprint records, retained):

- `path` (relative to snapshot root)
- `digest` (sha256 hex)
- `provenance` (source path under `canon-input/`)

Snapshot files MUST NOT be rewritten after the run is created.

### RunHandle (lookup result)

Returned by the new `persistence::lookup` resolver. Carries enough
information to dispatch any existing command:

| Field         | Type                | Notes                                                              |
|---------------|---------------------|--------------------------------------------------------------------|
| `uuid`        | `Uuid`              | From manifest.                                                     |
| `run_id`      | `String`            | From manifest.                                                     |
| `short_id`    | `String`            | From manifest.                                                     |
| `directory`   | `PathBuf`           | Absolute path to the run directory (new layout or legacy).         |
| `is_legacy`   | `bool`              | `true` when resolved against the legacy UUID-keyed layout.         |
| `created_at`  | `OffsetDateTime`    | For ordering / `@last`.                                            |

### LookupQuery

Input to the resolver:

| Variant            | Example                              | Resolution                               |
|--------------------|--------------------------------------|------------------------------------------|
| `FullRunId`        | `R-20260413-6f2b8d4e`                | Exact match on `run_id`.                 |
| `FullUuid`         | `6f2b8d4e-9d8b-7e4c-8e91-…`          | Exact match on `uuid`.                   |
| `ShortId`          | `6f2b8d4e`                           | Prefix match on `short_id`.              |
| `Last`             | `@last`                              | Greatest `run_id`, ties by `created_at`. |

**Errors**:

- `LookupError::NotFound { query }`
- `LookupError::Ambiguous { query, matches: Vec<String> }` — `matches`
  contains `run_id`s and is rendered to the user as a sorted list.
- `LookupError::EmptyHistory` — only for `@last` when no runs exist.

## Relationships

```
RunManifest 1 ─── 1 RunDirectory
RunDirectory 1 ── 1 RunInputSnapshot
RunInputSnapshot * ── 1..* AuthoredInputSurface (read-only source)
RunHandle 1 ── 1 RunManifest (via directory)
LookupQuery * ── 0..1 RunHandle (resolver output)
```

## Derivations summary

```
uuid       := Uuid::now_v7()
short_id   := uuid.to_string()[..8]            # lowercase hex
created_at := OffsetDateTime::now_utc()
date_str   := created_at.format("YYYYMMDD")    # UTC
run_id     := format!("R-{date_str}-{short_id}")
year_str   := created_at.format("YYYY")        # UTC
month_str  := created_at.format("MM")          # UTC, zero-padded
slug       := slugify(source_text)?            # see research R-004
dir_name   := match slug {
                Some(s) => format!("{run_id}--{s}"),
                None    => run_id.clone(),
              }
run_path   := canon_root / "runs" / year_str / month_str / dir_name
```
