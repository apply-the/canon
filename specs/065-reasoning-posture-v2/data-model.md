# Data Model: Governed Reasoning Posture v2

## GovernedReasoningPostureContractV2

Represents the Canon-owned stable producer contract for
`governed_reasoning_posture_v2`.

Identity fields:
- `owner`
- `contract_line`
- `schema_version`
- `stable_doc`
- `primary_consumer`
- `publication_status` (`active` or `legacy`)
- `supported_boundline_window`
- `supported_canon_window`
- `compatibility_window`
- `profile_selector`
- `minimum_independence`
- `confidence_handoff`
- `provenance`

Validation rules:
- `owner` remains `canon`
- `contract_line` remains `governed_reasoning_posture_v2` for this slice
- `publication_status` is explicit for every published line
- exactly one line may be `active` in a dual-line release
- `stable_doc` resolves to the Canon integration doc path

## ProfileSelector

Represents the typed contract field that tells the consumer which reasoning
selector Canon requires.

Fields:
- `selector_kind` (`profile_family` or `profile_id`)
- `required_profile_family` (required when `selector_kind = profile_family`)
- `required_profile_id` (required when `selector_kind = profile_id`)

Validation rules:
- exactly one selector kind is present per payload
- both-present and neither-present states fail closed
- selector values must stay inside the published `v2` vocabulary
- the selector carries the authoritative consumer obligation and cannot be
  overridden elsewhere in the payload

## MinimumIndependence

Represents the Canon-owned independence requirement contract for `v2`.

Fields:
- `hard_minima`
- `guidance` (optional)

Validation rules:
- the block is required in every `v2` payload
- `guidance` is advisory only and must never weaken or replace `hard_minima`
- absent, partial, contradictory, or impossible values fail closed

### IndependenceHardMinima

Fields:
- `route_distinct`
- `provider_distinct`
- `context_distinct`
- `prompt_pattern_distinct`
- `minimum_participants`

Validation rules:
- all fields are required
- `minimum_participants` is an integer greater than or equal to 2
- the combination of minima must be satisfiable by a downstream runtime

### IndependenceGuidance

Fields:
- `recommended_minimum_participants` (optional)
- `preferred_distinct_dimensions` (optional list of independence dimensions)
- `guidance_notes_ref` (optional provenance reference)

Validation rules:
- `recommended_minimum_participants`, when present, must be greater than or
  equal to `hard_minima.minimum_participants`
- `preferred_distinct_dimensions` may only strengthen or elaborate on the hard
  minima
- guidance can reference supporting provenance, but guidance cannot substitute
  for hard minima

## ConfidenceHandoff

Represents the structured producer assertion about downstream confidence
handling requirements.

Fields:
- `state` (`none` or `required`)
- `consumer_obligation` (required when `state = required`)
- `validation_rules` (required when `state = required`)
- `evidence_ref_ids` (required when `state = required`)
- `rejection_mode`

Validation rules:
- the block is required in every `v2` payload
- omission of the block or state fails closed
- `state = none` still requires an explicit block with no hidden handoff fields
- `state = required` must carry the required fields and fail-closed behavior
- evidence references must resolve through the `Provenance` block

## Provenance

Represents the typed source-and-evidence surface for a `v2` posture payload.

Fields:
- `state` (`minimal` or `evidence_backed`)
- `references`

Validation rules:
- the block is required in every `v2` payload
- omission of the block or its state fails closed
- provenance is payload-level and must not be inferred only from release
  metadata
- `state = evidence_backed` is required when `confidence_handoff.state = required`

### ProvenanceReference

Fields:
- `reference_kind` (`packet`, `artifact`, `stable_doc`, `validation_report`,
  `fixture`, or other published `v2` vocabulary value)
- `reference_id`
- `description` (optional)

Validation rules:
- every reference kind must belong to the published vocabulary
- `reference_id` must be stable enough for machine-checkable validation
- contradictory, stale, or missing references fail closed when relied upon by
  confidence handoff or migration validation

## CompatibilityWindow

Represents the active supported release window for a reasoning-posture
contract line.

Fields:
- `boundline_min`
- `boundline_max_exclusive`
- `canon_min`
- `canon_max_exclusive`
- `contract_line`

Validation rules:
- the window names the same contract line as its parent payload
- release metadata and workspace version must remain aligned with the window
- incompatible windows fail closed rather than degrade into guessed support

## ContractPublicationSet

Represents the release-level publication state when Canon advertises one or
more reasoning-posture contract lines.

Fields:
- `active_contract_line`
- `legacy_contract_lines`
- `published_lines`

Validation rules:
- when both `v1` and `v2` are published, exactly one line is active and the
  other is explicitly legacy
- mixed or ambiguous publication fails closed
- no implicit fallback exists between active and legacy lines

## PostureExampleCase

Represents one machine-checkable example payload or migration scenario.

Fields:
- `example_id`
- `payload_path`
- `expected_validation_result` (`accept` or `reject`)
- `expected_reason`
- `contract_lines_involved`

Validation rules:
- every example must state its expected result and reason
- the example corpus must cover valid `v2`, invalid selectors, invalid
  independence, invalid confidence handoff, invalid provenance, invalid
  compatibility windows, coexistence, and migration rejection

## Relationship Notes

- `GovernedReasoningPostureContractV2` owns the top-level producer boundary
  that downstream consumers read.
- `ProfileSelector`, `MinimumIndependence`, `ConfidenceHandoff`, `Provenance`,
  and `CompatibilityWindow` are required nested subcontracts of the `v2`
  payload.
- `ContractPublicationSet` governs how `v1` and `v2` may coexist at release
  time without creating consumer ambiguity.
- `PostureExampleCase` is part of the validation surface rather than ancillary
  documentation.