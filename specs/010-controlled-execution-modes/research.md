# Research: Controlled Execution Modes (`implementation` and `refactor`)

## R-001: Mode-specific artifact contracts must be explicit runtime contracts

- **Decision**: Add explicit `Mode::Implementation` and `Mode::Refactor` branches in `crates/canon-engine/src/artifacts/contract.rs` and keep `defaults/methods/{implementation,refactor}.toml` artifact lists aligned with those contracts.
- **Rationale**: Today both modes fall through to the generic `other => <mode>.md` contract, which cannot satisfy the spec's required artifact bundles or the release-readiness checks that depend on named artifacts. Canon already treats artifact contracts as the durable runtime truth for gated modes, so specializing the contract code is the lowest-risk extension point.
- **Alternatives considered**:
  - Derive the artifact contract directly from the TOML method artifact list at runtime. Rejected because the current contract system also carries required sections and gate linkage, not just filenames.
  - Keep one generic `implementation.md` / `refactor.md` file and embed subsections inside it. Rejected because it weakens gate specificity, inspectability, and publish readability.

## R-002: Machine-checkable mutation bounds should extend existing run context and invocation constraints

- **Decision**: Extend the persisted run context in `.canon/runs/<RUN_ID>/context.toml` with mode-specific execution metadata, and use `InvocationConstraintSet.allowed_paths` as the runtime enforcement surface for mutation bounds.
- **Rationale**: `context.toml` is already the durable run-level metadata record and is explicitly part of the repository's runtime contract. `allowed_paths` already exists in invocation constraints and is the narrowest place to bind adapters to a declared mutation surface. Combining the two gives Canon both durable traceability and enforcement without inventing a new storage family.
- **Alternatives considered**:
  - Store bounds only in Markdown artifacts such as `mutation-bounds.md`. Rejected because adapters and policy decisions need a machine-checkable source, not only prose.
  - Create a separate execution manifest outside `context.toml`. Rejected because it would add a second metadata surface and complicate inspect/resume compatibility.

## R-003: Recommendation-only posture should reuse the existing execution model, not a new run-state family

- **Decision**: Keep `RunState` unchanged and represent recommendation-only execution through the existing `InvocationConstraintSet.recommendation_only`, `ToolOutcomeKind::RecommendationOnly`, emitted artifact markers, and summary/status text.
- **Rationale**: The runtime already uses `recommendation_only` for `change` mutation. Reusing that posture preserves backward compatibility in `run.toml`, `state.toml`, `status`, `inspect`, and `publish`, while still making the posture explicit in decision rationales and artifacts.
- **Alternatives considered**:
  - Add new run states such as `RecommendationOnlyCompleted`. Rejected because it would ripple through list/status/inspect/publish logic without adding enforcement power.
  - Hide recommendation-only posture inside artifact prose only. Rejected because policy outcomes and inspect summaries must surface it before a human reads every artifact.

## R-004: `implementation` and `refactor` need dedicated gatekeeper evaluators

- **Decision**: Add explicit `evaluate_implementation_gates()` and `evaluate_refactor_gates()` paths in `crates/canon-engine/src/orchestrator/gatekeeper.rs`, with mode-specific checks layered on the existing risk, change-preservation, implementation-readiness, and release-readiness gate vocabulary.
- **Rationale**: Current gatekeeper coverage stops at `change`, `review`, and similar analysis/review modes. Execution-heavy modes without dedicated gate functions would bypass the exact invariants this feature is supposed to enforce. Dedicated evaluators let Canon keep the gate taxonomy stable while making the checks mode-specific.
- **Alternatives considered**:
  - Reuse `evaluate_change_gates()` for both modes. Rejected because `implementation` and `refactor` have different completion criteria, failure semantics, and artifact expectations.
  - Introduce brand-new gate kinds for every rule. Rejected because the existing gate set is mostly adequate if the mode-specific evaluation logic becomes richer.

