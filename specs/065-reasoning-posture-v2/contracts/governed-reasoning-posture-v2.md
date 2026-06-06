# Contract: Governed Reasoning Posture v2

## Summary

`governed_reasoning_posture_v2` is the successor Canon-owned producer contract
for downstream Boundline reasoning posture consumption. It replaces the weak or
underspecified `v1` surfaces with typed, fail-closed subcontracts for selector
resolution, independence requirements, confidence handoff, provenance, and
compatibility windows.

## Supported Release Pair

- `supported_boundline_window = 0.74.x`
- `supported_canon_window = 0.71.x`
- `boundline_min = 0.63.0`
- `boundline_max_exclusive = 0.64.0`
- `canon_min = 0.64.0`
- `canon_max_exclusive = 0.68.0`
- executable fixtures live under `tests/fixtures/governed_reasoning_posture_v2/`

## Normative Relationship

- The stable Canon contract document remains the normative source.
- This feature-local contract captures the `v2` interface shape for planning,
  review, and implementation.
- Any drift between the stable document and this file is a validation failure.

## Top-Level Required Fields

Every `governed_reasoning_posture_v2` payload must publish:

- `contract_line`
- `schema_version`
- `publication_status`
- `compatibility_window`
- `profile_selector`
- `minimum_independence`
- `confidence_handoff`
- `provenance`

## Profile Selector Contract

- Exactly one selector kind is allowed per payload.
- Valid selector kinds:
  - `profile_family`
  - `profile_id`
- When `selector_kind = profile_family`, `required_profile_family` is required.
- When `selector_kind = profile_id`, `required_profile_id` is required.
- Both-present and neither-present states fail closed.
- The contract does not define any precedence rule.

## Minimum Independence Contract

- `minimum_independence` is required in every `v2` payload.
- The block contains:
  - `hard_minima` (required)
  - `guidance` (optional)
- `hard_minima` publishes the non-negotiable validation requirements.
- `guidance` may only strengthen or elaborate on the minima; it cannot weaken,
  replace, or override them.
- Contradictory or impossible independence values fail closed.

### Hard Minima Keys

- `route_distinct`
- `provider_distinct`
- `context_distinct`
- `prompt_pattern_distinct`
- `minimum_participants`

## Confidence Handoff Contract

- `confidence_handoff` is required in every `v2` payload.
- The block publishes an explicit `state`.
- Supported states:
  - `none`
  - `required`
- Absence of the block or its `state` fails closed.
- When `state = required`, the block must also publish:
  - the consumer obligation
  - the validation rules the consumer must honor
  - the provenance or evidence references needed to justify the handoff
  - `rejection_mode = fail_closed` when the handoff cannot be validated
- When `state = none`, the block remains present and explicitly states that no
  confidence handoff is required while still publishing
  `rejection_mode = fail_closed`.

## Provenance Contract

- `provenance` is required in every `v2` payload.
- The block publishes an explicit `state` and stable reference-kind contract.
- Provenance cannot be inferred only from the contract line or release
  metadata.
- Minimal provenance is still explicit and machine-checkable.
- When `confidence_handoff.state = required`, the provenance block must supply
  the evidence or references needed to validate the handoff.

### Provenance Reference Requirements

Each reference publishes:

- `reference_kind`
- `reference_id`
- optional descriptive metadata

Reference kinds must belong to the published `v2` vocabulary.

## Compatibility Window Contract

- `compatibility_window` publishes:
  - `boundline_min`
  - `boundline_max_exclusive`
  - `canon_min`
  - `canon_max_exclusive`
  - `contract_line`
- Window validation is fail-closed.
- Release-facing metadata that claims support for the line must stay aligned
  with the published window.

## Coexistence Rules

- `v1` and `v2` may be published together only when exactly one line is marked
  `active` and the other is explicitly marked `legacy`.
- There is no implicit fallback between lines.
- Consumers validate the active line they support and reject mixed or ambiguous
  posture inputs.

## Validation Expectations

Executable validation must reject:

- missing required top-level blocks
- contradictory selector data
- absent or contradictory hard-minimum independence requirements
- guidance that weakens hard minima
- omitted `confidence_handoff`
- omitted or incompatible `provenance`
- unsupported vocabulary values
- invalid compatibility windows
- ambiguous dual-line publication

## Example Expectations

The delivered example corpus must include:

- one valid `governed_reasoning_posture_v2` payload
- invalid profile selector examples
- invalid independence examples
- invalid confidence handoff examples
- invalid provenance examples
- invalid compatibility-window examples
- one `v1`/`v2` coexistence example
- one migration-rejection example

Every example must declare the expected validation result and the rejection or
acceptance reason, plus a stable path under
`tests/fixtures/governed_reasoning_posture_v2/`.