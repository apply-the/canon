# Canon Governed Reasoning Posture Contract

## Contract Identity

- `owner`: `canon`
- `current_contract_line`: `governed_reasoning_posture_v1`
- `schema_version`: `v1`
- `stable_doc`: `tech-docs/integration/governed-reasoning-posture-contract.md`
- `primary_consumer`: `boundline`
- `supported_boundline_window`: `0.62.x`
- `supported_canon_window`: `0.63.x`

## Purpose

Define the Canon-owned posture signal that may require stronger bounded
challenge inside a Boundline governed stage. This contract governs the producer
shape, compatibility window, and vocabulary Canon publishes; it does not define
Boundline runtime orchestration.

## Authority And Sync Rules

- This stable document is the normative source for the reasoning-posture
  producer shape and compatibility window.
- Boundline feature-local briefs may mirror or elaborate the consumer side, but
  they do not supersede this Canon-owned contract.
- If this stable contract and a Boundline feature brief diverge, the stable
  Canon document wins until both repositories are realigned.
- Canon owns posture authoring, posture provenance, and contract-line
  evolution; Boundline owns runtime activation, participant routing, trace
  emission, and operator-facing execution summaries.

## Producer Shape

```toml
contract_line = "governed_reasoning_posture_v1"
boundline_min = "0.62.0"
boundline_max_exclusive = "0.63.0"
canon_min = "0.63.0"
canon_max_exclusive = "0.64.0"
required_profile_family = "blind_review"
admission_priority = "required_before_acceptance"
confidence_handoff_required = true
provenance_ref = "packet:reasoning-posture-123"

[minimum_independence]
route_distinct = true
provider_distinct = true
context_distinct = false
prompt_pattern_distinct = true
minimum_participants = 2
```

## Required Fields

Canon MUST publish all of the following fields for
`governed_reasoning_posture_v1`:

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

`minimum_independence` MUST publish these keys:

- `route_distinct`
- `provider_distinct`
- `context_distinct`
- `prompt_pattern_distinct`
- `minimum_participants`

## Compatibility Rules

- Canon MUST fail closed on unsupported or incomplete producer data before the
  posture is published to consumers.
- Additive optional fields MAY be introduced without a new contract line only
  when they preserve backward compatibility for Boundline `0.62.x`.
- Any breaking change to required fields, compatibility semantics, or the
  supported vocabulary requires a new contract line.

## Consumer Boundary

- Canon does not choose Boundline routes, participant identities, or runtime
  prompting patterns.
- Canon does not adjudicate final Boundline acceptance authority.
- Boundline may reject posture inputs that fall outside the contracted
  compatibility window or omit required fields.

## Explicit Exclusions

- No Canon-owned execution loop for debate, reflexion, or self-consistency
- No Canon-owned participant routing semantics
- No implicit fallback to a weaker contract line