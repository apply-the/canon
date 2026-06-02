---
description: "Task list for artifact indexing contract implementation"
---

# Tasks: Artifact Indexing Contract

**Input**: Design documents from `/specs/051-artifact-indexing-contract/`
**Prerequisites**: plan.md, spec.md

**Validation**: Layered validation is mandatory because this slice changes a
Canon-owned downstream contract and producer-side metadata behavior.

**Organization**: Tasks are grouped by user story so each contract increment can
be reviewed and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- Include exact file paths in descriptions

## Phase 0: Governance & Artifacts

**Purpose**: Establish the artifacts that authorize implementation

- [x] T001 Bump Canon workspace version to `0.55.0` in `Cargo.toml` and update `CHANGELOG.md`
- [x] T002 Create `specs/051-artifact-indexing-contract/decision-log.md`
- [x] T003 Create `specs/051-artifact-indexing-contract/validation-report.md`
- [x] T004 Record reviewer expectations and approval gates for the contract change in `specs/051-artifact-indexing-contract/decision-log.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare the supporting contract artifacts and validation scaffolds

- [x] T005 Create `specs/051-artifact-indexing-contract/research.md`, `specs/051-artifact-indexing-contract/data-model.md`, and `specs/051-artifact-indexing-contract/quickstart.md`
- [x] T006 [P] Draft feature-local derived contract docs in `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md` and `specs/051-artifact-indexing-contract/contracts/evidence-block-metadata-contract.md`, explicitly labeling them as non-normative mirrors of the stable Canon contract
- [x] T007 [P] Prepare validation scenarios and evidence capture sections in `specs/051-artifact-indexing-contract/validation-report.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared metadata primitives and publish-path guards

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T008 Extend shared artifact-indexing metadata primitives in `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T009 [P] Align artifact-class helpers and validation support in `crates/canon-engine/src/domain/artifact.rs`
- [x] T010 [P] Establish stable contract examples in `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`
- [x] T011 Implement publish-path guards for ambiguous or unsupported indexing vocabulary in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T012 Add shared test fixtures for artifact-indexing metadata validation in `tests/unit/artifact_indexing_fixtures.rs`

**Checkpoint**: Shared Canon metadata and publish-path guards are in place

---

## Phase 3: User Story 1 - Publish One Indexable Contract Surface (Priority: P1) 🎯 MVP

**Goal**: Publish one stable Canon-owned contract that defines indexable artifact classes and minimum metadata

**Independent Test**: A maintainer can inspect the Canon docs and producer-side metadata output and identify indexable artifact classes plus required fields without reading source code

### Validation for User Story 1

- [x] T013 [P] [US1] Add unit coverage for required and optional artifact-indexing metadata in `tests/unit/artifact_indexing_metadata.rs`
- [x] T014 [P] [US1] Add publish-path integration coverage for artifact indexing projection in `tests/integration/artifact_indexing_publish.rs`
- [x] T015 [US1] Record contract-surface decisions in `specs/051-artifact-indexing-contract/decision-log.md`

### Implementation for User Story 1

- [x] T016 [P] [US1] Codify required and optional indexing metadata in `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T017 [US1] Align publish metadata serialization and stable sidecar output in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T018 [US1] Extend the stable normative contract in `tech-docs/integration/project-memory-promotion-contract.md` and align the feature-local derived view in `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`
- [x] T019 [US1] Capture MVP validation evidence in `specs/051-artifact-indexing-contract/validation-report.md`

**Checkpoint**: User Story 1 is independently valid as a stable contract surface

---

## Phase 4: User Story 2 - Clarify Safety-Net And Evidence Semantics (Priority: P1)

**Goal**: Remove or define ambiguous artifact vocabulary and make evidence-block metadata explicit

**Independent Test**: Contract docs and publish output no longer leave `safety-net packets` ambiguous, and evidence blocks carry explicit producer and source metadata

### Validation for User Story 2

- [x] T020 [P] [US2] Add vocabulary-resolution, metadata-carrier discovery, and excluded-artifact coverage in `tests/integration/artifact_indexing_vocabulary.rs`
- [x] T021 [P] [US2] Add evidence-block metadata coverage in `tests/contract/evidence_block_metadata.rs`
- [x] T022 [US2] Record vocabulary and evidence-block decisions in `specs/051-artifact-indexing-contract/decision-log.md`

### Implementation for User Story 2

