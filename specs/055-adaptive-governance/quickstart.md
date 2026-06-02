# Quickstart: Adaptive Governance Semantics

## Goal

Validate that Canon can describe S4 adaptive-governance semantics as a stable
semantic companion to `authority-governance-v1` without taking over downstream
runtime orchestration.

## Steps

1. Review `specs/055-adaptive-governance/contracts/adaptive-governance-v1-contract.md` and confirm the required baseline versus optional companion relationship is explicit.
2. Review `specs/055-adaptive-governance/contracts/adaptive-governance-adapter-projection.md` and `tech-docs/integration/governance-adapter.md` to confirm the machine-facing projection preserves the same boundary.
3. Review the delivered Canon governance semantics guide and confirm the human-facing docs describe the same S4 governance-state and rollout-profile vocabulary.
4. Run targeted tests for the touched semantic surfaces and confirm supported, missing-required-baseline, missing-companion, unsupported-companion, and additive-field cases remain distinguishable.

## Expected Outcome

- `authority-governance-v1` remains the required S3 posture baseline.
- `adaptive-governance-v1` is clearly optional and additive when published.
- companion semantics remain semantic only and do not assign runtime behavior.
- downstream consumers can distinguish baseline unavailability from companion unavailability without guessing fallback meaning.
- older consumers may ignore additive optional companion fields without changing the meaning of the required baseline.