# Contract: Governance Adapter CLI

## Purpose

Define the public machine-facing CLI contract Canon exposes to external
orchestrators for governed `start`, `refresh`, and `capabilities` operations.

## Command Surface

- `canon governance start --json`
- `canon governance refresh --json`
- `canon governance capabilities --json`

## Request Contract

### Compatibility Rules

- Requests are encoded as exactly one JSON object on standard input.
- `adapter_schema_version` is optional in `v1`; omitted version markers must be
  interpreted as `v1` compatibility input.
- `request_kind` values are `start` and `refresh` and must match the invoked
  operation.
- Unknown additive request fields within a supported schema version must not
  break compatibility.

### Domain-Required Fields For Successful Execution

- Start requests require `request_kind`, `governance_attempt_id`, `stage_key`,
  `goal`, `workspace_ref`, `mode`, `system_context`, `risk`, `zone`, and
  `owner` for successful domain execution.
- Refresh requests require all successful start fields plus `run_ref`.
- Missing domain-required fields in a well-formed request must yield blocked
  domain outcomes rather than protocol failure.

### Optional v1 Fields

- `autopilot`
- `packet_ref`
- `bounded_context`
- `input_documents`

### Boundary Rules

- `governance_attempt_id` and `stage_key` are opaque consumer correlation
  fields.
- `workspace_ref` must bind to an accessible workspace.
- Consumer-supplied document paths and bounded-context refs must remain inside
  the declared workspace boundary.

## Response Contract

### Required v1 Fields

- `adapter_schema_version`
- `status`
- `approval_state`
- `message`

### Optional v1 Fields

- `run_ref`
- `packet_ref`
- `expected_document_refs`
- `document_refs`
- `packet_readiness`
- `missing_fields`
- `missing_sections`
- `headline`
- `reason_code`

### Published Vocabularies

- `status`: `pending_selection`, `running`, `governed_ready`,
  `awaiting_approval`, `blocked`, `completed`, `failed`
- `approval_state`: `not_needed`, `requested`, `granted`, `rejected`,
  `expired`
- `packet_readiness`: `pending`, `incomplete`, `reusable`, `rejected`

### Consistency Rules

- `status: governed_ready` requires `packet_readiness: reusable`, a present
  `packet_ref`, and a non-empty `document_refs` set.
- `status: awaiting_approval` requires `approval_state: requested`.
- `reason_code` is mandatory for `blocked` and `failed` outcomes.
- `reason_code` should also be present for `awaiting_approval` outcomes.
- `missing_fields`, when present, names request-validation fields that blocked
  domain execution and is distinct from packet reuse gaps.
- `missing_sections`, when present, names packet projection gaps only and must
  not be used for request-precondition failures.
- Refresh failures caused by unreadable or missing persisted packet contracts
  use `reason_code: artifact_contract_unreadable` or
  `reason_code: artifact_contract_missing`.
- `headline`, when present, must remain semantically consistent with the
  response outcome and may be omitted in favor of `message`.
- `packet_ref`, `expected_document_refs`, and `document_refs` must be canonical
  workspace-relative refs.

## Capabilities Contract

The capabilities response must include:

- `canon_version`
- `supported_schema_versions`
- `operations`
- `supported_modes`
- `status_values`
- `approval_state_values`
- `packet_readiness_values`

The capabilities response should also include additive compatibility notes so a
consumer can distinguish stable `v1` guarantees from future optional
extensions.

## Domain Outcomes Vs Protocol Failures

- Well-formed requests that reach Canon's domain logic return exit code `0`,
  including blocked, failed, approval-gated, running, completed, or
  governed-ready outcomes.
- Malformed JSON, unsupported schema versions, and CLI misuse may return
  non-zero exit codes and are protocol failures rather than domain outcomes.

## Release Alignment Contract

- This contract ships as part of `0.35.0`.
- Runtime-compatibility references, release docs, and `CHANGELOG.md` must
  describe the same adapter surface and vocabulary.