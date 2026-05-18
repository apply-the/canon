# Research: Governed Reasoning Posture Contract

## Decision: Keep the stable Canon reasoning-posture doc as the normative source

- **Decision**: `docs/integration/governed-reasoning-posture-contract.md` remains the normative Canon-owned contract, and the new feature-local contract brief mirrors it for planning and review.
- **Rationale**: Downstream consumers need one durable contract path. A feature-local brief is useful for branch reasoning, but it should never become a second competing truth source.
- **Alternatives considered**:
  - Use the feature-local brief as the only contract source: rejected because downstream consumers should not depend on a feature directory.
  - Keep only the stable doc and skip feature-local artifacts: rejected because the branch needs local planning, validation, and traceability artifacts.

## Decision: Preserve the active Boundline and Canon release pair for the first contract slice

- **Decision**: The first reasoning-posture contract slice keeps the active supported consumer window at Boundline `0.61.x` and Canon `0.57.x` unless a later coordinated release explicitly changes the pair.
- **Rationale**: The staged contract doc and Boundline consumer brief already agree on this window. Moving the release pair without explicit bilateral validation would create new drift instead of reducing it.
- **Alternatives considered**:
  - Advance the Canon window immediately to a future `0.58.x` line: rejected because the current branch evidence and consumer brief do not yet support that claim.
  - Remove explicit release windows from the contract: rejected because the consumer boundary is version-sensitive and must fail closed on unsupported pairs.

## Decision: Treat release metadata as part of the contract surface

- **Decision**: Workspace version references, plugin manifests, and runtime-compatibility metadata are in scope for this feature whenever downstream validation uses them to determine whether the contract is aligned.
- **Rationale**: The branch already demonstrated that a stale manifest version can break contract validation even when the stable doc is correct.
- **Alternatives considered**:
  - Treat metadata drift as unrelated housekeeping: rejected because the consumer-visible failure mode is still a contract-alignment failure.
  - Restrict the feature to docs and contract tests only: rejected because the active validation path reads package and runtime-compatibility metadata too.

## Decision: Keep the gatekeeper split behavior-preserving and subordinate to the contract slice

- **Decision**: The staged split of `crates/canon-engine/src/orchestrator/gatekeeper.rs` into sibling modules belongs to this branch only as maintainability follow-through and must preserve public gate evaluation behavior.
- **Rationale**: The branch already touches the gatekeeper surface, and leaving it monolithic increases review risk. But the contract feature does not justify new gate policy semantics.
- **Alternatives considered**:
  - Remove the gatekeeper refactor from the branch entirely: rejected because the branch already carries the split and the surface benefits from bounded review.
  - Treat the gatekeeper split as a separate feature with independent semantics: rejected because the staged diff shows structural extraction, not a new policy slice.

## Decision: Keep validation layered across contract, metadata, and behavior

- **Decision**: Validation for this slice must combine contract-doc alignment, release-surface alignment, and representative gatekeeper behavior checks.
- **Rationale**: A single validation layer cannot catch all failure modes in this branch. The contract can drift, the metadata can drift, or the gatekeeper split can regress behavior.
- **Alternatives considered**:
  - Validate only the stable contract doc: rejected because metadata drift is already known to cause failures.
  - Validate only the Rust tests: rejected because the stable doc and feature-local planning artifacts also need explicit review and evidence.

## Implementation Direction

- Keep the stable integration doc and the new feature-local contract brief synchronized on one producer shape and one active release pair.
- Use the governed reasoning posture contract tests as the primary executable guard for contract identity, vocabulary, and release-window alignment.
- Keep release metadata in scope wherever the executable validation reads it.
- Treat the gatekeeper split as a bounded code-organization change that must preserve public evaluation entrypoints and representative blocker behavior.

## Likely Touchpoints

- `docs/integration/governed-reasoning-posture-contract.md`
- `tests/contract/governed_reasoning_posture_contract.rs`
- `tests/governed_reasoning_posture_contract.rs`
- `assistant/plugin-metadata.json`
- `.claude-plugin/manifest.json`
- `.codex-plugin/plugin.json`
- `.cursor-plugin/manifest.json`
- `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- `README.md`
- `ROADMAP.md`
- `CHANGELOG.md`
- `Cargo.toml`
- `Cargo.lock`
- `crates/canon-engine/src/orchestrator/gatekeeper.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper/context.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper/entrypoints.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper/rules.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper/tests.rs`
