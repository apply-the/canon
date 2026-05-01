# Tasks: Industry-Standard Artifact Shapes Follow-On

**Input**: Design documents from `/specs/030-artifact-shapes-follow-on/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks
whenever behavior, interfaces, or regressions must be checked. Independent
review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story so each increment can be
implemented, validated, and audited independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with mode, risk, scope, and invariant artifact tasks.
- No implementation task may appear before the artifacts that authorize it.
- Every user story MUST include validation tasks and evidence capture.
- Systemic-impact work MUST include an independent review task separate from
  generation.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the follow-on shape, release, and validation boundary that permit implementation to start.

- [x] T001 Set Canon version to `0.30.0` in `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update follow-on decisions and explicit deferrals in `specs/030-artifact-shapes-follow-on/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/030-artifact-shapes-follow-on/validation-report.md`
- [x] T004 Confirm the shape, persona-boundary, and release-alignment contracts in `specs/030-artifact-shapes-follow-on/contracts/follow-on-artifact-shapes.md`, `specs/030-artifact-shapes-follow-on/contracts/persona-boundaries.md`, and `specs/030-artifact-shapes-follow-on/contracts/release-alignment.md`

---

## Phase 1: Setup

**Purpose**: Prepare shared implementation context and release-regression scaffolding.

- [x] T005 Update agent context from `specs/030-artifact-shapes-follow-on/plan.md` into `AGENTS.md`
- [x] T006 Consolidate follow-on docs and compatibility regression coverage in `tests/mode_authoring_follow_on_docs.rs` and `tests/skills_bootstrap.rs`

---

## Phase 2: Foundational

**Purpose**: Shared prerequisites that all user stories depend on.

**⚠️ CRITICAL**: No user story work starts until this phase is complete.

- [x] T007 [P] Review and extend discovery contract coverage in `tests/discovery_authoring_docs.rs`, `tests/discovery_authoring_renderer.rs`, `tests/discovery_authoring_contract.rs`, and `tests/contract/discovery_authoring_contract.rs`
- [x] T008 [P] Review and extend shared renderer and contract coverage for `system-shaping` and `review` in `crates/canon-engine/src/artifacts/markdown.rs`, `tests/system_shaping_authoring_renderer.rs`, `tests/system_shaping_contract.rs`, `tests/contract/system_shaping_contract.rs`, `tests/review_authoring_renderer.rs`, `tests/review_contract.rs`, and `tests/contract/review_contract.rs`
- [x] T009 Add non-targeted mode regression guard notes and touched-Rust-file coverage expectations in `specs/030-artifact-shapes-follow-on/validation-report.md`

**Checkpoint**: Existing mode coverage is understood and evidence scaffolding is ready.

---

## Phase 3: User Story 1 - Shape Discovery For Exploratory Work (Priority: P1) 🎯 MVP

**Goal**: Deliver the follow-on discovery packet shape and persona guidance.

**Independent Test**: A representative discovery brief produces a packet that reads like an OST and JTBD-flavored exploratory artifact while preserving Canon's exact discovery sections and missing-gap honesty.

### Validation for User Story 1 (MANDATORY)

- [x] T010 [P] [US1] Add failing discovery-shape coverage in `tests/discovery_authoring_docs.rs`, `tests/discovery_authoring_renderer.rs`, and `tests/discovery_authoring_run.rs`
- [x] T011 [US1] Record discovery-specific shape and persona decisions under `## User Story 1 Decisions` in `specs/030-artifact-shapes-follow-on/decision-log.md`

### Implementation for User Story 1

- [x] T012 [P] [US1] Update discovery shape and persona guidance in `defaults/embedded-skills/canon-discovery/skill-source.md` and `.agents/skills/canon-discovery/SKILL.md`
- [x] T013 [US1] Confirm discovery renderer-preservation and contract expectations in `tests/discovery_authoring_renderer.rs`, `tests/discovery_authoring_contract.rs`, and `tests/contract/discovery_authoring_contract.rs`
- [x] T014 [US1] Capture discovery validation evidence in `specs/030-artifact-shapes-follow-on/validation-report.md`

**Checkpoint**: `discovery` emits the intended exploratory packet and remains independently validated.

---

## Phase 4: User Story 2 - Shape System-Shaping For Domain And Structure Work (Priority: P2)

**Goal**: Deliver the follow-on system-shaping packet shape and persona guidance.

**Independent Test**: A representative system-shaping brief produces a packet that reads like a domain-map plus structural-options artifact while preserving Canon's exact shaping sections, boundary honesty, and missing-gap behavior.

### Validation for User Story 2 (MANDATORY)

- [x] T015 [P] [US2] Add failing system-shaping follow-on coverage in `tests/system_shaping_domain_modeling_docs.rs`, `tests/system_shaping_authoring_renderer.rs`, and `tests/system_shaping_run.rs`
- [x] T016 [US2] Record system-shaping-specific decisions under `## User Story 2 Decisions` in `specs/030-artifact-shapes-follow-on/decision-log.md`

### Implementation for User Story 2

- [x] T017 [P] [US2] Update system-shaping shape and persona guidance in `defaults/embedded-skills/canon-system-shaping/skill-source.md` and `.agents/skills/canon-system-shaping/SKILL.md`
- [x] T018 [US2] Confirm system-shaping renderer-preservation and contract expectations in `tests/system_shaping_authoring_renderer.rs`, `tests/system_shaping_contract.rs`, and `tests/contract/system_shaping_contract.rs`
- [x] T019 [US2] Capture system-shaping validation evidence in `specs/030-artifact-shapes-follow-on/validation-report.md`