## R-005: Canonical authored-input binding should extend the existing `canon-input/<mode>.md|/` pattern

- **Decision**: Add `canon-input/implementation.md` or `canon-input/implementation/` and `canon-input/refactor.md` or `canon-input/refactor/` as canonical authored-input locations in CLI auto-binding, runtime validation, MODE_GUIDE, and the shared skill runtime scripts.
- **Rationale**: The repository already documents and validates canonical authored-input paths for requirements, discovery, system-shaping, architecture, change, review, and verification. Extending the same convention keeps input binding predictable and ensures immutable input snapshotting uses the same proven model.
- **Alternatives considered**:
  - Require explicit `--input` only for these modes. Rejected because it would make the promoted modes inconsistent with the rest of the governed-mode UX.
  - Reuse `canon-input/change.*` for both. Rejected because it erases the distinction between bounded implementation and preservation-oriented refactor.

## R-006: Execution posture and evidence must distinguish `implementation` from `refactor`

- **Decision**: `implementation` will require task mapping, mutation bounds, completion evidence, validation hooks, and rollback notes; `refactor` will require preserved behavior, structural rationale, regression evidence, contract drift checks, and explicit no-feature-addition proof. Both will record safety-net evidence and execution posture, but with mode-specific validation rules.
- **Rationale**: The current mode profiles already distinguish the two modes conceptually. The promoted runtime must make that distinction durable enough that a reader can tell which kind of work happened by inspecting persisted artifacts alone.
- **Alternatives considered**:
  - Use one shared execution artifact bundle with a `mode` field. Rejected because the primary value of `refactor` is stronger preservation semantics, not only a label.
  - Push all mode distinction into docs and skills. Rejected because runtime artifacts and gates are the authoritative system of record.

## R-007: Existing publish routing is sufficient and should remain unchanged

- **Decision**: Keep `crates/canon-engine/src/orchestrator/publish.rs` default destinations unchanged: `docs/implementation` for `implementation` and `docs/refactors` for `refactor`.
- **Rationale**: Publish already routes these modes to stable visible destinations. The missing piece is meaningful artifact content and posture-aware summaries, not destination routing.
- **Alternatives considered**:
  - Publish both modes under `docs/changes`. Rejected because it would collapse distinct runtime modes into one visible output channel.
  - Create a new publish command surface or alternate destination logic. Rejected because the spec forbids a parallel publish model.

## R-008: Skill promotion must follow runtime delivery, not lead it

- **Decision**: Update `defaults/embedded-skills/canon-implementation/skill-source.md` and `defaults/embedded-skills/canon-refactor/skill-source.md` only after runtime support exists, then materialize the synced `.agents/skills/` copies and validate them with `scripts/validate-canon-skills.sh`.
- **Rationale**: The current skills are correctly honest about `modeled-only` support. The plan should preserve that honesty until contracts, gating, and authored-input binding exist. Once the runtime is promoted, the skills become runnable wrappers instead of support-state disclaimers.
- **Alternatives considered**:
  - Update the skill wording first. Rejected because it would temporarily reintroduce fabricated capability claims.
  - Leave the skills unchanged after runtime delivery. Rejected because product messaging would then contradict actual behavior.

## R-009: Non-regression coverage should reuse `change` as the reference pattern

- **Decision**: Add contract and integration suites for `implementation` and `refactor` modeled after the existing `change` tests, and update staged-depth assertions in `tests/integration/mode_profiles.rs` so the runtime truth remains explicit.
- **Rationale**: `change` already exercises bounded mutation surfaces, recommendation-only posture, and artifact bundle validation. Using it as the template reduces design drift and preserves the layered-verification approach required by the constitution.
- **Alternatives considered**:
  - Rely only on `direct_runtime_coverage.rs`. Rejected because the new modes need dedicated lifecycle, gating, and artifact assertions.
  - Treat docs and skills validation as sufficient. Rejected because they do not prove runtime gating or artifact completeness.
