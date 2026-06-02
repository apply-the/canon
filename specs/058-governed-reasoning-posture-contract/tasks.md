# Tasks: Governed Reasoning Posture Contract

**Input**: Design documents from `/specs/058-governed-reasoning-posture-contract/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. This feature combines a systemic-impact cross-repo contract, release-alignment metadata, and a runtime-adjacent maintainability refactor, so every story includes executable checks plus recorded evidence.

**Organization**: Tasks are grouped by user story so each increment can be implemented, validated, and audited independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this belongs to (e.g. `US1`, `US2`, `US3`)
- Include exact file paths in every task description

## Path Conventions

- Canon repo root paths are repo-relative (`crates/`, `tests/`, `tech-docs/`, `assistant/`)
- Boundline cross-repo references use repo-relative identifiers under `boundline/specs/061-reasoning-profile-contracts/`

## Phase 0: Governance & Artifacts

**Purpose**: Establish the durable artifacts that authorize implementation and review.

- [x] T001 Record execution mode, risk, scope boundaries, and invariants in `specs/058-governed-reasoning-posture-contract/spec.md` and `specs/058-governed-reasoning-posture-contract/plan.md`
- [x] T002 Create the durable decision and validation artifacts in `specs/058-governed-reasoning-posture-contract/decision-log.md` and `specs/058-governed-reasoning-posture-contract/validation-report.md`
- [x] T003 [P] Create the specification quality checklist and design scaffolds in `specs/058-governed-reasoning-posture-contract/checklists/requirements.md`, `specs/058-governed-reasoning-posture-contract/research.md`, and `specs/058-governed-reasoning-posture-contract/data-model.md`
- [x] T004 [P] Create the feature-local contract brief and quickstart in `specs/058-governed-reasoning-posture-contract/contracts/governed-reasoning-posture-contract.md` and `specs/058-governed-reasoning-posture-contract/quickstart.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Reconcile the branch-visible contract and release surfaces before deeper validation begins.