**Checkpoint**: `system-shaping` emits the intended domain-and-structure packet and remains independently validated.

---

## Phase 5: User Story 3 - Shape Review For Reviewer-Native Findings (Priority: P3)

**Goal**: Deliver the follow-on review packet shape and persona guidance.

**Independent Test**: A representative review packet produces a findings-first review bundle with severity, location, rationale, and recommended change framing while preserving Canon's disposition and evidence honesty.

### Validation for User Story 3 (MANDATORY)

- [x] T020 [P] [US3] Add failing review-shape coverage in `tests/review_authoring_docs.rs`, `tests/review_authoring_renderer.rs`, and `tests/review_run.rs`
- [x] T021 [US3] Record review-specific decisions under `## User Story 3 Decisions` in `specs/030-artifact-shapes-follow-on/decision-log.md`

### Implementation for User Story 3

- [x] T022 [P] [US3] Update review shape and persona guidance in `defaults/embedded-skills/canon-review/skill-source.md` and `.agents/skills/canon-review/SKILL.md`
- [x] T023 [US3] Confirm review renderer-preservation and contract expectations in `tests/review_authoring_renderer.rs`, `tests/review_contract.rs`, and `tests/contract/review_contract.rs`
- [x] T024 [US3] Capture review validation evidence in `specs/030-artifact-shapes-follow-on/validation-report.md`

**Checkpoint**: `review` emits the intended findings-first packet and remains independently validated.

---

## Phase 6: User Story 4 - Ship 0.30.0 With Aligned Docs And Validation (Priority: P4)

**Goal**: Make the shipped follow-on slice, `0.30.0` release surfaces, and final quality gates explicit and testable.

**Independent Test**: A maintainer can inspect the version surfaces, docs, changelog, task list, and validation report and confirm `0.30.0` alignment plus explicit evidence for touched-Rust-file coverage, `cargo clippy`, and `cargo fmt`.

### Validation for User Story 4 (MANDATORY)

- [x] T025 [P] [US4] Add follow-on docs and skills-mirror coverage in `tests/discovery_authoring_docs.rs`, `tests/system_shaping_domain_modeling_docs.rs`, `tests/review_authoring_docs.rs`, `tests/mode_authoring_follow_on_docs.rs`, and `tests/skills_bootstrap.rs`
- [x] T026 [US4] Record release-alignment decisions under `## User Story 4 Decisions` in `specs/030-artifact-shapes-follow-on/decision-log.md`

### Implementation for User Story 4

- [x] T027 [US4] Update version surfaces in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T028 [US4] Update impacted docs and changelog closeout in `README.md`, `ROADMAP.md`, `docs/guides/modes.md`, `CHANGELOG.md`, and any artifact-shape guidance touched by the feature
- [x] T029 [US4] Capture release-alignment validation evidence and touched-Rust-file coverage expectations in `specs/030-artifact-shapes-follow-on/validation-report.md`

**Checkpoint**: Runtime behavior, docs, and release-facing `0.30.0` surfaces align cleanly.

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and closeout.

- [x] T030 [P] Run `scripts/validate-canon-skills.sh` plus the focused current-tree suite for `tests/discovery_authoring_docs.rs`, `tests/system_shaping_domain_modeling_docs.rs`, `tests/review_authoring_docs.rs`, `tests/mode_authoring_follow_on_docs.rs`, `tests/requirements_authoring_docs.rs`, and `tests/skills_bootstrap.rs`, then record results in `specs/030-artifact-shapes-follow-on/validation-report.md`
- [x] T031 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and document coverage for every modified or newly created Rust file in `specs/030-artifact-shapes-follow-on/validation-report.md`
- [x] T032 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/030-artifact-shapes-follow-on/validation-report.md`
- [x] T033 [P] Run `cargo nextest run --workspace --all-features` and record results in `specs/030-artifact-shapes-follow-on/validation-report.md`
- [x] T034 Perform independent review of invariants, non-targeted mode stability, and the final diff in `specs/030-artifact-shapes-follow-on/validation-report.md`
- [x] T035 Confirm invariants still hold and close the final validation state in `specs/030-artifact-shapes-follow-on/validation-report.md`

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

- T007 and T008 can run in parallel.
- T010, T015, and T020 can run in parallel after Phase 2 if staffing allows.
- T012, T017, and T022 can run in parallel after their respective validation tasks.
- T030, T031, T032, and T033 can run in parallel once implementation is stable.

---

## Parallel Example: User Story 1

```bash
# Prepare discovery validation in parallel:
Task: "Add failing discovery-shape coverage in tests/discovery_authoring_docs.rs, tests/discovery_authoring_renderer.rs, and tests/discovery_authoring_run.rs"
Task: "Record discovery-specific decisions in specs/030-artifact-shapes-follow-on/decision-log.md"

# Update the source and mirrored discovery guidance in parallel with contract assertions:
Task: "Update discovery shape and persona guidance in defaults/embedded-skills/canon-discovery/skill-source.md and .agents/skills/canon-discovery/SKILL.md"
Task: "Update discovery renderer-preservation and contract expectations in crates/canon-engine/src/artifacts/markdown.rs and tests/discovery_authoring_contract.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phases 0, 1, and 2.
2. Complete User Story 1.
3. **STOP and VALIDATE**: Confirm the discovery packet shape works independently and update `validation-report.md`.

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
- `T028` is intentionally the impacted docs plus changelog closeout task as requested
- `T031`, `T032`, and `T033` explicitly cover touched-Rust-file coverage, `cargo clippy`, and `cargo fmt`