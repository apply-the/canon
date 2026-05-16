# Authority Governance Adapter Projection

## Purpose

Define the minimum machine-facing governance-adapter projection that Canon must
keep stable when `authority-governance-v1` metadata is available.

## Stable Adapter Fields

The existing flat governance-adapter response remains authoritative for:

- `status`
- `approval_state`
- `packet_readiness`
- `reason_code`
- `run_ref`
- `packet_ref`
- `document_refs`
- `expected_document_refs`

## Authority Projection Rules

- When compatible governed packet metadata is available, the adapter may also
  project an `authority_governance` object carrying the
  `AuthorityGovernanceV1Envelope`.
- The projected `authority_governance` object must preserve the required versus
  optional field profile defined by `authority-governance-v1`.
- The adapter must not fabricate absent optional provenance.
- The adapter must not reinterpret `stage_role_hints` as runtime directives.

## Consumer Safety Rules

- If `authority_governance` is absent, consumers may still use the stable flat
  lifecycle fields without assuming authority semantics exist.
- If `authority_governance` is present but missing required fields, consumers
  must treat the authority contract as unavailable.
- Additional optional metadata may be ignored by older consumers.