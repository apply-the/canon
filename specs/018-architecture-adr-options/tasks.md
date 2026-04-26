# Tasks: Architecture ADR And Options

**Input**: Design documents from `/specs/018-architecture-adr-options/`  
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

- [x] T001 Confirm execution mode `change`, risk `bounded-impact`, scope boundaries, and invariants in `specs/018-architecture-adr-options/spec.md` and `specs/018-architecture-adr-options/plan.md`
- [x] T002 Confirm the architecture decision contract in `specs/018-architecture-adr-options/contracts/architecture-decision-shape.md`
- [x] T003 Confirm decision logging and validation scaffolding in `specs/018-architecture-adr-options/decision-log.md` and `specs/018-architecture-adr-options/validation-report.md`
- [x] T004 Record the expected non-regression boundary for existing C4 artifacts and the required decision-facing section set in `specs/018-architecture-adr-options/decision-log.md`

---

## Phase 1: Setup (Shared Baseline)

**Purpose**: Verify the current architecture surfaces before feature work begins.

- [x] T005 Verify the focused architecture baseline with `cargo test --test architecture_c4_renderer --test architecture_c4_run --test architecture_run --test architecture_contract --test architecture_c4_contract` and record the result in `specs/018-architecture-adr-options/validation-report.md`
- [x] T006 [P] Confirm the current architecture guidance in `defaults/embedded-skills/canon-architecture/skill-source.md`, `.agents/skills/canon-architecture/SKILL.md`, `docs/templates/canon-input/architecture.md`, and `docs/examples/canon-input/architecture-state-management.md`
- [x] T007 [P] Confirm the current architecture packet contract in `defaults/methods/architecture.toml`, `crates/canon-engine/src/artifacts/contract.rs`, and `docs/guides/modes.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared runtime and contract primitives that all three stories depend on.

**⚠️ CRITICAL**: No user-story implementation begins until this phase is complete.

- [x] T008 Confirm `defaults/methods/architecture.toml` keeps the correct artifact family and extend `crates/canon-engine/src/artifacts/contract.rs` with the required decision-facing headings `Decision Drivers`, `Options Considered`, `Pros`, `Cons`, `Recommendation`, and `Why Not The Others` from `specs/018-architecture-adr-options/contracts/architecture-decision-shape.md`
- [x] T009 [P] Add or extend authored decision-section extraction and explicit missing-body rendering in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T010 [P] Confirm architecture summary and gating surfaces remain compatible in `crates/canon-engine/src/orchestrator/service/summarizers.rs` and `crates/canon-engine/src/orchestrator/gatekeeper.rs`

**Checkpoint**: Shared contract and rendering infrastructure exists for the strengthened architecture decision shape.

---

## Phase 3: User Story 1 - Author A Real Architecture Decision Packet (Priority: P1) 🎯 MVP

**Goal**: Make `architecture` emit a real ADR-like decision packet with explicit option analysis.

**Independent Test**: Run `architecture` with a brief containing multiple viable options and confirm the emitted decision artifacts preserve the authored decision drivers, options considered, recommendation, and rejection rationale.

### Validation for User Story 1 (MANDATORY)

- [x] T011 [P] [US1] Add failing artifact-contract coverage for authored decision sections in `tests/contract/architecture_contract.rs`
- [x] T012 [P] [US1] Add failing renderer coverage for ADR-like and option-analysis sections in `tests/architecture_c4_renderer.rs`
- [x] T013 [P] [US1] Add failing run coverage for emitted architecture decision artifacts in `tests/integration/architecture_run.rs`

### Implementation for User Story 1

- [x] T014 [US1] Update authored guidance in `defaults/embedded-skills/canon-architecture/skill-source.md`, `.agents/skills/canon-architecture/SKILL.md`, `docs/templates/canon-input/architecture.md`, and `docs/examples/canon-input/architecture-state-management.md`
- [x] T015 [US1] Extend `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/markdown.rs` so `architecture-decisions.md` and `tradeoff-matrix.md` preserve ADR-like and option-analysis sections
- [x] T016 [US1] Record story-specific decisions and evidence in `specs/018-architecture-adr-options/decision-log.md` and `specs/018-architecture-adr-options/validation-report.md`

**Checkpoint**: `architecture` emits a real ADR-style decision packet and option-analysis artifacts from authored input.

---

## Phase 4: User Story 2 - Review Tradeoffs Without Losing C4 Context (Priority: P2)

**Goal**: Preserve the already-delivered C4 packet unchanged while the strengthened decision artifacts ship alongside it.

**Independent Test**: Run `architecture` with authored C4 sections plus option-analysis sections and confirm the packet contains both the unchanged C4 artifacts and the strengthened decision artifacts.

### Validation for User Story 2 (MANDATORY)

- [x] T017 [P] [US2] Add failing non-regression coverage for C4 behavior in `tests/contract/architecture_c4_contract.rs` and `tests/architecture_c4_run.rs`
- [x] T018 [P] [US2] Add failing integration coverage proving decision artifacts and C4 artifacts coexist in `tests/integration/architecture_run.rs`

### Implementation for User Story 2

- [x] T019 [US2] Preserve existing C4 rendering behavior while integrating the stronger decision shape in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T020 [US2] Update architecture mode guidance in `docs/guides/modes.md` so the decision-facing slice and unchanged C4 slice are both explicit
- [x] T021 [US2] Record C4 non-regression evidence in `specs/018-architecture-adr-options/validation-report.md`

**Checkpoint**: Reviewers can inspect richer decision artifacts without any regression in the existing C4 packet.

---

## Phase 5: User Story 3 - Keep Missing Context Honest And Synchronized (Priority: P3)

**Goal**: Keep missing authored decision context explicit and keep skills, docs, template, example, and tests synchronized on the same contract.

**Independent Test**: Remove one required decision section from a focused architecture brief and confirm the emitted artifact shows `## Missing Authored Body` naming the missing section.

