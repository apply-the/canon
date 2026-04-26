# Tasks: Domain Modeling And Boundary Design

**Input**: Design documents from `/specs/017-domain-boundary-design/`
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

**Purpose**: Lock the controls that permit implementation to begin.

- [x] T001 Confirm execution mode `change`, risk `bounded-impact`, scope boundaries, and invariants in `specs/017-domain-boundary-design/spec.md` and `specs/017-domain-boundary-design/plan.md`
- [x] T002 Confirm the three mode-level contracts in `specs/017-domain-boundary-design/contracts/system-shaping-domain-modeling.md`, `specs/017-domain-boundary-design/contracts/architecture-context-map.md`, and `specs/017-domain-boundary-design/contracts/change-domain-slice.md`
- [x] T003 Confirm decision logging and validation scaffolding in `specs/017-domain-boundary-design/decision-log.md` and `specs/017-domain-boundary-design/validation-report.md`
- [x] T004 Record the expected gate ownership for `domain-model.md` and `context-map.md` in `specs/017-domain-boundary-design/decision-log.md`

---

## Phase 1: Setup (Shared Baseline)

**Purpose**: Verify the current workspace and the existing mode surfaces before feature work begins.

- [x] T005 Verify `cargo test --workspace` passes before implementation begins and record the baseline in `specs/017-domain-boundary-design/validation-report.md`
- [x] T006 [P] Confirm the current system-shaping guidance in `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `.agents/skills/canon-system-shaping/SKILL.md`, `docs/templates/canon-input/system-shaping.md`, and `docs/examples/canon-input/system-shaping-billing.md`
- [x] T007 [P] Confirm the current architecture guidance in `defaults/embedded-skills/canon-architecture/skill-source.md`, `.agents/skills/canon-architecture/SKILL.md`, `docs/templates/canon-input/architecture.md`, and `docs/examples/canon-input/architecture-state-management.md`
- [x] T008 [P] Confirm the current change guidance in `defaults/embedded-skills/canon-change/skill-source.md`, `.agents/skills/canon-change/SKILL.md`, `docs/templates/canon-input/change.md`, and `docs/examples/canon-input/change-add-caching.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared runtime and validation primitives that all three stories depend on.

**⚠️ CRITICAL**: No user-story implementation begins until this phase is complete.

- [x] T009 Add or extend shared authored-section helpers for domain-modeling extraction and explicit missing-body rendering in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T010 [P] Add shared artifact-contract plumbing for additive domain-modeling surfaces in `crates/canon-engine/src/artifacts/contract.rs`, `defaults/methods/system-shaping.toml`, and `defaults/methods/architecture.toml`
- [x] T011 [P] Add shared summary and packet-surface plumbing for the new or strengthened domain-modeling artifacts in `crates/canon-engine/src/orchestrator/service/summarizers.rs`

**Checkpoint**: Shared helper, contract, and summary infrastructure exists for the three target modes.

---

## Phase 3: User Story 1 - Shape A System Around Real Domain Boundaries (Priority: P1) 🎯 MVP

**Goal**: Make `system-shaping` emit a first-class domain-model artifact with bounded contexts, ubiquitous language, core/supporting hypotheses, and explicit domain invariants.

**Independent Test**: Run `system-shaping` with a realistic authored brief and confirm the packet includes `domain-model.md` with the required sections and honest missing-body behavior.

### Validation for User Story 1 (MANDATORY)

- [x] T012 [P] [US1] Extend system-shaping artifact-contract coverage in `tests/contract/system_shaping_contract.rs` and `tests/system_shaping_contract.rs`
- [x] T013 [P] [US1] Extend system-shaping run coverage in `tests/integration/system_shaping_run.rs` and `tests/system_shaping_run.rs`
- [x] T014 [P] [US1] Add docs-sync coverage for system-shaping domain-modeling guidance in `tests/system_shaping_domain_modeling_docs.rs`

