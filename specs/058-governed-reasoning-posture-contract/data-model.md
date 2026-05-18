# Data Model: Governed Reasoning Posture Contract

## GovernedReasoningPostureContract

Represents the Canon-owned stable contract consumed by downstream runtimes.

Identity fields:
- `owner`
- `current_contract_line`
- `schema_version`
- `stable_doc`
- `primary_consumer`
- `supported_boundline_window`
- `supported_canon_window`

Validation rules:
- `owner` remains `canon`
- `current_contract_line` remains `governed_reasoning_posture_v1` for the first slice
- `stable_doc` resolves to the Canon integration doc path
- the published consumer and supported windows remain aligned with the Boundline consumer brief

## ReasoningCompatibilityWindow

Represents the active supported release window for the reasoning-posture contract.

Fields:
- `boundline_min`
- `boundline_max_exclusive`
- `canon_min`
- `canon_max_exclusive`
- `contract_line`

Validation rules:
- the window must name the same contract line as the parent contract
- the window must fail closed when the local Canon version or downstream Boundline version falls outside the declared range
- changing compatibility semantics requires an explicit contract update rather than a silent prose edit

## ReasoningProfileRequirement

Represents the contract field that tells the consumer which reasoning profile family or explicit profile id is required.

Fields:
- `required_profile_family` (optional when `required_profile_id` is present)
- `required_profile_id` (optional when `required_profile_family` is present)

Validation rules:
- at least one of the two fields must be present
- supported values must stay inside the published Canon vocabulary for the active contract line
- new required values require explicit contract evolution

## MinimumIndependenceRequirement

Represents the Canon-owned posture sub-shape for independence expectations.

Fields:
- `route_distinct`
- `provider_distinct`
- `context_distinct`
- `prompt_pattern_distinct`
- `minimum_participants`

Validation rules:
- all five keys are required for the active contract line
- the shape is semantic input for the downstream runtime and does not itself assign routes or participants

## ReleaseAlignmentSurface

Represents the set of versioned Canon surfaces that downstream validation uses to determine whether the contract branch is aligned.

Fields:
- `workspace_version`
- `plugin_metadata_version`
- `host_manifest_versions`
- `runtime_compatibility_expected_workspace_version`
- `release_docs_version_lines`

Validation rules:
- all surfaced versions used by the executable validation path must agree on one Canon release line
- stale metadata is treated as contract drift when downstream checks read it

## GatekeeperEvaluationSurface

Represents the public Canon gate evaluation layer touched by this branch.

Components:
- public evaluation entrypoints
- typed gate contexts
- internal rule helpers
- gatekeeper-focused tests

Validation rules:
- public evaluation entrypoints remain stable during the module split
- representative gate outcomes and material blocker meaning remain behaviorally equivalent
- new gate policy semantics are out of scope unless separately specified

## Relationship Notes

- `GovernedReasoningPostureContract` owns the semantic producer boundary that Boundline consumes.
- `ReasoningCompatibilityWindow`, `ReasoningProfileRequirement`, and `MinimumIndependenceRequirement` are nested parts of the published contract shape.
- `ReleaseAlignmentSurface` is a validation-facing projection that must stay synchronized with the contract this branch claims to publish.
- `GatekeeperEvaluationSurface` is not part of the contract payload itself, but it is part of the branch boundary because the staged refactor must remain behavior-preserving.
