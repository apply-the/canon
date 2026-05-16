# Data Model: Authority Zone Contract

## AuthorityZone

Represents the Canon-owned authority classification exported through
`authority-governance-v1`.

Values:
- `green`
- `yellow`
- `red`
- `restricted`

## ChangeClass

Represents the Canon-owned change-impact classification exported through
`authority-governance-v1`.

Values:
- `low-impact`
- `bounded-impact`
- `systemic-impact`
- `critical-operations`

## IntendedPersonaProfile

Represents the Canon-owned authoring posture attached to a governed mode.

Fields:
- `intended_persona`: required persona label for the mode.
- `persona_anti_behaviors`: optional ordered list of anti-behaviors that further
  bound the posture.

Validation rules:
- `intended_persona` is required for `authority-governance-v1`.
- `persona_anti_behaviors` may be absent without invalidating the required
  persona semantics.

## StageRoleHint

Represents one advisory-only downstream role or capability hint.

Fields:
- `hint_kind`: advisory hint category such as reviewer capability or posture.
- `value`: stable hint label.
- `rationale`: optional explanation for why the hint exists.

Validation rules:
- Unknown hints are ignorable by compatible consumers.
- Hints must never encode provider routes, model routes, councils, or stop
  directives.

## AuthorityGovernanceV1Envelope

Represents the typed Canon contract envelope published with governed packet
metadata and projected by the machine-facing governance adapter.

Required fields:
- `contract_line`
- `authority_zone`
- `change_class`
- `intended_persona`
- `approval_state`
- `packet_readiness`
- `risk`

Optional fields:
- `persona_anti_behaviors`
- `primary_artifact`
- `artifact_order`
- `promotion_refs`
- `stage_role_hints`

Validation rules:
- `contract_line` must equal `authority-governance-v1` for first-slice
  compatibility.
- Missing required fields invalidate the envelope for consumer conformance.
- Missing optional fields leave the compatible remainder of the envelope usable.

## AuthorityGovernanceAdapterProjection

Represents the machine-facing projection of authority-governance data available
to external orchestrators.

Projected fields:
- `status`
- `approval_state`
- `packet_readiness`
- `reason_code`
- `run_ref`
- `packet_ref`
- `document_refs`
- `expected_document_refs`
- `authority_governance`: optional structured projection of the
  `AuthorityGovernanceV1Envelope` when packet metadata is available

## Relationship Notes

- `AuthorityGovernanceV1Envelope` attaches to governed packet metadata and may
  also appear in the governance adapter projection.
- `IntendedPersonaProfile` and `StageRoleHint` are semantic components used to
  populate the envelope from Canon mode metadata.
- `AuthorityZone` and `ChangeClass` are Canon-owned contract vocabulary and are
  not runtime directives for downstream systems.