# Contract: Run Identity, Display Id, and Lookup

**Feature**: 009-run-id-display  
**Date**: 2026-04-22  
**Surfaces governed**: run manifest schema, on-disk run directory layout,
authored-input vs snapshot contract, CLI run-id resolution.

This contract is binding for the implementation of feature 009 and is the
basis for the contract tests under `tests/contract/run_identity_contract.rs`.

## C-1. Manifest schema (TOML)

A new run MUST be persisted with at minimum the following fields:

```toml
[run]
uuid       = "0190f4cf-3a91-7a1c-9e8b-fa9203b1f0d4"   # UUIDv7, lowercase canonical
run_id     = "R-20260422-0190f4cf"                    # ^R-\d{8}-[0-9a-f]{8}$
short_id   = "0190f4cf"                               # uuid[..8]
created_at = "2026-04-22T10:22:03Z"                   # RFC 3339, UTC, Z suffix
mode       = "requirements"
owner      = "staff-engineer"
risk       = "bounded-impact"
zone       = "yellow"
slug       = "auth-hardening-scope"                   # optional
title      = "Auth hardening scope"                   # optional
```

**Required**: `uuid`, `run_id`, `short_id`, `created_at`, `mode`, `owner`,
`risk`, `zone`. **Optional**: `slug`, `title`.

Field grouping inside `[run]` MAY be adjusted to match existing TOML
conventions; the field names and semantics above are binding.

Read compatibility: a manifest missing `run_id`, `short_id`, or
`created_at` MUST be readable provided `uuid` is present; the missing
fields MUST be derived in-memory from `uuid` (and from the directory mtime
if `created_at` is absent) without rewriting the file.

## C-2. Display-id format

`run_id := format!("R-{YYYYMMDD}-{SHORTID}")` where:

- `YYYYMMDD` is the **UTC** date portion of `created_at`.
- `SHORTID` is the first 8 hex characters of the lowercase canonical
  UUID string.

Regex: `^R-\d{8}-[0-9a-f]{8}$`.

## C-3. Slug sanitization

`slug` MUST be the result of applying the following pipeline to a source
string (CLI flag, H1 heading, or mode default), in order:

1. Lowercase.
2. ASCII-fold; drop non-ASCII characters that have no fold.
3. Replace any run of characters not in `[a-z0-9]` with a single `-`.
4. Trim leading and trailing `-`.
5. Truncate to ≤ 60 characters; trim a trailing `-` left after truncation.
6. If the result is empty, the slug is `None`.

A persisted `slug`, when present, MUST satisfy this regex:

```
^[a-z0-9]([a-z0-9-]{0,58}[a-z0-9])?$
```

## C-4. On-disk layout

A new run directory MUST be created at:

```
.canon/runs/<YYYY>/<MM>/<DIR_NAME>/
```

where `YYYY` and `MM` are the UTC year and zero-padded month of
`created_at`, and:

```
DIR_NAME := if slug.is_some() { format!("{run_id}--{slug}") } else { run_id }
```

Directory-name parsing rule (binding):

- Use `dir_name.split_once("--")`.
- The portion **before** the first `--` is the display id.
- The portion **after** the first `--` (which itself MAY contain `--`)
  is the slug payload and is treated opaquely after this split.

Legacy compatibility: a directory directly under `.canon/runs/` whose
name parses as a UUID is also a run directory. The runtime MUST NOT
relocate it.

## C-5. Authored input vs run snapshot

- `canon-input/<mode>.md` and `canon-input/<mode>/` are the authored
  surface. The runtime MUST NOT write to any path under `canon-input/`
  during run creation, persistence, gate updates, invocation persistence,
  verification, evidence emission, approve, or resume.
- On run creation, the runtime MUST snapshot the authored sources used
  by the run into `.canon/runs/<…>/inputs/`, preserving content digests
  and provenance records that already exist in `evidence.toml`-adjacent
  fingerprints.
- Snapshot files MUST NOT be rewritten after the run is created.
- Mutating an authored file after run creation MUST NOT mutate the
  snapshot; reload, status, inspect, approve, and resume MUST continue
  to function.

## C-6. CLI run resolution

The resolver MUST accept the following query forms and resolve them to a
unique `RunHandle`:

| Form          | Example                                          | Match rule                                  |
|---------------|--------------------------------------------------|---------------------------------------------|
| Full `run_id` | `R-20260422-0190f4cf`                            | Exact match on `run_id`.                    |
| Full `uuid`   | `0190f4cf-3a91-7a1c-9e8b-fa9203b1f0d4`           | Exact match on `uuid`.                      |
| `short_id`    | `0190f4cf`                                       | Prefix match on `short_id`.                 |
| `@last`       | `@last`                                          | Greatest `run_id`; ties by `created_at`.    |

Behavior:

- Successful resolution MUST return one `RunHandle`.
- Ambiguous resolution (multiple matches) MUST return an error that
  enumerates the matching `run_id`s in deterministic order; the resolver
  MUST NOT pick one silently.
- An empty `@last` query against an empty repository MUST fail with a
  clear, dedicated error.
- Lookups MUST work uniformly across `status`, `inspect evidence`,
  `inspect artifacts`, `approve`, `resume`, and the listing surface.

## C-7. Listing

A listing surface (extension of an existing CLI command, e.g.
`canon list runs`) MUST emit, per row, at least:

- `run_id`
- `mode`
- `slug` or `title` (whichever is present; column header is
  `slug/title`)
- `created_at` (RFC 3339, UTC)
- `state`

Default sort: descending `created_at`. Both new-layout and legacy
UUID-keyed runs MUST appear in the listing.

## C-8. Backward compatibility

- Reading manifests that pre-date this feature MUST succeed; missing
  derived fields MUST be reconstructed in-memory.
- Lookups by full UUID MUST continue to resolve runs whose directory
  name is the bare UUID.
- No code path may move, rename, or delete a legacy run directory as a
  side effect of read, status, inspect, approve, resume, or list.
- An explicit migration command, if added later, is out of scope for
  this contract.
