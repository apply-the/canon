# Quickstart: Governed Reasoning Posture Contract

## Goal

Validate that Canon publishes one stable `governed_reasoning_posture_v1`
producer contract, keeps its release-alignment surfaces synchronized, and does
not change gatekeeper behavior while the touched runtime surface is split into
bounded sibling modules.

## Steps

1. Review `docs/integration/governed-reasoning-posture-contract.md` and confirm the contract line, required fields, supported vocabulary, and supported Boundline and Canon window are explicit.
2. Review `specs/058-governed-reasoning-posture-contract/contracts/governed-reasoning-posture-contract.md` and confirm it mirrors the stable Canon document rather than redefining it.
3. Review `specs/058-governed-reasoning-posture-contract/spec.md` and `specs/058-governed-reasoning-posture-contract/plan.md` to confirm the gatekeeper split is treated as behavior-preserving maintainability follow-through, not as a new policy feature.
4. Run `cargo test --test governed_reasoning_posture_contract` and confirm the contract identity, vocabulary, and release-window assertions pass.
5. Run `cargo test --test assistant_plugin_packages metadata_paths_and_versions_are_aligned -- --exact` and confirm the release-alignment surfaces used by downstream validation are synchronized.
6. Run the gatekeeper-focused test coverage under `crates/canon-engine/src/orchestrator/gatekeeper/tests.rs` and confirm representative requirements, change, implementation, and incident gate outcomes remain stable after the module split.
7. Record the results in `specs/058-governed-reasoning-posture-contract/validation-report.md` together with any Boundline cross-repo review notes.

## Expected Outcome

- Canon exposes one stable reasoning-posture contract with an explicit producer boundary.
- The active Boundline and Canon compatibility window is discoverable from repository artifacts.
- Stale plugin or runtime-compatibility metadata is detectable by executable validation.
- The gatekeeper split remains a maintainability change and does not introduce new gate policy semantics.
