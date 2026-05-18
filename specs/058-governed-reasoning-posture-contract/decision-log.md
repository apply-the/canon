# Decision Log: Governed Reasoning Posture Contract

## Purpose

Record the Canon-owned decisions that govern the reasoning-posture contract,
its release-alignment surface, and the bounded gatekeeper maintainability
follow-through attached to this branch.

## Active Decisions

### D-001 Canon owns posture authoring and compatibility, not downstream execution

- Status: accepted
- Date: 2026-05-18
- Decision: Canon remains the producer and semantic owner of `governed_reasoning_posture_v1`, while Boundline remains the runtime owner of reasoning activation, participant routing, confidence handling, and trace emission.
- Rationale: This keeps the producer-consumer boundary explicit and prevents the Canon contract from drifting into Boundline orchestration.
- Consequences: Canon must publish stable contract identity, vocabulary, and compatibility windows, but it must not choose downstream runtime behavior.
- Related artifacts: `spec.md`, `research.md`, `contracts/governed-reasoning-posture-contract.md`, `docs/integration/governed-reasoning-posture-contract.md`

### D-002 Unsupported contract lines and incompatible release windows fail closed

- Status: accepted
- Date: 2026-05-18
- Decision: Canon treats unsupported contract lines, incomplete posture payloads, and incompatible Boundline and Canon version windows as explicit incompatibility rather than degraded success.
- Rationale: Downstream reasoning activation is release-sensitive and cannot rely on guessed compatibility.
- Consequences: Contract validation, release metadata, and runtime-compatibility surfaces must stay synchronized and reject drift clearly.
- Related artifacts: `spec.md`, `research.md`, `validation-report.md`, `tests/contract/governed_reasoning_posture_contract.rs`

### D-003 The stable doc is canonical and the feature-local brief mirrors it

- Status: accepted
- Date: 2026-05-18
- Decision: `docs/integration/governed-reasoning-posture-contract.md` is the normative Canon-owned contract, while the feature-local contract brief exists to capture branch-local reasoning and review context.
- Rationale: Consumers need one stable contract path, but the feature branch still needs a local artifact for planning and review.
- Consequences: Any divergence between the two documents must be treated as drift and validated explicitly.
- Related artifacts: `contracts/governed-reasoning-posture-contract.md`, `docs/integration/governed-reasoning-posture-contract.md`, `validation-report.md`

### D-004 Release metadata belongs inside this feature boundary when it affects the contract

- Status: accepted
- Date: 2026-05-18
- Decision: Workspace version references, assistant plugin manifests, and runtime-compatibility metadata are in scope for this feature whenever downstream reasoning-posture validation depends on them.
- Rationale: A stable contract with stale release metadata is still an integration failure.
- Consequences: Release-surface alignment checks must be recorded with the contract evidence instead of treated as unrelated housekeeping.
- Related artifacts: `spec.md`, `plan.md`, `validation-report.md`, `assistant/plugin-metadata.json`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`

### D-005 Gatekeeper changes in this branch are maintainability follow-through only

- Status: accepted
- Date: 2026-05-18
- Decision: The split of the oversized gatekeeper surface into sibling modules is allowed only as a behavior-preserving maintainability refactor attached to this feature branch.
- Rationale: The branch already touches gatekeeper code, and the split reduces review complexity, but it is not justified as a new policy feature.
- Consequences: Public gate evaluation entrypoints, approval semantics, and material blocker behavior must remain stable; any policy change requires separate documentation and validation.
- Related artifacts: `spec.md`, `research.md`, `plan.md`, `crates/canon-engine/src/orchestrator/gatekeeper.rs`, `crates/canon-engine/src/orchestrator/gatekeeper/entrypoints.rs`, `crates/canon-engine/src/orchestrator/gatekeeper/rules.rs`, `crates/canon-engine/src/orchestrator/gatekeeper/tests.rs`

## Pending Follow-Up

- Confirm whether any post-merge release bump changes the supported Canon window for a successor contract line before updating the stable contract.
