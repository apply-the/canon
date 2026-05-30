---
description: "Task list for governed expertise inputs implementation"
---

# Tasks: Governed Expertise Inputs

**Input**: Design documents from `/specs/052-governed-expertise-inputs/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md, contracts/

**Validation**: Layered validation is mandatory because this slice changes a
Canon-owned downstream contract and source-level producer semantics.

**Organization**: Tasks are grouped by user story so each contract increment can
be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story or closeout group this task belongs to (`US1`, `US2`, `US3`, `Closeout`)
- Include exact file paths in descriptions

## Phase 0: Release & Stable Contract Baseline

**Purpose**: Establish the required release move and stable expertise-input
contract surface

- [x] T001 Bump the Canon workspace version from `0.51.0` to `0.52.0` in `Cargo.toml` and update `CHANGELOG.md`
- [x] T002 [P] Create the stable expertise-input contract scaffold and authority header in `docs/integration/governed-expertise-input-contract.md` and keep `specs/052-governed-expertise-inputs/contracts/governed-expertise-input-contract.md` aligned with the same contract line
- [x] T003 [P] Refresh approval gates and evidence sections in `specs/052-governed-expertise-inputs/decision-log.md` and `specs/052-governed-expertise-inputs/validation-report.md`

---

## Phase 1: Foundational (Blocking Prerequisites)

**Purpose**: Shared expertise classification primitives and validation entry points

**⚠️ CRITICAL**: No user story work should begin until this phase is complete

- [x] T004 Extend shared expertise-kind and classification helpers in `crates/canon-engine/src/domain/mode.rs`
- [x] T005 [P] Extend publish-profile lineage helpers for the `expertise_input` metadata carrier and domain-family matching data in `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T006 [P] Add focused expertise-input validation coverage in `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/domain/publish_profile.rs`, `tests/domain_analysis_direct_runtime.rs`, and `tests/publish_runtime.rs`

**Checkpoint**: Shared expertise classification primitives and test entry points are ready

---

## Phase 2: User Story 1 - Publish One Canon-Owned Expertise Input Contract (Priority: P1) 🎯 MVP

**Goal**: Expose a stable Canon contract and source-level classification for supported expertise inputs

**Independent Test**: A maintainer can identify supported expertise kinds and their source-level classification without reading Canon implementation internals

### Validation for User Story 1

- [x] T007 [P] [US1] Add supported expertise-kind and explicit exclusion coverage in `crates/canon-engine/src/domain/mode.rs`
- [x] T008 [P] [US1] Add source-level classification and metadata coverage in `crates/canon-engine/src/domain/publish_profile.rs` and `tests/domain_analysis_direct_runtime.rs`

### Implementation for User Story 1

- [x] T009 [P] [US1] Implement mode-to-expertise classification in `crates/canon-engine/src/domain/mode.rs`
- [x] T010 [US1] Add `expertise_input` classification and compatibility helpers in `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T011 [US1] Fill the supported expertise kinds, classification rules, and compatibility content in `docs/integration/governed-expertise-input-contract.md` and `specs/052-governed-expertise-inputs/contracts/governed-expertise-input-contract.md`
- [x] T012 [US1] Capture MVP decisions in `specs/052-governed-expertise-inputs/decision-log.md`

**Checkpoint**: User Story 1 is independently valid as the expertise-input contract MVP

---

## Phase 3: User Story 2 - Preserve The Canon And Boundline Ownership Boundary (Priority: P1)

**Goal**: Keep Canon on the producer side of governed expertise while reusing existing publication semantics

**Independent Test**: The stable docs and publish-path behavior make it explicit that expertise inputs carry knowledge only, not runtime-role or provider directives

### Validation for User Story 2

- [x] T013 [P] [US2] Add publication-boundary coverage in `tests/publish_runtime.rs`
- [x] T014 [P] [US2] Add ownership-boundary coverage in `crates/canon-engine/src/domain/publish_profile.rs` and `tests/domain_analysis_direct_runtime.rs`

### Implementation for User Story 2

- [x] T015 [P] [US2] Align publish-path and metadata projection helpers for `expertise_input.domain_families` and publication-state output in `crates/canon-engine/src/orchestrator/publish.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T016 [US2] Cross-reference publication semantics in `docs/integration/governed-expertise-input-contract.md` while keeping `docs/integration/project-memory-promotion-contract.md` as the companion publication contract
- [x] T017 [US2] Refresh the feature-local classification brief in `specs/052-governed-expertise-inputs/contracts/expertise-classification-contract.md` and capture evidence in `specs/052-governed-expertise-inputs/validation-report.md`