### Validation for User Story 3 (MANDATORY)

- [x] T022 [P] [US3] Add failing missing-body coverage in `tests/contract/architecture_contract.rs` and `tests/architecture_c4_renderer.rs`
- [x] T023 [P] [US3] Add failing docs-sync coverage for the strengthened architecture contract in a new `tests/architecture_decision_shape_docs.rs`

### Implementation for User Story 3

- [x] T024 [US3] Finalize explicit missing-body behavior for required decision sections in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/artifacts/contract.rs`
- [x] T025 [US3] Synchronize `defaults/embedded-skills/canon-architecture/skill-source.md`, `.agents/skills/canon-architecture/SKILL.md`, `docs/templates/canon-input/architecture.md`, `docs/examples/canon-input/architecture-state-management.md`, `docs/guides/modes.md`, `ROADMAP.md`, and confirm `AGENTS.md` remains current for this slice
- [x] T026 [US3] Record synchronization and missing-body evidence in `specs/018-architecture-adr-options/decision-log.md` and `specs/018-architecture-adr-options/validation-report.md`

**Checkpoint**: The strengthened architecture contract is honest when authored context is missing and synchronized across runtime and docs surfaces.

---

## Final Phase: Verification & Compliance

**Purpose**: Complete cross-cutting validation, documentation, and closeout.

- [x] T027 [P] Run structural validation: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`; record results in `specs/018-architecture-adr-options/validation-report.md`
- [x] T028 [P] Run logical validation: the focused architecture suites plus relevant non-regression tests; record results in `specs/018-architecture-adr-options/validation-report.md`
- [ ] T029 Perform human-owned independent review of `specs/018-architecture-adr-options/spec.md`, `specs/018-architecture-adr-options/plan.md`, and `specs/018-architecture-adr-options/tasks.md` after T028, then record findings in `specs/018-architecture-adr-options/validation-report.md`
- [ ] T030 Confirm invariants still hold, confirm non-target modes remain unchanged, and close `specs/018-architecture-adr-options/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 0 must complete first.
- Phase 1 depends on Phase 0.
- Phase 2 depends on Phase 1 and blocks all story implementation.
- Phase 3 depends on Phase 2.
- Phase 4 depends on Phase 2 and should follow Phase 3 because both stories modify `crates/canon-engine/src/artifacts/markdown.rs` and `tests/integration/architecture_run.rs`.
- Phase 5 depends on Phase 2 and should follow Phase 4 because it reuses the same runtime and docs surfaces.
- Final Phase depends on all user stories being complete.

### User Story Dependencies

- **US1** can ship independently once the strengthened decision artifacts, guidance, and tests pass.
- **US2** depends on the shared contract and renderer plumbing from Phase 2 and should follow US1 because both touch architecture rendering.
- **US3** depends on the shared contract and renderer plumbing from Phase 2 and should follow US2 because it hardens the final contract and sync surfaces.

### Within Each User Story

- Validation tasks happen before the corresponding implementation tasks.
- Guidance and authored contract updates land before the story is declared complete.
- Evidence capture happens before a story is marked done.

## Parallel Execution Examples

- T006 and T007 can run in parallel.
- T009 and T010 can run in parallel after T008.
- T011, T012, and T013 can run in parallel.
- T017 and T018 can run in parallel.
- T022 and T023 can run in parallel.
- T027 and T028 can run in parallel.

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Governance, Setup, and Foundational phases.
2. Complete User Story 1.
3. Stop and validate that `architecture` now emits a real ADR-style decision packet with explicit option analysis.

### Incremental Delivery

1. Deliver strengthened architecture decision artifacts (US1).
2. Deliver C4 coexistence and non-regression guarantees (US2).
3. Deliver missing-body hardening and docs synchronization (US3).
4. Finish with full validation, independent review, and closeout.

### Parallel Team Strategy

1. One stream prepares failing tests while another prepares doc and skill updates for the next story.
2. Shared runtime files (`crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/artifacts/contract.rs`) remain single-owner at a time.
3. Final validation and independent review remain separate from the implementation stream.