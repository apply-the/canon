# Tasks: Semantic Artifact Contract

**Input**: Design documents from `/specs/056-semantic-artifact-contract/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Validation**: Layered validation is mandatory. This slice changes Canon-owned
integration contracts, packet metadata semantics, and governance projection
surfaces, so every user story includes contract or integration validation plus
recorded evidence in `validation-report.md`. Independent maintainer review is
required before closeout.

**Organization**: Tasks are grouped by user story to preserve independent
validation and auditability per increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- Include exact file paths in descriptions

## Phase 0: Governance & Artifacts

**Purpose**: Establish the artifacts and review controls that authorize
implementation.

- [ ] T001 Confirm execution mode, risk, scope boundaries, and invariants stay aligned in `specs/056-semantic-artifact-contract/spec.md` and `specs/056-semantic-artifact-contract/plan.md`
- [ ] T002 Update the semantic-carrier, eligibility, and compatibility trail in `specs/056-semantic-artifact-contract/decision-log.md`
- [ ] T003 Create validation sections for structural, logical, and independent review evidence in `specs/056-semantic-artifact-contract/validation-report.md`
- [ ] T004 Record the required independent maintainer review checkpoints in `specs/056-semantic-artifact-contract/validation-report.md` and `specs/056-semantic-artifact-contract/checklists/requirements.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare the stable and feature-local semantic contract surfaces plus reusable fixtures.

- [ ] T005 Create the stable integration contract draft in `tech-docs/integration/semantic-artifact-contract.md`
- [ ] T006 [P] Create semantic contract validation scaffolding in `tests/contract/semantic_artifact_contract.rs`, `tests/integration/semantic_artifact_projection.rs`, and `tests/integration/semantic_artifact_compatibility.rs`
- [ ] T007 [P] Create semantic descriptor fixture payloads in `tests/fixtures/semantic_artifact_descriptor_v1.json` and `tests/fixtures/semantic_artifact_descriptor_excluded_v1.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared metadata and projection primitives that all user stories depend on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T008 Extend typed semantic descriptor and eligibility models in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T009 [P] Thread semantic descriptor serialization through the publish pipeline in `crates/canon-engine/src/orchestrator/publish.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T010 [P] Extend governance projection support for semantic descriptor visibility in `crates/canon-cli/src/commands/governance/projection.rs`
- [ ] T011 [P] Add shared contract-validation helpers for semantic descriptor fixtures in `tests/contract/semantic_artifact_contract.rs` and `tests/publish_runtime.rs`
- [ ] T012 Align carrier terminology between existing integration contracts in `tech-docs/integration/project-memory-promotion-contract.md` and `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`

**Checkpoint**: Foundation ready. Canon has typed semantic metadata primitives,
carrier alignment, and shared validation helpers.

---

## Phase 3: User Story 1 - Publish One Semantic Eligibility Contract (Priority: P1) 🎯 MVP

**Goal**: Publish one Canon-owned contract that explicitly names the semantically
eligible and excluded artifact classes plus the metadata carrier path.

**Independent Test**: A maintainer can read the feature-local and stable
contracts and determine which artifact classes are eligible, excluded, or out
of scope without reading implementation code.

### Validation for User Story 1

- [ ] T013 [P] [US1] Write failing contract coverage for semantic eligibility and exclusion rules in `tests/contract/semantic_artifact_contract.rs`
- [ ] T014 [P] [US1] Write failing integration coverage for published semantic descriptor discovery in `tests/integration/semantic_artifact_projection.rs`
- [ ] T015 [US1] Record story-specific semantic eligibility decisions in `specs/056-semantic-artifact-contract/decision-log.md`

### Implementation for User Story 1

- [ ] T016 [US1] Update eligible and excluded artifact-class rules in `specs/056-semantic-artifact-contract/contracts/semantic-artifact-contract.md`
- [ ] T017 [US1] Promote the stable consumer-facing semantic contract draft in `tech-docs/integration/semantic-artifact-contract.md`
- [ ] T018 [US1] Align artifact-class and carrier references across Canon integration docs in `tech-docs/integration/project-memory-promotion-contract.md` and `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`
- [ ] T019 [US1] Capture validation evidence for eligible versus excluded semantic surfaces in `specs/056-semantic-artifact-contract/validation-report.md`

**Checkpoint**: User Story 1 should deliver a clear, producer-owned semantic
eligibility contract with one stable integration path.

---

## Phase 4: User Story 2 - Preserve Provenance Without Owning Retrieval Runtime (Priority: P1)

**Goal**: Preserve Canon provenance boundaries and semantic descriptor transport
without turning Canon into the downstream retrieval runtime.

**Independent Test**: A consumer can inspect the published semantic metadata,
identify the producer-owned provenance boundary, and understand that Canon does
not own local fragment IDs, ranking, or fallback policy.

### Validation for User Story 2

- [ ] T020 [P] [US2] Write failing contract coverage for provenance boundary and mixed-surface ownership rules in `tests/contract/semantic_artifact_contract.rs`
- [ ] T021 [P] [US2] Write failing integration coverage for publish and governance projection of semantic metadata in `tests/integration/semantic_artifact_projection.rs`
- [ ] T022 [US2] Record provenance-boundary decisions in `specs/056-semantic-artifact-contract/decision-log.md`

