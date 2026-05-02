# Tasks: Architecture Clarification, Assumptions, And Readiness Reroute

**Input**: Design documents from `/specs/037-architecture-clarification-readiness/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/, quickstart.md

**Validation**: Layered validation is mandatory. Add executable test tasks whenever behavior, interfaces, or regressions must be checked. Independent review, coverage evidence, and closeout artifacts are always required.

**Organization**: Tasks are grouped by user story to enable independent implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with explicit version, scope, invariant, and validation artifacts.
- No implementation task may appear before the artifacts and checks that authorize it.
- Every user story MUST include validation tasks before implementation tasks.
- Independent review, coverage evidence, and repository quality gates are mandatory before completion.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the bounded architecture clarification scope, durable contracts, and version baseline that permit implementation to start.

- [ ] T001 Set the workspace release version to `0.37.0` in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [ ] T002 Update implementation-phase decisions for architecture clarification, assumptions, and reroute behavior in `specs/037-architecture-clarification-readiness/decision-log.md`
- [ ] T003 Update planned structural, logical, independent, and coverage validation checkpoints in `specs/037-architecture-clarification-readiness/validation-report.md`
- [ ] T004 [P] Confirm the canonical architecture clarity and readiness contracts in `specs/037-architecture-clarification-readiness/contracts/architecture-clarity.md` and `specs/037-architecture-clarification-readiness/contracts/architecture-readiness.md`
- [ ] T005 [P] Refresh `AGENTS.md` from `specs/037-architecture-clarification-readiness/plan.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the focused validation and fixture scaffolding required by every story.

- [ ] T006 [P] Create the focused architecture clarification regression test scaffold in `tests/architecture_037_clarification_readiness.rs`
- [ ] T007 [P] Extend architecture skill-sync and release-version scaffolding in `tests/architecture_decision_shape_docs.rs` and `tests/integration/skills_bootstrap.rs`
- [ ] T008 [P] Prepare clarification and reroute walkthrough expectations in `specs/037-architecture-clarification-readiness/quickstart.md` and `specs/037-architecture-clarification-readiness/validation-report.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared architecture clarity and readiness foundation that all user stories depend on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [ ] T009 [P] Add failing architecture clarity assertions for structured question metadata, default-if-skipped behavior, and reroute guidance in `tests/architecture_037_clarification_readiness.rs`
- [ ] T010 [P] Add failing readiness-assessment contract and run assertions in `tests/architecture_037_clarification_readiness.rs`
- [ ] T011 [P] Add failing docs and version-alignment assertions in `tests/architecture_037_clarification_readiness.rs`, `tests/architecture_decision_shape_docs.rs`, and `tests/integration/skills_bootstrap.rs`
- [ ] T012 Extend shared clarity inspect summaries for architecture question metadata in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-cli/src/output.rs`
- [ ] T013 Capture foundational clarification and readiness evidence in `specs/037-architecture-clarification-readiness/validation-report.md`

**Checkpoint**: Architecture clarity can now carry explicit question metadata and the feature has executable failing checks for reroute and readiness behavior.

---

## Phase 3: User Story 1 - Ask Only Decision-Changing Architecture Questions (Priority: P1) 🎯 MVP

**Goal**: Deliver bounded architecture clarification that asks only decision-changing questions, preserves materially closed decisions, and recommends reroute when architecture is premature.

**Independent Test**: A maintainer can run `canon inspect clarity --mode architecture` on ambiguous or under-bounded briefs and observe structured question metadata plus explicit reroute guidance, while materially closed briefs stay free of synthetic clarification churn.

### Validation for User Story 1 (MANDATORY)

- [ ] T014 [P] [US1] Extend failing materially closed and decision-changing question assertions in `tests/architecture_037_clarification_readiness.rs`
- [ ] T015 [US1] Record story-specific architecture clarification decisions under `## User Story 1 Decisions` in `specs/037-architecture-clarification-readiness/decision-log.md`

### Implementation for User Story 1

- [ ] T016 [US1] Implement structured architecture clarification question summaries in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-cli/src/output.rs`
- [ ] T017 [US1] Implement architecture-specific question filtering, materially closed preservation, and reroute guidance in `crates/canon-engine/src/orchestrator/service/clarity.rs` and `crates/canon-engine/src/orchestrator/service/inspect.rs`
- [ ] T018 [US1] Capture architecture clarity validation evidence in `specs/037-architecture-clarification-readiness/validation-report.md`

**Checkpoint**: Architecture clarity now behaves like a bounded decision filter instead of a generic interview.

---

## Phase 4: User Story 2 - Materialize Assumptions And Unresolved Questions In Readiness Output (Priority: P2)

**Goal**: Make architecture readiness output show working assumptions, unresolved questions, readiness posture, and recommended next mode explicitly.

**Independent Test**: A reviewer can inspect `readiness-assessment.md` from an architecture run and identify the packet's assumptions, unresolved questions, blockers, accepted risks, and recommended next mode without consulting chat history.

### Validation for User Story 2 (MANDATORY)

- [ ] T019 [P] [US2] Extend failing readiness-rendering and artifact-contract assertions in `tests/architecture_037_clarification_readiness.rs`
- [ ] T020 [US2] Record story-specific readiness decisions under `## User Story 2 Decisions` in `specs/037-architecture-clarification-readiness/decision-log.md`

### Implementation for User Story 2

