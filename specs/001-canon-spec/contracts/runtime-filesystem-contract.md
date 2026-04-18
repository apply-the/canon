# Runtime Filesystem Contract: Canon v0.1

## Contract Scope

This contract defines the externally inspectable runtime layout created inside a
governed repository.

## Root Layout

```text
.canon/
├── sessions/
├── artifacts/
├── decisions/
├── traces/
├── methods/
├── policies/
└── runs/
```

## Required Files and Directories

### `.canon/methods/`

- versioned TOML files for all supported modes
- file names match mode names
- unknown method files are ignored unless explicitly referenced

### `.canon/policies/`

- versioned TOML policy files for risk, zones, gates, verification, and
  adapters
- unknown fields fail policy loading

### `.canon/runs/<run-id>/`

Required files:

- `run.toml`
- `context.toml`
- `artifact-contract.toml`
- `state.toml`
- `links.toml`

Required directories:

- `gates/`
- `approvals/`
- `verification/`
- `invocations/`

Conditional directories:

- `inputs/` when the run captured one or more authored file-backed inputs

### `.canon/artifacts/<run-id>/<mode>/`

- contains all emitted artifacts for that run and mode
- file names are determined by the artifact contract
- machine-readable artifact manifests may appear alongside Markdown artifacts
- artifact records must resolve only to paths under the matching
  `.canon/artifacts/<run-id>/<mode>/` directory
- run-scoped analysis, summaries, and review packets must not be emitted as
  ad-hoc files in the repository root

### `.canon/decisions/`

- append-only decision records
- file names include the decision id
- decision records remain valid after a run is superseded

### `.canon/traces/`

- one JSONL trace stream per run
- each line represents one adapter invocation or related evidence event

## Runtime Contract Rules

- no run may exist without `run.toml`, `context.toml`, and `state.toml`
- no gate may be considered passed unless a persisted gate result exists under
  the run directory
- no artifact may satisfy the contract unless an `ArtifactRecord` points to it
- no `ArtifactRecord` may use absolute paths, `.` segments, `..` traversal, or
  any other relative path that escapes `.canon/artifacts/<run-id>/<mode>/`
- no override may exist without both an approval record and a linked decision
  record
- reruns create new run directories; they do not overwrite prior run state
- authored file-backed inputs, when present, are snapshotted under
  `.canon/runs/<run-id>/inputs/` and referenced from `context.toml`

## Compatibility Expectations

- file names and directory names in this contract are part of the v0.1 public
  surface
- additional files may appear, but required files may not disappear without a
  contract version bump
- future versions may add fields to TOML or JSONL records, but existing fields
  should remain readable
