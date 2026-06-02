# Governance Semantics And Authority Zones

Canon defines governed semantic meaning. Downstream runtimes decide what to do
with that meaning at execution time.

For the S4 adaptive-governance slice, Canon keeps the semantic boundary
explicit:

- `authority-governance-v1` remains the required posture baseline
- `adaptive-governance-v1` is an optional additive companion
- approval, readiness, project-memory, lineage, and promotion semantics remain
  Canon-owned
- runtime confidence, trust, degradation, escalation, councils, and stop
  behavior remain downstream-owned

## Contract Lines

Canon currently uses two distinct governance contract lines for downstream S4
consumers.

### Required Baseline

- Contract line: `authority-governance-v1`
- Purpose: publish the required governed posture baseline
- Ownership: Canon
- Consumer expectation: required for governed S4 consumption

### Optional Companion

- Contract line: `adaptive-governance-v1`
- Purpose: publish governance-maturity vocabulary for adaptive S4 semantics
- Ownership: Canon
- Consumer expectation: optional unless a downstream stage policy explicitly
  requires a compatible companion

The optional companion does not replace the required baseline. Missing required
baseline metadata remains a fail-closed compatibility problem.

## Governance-State Vocabulary

Canon defines these governance-state terms as semantic posture labels:

- `advisory`: low-friction guidance without implied runtime enforcement
- `catch`: observation-oriented governance that records findings and concerns
- `rule`: bounded enforcement semantics that may justify stronger downstream
  control
- `hook`: strict governance semantics intended for the strongest downstream
  runtime controls

These labels describe semantic maturity. They do not assign reviewers,
override outcomes, or stop transitions.

## Rollout-Profile Vocabulary

Canon defines these rollout profiles as maturity labels for governance
adoption:

- `minimal`: advisory-only adoption
- `guided`: guided adoption with observation-oriented maturity
- `governed`: stronger governed maturity suited to bounded enforcement
- `strict`: the strongest maturity profile, intended for downstream strict
  handling when a runtime supports it

Rollout profiles are not council profiles. They are also not replacements for
authority zones.

## Relation To Authority Zones

Authority zones and adaptive-governance vocabulary solve different problems.

- Authority zones classify governed posture and risk semantics.
- Governance states classify semantic governance maturity.
- Rollout profiles classify adoption depth.

Consumers must not collapse those vocabularies into one label set.

## Canon-Owned Semantics That Stay Stable

Canon continues to own the meaning of these governed fields and records:

- approval state
- packet readiness
- governance metadata
- project memory
- lineage
- promotion state

Those surfaces may appear beside adaptive-governance semantics without making
Canon the runtime controller.

## Compatibility Rules For Consumers

- Consumers must treat `authority-governance-v1` as the required baseline.
- Consumers may ignore unknown additive optional fields that preserve the
  existing meaning of a compatible contract line.
- Consumers must reject incompatible semantic changes through a new contract
  line rather than assuming silent reuse of an older contract.
- Consumers must distinguish a missing required baseline from a missing
  optional companion.
- An optional companion cannot repair a missing required baseline.

## Out Of Scope For Canon

Canon does not use adaptive-governance semantics to assign:

- runtime confidence scores
- trust evolution
- councils or reviewer sets
- provider or model routes
- override outcomes
- stop transitions

Those remain downstream runtime responsibilities.