- [ ] T021 [US2] Update the architecture artifact contract for the new readiness sections in `crates/canon-engine/src/artifacts/contract.rs`
- [ ] T022 [US2] Render working assumptions, unresolved questions, and recommended next mode in `crates/canon-engine/src/artifacts/markdown.rs`
- [ ] T023 [US2] Extend architecture readiness source extraction in `crates/canon-engine/src/orchestrator/service/clarity.rs` and any shared markdown helpers needed by `crates/canon-engine/src/artifacts/markdown.rs`
- [ ] T024 [US2] Capture readiness artifact validation evidence in `specs/037-architecture-clarification-readiness/validation-report.md`

**Checkpoint**: The architecture packet now makes its limiting assumptions and unresolved questions durable.

---

## Phase 5: User Story 3 - Keep Templates, Docs, And Roadmap Aligned With The 0.37.0 Contract (Priority: P3)

**Goal**: Align templates, skills, docs, roadmap, changelog, and version surfaces with the delivered architecture clarification contract.

**Independent Test**: A reviewer can inspect the architecture template, example input, skill mirror, roadmap, changelog, and version surfaces and find one coherent `0.37.0` story about bounded clarification, assumptions, readiness, and reroute.

### Validation for User Story 3 (MANDATORY)

- [ ] T025 [P] [US3] Extend failing template, skill, roadmap, and version-reference assertions in `tests/architecture_037_clarification_readiness.rs`, `tests/architecture_decision_shape_docs.rs`, and `tests/integration/skills_bootstrap.rs`
- [ ] T026 [US3] Record story-specific documentation and release-alignment decisions under `## User Story 3 Decisions` in `specs/037-architecture-clarification-readiness/decision-log.md`

### Implementation for User Story 3

- [ ] T027 [US3] Update impacted docs and changelog for the `0.37.0` architecture clarification release in `README.md`, `docs/guides/modes.md`, `docs/templates/canon-input/architecture.md`, `docs/examples/canon-input/architecture-state-management.md`, `defaults/embedded-skills/canon-architecture/skill-source.md`, `.agents/skills/canon-architecture/SKILL.md`, and `CHANGELOG.md`
- [ ] T028 [US3] Clean roadmap continuity after the delivered architecture clarification slice in `ROADMAP.md`
- [ ] T029 [US3] Capture docs, roadmap, and release-alignment evidence in `specs/037-architecture-clarification-readiness/validation-report.md`

**Checkpoint**: Documentation, roadmap, skill guidance, and release surfaces all describe the shipped `0.37.0` clarification contract consistently.

---

## Final Phase: Verification & Compliance

**Purpose**: Execute focused validation, capture coverage, and close the feature safely.

- [ ] T030 [P] Run `cargo test --test architecture_037_clarification_readiness --test architecture_decision_shape_docs --test skills_bootstrap` and record results in `specs/037-architecture-clarification-readiness/validation-report.md`
- [ ] T031 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and record changed Rust file coverage or direct-test coverage notes in `specs/037-architecture-clarification-readiness/validation-report.md`
- [ ] T032 [P] Run `cargo fmt` and `cargo fmt --check`, then record results in `specs/037-architecture-clarification-readiness/validation-report.md`
- [ ] T033 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and record results in `specs/037-architecture-clarification-readiness/validation-report.md`
- [ ] T034 [P] Run `cargo nextest run` and record results in `specs/037-architecture-clarification-readiness/validation-report.md`
- [ ] T035 Perform independent review of architecture clarification semantics, readiness honesty, reroute guidance, roadmap cleanup, and final evidence in `specs/037-architecture-clarification-readiness/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the bounded architecture clarification MVP.
- **User Story 2 (P2)**: Depends on the clarity metadata and reroute semantics from US1 because readiness output must reflect those clarified boundaries honestly.
- **User Story 3 (P3)**: Depends on the shipped clarification and readiness contract from US1 and US2 so docs and roadmap describe real delivered behavior.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or documentation changes land.
- Shared clarity or contract changes happen before downstream docs or evidence capture.
- Evidence capture happens before the story is declared complete.

### Parallel Opportunities

- `T004` and `T005` can run in parallel during governance.
- `T006` through `T008` can run in parallel during setup.
- `T009` through `T011` can run in parallel before `T012`.
- Within User Story 3, template, skill, and docs alignment tasks can proceed in parallel once the shipped contract is stable.
- Final validation tasks `T030` through `T034` can run in parallel once implementation is stable, but `T035` MUST be last.

---

## Parallel Example: User Story 3

```bash
# After the shipped readiness and clarity contract is stable:
Task: "Update docs/guides/modes.md, docs/templates/canon-input/architecture.md, and docs/examples/canon-input/architecture-state-management.md"
Task: "Update defaults/embedded-skills/canon-architecture/skill-source.md and .agents/skills/canon-architecture/SKILL.md"
Task: "Update README.md, CHANGELOG.md, and ROADMAP.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm architecture clarity now asks only decision-changing questions before widening into readiness rendering and docs alignment.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Finish with coverage, formatting, clippy, nextest, and independent review.

### Parallel Team Strategy

With multiple developers:

1. One developer owns clarity-summary contract and reroute behavior.
2. One developer owns readiness-assessment contract and markdown rendering.
3. One developer owns templates, skill guidance, docs, roadmap, and release-surface closeout after the runtime contract stabilizes.

---

## Notes

- `T001` is intentionally the explicit version-bump task requested for this feature.
- `T027` is intentionally the explicit impacted-docs-plus-changelog task requested for this feature.
- `T028` is intentionally the roadmap cleanup task requested for this feature.
- `T031`, `T032`, and `T033` are intentionally the explicit coverage, `cargo fmt`, and `cargo clippy` closeout tasks requested for this feature.
- Keep the decision log and validation report current as tasks close.