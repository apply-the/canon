---
description: "Task list for authority zone contract implementation"
---

# Tasks: Authority Zone Contract

**Input**: Design documents from `/specs/054-authority-zone-contract/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md, contracts/

**Validation**: Layered validation is mandatory because this slice changes a
Canon-owned downstream contract, stable metadata surfaces, and integration
documentation.

**Organization**: Tasks are grouped by user story so each contract increment can
be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story or closeout group this task belongs to (`US1`, `US2`, `US3`, `Closeout`)
- Include exact file paths in descriptions

## Phase 0: Release & Stable Contract Baseline

**Purpose**: Establish the required version bump, stable contract brief, and closeout scaffold

- [x] T001 Bump the Canon workspace version from `0.53.0` to `0.54.0` in `Cargo.toml` and update `CHANGELOG.md`
- [x] T002 [P] Align the local contract briefs in `specs/054-authority-zone-contract/contracts/authority-governance-v1-contract.md` and `specs/054-authority-zone-contract/contracts/authority-governance-adapter-projection.md` with the stable release intent
- [x] T003 [P] Create the closeout evidence scaffold in `specs/054-authority-zone-contract/validation-report.md` and align it with `specs/054-authority-zone-contract/decision-log.md`

---

## Phase 1: Foundational (Blocking Prerequisites)

**Purpose**: Shared authority vocabulary, envelope types, and validation entry points

**⚠️ CRITICAL**: No user story work should begin until this phase is complete

- [x] T004 Extend shared authority-zone and change-class primitives in `crates/canon-engine/src/domain/policy.rs`
- [x] T005 [P] Add typed `authority-governance-v1` envelope and metadata helpers in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T006 [P] Extend intended-persona and advisory stage-role-hint metadata in `crates/canon-engine/src/domain/mode.rs`
- [ ] T007 [P] Add foundational validation entry points in `tests/governance_adapter_surface.rs`, `tests/mode_profiles.rs`, and `tests/policy_and_traces.rs`

**Checkpoint**: Shared authority-contract primitives and test entry points are ready

---

## Phase 2: User Story 1 - Publish One Authority Contract For Consumers (Priority: P1) 🎯 MVP

**Goal**: Publish a stable `authority-governance-v1` contract with explicit required and optional fields

**Independent Test**: A downstream maintainer can recover the required contract fields and recognize optional provenance without reading Canon implementation internals

### Validation for User Story 1

- [ ] T008 [P] [US1] Add required-versus-optional field coverage in `tests/governance_adapter_surface.rs` and `tests/policy_and_traces.rs`
- [ ] T009 [P] [US1] Add contract-shape coverage for packet metadata carriers in `crates/canon-engine/src/domain/artifact.rs` and `tests/governance_adapter_surface.rs`

### Implementation for User Story 1

- [x] T010 [P] [US1] Implement the typed `authority-governance-v1` envelope in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T011 [US1] Extend policy vocabulary for `AuthorityZone` and `ChangeClass` in `crates/canon-engine/src/domain/policy.rs`
- [x] T012 [US1] Align the stable and feature-local contract docs in `docs/integration/governance-adapter.md` and `specs/054-authority-zone-contract/contracts/authority-governance-v1-contract.md`
- [x] T013 [US1] Capture the MVP contract decisions in `specs/054-authority-zone-contract/decision-log.md`

**Checkpoint**: User Story 1 is independently valid as the authority-contract MVP

---

## Phase 3: User Story 2 - Preserve The Semantic And Runtime Boundary (Priority: P2)

**Goal**: Keep Canon on the semantic side of the boundary while publishing intended personas and advisory-only role hints

**Independent Test**: Mode metadata and docs make it explicit that Canon publishes semantics and hints only, while downstream runtimes keep runtime choice and stop behavior

### Validation for User Story 2

- [ ] T014 [P] [US2] Add mode-profile coverage for intended personas and advisory-only role hints in `tests/mode_profiles.rs`
- [ ] T015 [P] [US2] Add boundary coverage that rejects runtime-directive semantics in `tests/governance_adapter_surface.rs` and `tests/policy_and_traces.rs`

### Implementation for User Story 2

- [x] T016 [P] [US2] Extend mode metadata for intended personas, optional anti-behaviors, and optional stage-role hints in `crates/canon-engine/src/domain/mode.rs`
- [x] T017 [US2] Keep adapter and metadata projection advisory-only in `crates/canon-engine/src/domain/artifact.rs`, `crates/canon-engine/src/domain/publish_profile.rs`, and `docs/integration/governance-adapter.md`
- [x] T018 [US2] Publish the human-facing boundary guide in `docs/guides/governed-personas-and-authority-zones.md` and keep `specs/054-authority-zone-contract/contracts/authority-governance-adapter-projection.md` aligned

**Checkpoint**: User Stories 1 and 2 both work independently and preserve the ownership boundary

---

## Phase 4: User Story 3 - Evolve The Contract Safely (Priority: P3)

**Goal**: Make missing required fields, unsupported contract lines, and unknown optional metadata compatibility-safe for downstream consumers

**Independent Test**: Compatible packets remain consumable, missing required fields fail closed, and additive metadata stays ignorable without changing the meaning of the contract line

### Validation for User Story 3

- [ ] T019 [P] [US3] Add fail-closed coverage for unsupported contract lines and missing required fields in `tests/governance_adapter_surface.rs` and `tests/policy_and_traces.rs`
- [ ] T020 [P] [US3] Add additive-compatibility coverage for unknown optional metadata and optional missing provenance in `tests/mode_profiles.rs` and `tests/governance_adapter_surface.rs`

### Implementation for User Story 3

- [x] T021 [P] [US3] Encode fail-closed required-field validation and additive optional-field behavior in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T022 [US3] Document compatibility rules and consumer guidance in `docs/integration/governance-adapter.md` and `specs/054-authority-zone-contract/contracts/authority-governance-v1-contract.md`
- [x] T023 [US3] Capture compatibility evidence and closeout notes in `specs/054-authority-zone-contract/decision-log.md` and `specs/054-authority-zone-contract/validation-report.md`

**Checkpoint**: All user stories are independently functional and compatibility-safe

---

## Phase 5: Verification & Closeout

**Purpose**: Finish docs, release surfaces, formatting, lint, tests, and coverage

- [x] T024 [P] Update release-facing and integration docs in `README.md`, `docs/integration/governance-adapter.md`, `docs/guides/governed-personas-and-authority-zones.md`, and `specs/054-authority-zone-contract/quickstart.md`
- [x] T025 [P] Record cross-repo alignment evidence against `../boundline/specs/056-authority-zoned-councils/spec.md` in `specs/054-authority-zone-contract/validation-report.md`
- [x] T026 Run `cargo fmt --all` from the repository root
- [x] T027 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` from the repository root
- [x] T028 Run `cargo test --no-run --all-targets` and `cargo nextest run --workspace --all-features` from the repository root
- [x] T029 Run focused modified-file coverage from the repository root and confirm at least 95% coverage for every modified file in `specs/054-authority-zone-contract/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0** starts immediately and includes the required first-task version bump
- **Phase 1** depends on Phase 0 and blocks story work
- **Phases 2-4** depend on Phase 1 and should proceed in priority order
- **Phase 5** depends on all desired user stories

### User Story Dependencies

- **US1** delivers the contract MVP and depends only on the foundational authority vocabulary and envelope types
- **US2** builds on US1 and binds the contract to intended personas and advisory-only role hints without widening Canon scope
- **US3** builds on US1 and US2 and hardens compatibility behavior for downstream consumers

### Parallel Opportunities

- T002 and T003 can run in parallel after release intent is fixed
- T005, T006, and T007 can run in parallel after T004 starts
- Validation tasks marked [P] can run in parallel within each story
- T024 and T025 can run in parallel with final validation once behavior stabilizes

## Notes

- The first task is the Canon version bump, as requested
- The task list includes explicit docs, quickstart, and changelog work, as requested
- The final task is modified-file coverage verification at 95% or higher, as requested
- Canon remains a semantic producer only; downstream runtime choice stays outside this implementation