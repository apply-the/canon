# Tasks: Mode Clarification And Run Refinement

**Input**: Design documents from `specs/062-clarify-run-refinement/`
**Prerequisites**: plan.md (required), spec.md (required for user stories),
research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks
whenever behavior, interfaces, or regressions must be checked. Independent
review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent
implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with mode, risk, scope, and invariant artifact tasks.
- No implementation task may appear before the artifacts that authorize it.
- Every user story MUST include validation tasks and evidence capture.
- Systemic-impact work MUST include an independent review task separate from
  generation.

## Path Conventions

- **Engine runtime**: `crates/canon-engine/src/`
- **CLI surfaces**: `crates/canon-cli/src/`
- **Contracts and evidence**: `specs/062-clarify-run-refinement/`
- **Templates and methods**: `docs/templates/canon-input/`, `defaults/methods/`
- **Skill guidance**: `.agents/skills/`, `defaults/embedded-skills/`
- **Fixtures and integration checks**: `tests/fixtures/`, `tests/contract/`, `tests/integration/`
- **Wiki closeout**: `../canon.wiki/`

---

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that permit implementation to start

- [X] T001 Bump Canon version to `0.62.0` in `Cargo.toml` and `assistant/plugin-metadata.json`
- [X] T002 Refresh implementation-stage decisions in `specs/062-clarify-run-refinement/decision-log.md` for draft identity reuse, advisory continuation, successor lineage, and working-brief rendering boundaries
- [X] T003 Extend `specs/062-clarify-run-refinement/validation-report.md` with the touched-file coverage target above 95%, the planned `cargo llvm-cov` evidence flow, the recorded under-2-minute reviewer walkthrough evidence format, and the independent reviewer checkpoint for systemic-impact persistence changes plus publish-destination or artifact-family or honesty-marker regression review
- [X] T004 Record the release-closeout scope for runtime, docs, and wiki updates in `specs/062-clarify-run-refinement/plan.md` and `specs/062-clarify-run-refinement/quickstart.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare reusable fixtures and harnesses before implementation

- [X] T005 [P] Create targeted-mode authored-input fixtures in `tests/fixtures/clarify-run-refinement/requirements/brief.md`, `tests/fixtures/clarify-run-refinement/discovery/brief.md`, `tests/fixtures/clarify-run-refinement/system-shaping/brief.md`, `tests/fixtures/clarify-run-refinement/architecture/brief.md`, and `tests/fixtures/clarify-run-refinement/change/brief.md`
- [X] T006 [P] Create ambiguity and representative non-targeted fixtures in `tests/fixtures/clarify-run-refinement/review.md`, `tests/fixtures/clarify-run-refinement/verification.md`, `tests/fixtures/clarify-run-refinement/implementation.md`, `tests/fixtures/clarify-run-refinement/refactor.md`, `tests/fixtures/clarify-run-refinement/incident.md`, `tests/fixtures/clarify-run-refinement/migration.md`, and `tests/fixtures/clarify-run-refinement/multi-candidate/`
- [X] T007 [P] Create integration and contract test scaffolds in `tests/integration/refinement_flow.rs` and `tests/contract/refinement_contracts.rs`
- [X] T008 [P] Add reusable sample refinement builders for renderer and service tests in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/orchestrator/service/tests.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core runtime shapes that MUST be complete before ANY user story can
be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T009 Add typed refinement entities and serde-backed enums to `crates/canon-engine/src/domain/run.rs` for `ClarificationRefinementContext`, `ClarificationRecord`, `ReadinessDeltaItem`, `ContinuationCandidateSummary`, and successor-lineage support
- [X] T010 Extend persisted manifest lineage and compatibility handling in `crates/canon-engine/src/persistence/manifests.rs`
- [X] T011 Persist and reload refinement state through `crates/canon-engine/src/persistence/store.rs` and `crates/canon-engine/src/persistence/store/runtime.rs`
- [X] T012 Add shared refinement summary types and readiness-delta derivation hooks in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/service/input_handling.rs`
- [X] T013 Add the `inspect refinement` command skeleton and target wiring in `crates/canon-cli/src/app.rs` and `crates/canon-cli/src/commands/inspect.rs`

**Checkpoint**: Typed runtime persistence, manifest lineage, and CLI command wiring are ready for story-level behavior

---

## Phase 3: User Story 1 - Refine The Same Work Item (Priority: P1) 🎯 MVP

**Goal**: Preserve the same run identity through clarification, enforce explicit continuation intent, and prevent silent attachment of new work to an older run

**Independent Test**: Given a targeted-mode packet with readiness gaps, an operator can continue the same run identity through clarification and resume, while a fresh request without explicit continuation intent starts a new run instead of mutating the existing candidate

