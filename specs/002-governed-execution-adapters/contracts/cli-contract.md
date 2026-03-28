# CLI Contract: Governed Execution Adapters

## Scope

This contract defines the implemented user-visible CLI surface added or changed
by the governed execution increment.

## Existing Commands Extended

### `canon run`

- continues to start a run for a given mode
- may now emit invocation-related summary fields in JSON output:
  - `invocations_total`
  - `invocations_denied`
  - `invocations_pending_approval`
  - `evidence_bundle`

### `canon status --run <run-id>`

- must report pending invocation approvals separately from gate approvals
- must report whether validation independence is satisfied for consequential
  outputs

### `canon approve`

Implemented shape:

```text
canon approve --run <run-id> --target gate:<gate-kind>|invocation:<request-id> --decision approve|reject --by <owner> --rationale <text>
```

Rules:

- invocation approvals attach to `request_id`
- approvals are invalidated if the request’s scope or context changes before
  resume
- gate targets remain supported for brownfield and pr-review disposition flows

### `canon resume --run <run-id>`

- re-evaluates pending invocation requests
- refuses to continue if the approved request is stale
- creates a new attempt when retrying the same valid request

## New Inspection Commands

### `canon inspect invocations --run <run-id>`

Outputs:

- request id
- adapter
- capability
- orientation
- policy decision
- approval state
- latest outcome
- linked artifacts
- linked decisions
- linked evidence bundle

Formats:

- `json`
- `yaml`
- `markdown`

### `canon inspect evidence --run <run-id>`

Outputs:

- generation paths
- validation paths
- independence assessments
- denied invocations
- approval links
- decision links
- artifact provenance links

Formats:

- `json`
- `yaml`
- `markdown`

## Non-Goals

- no command for generic adapter registration
- no arbitrary plugin management surface
- no queue-management commands

## First Tranche Expectation

- `requirements`, `brownfield-change`, and `pr-review` fully support governed
  invocation, evidence linkage, and inspection
- `requirements` and `brownfield-change` support invocation-scoped approvals
  plus `resume`
- `pr-review` preserves diff inspection payload references and review evidence
  while gate-scoped review disposition approvals remain supported
- MCP runtime execution remains denied even though MCP stays modeled in the
  domain and policy surfaces
