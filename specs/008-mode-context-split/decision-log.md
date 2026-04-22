# Decision Log: Mode Context Split

## D-001: Separate governed work type from system state

- **Decision**: Model `mode` and `system_context` as distinct runtime concepts instead of embedding system state in the mode name.
- **Rationale**: The current `brownfield-change` label mixes concerns and makes the catalog inconsistent.

## D-002: Replace `brownfield-change` with `change` and reject legacy public names

- **Decision**: Canon will expose `change` as the replacement mode and reject legacy public names rather than aliasing them.
- **Rationale**: The feature explicitly allows breaking changes and requires a clean public API.

## D-003: Preserve bounded-change behavior only through `change + existing`

- **Decision**: The previous brownfield gate stack, artifact contract, and preserved-behavior semantics move intact to `change` when `system_context = existing`.
- **Rationale**: This preserves the valuable existing workflow without keeping the old overloaded name.

## D-004: Reject `change + new` in the first release of the split model

- **Decision**: `change` with `system_context = new` fails before run creation.
- **Rationale**: Supporting the combination now would give `change` two meanings and immediately weaken the new model.

## D-005: Rename all public-facing brownfield surfaces in the same tranche

- **Decision**: Gate labels, canonical input hints, artifact namespaces, docs, and skill entry points are renamed together with the mode.
- **Rationale**: Any surviving public brownfield label would leave the new model semantically incomplete.

## D-006: Coverage recovery is a release gate for this feature

- **Decision**: The implementation must add targeted tests for classifier, persistence, gatekeeper, artifact rendering, CLI output, and adapter summaries, then prove the touched patch reaches the agreed coverage threshold.
- **Rationale**: The failing patch coverage is direct evidence that the current touched branches are under-validated.

## D-007: Phase 0 governance was revalidated before implementation

- **Decision**: `/speckit.implement` proceeds with the original architecture-mode packet unchanged because the mode, risk, scope boundaries, and invariants still match the requested feature.
- **Rationale**: The implementation step did not discover a contradiction large enough to require replanning or a narrower execution slice.

## D-008: Independent review stays a merge-time requirement

- **Decision**: The implementation may prepare evidence and checkpoints, but the final systemic-impact acceptance still requires named human merge ownership and a review separated from generation.
- **Rationale**: The feature changes public Canon semantics across runtime, docs, and validators, so merge-time independence remains part of the contract.