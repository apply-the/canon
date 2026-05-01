# Tasks: Remaining Industry-Standard Artifact Shapes

**Input**: Design documents from `/specs/031-remaining-artifact-shapes/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks
whenever behavior, interfaces, or regressions must be checked. Independent
review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story so each increment can be
implemented, validated, and audited independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with mode, risk, scope, and invariant artifact tasks.
- No implementation task may appear before the artifacts that authorize it.
- Every user story MUST include validation tasks and evidence capture.
- Systemic-impact work MUST include an independent review task separate from
  generation.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the remaining-shape, release, and validation boundary that permits implementation to start.

- [x] T001 Set Canon version to `0.31.0` in `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update remaining-rollout decisions and explicit deferrals in `specs/031-remaining-artifact-shapes/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/031-remaining-artifact-shapes/validation-report.md`
- [x] T004 Confirm the shape, persona-boundary, and release-alignment contracts in `specs/031-remaining-artifact-shapes/contracts/remaining-artifact-shapes.md`, `specs/031-remaining-artifact-shapes/contracts/persona-boundaries.md`, and `specs/031-remaining-artifact-shapes/contracts/release-alignment.md`

---

## Phase 1: Setup

**Purpose**: Prepare shared implementation context and release-regression scaffolding.

- [x] T005 Update agent context from `specs/031-remaining-artifact-shapes/plan.md` into `AGENTS.md`
- [x] T006 Consolidate remaining-rollout docs and skills-mirror regression coverage in `tests/implementation_authoring_docs.rs`, `tests/refactor_authoring_docs.rs`, `tests/verification_authoring_docs.rs`, and `tests/skills_bootstrap.rs`

---

## Phase 2: Foundational

**Purpose**: Shared prerequisites that all user stories depend on.

**⚠️ CRITICAL**: No user story work starts until this phase is complete.

- [x] T007 [P] Review and extend implementation contract coverage in `tests/implementation_authoring_docs.rs`, `tests/implementation_authoring_renderer.rs`, `tests/implementation_contract.rs`, and `tests/contract/implementation_contract.rs`
- [x] T008 [P] Review and extend refactor contract coverage in `tests/refactor_authoring_docs.rs`, `tests/refactor_authoring_renderer.rs`, `tests/refactor_contract.rs`, `tests/refactor_run.rs`, `tests/refactor_preservation_run.rs`, and `tests/contract/refactor_contract.rs`
- [x] T009 [P] Review and extend verification contract coverage in `tests/verification_authoring_docs.rs`, `tests/verification_authoring_renderer.rs`, `tests/verification_contract.rs`, `tests/verification_run.rs`, and `tests/contract/verification_contract.rs`
- [x] T010 Add non-targeted mode regression guard notes and touched-Rust-file coverage expectations in `specs/031-remaining-artifact-shapes/validation-report.md`

**Checkpoint**: Existing mode coverage is understood and evidence scaffolding is ready.

---

## Phase 3: User Story 1 - Shape Implementation For Delivery Work (Priority: P1) 🎯 MVP

**Goal**: Deliver the implementation packet shape and persona guidance.

**Independent Test**: A representative implementation brief produces a packet that reads like a task-mapped delivery artifact with contract-test intent and implementation notes while preserving Canon's exact implementation sections and missing-gap honesty.

### Validation for User Story 1 (MANDATORY)

- [x] T011 [P] [US1] Add failing implementation-shape coverage in `tests/implementation_authoring_docs.rs`, `tests/implementation_authoring_renderer.rs`, and `tests/implementation_run.rs`
- [x] T012 [US1] Record implementation-specific decisions under `## User Story 1 Decisions` in `specs/031-remaining-artifact-shapes/decision-log.md`

### Implementation for User Story 1