**Checkpoint**: User Stories 1 and 2 both work independently and preserve the ownership boundary

---

## Phase 4: User Story 3 - Ignore Unknown Or Incompatible Expertise Inputs Safely (Priority: P2)

**Goal**: Fail closed for unknown expertise kinds, missing required classification metadata, blocked or pending publication states, and unsupported contract lines

**Independent Test**: Supported expertise inputs classify cleanly while unsupported kinds, blocked or pending publication states, or unsupported contract lines are rejected without invented fallback behavior

### Validation for User Story 3

- [x] T018 [P] [US3] Add compatibility coverage for unknown expertise kinds, blocked or pending publication states, unsupported contract lines, and non-matching `expertise_input.domain_families` in `crates/canon-engine/src/domain/publish_profile.rs` and `tests/publish_runtime.rs`
- [x] T019 [P] [US3] Add fail-closed contract-alignment coverage in `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/domain/publish_profile.rs`, and `tests/publish_runtime.rs`

### Implementation for User Story 3

- [x] T020 [P] [US3] Encode fail-closed compatibility helpers in `crates/canon-engine/src/domain/publish_profile.rs` and `crates/canon-engine/src/domain/mode.rs`
- [x] T021 [US3] Document additive-versus-breaking expertise evolution rules in `docs/integration/governed-expertise-input-contract.md` and `specs/052-governed-expertise-inputs/contracts/governed-expertise-input-contract.md`
- [x] T022 [US3] Capture compatibility decisions and validation evidence in `specs/052-governed-expertise-inputs/decision-log.md` and `specs/052-governed-expertise-inputs/validation-report.md`

**Checkpoint**: All user stories are independently functional and compatibility-safe

---

## Phase 5: Verification & Closeout

**Purpose**: Finish docs, formatting, lint, tests, cross-repo review, and coverage

- [x] T023 [P] Update downstream-facing runtime-compatibility and metadata surfaces in `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `assistant/plugin-metadata.json` when release drift affects consumer bootstrap guidance
- [x] T024 Run `cargo fmt --all` from the repository root
- [x] T025 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` from the repository root
- [x] T026 Run `cargo test --no-run --all-targets` and `cargo nextest run --workspace --all-features` from the repository root and record outcomes in `specs/052-governed-expertise-inputs/validation-report.md`
- [x] T027 Perform a cross-repo consistency review against `../boundline/specs/053-expert-pack-selection/spec.md` and record findings in `specs/052-governed-expertise-inputs/validation-report.md`
- [x] T028 Run focused modified-file coverage from the repository root and confirm at least 95% coverage for every modified file in `specs/052-governed-expertise-inputs/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0** starts immediately and includes the required first task version bump
- **Phase 1** depends on Phase 0 and blocks story work
- **Phases 2-4** depend on Phase 1 and should proceed in priority order
- **Phase 5** depends on all desired user stories

### User Story Dependencies

- **US1** delivers the expertise-input contract MVP and depends only on the foundational classification primitives
- **US2** builds on US1 and binds the contract to Canon publication semantics without expanding Canon scope
- **US3** builds on US1 and US2 and hardens compatibility behavior

### Parallel Opportunities

- T002 and T003 can run in parallel after release intent is fixed
- T005 and T006 can run in parallel after T004 starts
- Validation tasks marked [P] can run in parallel within each story
- T023 can run in parallel with final validation once the contract stabilizes

## Notes

- The first task is the Canon version bump, as requested
- The final task is modified-file coverage verification at 95% or higher, as requested
- Canon remains a semantic producer only throughout this implementation
- Implementation reused existing runtime and publish integration suites instead of creating new dedicated `governed_expertise_*.rs` test targets.
