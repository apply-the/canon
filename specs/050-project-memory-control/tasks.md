# Tasks: Project Memory And Delivery Control Contracts

**Input**: Design documents from `specs/050-project-memory-control/`  
**Prerequisites**: plan.md (required), spec.md (required), research.md,
data-model.md, quickstart.md, contracts/

**Validation**: Layered validation is mandatory. Contract and producer-side
behavior changes require focused publish-profile tests plus workspace-wide
format, lint, and test validation.

**Organization**: Tasks are grouped by user story so each slice can ship
independently and keep Canon ownership clear.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Contract Bump)

**Purpose**: Freeze the owner-side contract direction before code changes.

- [ ] T001 Refresh or create the canonical contract bump in `docs/integration/project-memory-promotion-contract.md` and `specs/050-project-memory-control/contracts/project-memory-promotion-contract.md`
- [ ] T002 [P] Align shared contract examples in `specs/050-project-memory-control/contracts/governed-stage-ref-contract.md`, `specs/050-project-memory-control/contracts/promotion-event-contract.md`, and `specs/050-project-memory-control/contracts/evidence-ref-contract.md`
- [ ] T003 [P] Record accepted owner-side deltas in `specs/050-project-memory-control/decision-log.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared producer-side primitives required by all user stories.

**⚠️ CRITICAL**: No user story work starts until these tasks are complete.

- [ ] T004 Extend producer contract primitives in `crates/canon-engine/src/domain/publish_profile.rs` for producer-neutral managed blocks and required versus optional V1 lineage
- [ ] T005 [P] Add default target-category helpers for `docs/project/` and `docs/evidence/` in `crates/canon-engine/src/orchestrator/publish.rs`
- [ ] T006 [P] Add deterministic builders or fixtures for stable, pending, proposal, and evidence-only outputs in `crates/canon-engine/src/orchestrator/publish.rs`
- [ ] T007 Preserve stable contract linkage notes in `docs/integration/project-memory-promotion-contract.md` and `specs/050-project-memory-control/quickstart.md`, then record the Phase 2 checkpoint decision in `specs/050-project-memory-control/decision-log.md`

**Checkpoint**: Canon has one coherent contract baseline and shared producer-side
primitives for targets, markers, and lineage.

---

## Phase 3: User Story 1 - Publish One Canonical Contract Bundle (Priority: P1) 🎯 MVP

**Goal**: A consumer can discover one stable Canon-owned contract bundle and map
it back to the detailed feature-local contract set.

**Independent Test**: Inspect the stable and feature-local contract docs and
verify that ownership, required fields, compatibility rules, and linked shared
contract shapes are consistent.

### Tests for User Story 1

- [ ] T008 [P] [US1] Add unit tests for the current contract line and required field constants in `crates/canon-engine/src/domain/publish_profile.rs`
- [ ] T009 [P] [US1] Add focused publish-path tests for stable project-memory target selection in `crates/canon-engine/src/orchestrator/publish.rs`

### Implementation for User Story 1

- [ ] T010 [US1] Update `docs/integration/project-memory-promotion-contract.md` to the V1 control-layer wording, target mapping, and compatibility policy
- [ ] T011 [US1] Finalize `specs/050-project-memory-control/contracts/project-memory-promotion-contract.md` to match the stable contract and owner boundary
- [ ] T012 [US1] Link governed stage, promotion event, and evidence ref briefs from `docs/integration/project-memory-promotion-contract.md`

**Checkpoint**: Canon exposes one stable owner-side contract path plus linked
feature-local detail.

---

## Phase 4: User Story 2 - Define Safe Repo-Visible Publication Surfaces (Priority: P1)

**Goal**: Canon publishes stable, pending, proposal, and evidence outputs using
safe default targets and producer-neutral managed blocks.

**Independent Test**: Exercise managed, pending, proposal, and evidence-only
publication paths and verify targets plus markers match the documented rules.

### Tests for User Story 2

- [ ] T013 [P] [US2] Add unit tests for producer-neutral managed-block emission and Canon-only policy-field ownership in `crates/canon-engine/src/orchestrator/publish.rs`
- [ ] T014 [P] [US2] Add unit tests for pending, proposal, blocked, conflicting, index-only, and evidence-only target routing in `crates/canon-engine/src/orchestrator/publish.rs`

### Implementation for User Story 2

- [ ] T015 [US2] Update `crates/canon-engine/src/domain/publish_profile.rs` to encode producer-neutral managed blocks and the V1 lineage required versus optional split
- [ ] T016 [US2] Update `crates/canon-engine/src/orchestrator/publish.rs` to emit `project-memory:managed` markers for stable surfaces
- [ ] T017 [US2] Update `crates/canon-engine/src/orchestrator/publish.rs` to route pending, proposal, blocked, conflicting, index-only, and evidence-only outputs under the clarified defaults
- [ ] T018 [US2] Align feature-local contract briefs in `specs/050-project-memory-control/contracts/` with the implemented target and marker behavior

**Checkpoint**: Repo-visible publication rules are safe, attributable, and
consistent with the contract text.

---

## Phase 5: User Story 3 - Freeze Minimum V1 Lineage And Compatibility (Priority: P2)

**Goal**: Canon documents and emits an implementable V1 contract line with clear
additive-versus-breaking rules.

**Independent Test**: Inspect the emitted metadata model and contract docs and
verify that required fields, optional fields, and breaking-change rules are
unambiguous.

### Tests for User Story 3

- [ ] T019 [P] [US3] Add unit tests for required versus optional lineage serialization in `crates/canon-engine/src/domain/publish_profile.rs`
- [ ] T020 [P] [US3] Add unit tests for additive versus breaking contract-line examples in `crates/canon-engine/src/domain/publish_profile.rs`

### Implementation for User Story 3

- [ ] T021 [US3] Update `docs/integration/project-memory-promotion-contract.md` and `specs/050-project-memory-control/contracts/project-memory-promotion-contract.md` to document additive V1 changes, breaking changes, and previous-minor support
- [ ] T022 [US3] Align producer metadata descriptions in `crates/canon-engine/src/domain/publish_profile.rs` with the V1 contract line

**Checkpoint**: Consumers can decide whether to proceed or stop from the owner-side contract alone.

---

## Final Phase: Verification & Cross-Cutting Concerns

**Purpose**: Validate the owner-side contract and producer behavior end to end.

- [ ] T023 [P] Update `CHANGELOG.md` and any release-facing notes affected by the stable contract surface
- [ ] T024 Run focused Canon tests for `crates/canon-engine/src/domain/publish_profile.rs` and `crates/canon-engine/src/orchestrator/publish.rs`
- [ ] T025 Run `cargo fmt --check`
- [ ] T026 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] T027 Run `cargo nextest run`
- [ ] T028 Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` if modified-file coverage needs refresh
- [ ] T029 Run an independent Boundline consumer walkthrough against `docs/integration/project-memory-promotion-contract.md` and record the outcome in `specs/050-project-memory-control/decision-log.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: starts immediately and freezes the owner-side contract bump
- **Foundational (Phase 2)**: depends on Setup completion and blocks all story work
- **User Story 1-3**: depend on Foundational completion; US1 finishes the stable contract path before US2 mutates overlapping publish surfaces in `crates/canon-engine/src/orchestrator/publish.rs`, and US3 depends on the finalized V1 lineage shape
- **Final Phase**: depends on all selected user stories completing

### User Story Dependencies

- **User Story 1 (P1)**: starts after Foundational and establishes the stable contract path
- **User Story 2 (P1)**: starts after Foundational and depends on the managed-block and target primitives from Phase 2
- **User Story 3 (P2)**: starts after Foundational and should finish after the V1 lineage implementation is stable

### Within Each User Story

- Test tasks fail before implementation when the story changes executable behavior
- Contract docs and constants align before routing or emission logic changes
- Producer primitives land before story-specific publish behavior
- Story checkpoints must pass before final verification

## Parallel Opportunities

- T002 and T003 can run in parallel during Setup
- T005 and T006 can run in parallel during Foundational
- T008 and T009 can run in parallel for US1
- T013 and T014 can run in parallel for US2
- T019 and T020 can run in parallel for US3
- T023 can run in parallel with the focused validation runs once implementation is stable

US1 and US2 implementation tasks are intentionally serialized because both
stories touch `crates/canon-engine/src/orchestrator/publish.rs`.

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate the stable contract path and required field model independently

### Incremental Delivery

1. Ship the stable contract path and linked feature-local contracts
2. Add safe target routing and managed-block behavior
3. Finalize the V1 lineage and compatibility rules
4. Run full verification only after the owner-side contract and producer behavior align

## Notes

- The first task is intentionally a contract bump, not a release-version bump.
- Canon remains the sole source of truth for producer semantics throughout this task set.
- Boundline consumption work should not begin against moving owner-side semantics until the Setup checkpoint is done.