- [x] T013 [P] [US1] Update implementation shape and persona guidance in `defaults/embedded-skills/canon-implementation/skill-source.md` and `.agents/skills/canon-implementation/SKILL.md`
- [x] T014 [P] [US1] Update implementation input template and example in `docs/templates/canon-input/implementation.md` and `docs/examples/canon-input/implementation-auth-session-revocation.md`
- [x] T015 [US1] Confirm implementation renderer-preservation and contract expectations in `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `tests/implementation_contract.rs`, and `tests/contract/implementation_contract.rs`
- [x] T016 [US1] Capture implementation validation evidence in `specs/031-remaining-artifact-shapes/validation-report.md`

**Checkpoint**: `implementation` emits the intended delivery packet and remains independently validated.

---

## Phase 4: User Story 2 - Shape Refactor For Preserved Behavior (Priority: P2)

**Goal**: Deliver the refactor packet shape and persona guidance.

**Independent Test**: A representative refactor brief produces a packet that reads like a preserved-behavior matrix plus structural-rationale artifact while preserving Canon's exact refactor sections, scope boundaries, and missing-gap behavior.

### Validation for User Story 2 (MANDATORY)

- [x] T017 [P] [US2] Add failing refactor-shape coverage in `tests/refactor_authoring_docs.rs`, `tests/refactor_authoring_renderer.rs`, `tests/refactor_run.rs`, and `tests/refactor_preservation_run.rs`
- [x] T018 [US2] Record refactor-specific decisions under `## User Story 2 Decisions` in `specs/031-remaining-artifact-shapes/decision-log.md`

### Implementation for User Story 2

- [x] T019 [P] [US2] Update refactor shape and persona guidance in `defaults/embedded-skills/canon-refactor/skill-source.md` and `.agents/skills/canon-refactor/SKILL.md`
- [x] T020 [P] [US2] Update refactor input template and example in `docs/templates/canon-input/refactor.md` and `docs/examples/canon-input/refactor-auth-session-cleanup.md`
- [x] T021 [US2] Confirm refactor renderer-preservation and contract expectations in `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `tests/refactor_contract.rs`, and `tests/contract/refactor_contract.rs`
- [x] T022 [US2] Capture refactor validation evidence in `specs/031-remaining-artifact-shapes/validation-report.md`

**Checkpoint**: `refactor` emits the intended preserved-behavior packet and remains independently validated.

---

## Phase 5: User Story 3 - Shape Verification For Claims And Evidence (Priority: P3)

**Goal**: Deliver the verification packet shape and persona guidance.

**Independent Test**: A representative verification brief produces a claims-evidence-independence matrix with explicit unresolved-support posture while preserving Canon's exact verification sections and missing-gap behavior.

### Validation for User Story 3 (MANDATORY)

- [x] T023 [P] [US3] Add failing verification-shape coverage in `tests/verification_authoring_docs.rs`, `tests/verification_authoring_renderer.rs`, and `tests/verification_run.rs`
- [x] T024 [US3] Record verification-specific decisions under `## User Story 3 Decisions` in `specs/031-remaining-artifact-shapes/decision-log.md`

### Implementation for User Story 3

- [x] T025 [P] [US3] Update verification shape and persona guidance in `defaults/embedded-skills/canon-verification/skill-source.md` and `.agents/skills/canon-verification/SKILL.md`
- [x] T026 [P] [US3] Update verification input template and example in `docs/templates/canon-input/verification.md` and `docs/examples/canon-input/verification-e2e-flakiness.md`
- [x] T027 [US3] Confirm verification renderer-preservation and contract expectations in `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `tests/verification_contract.rs`, and `tests/contract/verification_contract.rs`
- [x] T028 [US3] Capture verification validation evidence in `specs/031-remaining-artifact-shapes/validation-report.md`

**Checkpoint**: `verification` emits the intended claims-and-evidence packet and remains independently validated.

---

## Phase 6: User Story 4 - Ship 0.31.0 With Aligned Docs And Validation (Priority: P4)

**Goal**: Make the shipped remaining slice, `0.31.0` release surfaces, and final quality gates explicit and testable.

**Independent Test**: A maintainer can inspect the version surfaces, docs, changelog, task list, and validation report and confirm `0.31.0` alignment plus explicit evidence for touched-Rust-file coverage, `cargo clippy`, and `cargo fmt`.

### Validation for User Story 4 (MANDATORY)

- [x] T029 [P] [US4] Add remaining-rollout docs and skills-mirror coverage in `tests/implementation_authoring_docs.rs`, `tests/refactor_authoring_docs.rs`, `tests/verification_authoring_docs.rs`, and `tests/skills_bootstrap.rs`
- [x] T030 [US4] Record release-alignment decisions under `## User Story 4 Decisions` in `specs/031-remaining-artifact-shapes/decision-log.md`

### Implementation for User Story 4

