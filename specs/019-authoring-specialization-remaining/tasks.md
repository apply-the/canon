# Tasks: Mode Authoring Specialization Follow-On

**Input**: Design documents from `/specs/019-authoring-specialization-remaining/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`, `decision-log.md`, `validation-report.md`

**Validation**: Layered validation is mandatory. Add executable test tasks wherever artifact contracts, renderer behavior, docs synchronization, or run output can regress.

**Organization**: Tasks are grouped by user story so each story can be implemented, validated, and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no incomplete dependencies)
- **[Story]**: Which user story this belongs to (`US1`, `US2`, `US3`)
- All paths are repo-root-relative.

## Constitution Alignment

- Every feature starts with mode, risk, scope, invariant, and artifact tasks.
- No implementation task appears before the artifacts that authorize it.
- Every user story includes validation and evidence-capture work.
- Independent review remains separate from generation work.

## Phase 0: Governance & Artifacts

**Purpose**: Lock the controls that authorize implementation.

- [x] T001 Confirm execution mode `change`, risk `bounded-impact`, scope boundaries, and invariants in `specs/019-authoring-specialization-remaining/spec.md` and `specs/019-authoring-specialization-remaining/plan.md`
- [x] T002 Confirm the targeted authored-body contract and artifact-to-heading mapping in `specs/019-authoring-specialization-remaining/contracts/mode-authored-body-contracts.md`
- [x] T003 Confirm decision logging and validation scaffolding in `specs/019-authoring-specialization-remaining/decision-log.md` and `specs/019-authoring-specialization-remaining/validation-report.md`
- [x] T004 Record the required non-regression boundaries for recommendation-only posture, missing-body honesty, and non-target modes in `specs/019-authoring-specialization-remaining/decision-log.md`

---

## Phase 1: Setup (Shared Baseline)

**Purpose**: Capture the current targeted surfaces before feature work begins.

- [x] T005 Capture the focused baseline with `cargo test --test system_shaping_contract --test implementation_contract --test refactor_contract --test system_shaping_run --test implementation_run --test refactor_run` and record results in `specs/019-authoring-specialization-remaining/validation-report.md`
- [x] T006 [P] Confirm the current authored guidance surfaces in `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `defaults/embedded-skills/canon-implementation/skill-source.md`, `defaults/embedded-skills/canon-refactor/skill-source.md`, `.agents/skills/canon-system-shaping/SKILL.md`, `.agents/skills/canon-implementation/SKILL.md`, and `.agents/skills/canon-refactor/SKILL.md`
- [x] T007 [P] Confirm the current runtime and example surfaces in `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/service/mode_change.rs`, `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`, `docs/templates/canon-input/system-shaping.md`, `docs/templates/canon-input/implementation.md`, `docs/templates/canon-input/refactor.md`, `docs/examples/canon-input/system-shaping-billing.md`, `docs/examples/canon-input/implementation-auth-session-revocation.md`, and `docs/examples/canon-input/refactor-auth-session-cleanup.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared contract and runtime scaffolding that all user stories depend on.

**⚠️ CRITICAL**: No user-story implementation begins until this phase is complete.

- [x] T008 Extend `crates/canon-engine/src/artifacts/contract.rs` to align `system-shaping`, `implementation`, and `refactor` required sections with `specs/019-authoring-specialization-remaining/contracts/mode-authored-body-contracts.md`
- [x] T009 [P] Introduce the shared authored-section mapping and missing-body preservation scaffolding for targeted modes in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T010 [P] Restore authored-brief handoff for targeted modes in `crates/canon-engine/src/orchestrator/service/mode_change.rs` and verify the shaping handoff in `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`

**Checkpoint**: Shared contract, renderer scaffolding, and authored-brief handoff are ready for story work.

---

## Phase 3: User Story 1 - Authors Know Exactly What To Write (Priority: P1) 🎯 MVP

**Goal**: Make the user-facing skill, template, and example surfaces describe the authored H2 contract clearly for the three targeted modes.

**Independent Test**: Read the updated skill, template, and example for any targeted mode and confirm they enumerate the same required authored H2 sections without source-code inspection.

### Validation for User Story 1 (MANDATORY)

- [x] T011 [P] [US1] Add failing docs-sync coverage for `system-shaping` authored guidance in `tests/system_shaping_domain_modeling_docs.rs`
- [x] T012 [P] [US1] Add failing docs-sync coverage for `implementation` and `refactor` authored guidance in `tests/implementation_authoring_docs.rs` and `tests/refactor_authoring_docs.rs`