### Implementation for User Story 1

- [x] T015 [US1] Update `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `.agents/skills/canon-system-shaping/SKILL.md`, `docs/templates/canon-input/system-shaping.md`, and `docs/examples/canon-input/system-shaping-billing.md` with the domain-model authored sections
- [x] T016 [US1] Add `domain-model.md` to the runtime contract and method metadata in `crates/canon-engine/src/artifacts/contract.rs` and `defaults/methods/system-shaping.toml`
- [x] T017 [US1] Extend `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/orchestrator/service/mode_shaping.rs` so `system-shaping` renders and persists `domain-model.md`
- [x] T018 [US1] Record system-shaping design and validation evidence in `specs/017-domain-boundary-design/decision-log.md` and `specs/017-domain-boundary-design/validation-report.md`

**Checkpoint**: `system-shaping` emits a first-class domain-model artifact and the authored guidance is synchronized.

---

## Phase 4: User Story 2 - Formalize Context Boundaries In Architecture (Priority: P2)

**Goal**: Make `architecture` emit a first-class context map with bounded contexts, context relationships, seams, ownership boundaries, and shared invariants.

**Independent Test**: Run `architecture` with a context-rich authored brief and confirm the packet includes `context-map.md` with the required sections and honest missing-body behavior.

### Validation for User Story 2 (MANDATORY)

- [x] T019 [P] [US2] Extend architecture artifact-contract coverage in `tests/contract/architecture_contract.rs` and `tests/architecture_contract.rs`
- [x] T020 [P] [US2] Extend architecture run coverage in `tests/integration/architecture_run.rs` and `tests/architecture_run.rs`
- [x] T021 [P] [US2] Add docs-sync coverage for architecture context-map guidance in `tests/architecture_domain_modeling_docs.rs`

### Implementation for User Story 2

- [x] T022 [US2] Update `defaults/embedded-skills/canon-architecture/skill-source.md`, `.agents/skills/canon-architecture/SKILL.md`, `docs/templates/canon-input/architecture.md`, and `docs/examples/canon-input/architecture-state-management.md` with the context-map authored sections
- [x] T023 [US2] Add `context-map.md` to the runtime contract and method metadata in `crates/canon-engine/src/artifacts/contract.rs` and `defaults/methods/architecture.toml`
- [x] T024 [US2] Extend `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/orchestrator/service/mode_shaping.rs` so `architecture` renders and persists `context-map.md`
- [x] T025 [US2] Record architecture context-map decisions and validation evidence in `specs/017-domain-boundary-design/decision-log.md` and `specs/017-domain-boundary-design/validation-report.md`

**Checkpoint**: `architecture` emits a first-class context map and the authored guidance is synchronized.

---

## Phase 5: User Story 3 - Keep Changes Honest About Domain Impact (Priority: P3)

**Goal**: Make `change` explicitly capture domain slice, preserved domain invariants, ownership boundaries, and cross-context risk inside its existing packet.

**Independent Test**: Run `change` with a bounded authored brief and confirm the packet surfaces the domain slice, preserved domain invariants, and explicit cross-context risk when boundaries are stressed.

### Validation for User Story 3 (MANDATORY)

- [x] T026 [P] [US3] Extend change artifact-contract coverage in `tests/contract/change_contract.rs` and `tests/change_contract.rs`
- [x] T027 [P] [US3] Extend change run coverage in `tests/integration/change_run.rs`, `tests/change_run.rs`, and `tests/integration/change_governed_execution.rs`
- [x] T028 [P] [US3] Add docs-sync coverage for change domain-slice guidance in `tests/change_domain_modeling_docs.rs`

### Implementation for User Story 3

- [x] T029 [US3] Update `defaults/embedded-skills/canon-change/skill-source.md`, `.agents/skills/canon-change/SKILL.md`, `docs/templates/canon-input/change.md`, and `docs/examples/canon-input/change-add-caching.md` with the domain-slice authored sections
- [x] T030 [US3] Extend `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/orchestrator/service/mode_change.rs` so `change` renders domain slice, domain invariants, ownership boundaries, and cross-context risks in the existing packet
- [x] T031 [US3] Strengthen `change` contract and summary surfaces in `crates/canon-engine/src/artifacts/contract.rs`, `defaults/methods/change.toml`, and `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [x] T032 [US3] Record change-specific boundary decisions and validation evidence in `specs/017-domain-boundary-design/decision-log.md` and `specs/017-domain-boundary-design/validation-report.md`

