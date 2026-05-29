# Feature-Local Contract Brief: Governed Reasoning Posture Contract

## Contract Identity

- `owner`: `canon`
- `current_contract_line`: `governed_reasoning_posture_v1`
- `schema_version`: `v1`
- `stable_doc`: `docs/integration/governed-reasoning-posture-contract.md`
- `primary_consumer`: `boundline`
- `supported_boundline_window`: `0.62.x`
- `supported_canon_window`: `0.62.x`

## Purpose

Capture the Canon-owned producer contract that may require stronger bounded
reasoning inside a downstream governed stage. This feature-local brief mirrors
the stable Canon document for branch planning and review. It does not supersede
the stable document.

## Sync Rules

- The stable Canon document remains normative.
- This brief must stay aligned on contract line, required fields, supported vocabulary, and release windows.
- Any divergence between this brief and the stable Canon document is treated as drift and must be resolved before the feature is considered aligned.

## Required Producer Shape

Canon publishes these required fields for `governed_reasoning_posture_v1`:

- `contract_line`
- `boundline_min`
- `boundline_max_exclusive`
- `canon_min`
- `canon_max_exclusive`
- one of `required_profile_family` or `required_profile_id`
- `minimum_independence`
- `admission_priority`
- `confidence_handoff_required`
- `provenance_ref`

## Supported Vocabulary

Supported `required_profile_family` values:

- `self_consistency`
- `blind_review`
- `heterogeneous_review`
- `reflexion`
- `debate_enabled`

Supported explicit `required_profile_id` values:

- `bounded_self_consistency`
- `independent_pair_review`
- `heterogeneous_security_review`
- `bounded_reflexion`

Supported `admission_priority` values:

- `advisory`
- `required_before_continue`
- `required_before_acceptance`

`minimum_independence` publishes these keys:

- `route_distinct`
- `provider_distinct`
- `context_distinct`
- `prompt_pattern_distinct`
- `minimum_participants`

## Compatibility Rules

- Unsupported contract lines fail closed.
- Incompatible Boundline and Canon release windows fail closed.
- Incomplete required fields fail closed.
- Additive optional fields may be introduced only when they preserve backward compatibility for the active supported consumer window.
- Breaking changes to required fields, compatibility semantics, or supported vocabulary require a new contract line.

## Consumer Boundary

- Canon owns posture authoring, provenance, and compatibility semantics.
- Boundline owns runtime activation, participant routing, confidence synthesis, trace emission, and operator-facing execution decisions.
- The contract does not assign routes, participants, providers, or runtime prompts.

## Branch-Scope Note

This branch also contains a gatekeeper module split. That refactor is not part
of the contract payload or vocabulary. It is allowed only as behavior-
preserving maintainability follow-through for the touched Canon runtime surface.