- [x] T005 [P] Reconcile `tech-docs/integration/governed-reasoning-posture-contract.md` with `specs/058-governed-reasoning-posture-contract/contracts/governed-reasoning-posture-contract.md`
- [x] T006 [P] Reconcile the release-alignment surfaces in `Cargo.toml`, `Cargo.lock`, `assistant/plugin-metadata.json`, `.claude-plugin/manifest.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/manifest.json`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `README.md`, `ROADMAP.md`, and `CHANGELOG.md`
- [x] T007 Create or reconcile executable contract scaffolds in `tests/contract/governed_reasoning_posture_contract.rs` and `tests/governed_reasoning_posture_contract.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the bounded feature surface that all user stories rely on.

**⚠️ CRITICAL**: No user story is complete until the contract, release metadata, and gatekeeper scope are all bounded under this phase.

- [x] T008 Record the active Boundline consumer pairing and validation ownership in `specs/058-governed-reasoning-posture-contract/validation-report.md`
- [x] T009 [P] Reconcile the feature-local data model and research notes with the stable contract in `specs/058-governed-reasoning-posture-contract/data-model.md` and `specs/058-governed-reasoning-posture-contract/research.md`
- [x] T010 [P] Bound the gatekeeper maintainability surface in `crates/canon-engine/src/orchestrator/gatekeeper.rs`, `crates/canon-engine/src/orchestrator/gatekeeper/context.rs`, `crates/canon-engine/src/orchestrator/gatekeeper/entrypoints.rs`, `crates/canon-engine/src/orchestrator/gatekeeper/rules.rs`, and `crates/canon-engine/src/orchestrator/gatekeeper/tests.rs`
- [x] T011 Capture foundational reconciliation evidence in `specs/058-governed-reasoning-posture-contract/validation-report.md`

**Checkpoint**: The branch has one explicit contract boundary, one explicit release-alignment boundary, and one explicit gatekeeper maintainability boundary.

---

## Phase 3: User Story 1 - Publish One Stable Reasoning Posture Contract (Priority: P1) 🎯 MVP

**Goal**: Publish a stable Canon-owned reasoning posture contract that downstream consumers can rely on without reading Canon implementation code.

**Independent Test**: A maintainer can read the Canon stable doc and the paired contract test and determine the contract line, required fields, supported vocabulary, and supported release window without source-code inference.

### Validation for User Story 1

- [x] T012 [P] [US1] Add or reconcile failing assertions for contract identity, required fields, supported vocabulary, and release windows in `tests/contract/governed_reasoning_posture_contract.rs`
- [x] T013 [US1] Record contract identity and producer-boundary decisions in `specs/058-governed-reasoning-posture-contract/decision-log.md`

### Implementation for User Story 1

- [x] T014 [US1] Finalize the stable Canon contract wording in `tech-docs/integration/governed-reasoning-posture-contract.md`
- [x] T015 [P] [US1] Sync the feature-local contract brief and data model in `specs/058-governed-reasoning-posture-contract/contracts/governed-reasoning-posture-contract.md` and `specs/058-governed-reasoning-posture-contract/data-model.md`
- [x] T016 [US1] Capture User Story 1 evidence in `specs/058-governed-reasoning-posture-contract/validation-report.md`

**Checkpoint**: The Canon producer contract is explicit and independently reviewable.

---

## Phase 4: User Story 2 - Fail Closed On Drift And Version Mismatch (Priority: P2)

**Goal**: Make contract drift and release-surface drift executable and visible before downstream execution starts.

**Independent Test**: The contract test plus release-surface validation fail on stale manifests, stale runtime-compatibility metadata, unsupported contract lines, and incompatible release windows.

### Validation for User Story 2

- [x] T017 [P] [US2] Add or reconcile failing release-alignment checks in `tests/assistant_plugin_packages.rs` and `tests/contract/governed_reasoning_posture_contract.rs`
- [x] T018 [US2] Record release-alignment decisions in `specs/058-governed-reasoning-posture-contract/decision-log.md`

### Implementation for User Story 2

- [x] T019 [US2] Align versioned contract-facing metadata in `Cargo.toml`, `Cargo.lock`, `assistant/plugin-metadata.json`, `.claude-plugin/manifest.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/manifest.json`, and `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- [x] T020 [P] [US2] Align operator-facing release docs in `README.md`, `ROADMAP.md`, and `CHANGELOG.md`
- [x] T021 [US2] Capture User Story 2 evidence in `specs/058-governed-reasoning-posture-contract/validation-report.md`

**Checkpoint**: Contract identity and release metadata align on one executable truth.

---

## Phase 5: User Story 3 - Preserve Gatekeeper Behavior While Restoring Maintainability (Priority: P3)

**Goal**: Keep the already touched gatekeeper runtime surface reviewable through a sibling-module split without changing gate semantics.

**Independent Test**: Representative gatekeeper tests confirm stable public entrypoints and gate outcomes for the touched modes after the split.

### Validation for User Story 3

- [x] T022 [P] [US3] Add or reconcile behavior-preservation coverage in `crates/canon-engine/src/orchestrator/gatekeeper/tests.rs`
- [x] T023 [US3] Record the maintainability-only boundary for the gatekeeper work in `specs/058-governed-reasoning-posture-contract/decision-log.md`

### Implementation for User Story 3