### Implementation for User Story 1

- [x] T013 [US1] Update authored guidance in `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `defaults/embedded-skills/canon-implementation/skill-source.md`, `defaults/embedded-skills/canon-refactor/skill-source.md`, `.agents/skills/canon-system-shaping/SKILL.md`, `.agents/skills/canon-implementation/SKILL.md`, and `.agents/skills/canon-refactor/SKILL.md`
- [x] T014 [US1] Convert the starter inputs to the canonical H2 contract in `docs/templates/canon-input/system-shaping.md`, `docs/templates/canon-input/implementation.md`, and `docs/templates/canon-input/refactor.md`
- [x] T015 [US1] Rewrite the worked examples to exercise the full authored packet contract in `docs/examples/canon-input/system-shaping-billing.md`, `docs/examples/canon-input/implementation-auth-session-revocation.md`, and `docs/examples/canon-input/refactor-auth-session-cleanup.md`
- [x] T016 [US1] Record authored-guidance and docs-sync evidence in `specs/019-authoring-specialization-remaining/decision-log.md` and `specs/019-authoring-specialization-remaining/validation-report.md`

**Checkpoint**: A Canon user can author a compliant packet for the three targeted modes without reading the Rust implementation.

---

## Phase 4: User Story 2 - Renderers Preserve Authored Body Honestly (Priority: P2)

**Goal**: Make the targeted runtime artifacts preserve canonical authored H2 bodies verbatim and emit explicit missing-body markers when they are absent.

**Independent Test**: Run each targeted mode with one complete authored brief and one incomplete brief, then confirm complete packets preserve authored sections verbatim and incomplete packets emit `## Missing Authored Body` naming the canonical heading.

### Validation for User Story 2 (MANDATORY)

- [x] T017 [P] [US2] Add failing contract coverage for the targeted authored-body section requirements in `tests/contract/system_shaping_contract.rs`, `tests/contract/implementation_contract.rs`, and `tests/contract/refactor_contract.rs`
- [x] T018 [P] [US2] Add failing renderer coverage for verbatim preservation, missing-body markers, and near-match heading rejection in `tests/system_shaping_authoring_renderer.rs`, `tests/implementation_authoring_renderer.rs`, and `tests/refactor_authoring_renderer.rs`
- [x] T019 [P] [US2] Add failing run coverage for complete and incomplete authored packets in `tests/integration/system_shaping_run.rs`, `tests/integration/implementation_run.rs`, and `tests/integration/refactor_run.rs`

### Implementation for User Story 2

- [x] T020 [US2] Extend authored-body rendering across all targeted packet artifacts in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T021 [US2] Update authored-source handoff and canonical H2 fixtures in `crates/canon-engine/src/orchestrator/service/mode_change.rs`, `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`, and `tests/direct_runtime_coverage.rs`
- [x] T022 [US2] Record runtime preservation, missing-body, and unchanged execution-posture evidence in `specs/019-authoring-specialization-remaining/decision-log.md` and `specs/019-authoring-specialization-remaining/validation-report.md`

**Checkpoint**: Reviewers can trust that the runtime preserves authored packet bodies honestly and surfaces missing context explicitly.

---

## Phase 5: User Story 3 - Maintainers Can Ship The Slice Safely (Priority: P3)

**Goal**: Keep roadmap, mode guidance, shared validation suites, and non-regression evidence synchronized so the delivered slice is honest and supportable.

**Independent Test**: Read the updated roadmap and mode guide, then run the targeted validation suite and confirm the repository documents the delivered slice, keeps remaining scope explicit, and proves non-regression.

### Validation for User Story 3 (MANDATORY)

- [x] T023 [P] [US3] Add failing docs-sync coverage for mode guidance and roadmap wording in `tests/mode_authoring_follow_on_docs.rs`
- [x] T024 [P] [US3] Add failing non-regression coverage for recommendation-only posture and canonical authored fixtures in `tests/policy_and_traces.rs`, `tests/refactor_preservation_run.rs`, and `tests/direct_runtime_coverage.rs`

### Implementation for User Story 3

- [x] T025 [US3] Update rollout guidance in `docs/guides/modes.md`, `ROADMAP.md`, and confirm `AGENTS.md` remains current for feature 019 plan context
- [x] T026 [US3] Update shared execution and preservation fixtures in `tests/policy_and_traces.rs`, `tests/refactor_preservation_run.rs`, and `tests/direct_runtime_coverage.rs`
- [x] T027 [US3] Record roadmap, guide, and non-regression evidence in `specs/019-authoring-specialization-remaining/decision-log.md` and `specs/019-authoring-specialization-remaining/validation-report.md`