- [x] T031 [US4] Update version surfaces in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T032 [US4] Update impacted docs and changelog closeout in `README.md`, `ROADMAP.md`, `docs/guides/modes.md`, `CHANGELOG.md`, and any touched authoring guidance
- [x] T033 [US4] Capture release-alignment validation evidence and touched-Rust-file coverage expectations in `specs/031-remaining-artifact-shapes/validation-report.md`

**Checkpoint**: Runtime behavior, docs, and release-facing `0.31.0` surfaces align cleanly.

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and closeout.

- [x] T034 [P] Run `scripts/validate-canon-skills.sh` plus the focused targeted suite for `tests/implementation_authoring_docs.rs`, `tests/refactor_authoring_docs.rs`, `tests/verification_authoring_docs.rs`, and `tests/skills_bootstrap.rs`, then record results in `specs/031-remaining-artifact-shapes/validation-report.md`
- [x] T035 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and document coverage for every modified or newly created Rust file in `specs/031-remaining-artifact-shapes/validation-report.md`
- [x] T036 [P] Run `cargo fmt` and `cargo fmt --check`, then record results in `specs/031-remaining-artifact-shapes/validation-report.md`
- [x] T037 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/031-remaining-artifact-shapes/validation-report.md`
- [x] T038 [P] Run `cargo nextest run --workspace --all-features` and record results in `specs/031-remaining-artifact-shapes/validation-report.md`
- [x] T039 Perform independent review of invariants, non-targeted mode stability, and the final diff in `specs/031-remaining-artifact-shapes/validation-report.md`
- [x] T040 Confirm invariants still hold and close the final validation state in `specs/031-remaining-artifact-shapes/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: No dependencies. MUST complete first.
- **Phase 1: Setup**: Depends on Phase 0.
- **Phase 2: Foundational**: Depends on Phase 1. BLOCKS all user stories.
- **Phase 3: User Story 1**: Depends on Phase 2.
- **Phase 4: User Story 2**: Depends on Phase 2.
- **Phase 5: User Story 3**: Depends on Phase 2.
- **Phase 6: User Story 4**: Depends on User Stories 1, 2, and 3 so release-facing docs describe the implemented slice.
- **Final Phase**: Depends on all selected user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the MVP.
- **User Story 2 (P2)**: Can start after Foundational. Reuses the shared renderer and contract surfaces but remains independently testable.
- **User Story 3 (P3)**: Can start after Foundational. Reuses the shared renderer and contract surfaces but remains independently testable.
- **User Story 4 (P4)**: Depends on the implemented mapping from the earlier stories so the shipped docs and release tests match reality.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- Skill source changes happen before or alongside mirrored skill changes.
- Evidence capture happens before the story is declared complete.

### Parallel Opportunities

- T007, T008, and T009 can run in parallel.
- T011, T017, and T023 can run in parallel after Phase 2 if staffing allows.
- T013 and T014, T019 and T020, T025 and T026 can run in parallel after their respective validation tasks.
- T034, T035, T036, T037, and T038 can run in parallel once implementation is stable.

---

## Parallel Example: User Story 1

```bash
# Prepare implementation validation in parallel:
Task: "Add failing implementation-shape coverage in tests/implementation_authoring_docs.rs, tests/implementation_authoring_renderer.rs, and tests/implementation_run.rs"
Task: "Record implementation-specific decisions in specs/031-remaining-artifact-shapes/decision-log.md"

# Update the source and mirrored implementation guidance in parallel with examples:
Task: "Update implementation shape and persona guidance in defaults/embedded-skills/canon-implementation/skill-source.md and .agents/skills/canon-implementation/SKILL.md"
Task: "Update implementation input template and example in docs/templates/canon-input/implementation.md and docs/examples/canon-input/implementation-auth-session-revocation.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phases 0, 1, and 2.
2. Complete User Story 1.
3. **STOP and VALIDATE**: Confirm the implementation packet shape works independently and update `validation-report.md`.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Add User Story 4 for release alignment and validate independently.
6. Finish with Verification & Compliance and repository closeout.

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together.
2. Once Foundational is done:
   - Developer A: User Story 1.
   - Developer B: User Story 2.
   - Developer C: User Story 3.
3. User Story 4 closes only after the earlier stories are stable.

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- `T001` is intentionally the version bump task as requested
- `T032` is intentionally the impacted docs plus changelog closeout task as requested
- `T035`, `T036`, and `T037` explicitly cover touched-Rust-file coverage, `cargo fmt`, and `cargo clippy`