- [x] T024 [US3] Finalize the public gatekeeper export surface in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/gatekeeper/entrypoints.rs`
- [x] T025 [P] [US3] Finalize the bounded context and helper split in `crates/canon-engine/src/orchestrator/gatekeeper/context.rs` and `crates/canon-engine/src/orchestrator/gatekeeper/rules.rs`
- [x] T026 [US3] Capture User Story 3 evidence in `specs/058-governed-reasoning-posture-contract/validation-report.md`

**Checkpoint**: The gatekeeper split remains reviewable and behavior-preserving.

---

## Final Phase: Verification & Compliance

**Purpose**: Run final validation, record cross-repo evidence, and close release-facing follow-through.

- [x] T027 Run the governed reasoning posture contract tests and append the results to `specs/058-governed-reasoning-posture-contract/validation-report.md`
- [x] T028 Run the release-alignment validation, including `tests/assistant_plugin_packages.rs`, and append the results to `specs/058-governed-reasoning-posture-contract/validation-report.md`
- [x] T029 Run the gatekeeper-focused tests, `cargo test --no-run --all-targets`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, and `cargo fmt`, then append the results to `specs/058-governed-reasoning-posture-contract/validation-report.md`
- [x] T030 Perform the Canon and Boundline cross-repo review against `boundline/specs/061-reasoning-profile-contracts/`, classify any non-058 staged carry-forward surfaces in `specs/058-governed-reasoning-posture-contract/validation-report.md`, and update any final release-facing docs that remain affected

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. Must complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion and bounds every later story.
- **User Story 1 (Phase 3)**: Depends on Foundational completion.
- **User Story 2 (Phase 4)**: Depends on Foundational completion and reuses the explicit contract boundary stabilized in User Story 1.
- **User Story 3 (Phase 5)**: Depends on Foundational completion and the gatekeeper branch inventory captured there.
- **Final Phase**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Independent MVP after Foundational.
- **User Story 2 (P2)**: Builds on the explicit contract identity but remains independently testable through metadata and version drift checks.
- **User Story 3 (P3)**: Builds on the bounded runtime surface from Foundational but remains independently testable through gatekeeper behavior preservation.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation sign-off.
- Decision updates MUST be recorded before story closeout.
- Stable-doc and feature-local-doc alignment must be established before evidence is recorded.
- Gatekeeper organization work must remain subordinate to behavior preservation.

### Parallel Opportunities

- `T005` and `T006` can run in parallel after the governance artifacts exist.
- `T009` and `T010` can run in parallel during Foundational because they touch different surfaces.
- Story validation tasks marked `[P]` can run in parallel.
- In User Story 2, `T019` and `T020` can run in parallel.
- In User Story 3, `T024` and `T025` can run in parallel once behavior-preservation tests exist.

---

## Parallel Example: User Story 1

```bash
# Prepare contract validation and doc sync together:
Task: "Add or reconcile failing assertions for contract identity, required fields, supported vocabulary, and release windows in tests/contract/governed_reasoning_posture_contract.rs"
Task: "Sync the feature-local contract brief and data model in specs/058-governed-reasoning-posture-contract/contracts/governed-reasoning-posture-contract.md and specs/058-governed-reasoning-posture-contract/data-model.md"
```

## Parallel Example: User Story 2

```bash
# Prepare metadata drift checks and doc alignment together:
Task: "Add or reconcile failing release-alignment checks in tests/assistant_plugin_packages.rs and tests/contract/governed_reasoning_posture_contract.rs"
Task: "Align operator-facing release docs in README.md, ROADMAP.md, and CHANGELOG.md"
```

## Parallel Example: User Story 3

```bash
# Prepare gatekeeper behavior coverage and module finalization together:
Task: "Add or reconcile behavior-preservation coverage in crates/canon-engine/src/orchestrator/gatekeeper/tests.rs"
Task: "Finalize the bounded context and helper split in crates/canon-engine/src/orchestrator/gatekeeper/context.rs and crates/canon-engine/src/orchestrator/gatekeeper/rules.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm the Canon producer contract is clear and aligned before widening scope.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Deliver User Story 1 and validate the stable contract boundary.
3. Deliver User Story 2 and validate drift and version alignment.
4. Deliver User Story 3 and validate gatekeeper behavior preservation.
5. Finish with the Final Phase cross-repo and workspace validation.

### Parallel Team Strategy

With multiple developers:

1. One maintainer owns the stable contract and release metadata.
2. A second maintainer owns the feature-local spec artifacts and evidence log.
3. A third maintainer or reviewer owns gatekeeper behavior-preservation validation.
4. Merge only after the validation report records both Canon-local evidence and Boundline cross-repo review.