**Checkpoint**: `change` packets express domain impact explicitly without widening the mode.

---

## Final Phase: Verification & Compliance

**Purpose**: Complete cross-cutting validation, documentation, and closeout.

- [x] T033 [P] Run structural validation: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`; record results in `specs/017-domain-boundary-design/validation-report.md`
- [x] T034 [P] Run logical validation: the targeted system-shaping, architecture, and change tests plus relevant non-regression suites; record results in `specs/017-domain-boundary-design/validation-report.md`
- [x] T035 Perform independent review of `specs/017-domain-boundary-design/spec.md`, `specs/017-domain-boundary-design/plan.md`, and `specs/017-domain-boundary-design/tasks.md`, then record findings in `specs/017-domain-boundary-design/validation-report.md`
- [x] T036 Update `docs/guides/modes.md` and `ROADMAP.md` to document the delivered domain-modeling slice and capture the documentation evidence in `specs/017-domain-boundary-design/validation-report.md`
- [x] T037 Confirm invariants still hold, confirm non-target modes remain unchanged, and close `specs/017-domain-boundary-design/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 0 must complete first.
- Phase 1 depends on Phase 0.
- Phase 2 depends on Phase 1 and blocks all story implementation.
- Phase 3 depends on Phase 2.
- Phase 4 depends on Phase 2 and should follow Phase 3 because both stories share `mode_shaping.rs` and `markdown.rs`.
- Phase 5 depends on Phase 2 and should follow Phase 4 because it shares `markdown.rs`, `contract.rs`, and docs surfaces with the earlier stories.
- Final Phase depends on all user stories being complete.

### User Story Dependencies

- **US1** can ship independently once the new system-shaping artifact, docs, and tests pass.
- **US2** depends on the shared helper and contract plumbing from Phase 2 and should follow US1 because both modify `mode_shaping.rs`.
- **US3** depends on the shared helper and contract plumbing from Phase 2 and should follow US2 because it shares `markdown.rs` and validation surfaces.

### Within Each User Story

- Validation tasks happen before the corresponding implementation tasks.
- Skills/templates/examples are updated before runtime behavior is declared complete.
- Contract and method metadata changes land before summary and packet evidence is finalized.
- Evidence capture happens before a story is marked done.

## Parallel Execution Examples

- T006, T007, and T008 can run in parallel.
- T010 and T011 can run in parallel after T009.
- T012, T013, and T014 can run in parallel.
- T019, T020, and T021 can run in parallel.
- T026, T027, and T028 can run in parallel.
- T033 and T034 can run in parallel in the final phase.

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Governance, Setup, and Foundational phases.
2. Complete User Story 1.
3. Stop and validate that `system-shaping` now emits a first-class domain-model artifact.

### Incremental Delivery

1. Deliver `system-shaping` domain modeling (US1).
2. Deliver `architecture` context mapping (US2).
3. Deliver `change` domain-slice strengthening (US3).
4. Finish with full validation, docs closeout, and independent review.

### Parallel Team Strategy

1. One stream prepares failing tests while another updates skills/templates/examples for the next story.
2. Shared runtime files (`markdown.rs`, `contract.rs`, `mode_shaping.rs`, `mode_change.rs`) remain single-owner at a time.
3. Final validation and independent review remain separate from the implementation stream.
