# Adaptive Governance Adapter Projection

## Purpose

Define the machine-facing governance-adapter projection that downstream
orchestrators may consume when Canon publishes adaptive-governance companion
semantics.

## Projected Shape

The adapter projection continues to expose the existing governed packet fields
and may add an optional `adaptive_governance` object beside the existing
`authority_governance` object.

Projected fields:

- `status`
- `approval_state`
- `packet_readiness`
- `reason_code`
- `run_ref`
- `packet_ref`
- `document_refs`
- `expected_document_refs`
- `authority_governance`
- `adaptive_governance`

## Projection Rules

- `authority_governance` remains the required S3 posture baseline when downstream governance depends on Canon posture semantics.
- `adaptive_governance` is optional and is projected only when compatible companion metadata is available.
- Companion unavailability must remain distinguishable from baseline unavailability.
- The adapter projection must preserve the same semantic boundary described by Canon documentation and must not assign runtime operational behavior.

## Fail-Closed Distinctions

- Missing or unsupported required baseline semantics make the posture contract unavailable to downstream consumers.
- Missing or unsupported companion semantics make only the companion contract unavailable unless a downstream policy explicitly requires it.
- Optional companion metadata must not cause the adapter to hide or reinterpret the required baseline contract state.

## Explicit Exclusions

The adapter projection does not define:

- downstream runtime confidence values
- trust scores or trust transitions
- council assembly results
- reviewer routing
- degradation decisions
- escalation targets
- stop-transition behavior