- [x] T023 [P] [US2] Resolve `safety-net packets` vocabulary, document at least one explicitly non-indexable artifact class, and define the metadata carrier and discovery rule in `tech-docs/integration/project-memory-promotion-contract.md` and `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`
- [x] T024 [US2] Align evidence-block metadata definitions in `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T025 [US2] Ensure publish output enforces the clarified evidence metadata in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T026 [US2] Capture validation evidence in `specs/051-artifact-indexing-contract/validation-report.md`

**Checkpoint**: User Stories 1 and 2 both work independently and ambiguity is removed

---

## Phase 5: User Story 3 - Version Metadata Without Expanding Canon Scope (Priority: P2)

**Goal**: Freeze additive-versus-breaking compatibility rules while preserving Canon as a semantic producer only

**Independent Test**: Maintainers can determine from docs and tests whether a metadata change is additive-compatible or requires a new contract line

### Validation for User Story 3

- [x] T027 [P] [US3] Add compatibility-rule coverage in `tests/contract/artifact_indexing_versioning.rs`
- [x] T028 [US3] Record compatibility-boundary decisions in `specs/051-artifact-indexing-contract/decision-log.md`

### Implementation for User Story 3

- [x] T029 [P] [US3] Document contract-line compatibility rules in `tech-docs/integration/project-memory-promotion-contract.md` and `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`
- [x] T030 [US3] Align artifact validation helpers with compatibility expectations in `crates/canon-engine/src/domain/artifact.rs`
- [x] T031 [US3] Preserve the producer-only boundary in publish-path metadata handling in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T032 [US3] Capture versioning validation evidence in `specs/051-artifact-indexing-contract/validation-report.md`

**Checkpoint**: All user stories are independently functional and version-safe

---

## Phase 6: Verification & Compliance

**Purpose**: Cross-cutting validation, documentation, and release closeout

- [x] T033 [P] Update downstream-facing docs in `tech-docs/integration/project-memory-promotion-contract.md`, `README.md`, and `AGENTS.md` if touched by the stabilized contract
- [x] T034 [P] Refresh `specs/051-artifact-indexing-contract/quickstart.md` and `specs/051-artifact-indexing-contract/validation-report.md`
- [x] T035 Run `cargo fmt --all` from the repository root
- [x] T036 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` from the repository root
- [x] T037 Run `cargo test --no-run --all-targets`, focused publish-path validation, and `cargo nextest run --workspace --all-features` from the repository root, then record outcomes in `specs/051-artifact-indexing-contract/validation-report.md`
- [x] T038 Perform an independent maintainer comparison review against existing Canon artifact-producing specs and record findings in `specs/051-artifact-indexing-contract/validation-report.md`
- [x] T039 Verify at least 95% coverage for every modified file and record the result in `specs/051-artifact-indexing-contract/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: MUST complete first
- **Setup (Phase 1)**: Depends on Phase 0
- **Foundational (Phase 2)**: Depends on Phase 1 and BLOCKS all user stories
- **User Stories (Phases 3-5)**: Depend on Phase 2
- **Verification & Compliance (Phase 6)**: Depends on all desired user stories

### User Story Dependencies

- **User Story 1 (P1)**: Delivers the stable contract MVP
- **User Story 2 (P1)**: Depends on the shared metadata primitives from Phase 2 and builds on the US1 contract surface
- **User Story 3 (P2)**: Depends on the final artifact vocabulary and metadata surface from US1 and US2

### Within Each User Story

- Validation tasks and failing checks before implementation
- Contract text and decision logging before producer-side code lands
- Metadata primitives before publish-path integration
- Evidence capture before story sign-off

### Parallel Opportunities

- T006 and T007 can run in parallel after Phase 0
- T009 and T010 can run in parallel after T008 starts
- Validation tasks marked [P] can run in parallel within each story
- T033 and T034 can run in parallel once implementation stabilizes

---

## Implementation Strategy

### MVP First

1. Complete Phases 0 and 1
2. Complete Phase 2
3. Complete Phase 3
4. Stop and validate the stable contract surface before expanding ambiguity or versioning work

### Incremental Delivery

1. Land the stable indexable artifact contract
2. Remove vocabulary ambiguity and clarify evidence metadata
3. Freeze additive-versus-breaking versioning rules
4. Finish with docs, lint, tests, and coverage

## Notes

- The first task is the version bump, as requested
- The final task is modified-file coverage verification at 95% or higher, as requested
- Canon remains a semantic producer only throughout this implementation