### Validation for User Story 1 (MANDATORY)

- [X] T014 [P] [US1] Add failing lifecycle tests for draft reuse, explicit continuation gating, and pre-start versus post-start mode changes in `crates/canon-engine/src/orchestrator/service/tests.rs`
- [X] T015 [P] [US1] Add failing CLI summary tests for advisory candidate surfacing and explicit continuation behavior in `crates/canon-cli/src/commands/status.rs` and `crates/canon-cli/src/commands/resume.rs`
- [X] T016 [US1] Record US1 lifecycle decisions and invariants in `specs/062-clarify-run-refinement/decision-log.md`

### Implementation for User Story 1

- [X] T017 [US1] Implement draft creation, candidate matching, and explicit continuation gating in `crates/canon-engine/src/orchestrator/service/identity.rs`
- [X] T018 [US1] Implement in-place draft mode switching and successor creation for started runs in `crates/canon-engine/src/orchestrator/service/run_lifecycle.rs` and `crates/canon-engine/src/persistence/manifests.rs`
- [X] T019 [US1] Expose refinement summary and suggested continuation state in `crates/canon-engine/src/orchestrator/service/run_summary.rs` and `crates/canon-cli/src/commands/status.rs`
- [X] T020 [US1] Preserve the explicit `resume --run <RUN_ID>` continuation path while enforcing the new semantics in `crates/canon-cli/src/commands/resume.rs` and `crates/canon-cli/src/app.rs`
- [X] T021 [US1] Capture US1 validation evidence and review notes in `specs/062-clarify-run-refinement/validation-report.md`

**Checkpoint**: User Story 1 preserves same-work identity, blocks silent mutation, and is independently testable through `run`, `resume`, and `status`

---

## Phase 4: User Story 2 - Update A Working Brief, Not The Source Inputs (Priority: P2)

**Goal**: Materialize a run-local authoritative working brief, keep `canon-input/` immutable, and expose the full refinement state through a dedicated inspect surface

**Independent Test**: Given file-backed or inline targeted input, Canon materializes `.canon/runs/<RUN_ID>/artifacts/<mode>/working-brief.md`, keeps the authored inputs unchanged, and returns the same refinement details through `canon inspect refinement --run <RUN_ID>`

### Validation for User Story 2 (MANDATORY)

- [X] T022 [P] [US2] Add failing contract, renderer, and decision-changing question-selection tests for `specs/062-clarify-run-refinement/contracts/runtime-refinement-state-contract.md` and `specs/062-clarify-run-refinement/contracts/working-brief-artifact-contract.md` in `tests/contract/refinement_contracts.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, and `crates/canon-engine/src/orchestrator/service/tests.rs`
- [X] T023 [P] [US2] Add failing inspect output tests for `canon inspect refinement --run <RUN_ID>` in `crates/canon-cli/src/commands/inspect.rs` and `crates/canon-cli/src/output/inspect.rs`
- [X] T024 [US2] Record US2 contract decisions and source-immutability evidence requirements in `specs/062-clarify-run-refinement/decision-log.md`

### Implementation for User Story 2

- [X] T025 [US2] Implement working-brief artifact rendering and the standard refinement appendix in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/artifacts/markdown/`
- [X] T026 [US2] Implement authoritative-input selection, decision-changing question selection, clarification-record persistence, and structured readiness-delta derivation in `crates/canon-engine/src/orchestrator/service/input_handling.rs` and `crates/canon-engine/src/orchestrator/service/clarity.rs`
- [X] T027 [US2] Implement the run-scoped refinement inspect surface in `crates/canon-engine/src/orchestrator/service/inspect.rs`, `crates/canon-cli/src/app.rs`, and `crates/canon-cli/src/commands/inspect.rs`
- [X] T028 [US2] Render refinement-state text, markdown, JSON, and YAML output in `crates/canon-cli/src/output/inspect.rs` and `crates/canon-cli/src/commands/status.rs`
- [X] T029 [US2] Capture US2 validation evidence, quickstart walkthrough notes, and recorded under-2-minute reviewer walkthrough results for `status` and `inspect refinement` in `specs/062-clarify-run-refinement/validation-report.md`

**Checkpoint**: User Story 2 produces a run-local working brief, keeps authored inputs immutable, and exposes inspectable refinement state end to end

---

## Phase 5: User Story 3 - Keep Refinement Behavior Consistent Across Modes (Priority: P3)

**Goal**: Align runtime behavior, methods, templates, and skill guidance so targeted modes get the full working-brief lifecycle and all modes get explicit continuation continuity without silent new-run fallback

