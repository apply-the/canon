# Tasks: Canon S7 Delight Provider Contracts

**Input**: Design documents from `/specs/057-s7-delight-provider/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable contract-test
tasks before implementation changes, record evidence in
`specs/057-s7-delight-provider/validation-report.md`, and preserve the
Systemic-Impact independent review gate before the stable integration contract
lands.

**Organization**: Tasks are grouped by user story so each story remains
independently deliverable and auditable. Per user instruction, the first task is
the version bump, the literal penultimate task is the cyclomatic-complexity
review/refactor pass, and the final task is the clippy/coverage/fmt/commit
message closeout. The documentation/changelog/roadmap refresh is placed
immediately before the penultimate complexity task.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: User story label for story-phase tasks only (`[US1]`, `[US2]`, `[US3]`)
- Every task includes an exact file path

## Constitution Alignment

- Start from governance artifacts: mode, risk, scope, invariants, approval
  ownership, and evidence paths remain current before implementation starts.
- Keep Canon authoritative: the contract may expose only explicitly authorized
  artifact classes and must not drift into Boundline UX or orchestration.
- Preserve separation of generation and validation: contract-test tasks and
  independent review tasks are distinct from authoring tasks.
- Preserve the Systemic-Impact approval gate before publishing the stable
  integration document.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the delivery controls that authorize implementation.

- [X] T001 Bump the workspace and plugin version in `Cargo.toml` and `assistant/plugin-metadata.json`
- [X] T002 Reconfirm execution mode, risk classification, scope boundaries, and invariants in `specs/057-s7-delight-provider/spec.md` and `specs/057-s7-delight-provider/plan.md`
- [X] T003 Update implementation-phase decisions and approval-gate language in `specs/057-s7-delight-provider/decision-log.md`
- [X] T004 Update required reviewers, evidence placeholders, and human approval checkpoints in `specs/057-s7-delight-provider/validation-report.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the stable-doc and validation harness surfaces used by all stories.