### Implementation for User Story 2

- [x] T023 [US2] Add typed semantic provenance and descriptor models in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T024 [US2] Thread semantic descriptor metadata through the publish flow without adding a second discovery path in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T025 [US2] Expose semantic provenance and exclusion state on governance inspection surfaces in `crates/canon-cli/src/commands/governance/projection.rs`
- [ ] T026 [US2] Capture provenance validation evidence and reviewer notes in `specs/056-semantic-artifact-contract/validation-report.md`

**Checkpoint**: User Story 2 should preserve Canon-owned provenance boundaries
while keeping downstream retrieval behavior outside Canon's authority.

---

## Phase 5: User Story 3 - Version Semantic Metadata Safely (Priority: P2)

**Goal**: Define additive versus breaking semantic changes clearly enough that
consumers can adopt or reject new contract lines safely.

**Independent Test**: A maintainer can inspect the contract and determine which
semantic changes are backward-compatible, which require a new major line, and
which rejection reasons downstream consumers should surface.

### Validation for User Story 3

- [ ] T027 [P] [US3] Write failing contract coverage for additive versus breaking semantic changes in `tests/contract/semantic_artifact_contract.rs`
- [ ] T028 [P] [US3] Write failing integration coverage for unsupported contract lines and missing semantic fields in `tests/integration/semantic_artifact_compatibility.rs`
- [ ] T029 [US3] Record contract-line evolution decisions in `specs/056-semantic-artifact-contract/decision-log.md`

### Implementation for User Story 3

- [ ] T030 [US3] Encode compatibility and rejection rules in `specs/056-semantic-artifact-contract/contracts/semantic-artifact-contract.md` and `tech-docs/integration/semantic-artifact-contract.md`
- [x] T031 [US3] Align runtime packet metadata validation with semantic contract-line checks in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/orchestrator/publish.rs`
- [ ] T032 [US3] Surface unsupported-contract and missing-field rejection reasons in `crates/canon-cli/src/commands/governance/projection.rs` and `specs/056-semantic-artifact-contract/validation-report.md`

**Checkpoint**: User Story 3 should make semantic compatibility evolution explicit and auditable.

---

## Final Phase: Verification & Compliance

**Purpose**: Complete validation, documentation, and independent review.

- [ ] T033 [P] Run structural contract validation and record diffs in `specs/056-semantic-artifact-contract/validation-report.md` and `tech-docs/integration/semantic-artifact-contract.md`
- [ ] T034 [P] Run compile and targeted test validation for semantic metadata surfaces in `Cargo.toml`, `tests/contract/semantic_artifact_contract.rs`, and `tests/integration/semantic_artifact_projection.rs`
- [ ] T035 Perform independent maintainer review and close the evidence trail in `specs/056-semantic-artifact-contract/validation-report.md`
- [ ] T036 Update contributor guidance for the semantic contract in `README.md`, `DEVELOPER.md`, and `AGENTS.md`
- [ ] T037 Bump the Canon workspace version and record release notes in `Cargo.toml`, `Cargo.lock`, and `CHANGELOG.md`
- [ ] T038 Refresh coverage artifacts for touched semantic metadata files and record the evidence in `lcov.info` and `specs/056-semantic-artifact-contract/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies and MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user stories.
- **User Stories (Phases 3-5)**: Depend on Foundational completion.
- **Verification & Compliance (Final Phase)**: Depends on all selected user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Starts after Foundational and is the MVP.
- **User Story 2 (P1)**: Starts after Foundational and builds on the semantic contract surface established by US1.
- **User Story 3 (P2)**: Starts after Foundational and depends on stable descriptor transport and provenance rules from US1 and US2.

### Within Each User Story

- Validation tasks MUST fail before implementation when the behavior is executable.
- Decision-log updates happen before the story is declared complete.
- Contract and metadata model changes happen before publish and CLI projection changes.
- Validation evidence is required before story sign-off.

### Parallel Opportunities

- Setup and Foundational tasks marked `[P]` can run in parallel.
- Validation tasks within a user story marked `[P]` can run in parallel.
- After Foundational completion, US1 and US2 may progress in parallel if the shared semantic descriptor model remains stable.

---

## Parallel Example: User Story 1

```bash
# Launch semantic eligibility validation together:
Task: "Write failing contract coverage for semantic eligibility and exclusion rules in tests/contract/semantic_artifact_contract.rs"
Task: "Write failing integration coverage for published semantic descriptor discovery in tests/integration/semantic_artifact_projection.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. Stop and validate the published semantic eligibility contract before proceeding.

### Incremental Delivery

1. Deliver US1 for semantic eligibility and carrier clarity.
2. Add US2 for provenance boundaries and projection visibility.
3. Add US3 for compatibility and rejection rules.
4. Finish with independent review and documentation closeout.

### Parallel Team Strategy

1. One contributor stabilizes the engine metadata model and publish flow.
2. A second contributor prepares contract docs and integration tests.
3. A third contributor can prepare governance projection updates once the descriptor model is stable.

---

## Notes

- `[P]` tasks touch different files and do not depend on unfinished work.
- `[US#]` labels keep traceability back to the feature specification.
- Keep producer ownership boundaries explicit in both code and contracts.
- Do not introduce a second semantic discovery path or retrieval runtime responsibility in this slice.