**Independent Test**: A maintainer can compare the targeted-mode methods, templates, skill-source guidance, and runtime outputs and verify that they all describe the same refinement lifecycle, while non-targeted modes still preserve identity continuity and explicit disambiguation

### Validation for User Story 3 (MANDATORY)

- [X] T030 [P] [US3] Add failing cross-mode tests for representative non-targeted continuity across `review`, `verification`, `implementation`, `refactor`, `incident`, and `migration`, plus successor lineage visibility and targeted-mode lifecycle alignment in `tests/integration/refinement_flow.rs` and `crates/canon-engine/src/orchestrator/service/tests.rs`
- [X] T031 [P] [US3] Add failing contract assertions for `specs/062-clarify-run-refinement/contracts/status-and-inspect-refinement-contract.md` and targeted template headings in `tests/contract/refinement_contracts.rs`
- [X] T032 [US3] Record cross-mode consistency decisions and review checkpoints in `specs/062-clarify-run-refinement/decision-log.md`

### Implementation for User Story 3

- [X] T033 [P] [US3] Update targeted method definitions in `defaults/methods/requirements.toml`, `defaults/methods/discovery.toml`, `defaults/methods/system-shaping.toml`, `defaults/methods/architecture.toml`, and `defaults/methods/change.toml`
- [X] T034 [P] [US3] Update targeted brief templates in `docs/templates/canon-input/requirements.md`, `docs/templates/canon-input/discovery.md`, `docs/templates/canon-input/system-shaping.md`, `docs/templates/canon-input/architecture.md`, and `docs/templates/canon-input/change.md`
- [X] T035 [P] [US3] Update targeted embedded skill guidance in `defaults/embedded-skills/canon-requirements/skill-source.md`, `defaults/embedded-skills/canon-discovery/skill-source.md`, `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `defaults/embedded-skills/canon-architecture/skill-source.md`, and `defaults/embedded-skills/canon-change/skill-source.md`
- [X] T036 [US3] Update non-targeted continuity guidance in `defaults/embedded-skills/canon-review/skill-source.md`, `defaults/embedded-skills/canon-verification/skill-source.md`, `defaults/embedded-skills/canon-implementation/skill-source.md`, `defaults/embedded-skills/canon-refactor/skill-source.md`, `defaults/embedded-skills/canon-incident/skill-source.md`, `defaults/embedded-skills/canon-migration/skill-source.md`, `defaults/embedded-skills/canon-resume/skill-source.md`, and `defaults/embedded-skills/canon-status/skill-source.md`
- [X] T037 [US3] Align repo-local skill fronts in `.agents/skills/canon-requirements/SKILL.md`, `.agents/skills/canon-discovery/SKILL.md`, `.agents/skills/canon-system-shaping/SKILL.md`, `.agents/skills/canon-architecture/SKILL.md`, `.agents/skills/canon-change/SKILL.md`, `.agents/skills/canon-review/SKILL.md`, `.agents/skills/canon-verification/SKILL.md`, `.agents/skills/canon-implementation/SKILL.md`, `.agents/skills/canon-refactor/SKILL.md`, `.agents/skills/canon-incident/SKILL.md`, `.agents/skills/canon-migration/SKILL.md`, `.agents/skills/canon-resume/SKILL.md`, and `.agents/skills/canon-status/SKILL.md`
- [X] T038 [US3] Capture US3 evidence, including the representative non-targeted mode coverage matrix and independent review findings, in `specs/062-clarify-run-refinement/validation-report.md`

**Checkpoint**: All targeted modes share one documented refinement lifecycle, and non-targeted modes preserve explicit continuity without claiming unsupported working-brief behavior

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, regression repair, coverage closure, and release-facing documentation

- [X] T039 [P] Run targeted formatting and the no-panic clippy gate for touched Rust sources with `cargo fmt --check` and `cargo clippy --workspace --lib --bins --all-features -- -D clippy::unwrap_used -D clippy::expect_used -D clippy::panic -D clippy::todo -D clippy::unimplemented -D clippy::unreachable`, then record results in `specs/062-clarify-run-refinement/validation-report.md`
- [X] T040 [P] Run focused and workspace test suites, fix regressions in `crates/canon-engine/src/**`, `crates/canon-cli/src/**`, `tests/contract/refinement_contracts.rs`, and `tests/integration/refinement_flow.rs`, and record repaired failures in `specs/062-clarify-run-refinement/validation-report.md`
- [X] T041 [P] Generate changed-file coverage evidence above 95% using `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` plus package-scoped overlays for `canon-cli` or `canon-engine`, add tests until touched files clear the threshold, and record the result in `lcov.info` and `specs/062-clarify-run-refinement/validation-report.md`
- [ ] T042 Perform the systemic-impact independent review of persistence, lineage, explicit-intent semantics, approval-gate preservation, recorded under-2-minute operator walkthrough results, and publish-destination or artifact-family or honesty-marker preservation in `specs/062-clarify-run-refinement/validation-report.md`
- [ ] T043 Update `CHANGELOG.md`, `README.md`, `docs/guides/modes.md`, `docs/templates/canon-input/README.md`, `../canon.wiki/Home.md`, `../canon.wiki/Canon-Modes.md`, and `../canon.wiki/Lineage-And-Provenance.md` with the `0.62.0` refinement lifecycle release notes and operator guidance

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational completion. MVP slice.
- **User Story 2 (Phase 4)**: Depends on User Story 1 because it builds on persisted refinement state and continuation semantics.
- **User Story 3 (Phase 5)**: Depends on User Story 2 so docs and guidance align with the final runtime and inspect contracts.
- **Verification & Compliance (Final Phase)**: Depends on all user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Establishes draft reuse, explicit continuation, and successor lineage semantics for the feature.
- **User Story 2 (P2)**: Depends on US1 runtime continuity so the working brief and inspect surfaces attach to the same persisted refinement state.
- **User Story 3 (P3)**: Depends on US2 because methods, templates, and skill guidance must mirror the implemented runtime and inspect behavior.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision and invariant updates MUST be recorded before the affected slice is declared complete.
- Typed runtime models and persistence changes come before CLI rendering and docs alignment.
- Evidence capture MUST land before the story is declared complete.

---

## Parallel Opportunities

- **Phase 1**: T005-T008 are parallel once Phase 0 is complete.
- **Phase 2**: T010-T013 can be split across persistence, service, and CLI wiring after T009 establishes the shared domain types.
- **US1**: T014 and T015 are parallel failing-check tasks; T019 and T020 can proceed in parallel after T017-T018 land.
- **US2**: T022 and T023 are parallel failing-check tasks; T027 and T028 can proceed in parallel after T025-T026 land.
- **US3**: T033-T035 are parallel updates across methods, templates, and embedded skill guidance; T036 and T037 can proceed after the runtime-facing guidance language is fixed.
- **Final Phase**: T039-T041 are parallelizable validation streams once implementation is stable.

---

## Parallel Example: User Story 1

```bash
# Launch story-level failing checks in parallel:
Task: "Add failing lifecycle tests in crates/canon-engine/src/orchestrator/service/tests.rs"
Task: "Add failing CLI summary tests in crates/canon-cli/src/commands/status.rs and crates/canon-cli/src/commands/resume.rs"

# After runtime semantics land, split the operator-facing work:
Task: "Expose refinement summary in crates/canon-engine/src/orchestrator/service/run_summary.rs and crates/canon-cli/src/commands/status.rs"
Task: "Preserve explicit resume semantics in crates/canon-cli/src/commands/resume.rs and crates/canon-cli/src/app.rs"
```

## Parallel Example: User Story 2

```bash
# Run contract and output checks in parallel:
Task: "Add failing contract and renderer tests in tests/contract/refinement_contracts.rs and crates/canon-engine/src/artifacts/markdown.rs"
Task: "Add failing inspect output tests in crates/canon-cli/src/commands/inspect.rs and crates/canon-cli/src/output/inspect.rs"

# After shared refinement data lands, split inspect and renderer work:
Task: "Implement working-brief rendering in crates/canon-engine/src/artifacts/markdown.rs and crates/canon-engine/src/artifacts/markdown/"
Task: "Implement inspect refinement output in crates/canon-engine/src/orchestrator/service/inspect.rs and crates/canon-cli/src/output/inspect.rs"
```

## Parallel Example: User Story 3

```bash
# Update lifecycle guidance in parallel across distinct surfaces:
Task: "Update targeted methods in defaults/methods/*.toml"
Task: "Update targeted templates in docs/templates/canon-input/*.md"
Task: "Update targeted embedded skill guidance in defaults/embedded-skills/canon-*/skill-source.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: confirm same-run continuation, explicit-intent gating,
   and successor lineage behavior before widening scope.

### Incremental Delivery

1. Deliver US1 to establish durable continuation semantics.
2. Add US2 to materialize and inspect working-brief refinement state.
3. Add US3 to align methods, templates, and skill guidance with the implemented lifecycle.
4. Finish with the verification, coverage, changelog, docs, and wiki closeout phase.

### Suggested MVP Scope

- **MVP**: Phase 0 through Phase 3 (User Story 1).
- **Second increment**: Phase 4 (User Story 2).
- **Third increment**: Phase 5 plus the final compliance closeout.