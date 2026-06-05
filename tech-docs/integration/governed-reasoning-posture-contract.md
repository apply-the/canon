# Canon Governed Reasoning Posture Contract

## Contract Identity

- `owner`: `canon`
- `current_contract_line`: `governed_reasoning_posture_v2`
- `legacy_contract_line`: `governed_reasoning_posture_v1`
- `schema_version`: `v2`
- `stable_doc`: `tech-docs/integration/governed-reasoning-posture-contract.md`
- `example_corpus`: `tests/fixtures/governed_reasoning_posture_v2/`
- `migration_contract`: `specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2-migration.md`
- `primary_consumer`: `boundline`
- `supported_boundline_window`: `0.63.x`
- `supported_canon_window`: `0.64.x`

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
  evolution, active-versus-legacy publication, and compatibility-window
  truth; Boundline owns runtime activation, participant routing, trace
  emission, and operator-facing execution summaries.

## Producer Shape

```toml
contract_line = "governed_reasoning_posture_v2"
schema_version = "v2"
publication_status = "active"

[compatibility_window]
boundline_min = "0.63.0"
boundline_max_exclusive = "0.68.0"
canon_min = "0.68.0"
canon_max_exclusive = "0.68.0"
contract_line = "governed_reasoning_posture_v2"

[profile_selector]
selector_kind = "profile_family"
required_profile_family = "blind_review"

[minimum_independence.hard_minima]
route_distinct = true
provider_distinct = true
context_distinct = true
prompt_pattern_distinct = true
minimum_participants = 2

[minimum_independence.guidance]
recommended_minimum_participants = 3
preferred_distinct_dimensions = ["route_distinct", "provider_distinct"]

[confidence_handoff]
state = "required"
consumer_obligation = "require_structured_confidence_review"
validation_rules = ["reference_kind_required", "evidence_backed_provenance_required"]
evidence_ref_ids = ["validation-report:reasoning-posture-v2"]
rejection_mode = "fail_closed"

[provenance]
state = "evidence_backed"

[[provenance.references]]
reference_kind = "validation_report"
reference_id = "validation-report:reasoning-posture-v2"
description = "Validation report entry for the canonical v2 posture payload"
```

## Required Fields

Canon MUST publish all of the following fields for
`governed_reasoning_posture_v2`:

- `contract_line`
- `schema_version`
- `publication_status`
- `compatibility_window`
- `profile_selector`
- `minimum_independence`
- `confidence_handoff`
- `provenance`

## Supported Vocabulary

Supported `selector_kind` values:

- `profile_family`
- `profile_id`

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

Supported `confidence_handoff.state` values:

- `none`
- `required`

Every `confidence_handoff` block must also publish `rejection_mode`.

Supported `rejection_mode` values:

- `fail_closed`

Supported `provenance.state` values:

- `minimal`
- `evidence_backed`

Supported provenance `reference_kind` values:

- `packet`
- `artifact`
- `stable_doc`
- `validation_report`
- `fixture`

`minimum_independence.hard_minima` MUST publish these keys:

- `route_distinct`
- `provider_distinct`
- `context_distinct`
- `prompt_pattern_distinct`
- `minimum_participants`

## Fail-Closed Rejection Rules

Executable validation MUST reject:

- missing required top-level blocks
- contradictory selector data
- selector payloads where both `required_profile_family` and
  `required_profile_id` are present
- selector payloads where neither `required_profile_family` nor
  `required_profile_id` is present
- absent or contradictory hard-minimum independence requirements
- guidance that weakens hard minima
- omitted `confidence_handoff`
- `confidence_handoff.state = none` payloads that still carry required-handoff
  semantics
- omitted or incompatible `provenance`
- provenance references that omit `reference_kind`
- stale provenance references
- contradictory provenance states or evidence
- unsupported vocabulary values
- invalid compatibility windows
- stale release metadata
- contradictory release metadata

## Compatibility Rules

- Canon MUST fail closed on unsupported or incomplete producer data before the
  posture is published to consumers.
- Additive optional fields MAY be introduced without a new contract line only
  when they preserve backward compatibility for Boundline `0.63.x`.
- Any breaking change to required fields, compatibility semantics, or the
  supported vocabulary requires a new contract line.
- Release-facing metadata that claims support for `governed_reasoning_posture_v2`
  MUST align with Boundline `0.63.x` and Canon `0.64.x`.

## Coexistence And Migration Rules

- `v1` and `v2` may coexist only when exactly one line is marked `active` and
  the other is explicitly marked `legacy`.
- There is no implicit fallback from `governed_reasoning_posture_v2` to
  `governed_reasoning_posture_v1`.
- A `v1`-only consumer rejects `v2` with an explicit incompatibility reason.
- A `v2`-required workflow rejects `v1` even when `v1` remains published as
  `legacy`.
- Mixed publication states that make the active line ambiguous fail closed.

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