- [X] T005 Create `docs/integration/delight-provider-contract.md` from `specs/057-s7-delight-provider/contracts/delight-provider-contract.md`
- [X] T006 [P] Create the contract validation scaffold in `tests/contract/delight_provider_contract.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the shared contract vocabulary, sync rules, and baseline verification before any story-specific work.

**⚠️ CRITICAL**: No user story work starts until this phase is complete.

- [X] T007 Align the contract identity block and publish-target references in `docs/integration/delight-provider-contract.md` and `specs/057-s7-delight-provider/contracts/delight-provider-contract.md`
- [X] T008 [P] Normalize authorized artifact-class names, Rust anchors, and metadata vocabulary in `specs/057-s7-delight-provider/data-model.md`
- [X] T009 [P] Normalize amendment, approval, and validation vocabulary in `specs/057-s7-delight-provider/decision-log.md` and `specs/057-s7-delight-provider/validation-report.md`
- [X] T010 Implement baseline sync assertions for owner, contract line, primary consumer, and stable-doc path in `tests/contract/delight_provider_contract.rs`
- [X] T011 Capture foundational validation evidence and reviewer ownership in `specs/057-s7-delight-provider/validation-report.md`

**Checkpoint**: Stable-doc scaffold, baseline contract vocabulary, and sync assertions are in place.

---

## Phase 3: User Story 1 - Contracted Canon Artifact Provision (Priority: P1) 🎯 MVP

**Goal**: Publish one stable Canon contract that enumerates exactly which governed artifact classes, metadata fields, and amendment rules Boundline S7 may consume.

**Independent Test**: The contract tests verify that only the six authorized artifact classes appear (packets, approval states, readiness signals, security findings, audit findings, promotion references), each class carries the required metadata, and the stable document exposes explicit amendment rules with no ambient Canon concepts.

### Validation for User Story 1 (MANDATORY)

- [X] T012 [P] [US1] Add failing assertions for the six authorized artifact classes and their required metadata tables in `tests/contract/delight_provider_contract.rs`
- [X] T013 [US1] Record US1 inventory decisions and invariant confirmations in `specs/057-s7-delight-provider/decision-log.md`

### Implementation for User Story 1

- [X] T014 [US1] Finalize the authorized artifact-class inventory and metadata sections in `docs/integration/delight-provider-contract.md`
- [X] T015 [P] [US1] Sync the same artifact inventory and metadata field definitions in `specs/057-s7-delight-provider/contracts/delight-provider-contract.md`
- [X] T016 [P] [US1] Sync `DelightProviderContractLine`, `DelightArtifactClass`, and `RequiredMetadataField` details in `specs/057-s7-delight-provider/data-model.md`
- [X] T017 [US1] Capture US1 contract-evidence results in `specs/057-s7-delight-provider/validation-report.md`

**Checkpoint**: User Story 1 is independently reviewable as the MVP contract boundary.

---

## Phase 4: User Story 2 - Degradation And Compatibility Signaling (Priority: P2)

**Goal**: Define and verify the degraded-state and compatibility-signaling rules that keep Boundline safe when Canon inputs are stale, incompatible, absent, or contradictory.

**Independent Test**: The contract tests verify that `available`, `stale`, `incompatible`, `absent`, and `contradicted` are defined and mapped to the relevant artifact classes and schema-version rules.

### Validation for User Story 2 (MANDATORY)

- [X] T018 [P] [US2] Add failing assertions for `available`, `stale`, `incompatible`, `absent`, and `contradicted` signaling plus schema-version rules in `tests/contract/delight_provider_contract.rs`
- [X] T019 [US2] Record US2 degradation and versioning decisions in `specs/057-s7-delight-provider/decision-log.md`

### Implementation for User Story 2

- [X] T020 [US2] Finalize compatibility signaling and schema-versioning sections in `docs/integration/delight-provider-contract.md`
- [X] T021 [P] [US2] Sync degradation conditions and eligibility rules in `specs/057-s7-delight-provider/contracts/delight-provider-contract.md`
- [X] T022 [P] [US2] Sync `CompatibilitySignal`, `DeprecatedArtifactClass`, and removal-epoch rules in `specs/057-s7-delight-provider/data-model.md`
- [X] T023 [US2] Capture US2 degraded-state evidence in `specs/057-s7-delight-provider/validation-report.md`

**Checkpoint**: User Stories 1 and 2 now define both the allowed inputs and the required degraded-state behavior.

---

## Phase 5: User Story 3 - Validation And Boundary Maintenance (Priority: P3)

**Goal**: Add the boundary-maintenance and amendment-validation rules that let Canon and Boundline detect contract drift over time.

**Independent Test**: The contract tests verify amendment procedure language, deprecated-class handling, and cross-document sync between the feature brief and the stable integration document.

### Validation for User Story 3 (MANDATORY)

- [X] T024 [P] [US3] Add failing assertions for amendment procedure, deprecated-class handling, and cross-document drift checks in `tests/contract/delight_provider_contract.rs`
- [X] T025 [US3] Record US3 boundary-maintenance and reviewer-checkpoint decisions in `specs/057-s7-delight-provider/decision-log.md`

### Implementation for User Story 3

- [X] T026 [US3] Finalize amendment procedure, deprecated classes, and out-of-scope boundary rules in `docs/integration/delight-provider-contract.md`
- [X] T027 [P] [US3] Sync `ContractAmendment` and boundary-maintenance rules in `specs/057-s7-delight-provider/data-model.md`
- [X] T028 [P] [US3] Sync amendment and cross-repo acknowledgment rules in `specs/057-s7-delight-provider/contracts/delight-provider-contract.md`
- [X] T029 [US3] Capture US3 validation evidence and approval-owner notes in `specs/057-s7-delight-provider/validation-report.md`

**Checkpoint**: All three user stories are independently testable and aligned to the same contract boundary.

---

## Final Phase: Verification & Compliance

**Purpose**: Run final validation, independent review, documentation closeout, maintainability review, and quality gates.

- [X] T030 Run the contract test suite for `tests/contract/delight_provider_contract.rs` and record structural and logical validation results in `specs/057-s7-delight-provider/validation-report.md`
- [X] T031 Perform the independent Canon↔Boundline review and capture the Systemic-Impact human approval evidence in `specs/057-s7-delight-provider/validation-report.md`
- [X] T032 Update `README.md`, `CHANGELOG.md`, `ROADMAP.md`, and any affected `docs/integration/*.md` references to reflect the delivered S7 delight-provider contract and remove roadmap items completed by this feature
- [X] T033 Review cyclomatic complexity and file length for modified Rust files, including `tests/contract/delight_provider_contract.rs` and any touched files under `crates/canon-engine/src/`, and refactor long functions or oversized files while preserving behavior
- [X] T034 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`, raise touched Rust files to at least 95% line coverage, run `cargo fmt`, and append two candidate commit messages to `specs/057-s7-delight-provider/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: starts first and authorizes all later work.
- **Phase 1: Setup**: depends on Phase 0 completion.
- **Phase 2: Foundational**: depends on Phase 1 and blocks every user story.
- **Phase 3: US1**: depends on Phase 2 and is the MVP release slice.
- **Phase 4: US2**: depends on US1 because degradation rules build on the authorized artifact inventory.
- **Phase 5: US3**: depends on US1 and US2 because amendment and drift checks require the finalized contract surface.
- **Final Phase**: depends on all chosen user stories being complete.

### User Story Dependencies

- **US1 (P1)**: no story dependency beyond Foundational; this is the minimum viable boundary.
- **US2 (P2)**: depends on US1's published artifact inventory and metadata vocabulary.
- **US3 (P3)**: depends on US1 and US2 because the drift-check rules must validate the completed contract line.

### Within Each User Story

- Add failing contract assertions before editing the stable integration document.
- Record story-specific decisions before declaring the story complete.
- Update the stable integration document before syncing the feature-local brief and data model.
- Capture evidence in `validation-report.md` before moving to the next story.

## Parallel Opportunities

- `T006` can run in parallel with `T005` once Phase 0 is complete.
- `T008` and `T009` can run in parallel during Foundational because they touch different artifacts.
- In US1, `T015` and `T016` can run in parallel after `T014`.
- In US2, `T021` and `T022` can run in parallel after `T020`.
- In US3, `T027` and `T028` can run in parallel after `T026`.

---

## Parallel Example: User Story 1

```bash
# After the stable contract inventory is finalized:
Task: "Sync the artifact inventory and metadata field definitions in specs/057-s7-delight-provider/contracts/delight-provider-contract.md"
Task: "Sync DelightProviderContractLine, DelightArtifactClass, and RequiredMetadataField details in specs/057-s7-delight-provider/data-model.md"
```

## Parallel Example: User Story 2

```bash
# After compatibility signaling is finalized in the stable doc:
Task: "Sync degradation conditions and eligibility rules in specs/057-s7-delight-provider/contracts/delight-provider-contract.md"
Task: "Sync CompatibilitySignal, DeprecatedArtifactClass, and removal-epoch rules in specs/057-s7-delight-provider/data-model.md"
```

## Parallel Example: User Story 3

```bash
# After amendment procedure and out-of-scope rules are finalized in the stable doc:
Task: "Sync ContractAmendment and boundary-maintenance rules in specs/057-s7-delight-provider/data-model.md"
Task: "Sync amendment and cross-repo acknowledgment rules in specs/057-s7-delight-provider/contracts/delight-provider-contract.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0, Phase 1, and Phase 2.
2. Deliver Phase 3 (US1) as the first shippable slice.
3. Run `T030` for US1 scope and update `specs/057-s7-delight-provider/validation-report.md`.
4. Stop for review before layering degradation and drift-management rules.

### Incremental Delivery

1. Ship US1 to establish the bounded contract line.
2. Add US2 to define degraded-state behavior and versioning safety.
3. Add US3 to harden amendment workflow and drift checks.
4. Finish with the final verification/compliance phase.

### Parallel Team Strategy

1. One engineer owns the stable integration document in `docs/integration/delight-provider-contract.md`.
2. A second engineer owns sync artifacts under `specs/057-s7-delight-provider/`.
3. A validator owns `tests/contract/delight_provider_contract.rs` and the evidence trail in `specs/057-s7-delight-provider/validation-report.md`.