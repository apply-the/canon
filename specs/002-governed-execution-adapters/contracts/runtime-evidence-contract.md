# Runtime Evidence Contract: Governed Execution Adapters

## Scope

This contract extends Canon's runtime filesystem contract with invocation and
evidence records.

## Required Run-Level Additions

```text
.canon/runs/<run-id>/
├── evidence.toml
└── invocations/
    └── <request-id>/
        ├── request.toml
        ├── decision.toml
        ├── attempt-01.toml
        └── payload/
```

## Required Files

### `request.toml`

- request id
- run id
- adapter
- capability
- orientation
- requested scope
- requested mutability
- policy evaluation prerequisites snapshot

### `decision.toml`

- policy decision kind
- constraint set
- approval requirement, if any
- rationale
- effective policy references

### `attempt-XX.toml`

- attempt number
- started and finished timestamps
- executor metadata
- normalized outcome kind
- retained payload references

### `evidence.toml`

- generation path summaries
- validation path summaries
- denied invocation refs
- approval refs
- decision refs
- artifact provenance refs

## JSONL Trace Stream

`.canon/traces/<run-id>.jsonl` continues to exist and must contain append-only
events for:

- request persisted
- decision persisted
- approval required
- approval granted or rejected
- dispatch started
- outcome recorded
- evidence bundle updated
- runtime denial recorded for unsupported adapters such as `McpStdio`

## Retention Rules

- raw payloads are optional and policy-governed
- summaries and digests are mandatory for consequential invocations
- trace events must remain readable without opening raw payloads

## Compatibility

- existing run manifests remain valid
- `links.toml` gains `evidence` and `invocations` references
- prior runs without invocation manifests remain readable as pre-governed
  execution runs
- gate-scoped approvals remain readable while new invocation-scoped approvals
  are added
