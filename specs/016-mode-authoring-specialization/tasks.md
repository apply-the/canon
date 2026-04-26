# Tasks: Mode Authoring Specialization

**Input**: Design documents from `/specs/016-mode-authoring-specialization/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`, `decision-log.md`, `validation-report.md`

**Validation**: Layered validation is mandatory. Add executable test tasks wherever renderer behavior, run output, docs synchronization, or regressions must be checked.

**Organization**: Tasks are grouped by user story so each story can be implemented, validated, and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no incomplete dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- All paths are repo-root-relative.

## Phase 0: Governance & Artifacts

**Purpose**: Lock governance, authoring contracts, and validation scaffolding before code changes begin.

- [x] T001 Confirm execution mode `change`, risk `bounded-impact`, scope boundaries, invariants, and exact-heading policy in `specs/016-mode-authoring-specialization/spec.md` and `specs/016-mode-authoring-specialization/plan.md`
- [x] T002 Confirm the per-mode authoring contracts in `specs/016-mode-authoring-specialization/contracts/requirements-authoring.md`, `specs/016-mode-authoring-specialization/contracts/discovery-authoring.md`, and `specs/016-mode-authoring-specialization/contracts/change-authoring.md`
- [x] T003 Confirm decision-log seed and validation-report scaffold in `specs/016-mode-authoring-specialization/decision-log.md` and `specs/016-mode-authoring-specialization/validation-report.md`
- [x] T004 Record validator parity expectations for `scripts/validate-canon-skills.sh` and `scripts/validate-canon-skills.ps1` in `specs/016-mode-authoring-specialization/validation-report.md`

---

## Phase 1: Setup (Shared Baseline)

**Purpose**: Verify the current workspace and the existing mode/doc surfaces before specialization changes.

- [x] T005 Verify `cargo test --workspace` passes before implementation begins
- [x] T006 [P] Confirm the current authored-input docs in `docs/templates/canon-input/requirements.md`, `docs/templates/canon-input/discovery.md`, and `docs/templates/canon-input/change.md`
- [x] T007 [P] Confirm the current worked examples in `docs/examples/canon-input/requirements-api-v2.md`, `docs/examples/canon-input/discovery-legacy-migration.md`, and `docs/examples/canon-input/change-add-caching.md`
- [x] T008 [P] Confirm the current skill sources in `defaults/embedded-skills/canon-requirements/skill-source.md`, `defaults/embedded-skills/canon-discovery/skill-source.md`, and `defaults/embedded-skills/canon-change/skill-source.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared authored-body primitives that all three updated modes depend on.

**⚠️ CRITICAL**: No user-story runtime work begins until this phase is complete.

- [x] T009 Add a generic authored-body missing marker constant (keeping any existing compatibility alias) in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T010 Add shared exact-heading authored-body extraction and missing-body rendering helpers in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T011 Add helper-level unit coverage for the shared authored-body extraction and missing-heading reference behavior in `crates/canon-engine/src/artifacts/markdown.rs`

**Checkpoint**: Shared authored-body helpers and fallback contract exist.

---

## Phase 3: User Story 1 - Authoring Contract Is Explicit Before Run (Priority: P1) 🎯 MVP

**Goal**: Make the authored-body contract discoverable and synchronized across skills, templates, and examples for `requirements`, `discovery`, and `change`.

**Independent Test**: Read the updated skill, template, and example for each first-slice mode and confirm they enumerate the same canonical authored H2 headings.

### Validation for User Story 1 (MANDATORY)

- [x] T012 [P] [US1] Add a requirements docs-sync test in `tests/requirements_authoring_docs.rs` covering `specs/016-mode-authoring-specialization/contracts/requirements-authoring.md`, `defaults/embedded-skills/canon-requirements/skill-source.md`, `.agents/skills/canon-requirements/SKILL.md`, `docs/templates/canon-input/requirements.md`, and `docs/examples/canon-input/requirements-api-v2.md`
- [x] T013 [P] [US1] Add a discovery docs-sync test in `tests/discovery_authoring_docs.rs` covering `specs/016-mode-authoring-specialization/contracts/discovery-authoring.md`, `defaults/embedded-skills/canon-discovery/skill-source.md`, `.agents/skills/canon-discovery/SKILL.md`, `docs/templates/canon-input/discovery.md`, and `docs/examples/canon-input/discovery-legacy-migration.md`
- [x] T014 [P] [US1] Add a change docs-sync test in `tests/change_authoring_docs.rs` covering `specs/016-mode-authoring-specialization/contracts/change-authoring.md`, `defaults/embedded-skills/canon-change/skill-source.md`, `.agents/skills/canon-change/SKILL.md`, `docs/templates/canon-input/change.md`, and `docs/examples/canon-input/change-add-caching.md`

### Implementation for User Story 1

- [x] T015 [US1] Update `defaults/embedded-skills/canon-requirements/skill-source.md` and `.agents/skills/canon-requirements/SKILL.md` with an explicit `Author Requirements Body Before Invoking Canon` section and canonical authored H2 headings
- [x] T016 [US1] Update `defaults/embedded-skills/canon-discovery/skill-source.md` and `.agents/skills/canon-discovery/SKILL.md` with an explicit `Author Discovery Body Before Invoking Canon` section and canonical authored H2 headings
- [x] T017 [US1] Update `defaults/embedded-skills/canon-change/skill-source.md` and `.agents/skills/canon-change/SKILL.md` with an explicit `Author Change Body Before Invoking Canon` section and canonical authored H2 headings
- [x] T018 [US1] Update `docs/templates/canon-input/requirements.md` and `docs/examples/canon-input/requirements-api-v2.md` to reflect the requirements authored-body contract
- [x] T019 [US1] Update `docs/templates/canon-input/discovery.md` and `docs/examples/canon-input/discovery-legacy-migration.md` to reflect the discovery authored-body contract
- [x] T020 [US1] Update `docs/templates/canon-input/change.md` and `docs/examples/canon-input/change-add-caching.md` to replace inline labels with canonical H2-authored sections and reflect the change authored-body contract
- [x] T021 [US1] Capture the synchronized skill/template/example contract evidence in `specs/016-mode-authoring-specialization/validation-report.md`

**Checkpoint**: All three first-slice modes have explicit authored-body contracts in skills, templates, and examples.

---

## Phase 4: User Story 2 - Renderer Preserves Authored Sections Honestly (Priority: P2)

**Goal**: Make `requirements`, `discovery`, and `change` preserve canonical authored sections verbatim and emit `## Missing Authored Body` naming the missing heading when sections are absent.

