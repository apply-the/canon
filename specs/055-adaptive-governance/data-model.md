# Data Model: Adaptive Governance Semantics

## GovernanceStateVocabulary

Represents the Canon-owned S4 semantic governance states.

Values:
- `advisory`
- `catch`
- `rule`
- `hook`

Meaning notes:
- these values describe semantic posture and maturity, not executable runtime behavior
- downstream runtimes remain responsible for what operational behavior each state triggers locally

## RolloutProfileVocabulary

Represents the Canon-owned S4 maturity labels for governance adoption.

Values:
- `minimal`
- `guided`
- `governed`
- `strict`

Validation rules:
- rollout profiles remain distinct from S3 authority zones
- rollout profiles remain distinct from downstream council profiles

## AdaptiveGovernanceV1Envelope

Represents the optional Canon-owned S4 companion contract published alongside
the required `authority-governance-v1` posture baseline when machine-readable
adaptive semantics are emitted.

Required fields:
- `contract_line`
- `governance_state`
- `rollout_profile`

Optional fields:
- `state_rationale`
- `profile_rationale`

Validation rules:
- `contract_line` must equal `adaptive-governance-v1` for first-slice companion compatibility
- missing required fields invalidate the companion envelope for adaptive-governance compatibility
- missing optional rationale fields leave the compatible remainder of the envelope usable
- the companion envelope is optional overall and cannot repair a missing required `authority-governance-v1` baseline

## GovernedSemanticPairing

Represents the relationship between the required S3 posture baseline and the
optional S4 companion semantics.

Fields:
- `authority_governance`: required baseline semantic envelope when downstream governance relies on Canon posture semantics
- `adaptive_governance`: optional companion envelope when Canon emits adaptive-governance semantics
- `compatibility_state`: whether the baseline is compatible, the companion is compatible, both are compatible, or the companion is unavailable

Validation rules:
- downstream consumers may use the baseline without the companion
- downstream consumers may not treat the companion as a replacement for the baseline

## AdaptiveGovernanceAdapterProjection

Represents the machine-facing projection of adaptive-governance data available
to downstream orchestrators.

Projected fields:
- `status`
- `approval_state`
- `packet_readiness`
- `reason_code`
- `run_ref`
- `packet_ref`
- `document_refs`
- `expected_document_refs`
- `authority_governance`: optional structured projection of the required baseline envelope when packet metadata is available
- `adaptive_governance`: optional structured projection of `AdaptiveGovernanceV1Envelope` when the companion metadata is available

## Relationship Notes

- `GovernanceStateVocabulary` and `RolloutProfileVocabulary` are Canon semantic terms, not downstream runtime instructions.
- `AdaptiveGovernanceV1Envelope` is additive to, not a replacement for, `authority-governance-v1`.
- `GovernedSemanticPairing` keeps the required baseline and optional companion semantics distinguishable for consumers.
- `AdaptiveGovernanceAdapterProjection` exposes the same semantic boundary through the machine-facing adapter path used by downstream systems.