**Checkpoint**: Maintainers have synchronized guidance, explicit remaining scope, and validation evidence for the delivered slice.

---

## Final Phase: Verification & Compliance

**Purpose**: Complete cross-cutting validation, independent review, and closeout.

- [x] T028 [P] Run structural validation: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`; record results in `specs/019-authoring-specialization-remaining/validation-report.md`
- [x] T029 [P] Run logical validation: the focused `system-shaping`, `implementation`, and `refactor` contract, renderer, run, docs-sync, and non-regression suites; record results in `specs/019-authoring-specialization-remaining/validation-report.md`
- [x] T030 Perform independent review of `specs/019-authoring-specialization-remaining/spec.md`, `specs/019-authoring-specialization-remaining/plan.md`, `specs/019-authoring-specialization-remaining/tasks.md`, and `specs/019-authoring-specialization-remaining/quickstart.md`, then record findings in `specs/019-authoring-specialization-remaining/validation-report.md`
- [x] T031 Confirm invariants, unchanged recommendation-only posture, non-target mode stability, and close `specs/019-authoring-specialization-remaining/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 0 must complete first.
- Phase 1 depends on Phase 0.
- Phase 2 depends on Phase 1 and blocks all story implementation.
- Phase 3 depends on Phase 2.
- Phase 4 depends on Phase 2 and should follow Phase 3 if one owner is carrying the shared runtime files in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/orchestrator/service/mode_change.rs`.
- Phase 5 depends on Phase 2 and should follow Phases 3 and 4 because it closes the shared docs and non-regression surfaces.
- Final Phase depends on all desired user stories being complete.

### User Story Dependencies

- **US1** can ship independently once the skills, templates, examples, and docs-sync tests agree on the same authored H2 contract.
- **US2** depends on the shared contract and authored-brief handoff from Phase 2 and can start after that point, but it should land after US1 if the same contributor owns the contract wording across docs and runtime.
- **US3** depends on the shared runtime behavior from US2 and on the authored guidance from US1 because it closes the guide, roadmap, and non-regression story.

### Within Each User Story

- Validation tasks happen before the corresponding implementation tasks.
- Contract and guidance changes land before a story is declared complete.
- Evidence capture happens before a story is marked done.

## Parallel Execution Examples

- T006 and T007 can run in parallel.
- T009 and T010 can run in parallel after T008.
- T011 and T012 can run in parallel.
- T017, T018, and T019 can run in parallel.
- T023 and T024 can run in parallel.
- T028 and T029 can run in parallel.

### Parallel Example: User Story 1

```bash
Task: "Add failing docs-sync coverage in tests/system_shaping_domain_modeling_docs.rs"
Task: "Add failing docs-sync coverage in tests/implementation_authoring_docs.rs and tests/refactor_authoring_docs.rs"
```

### Parallel Example: User Story 2

```bash
Task: "Add failing contract coverage in tests/contract/system_shaping_contract.rs, tests/contract/implementation_contract.rs, and tests/contract/refactor_contract.rs"
Task: "Add failing renderer coverage in tests/system_shaping_authoring_renderer.rs, tests/implementation_authoring_renderer.rs, and tests/refactor_authoring_renderer.rs"
Task: "Add failing run coverage in tests/integration/system_shaping_run.rs, tests/integration/implementation_run.rs, and tests/integration/refactor_run.rs"
```

### Parallel Example: User Story 3

```bash
Task: "Add failing docs-sync coverage in tests/mode_authoring_follow_on_docs.rs"
Task: "Add failing non-regression coverage in tests/policy_and_traces.rs, tests/refactor_preservation_run.rs, and tests/direct_runtime_coverage.rs"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Governance, Setup, and Foundational phases.
2. Complete User Story 1.
3. Stop and validate that authors can discover the full contract from skills, templates, and worked examples alone.

### Incremental Delivery

1. Deliver authored guidance and contract discoverability (US1).
2. Deliver honest runtime preservation and missing-body behavior (US2).
3. Deliver roadmap, guide, and non-regression closeout (US3).
4. Finish with structural validation, logical validation, independent review, and closeout.

### Parallel Team Strategy

1. One stream can prepare failing tests while another updates the next story's docs surfaces.
2. Shared runtime files in `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/artifacts/contract.rs`, and `crates/canon-engine/src/orchestrator/service/mode_change.rs` remain single-owner at a time.
3. Final validation and independent review stay separate from the implementation stream.