**Independent Test**: Run each updated mode with a complete authored brief and a derived incomplete brief, then confirm verbatim preservation and honest missing-body fallback.

### Validation for User Story 2 (MANDATORY)

- [x] T022 [P] [US2] Add requirements authored-body contract tests in `tests/requirements_authoring_contract.rs` and `tests/contract/requirements_authoring_contract.rs`
- [x] T023 [P] [US2] Add discovery authored-body contract tests in `tests/discovery_authoring_contract.rs` and `tests/contract/discovery_authoring_contract.rs`
- [x] T024 [P] [US2] Add change authored-body contract tests in `tests/change_authoring_contract.rs` and `tests/contract/change_authoring_contract.rs`
- [x] T025 [P] [US2] Add renderer behavior tests in `tests/requirements_authoring_renderer.rs`, `tests/discovery_authoring_renderer.rs`, and `tests/change_authoring_renderer.rs` for verbatim preservation, exact-heading matching, and missing-heading references
- [x] T026 [P] [US2] Add end-to-end run tests in `tests/requirements_authoring_run.rs`, `tests/discovery_authoring_run.rs`, and `tests/change_authoring_run.rs` using complete examples and derived negative fixtures with one removed required H2 section

### Implementation for User Story 2

- [x] T027 [US2] Extend `crates/canon-engine/src/artifacts/markdown.rs::render_requirements_artifact_from_evidence` and related helpers so requirements artifacts preserve canonical authored sections verbatim and emit heading-aware missing-body fallbacks
- [x] T028 [US2] Update `crates/canon-engine/src/orchestrator/service/mode_requirements.rs` so `render_requirements_artifact_from_evidence` receives raw authored `context_summary` alongside generation, critique, and denied-invocation evidence
- [x] T029 [US2] Extend `crates/canon-engine/src/artifacts/markdown.rs::render_discovery_artifact` and `crates/canon-engine/src/orchestrator/service/mode_discovery.rs` so discovery artifacts preserve canonical authored sections verbatim and emit heading-aware missing-body fallbacks
- [x] T030 [US2] Extend `crates/canon-engine/src/artifacts/markdown.rs::render_change_artifact` and `crates/canon-engine/src/orchestrator/service/mode_change.rs` so change artifacts preserve canonical authored sections verbatim and emit heading-aware missing-body fallbacks
- [x] T031 [US2] Capture runtime evidence, derived negative-fixture behavior, and non-regression notes in `specs/016-mode-authoring-specialization/validation-report.md`

**Checkpoint**: The three updated modes preserve authored sections honestly and expose explicit incompleteness when required content is absent.

---

## Phase 5: User Story 3 - Maintainers Can Review and Ship the Slice Safely (Priority: P3)

**Goal**: Make the slice auditable and shippable through roadmap, mode guidance, validator review, and documentation of remaining scope.

**Independent Test**: Read the updated roadmap and mode guidance, then confirm they describe the delivered first slice and the remaining roadmap scope without implying a full rollout.

### Validation for User Story 3 (MANDATORY)

- [x] T032 [P] [US3] Extend `tests/requirements_authoring_docs.rs`, `tests/discovery_authoring_docs.rs`, and `tests/change_authoring_docs.rs` or add sibling assertions so docs coverage includes `docs/guides/modes.md` and `ROADMAP.md`
- [x] T033 [P] [US3] Run `scripts/validate-canon-skills.sh` and execute `pwsh -File scripts/validate-canon-skills.ps1` when PowerShell is available, otherwise record a parity review of `scripts/validate-canon-skills.ps1` against the shell validator in `specs/016-mode-authoring-specialization/validation-report.md`

### Implementation for User Story 3

- [x] T034 [US3] Update `docs/guides/modes.md` to document authored-body expectations for `requirements`, `discovery`, and `change`
- [x] T035 [US3] Update `ROADMAP.md` to record the delivered first slice of Mode Authoring Specialization and preserve the remaining roadmap scope honestly
- [x] T036 [US3] Capture docs, roadmap, and validator-parity evidence in `specs/016-mode-authoring-specialization/validation-report.md`

**Checkpoint**: Maintainers can review, validate, and communicate the delivered first slice without scope ambiguity.

---

## Final Phase: Verification & Compliance

**Purpose**: Complete repository-wide validation, independent review, and closeout.

- [x] T037 [P] Run structural validation: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`; review `scripts/validate-canon-skills.ps1` for parity if validator logic changed
- [x] T038 [P] Run logical validation: targeted authored-body tests plus `cargo test --test requirements_contract --test discovery_contract --test change_contract --test backlog_contract --test architecture_contract --test pr_review_contract` for non-regression
- [x] T039 Perform independent review of `spec.md`, `plan.md`, and `tasks.md`, then run an isolated walkthrough using the updated examples and derived negative fixtures; record findings in `specs/016-mode-authoring-specialization/validation-report.md`
- [x] T040 Confirm invariants still hold, confirm non-target reference modes remain unchanged, and close `specs/016-mode-authoring-specialization/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 0 must complete first.
- Phase 1 depends on Phase 0.
- Phase 2 depends on Phase 1 and blocks all story implementation.
- Phase 3 (US1) depends on Phase 2.
- Phase 4 (US2) depends on Phase 2 and should follow US1 so runtime behavior matches the published authored-body contract.
- Phase 5 (US3) depends on US1 and US2 so docs and roadmap reflect shipped behavior.
- Final Phase depends on all user stories being complete.

### User Story Dependencies

- **US1** can ship independently once skills, templates, examples, and docs-sync tests pass.
- **US2** depends on the authored contract from US1 and the shared helpers from Phase 2.
- **US3** depends on US1 and US2 so roadmap and guidance reflect actual delivered behavior.

### Within Each User Story

- Validation tasks happen before the corresponding implementation tasks.
- Skill/template/example synchronization lands before runtime references to those canonical headings are declared complete.
- Renderer changes land before orchestrator call-path refinements are considered complete.
- Validation evidence is recorded before a story is marked done.

## Parallel Execution Examples

- T012, T013, and T014 can run in parallel.
- T015, T016, and T017 can run in parallel once the authoring contracts are stable.
- T018, T019, and T020 can run in parallel after the canonical headings are settled.
- T022, T023, T024, T025, and T026 can run in parallel as independent failing validations.
- T037 and T038 can run in parallel in the final phase.

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Governance, Setup, and Foundational phases.
2. Complete User Story 1.
3. Stop and validate that skills, templates, and examples are synchronized.

### Incremental Delivery

1. Deliver explicit authored-body contracts (US1).
2. Deliver honest runtime preservation and fallback behavior (US2).
3. Deliver maintainer-facing docs, roadmap, and validator closeout (US3).
4. Finish with full validation and independent review.

### Parallel Team Strategy

1. One stream updates skills and docs while another writes failing tests.
2. After Phase 2, renderer and orchestrator work can be split by mode.
3. Final validation and independent review remain separate from generation work.