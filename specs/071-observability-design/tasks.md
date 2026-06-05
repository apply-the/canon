# Tasks: Observability Design Mode

**Input**: Design documents from `/specs/071-observability-design/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/cli.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Phase 0: Governance & Artifacts

**Purpose**: Establish governance anchors before code.

- [x] T001 Initialize verification context with `observability-design` mode, Green risk profile, and scope invariants from `plan.md`.
- [x] T002 Ensure `validation-report.md` exists and is tracked for final independent review.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T003 Create project structure per implementation plan: `crates/canon-engine/src/observability/mod.rs`
- [x] T004 Create CLI structure: `crates/canon-cli/src/commands/observability_design.rs`
- [x] T005 [P] Create test modules: `crates/canon-engine/tests/unit/test_observability_design.rs` and `crates/canon-cli/tests/integration/test_observability_design.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 [P] Implement `TelemetryPlan` and `BoundarySignalMap` typed schemas in `crates/canon-engine/src/observability/mod.rs` per `data-model.md`
- [x] T007 [P] Implement `Signal`, `SloAlert`, and `RunbookStub` typed schemas in `crates/canon-engine/src/observability/mod.rs` per `data-model.md`
- [x] T008 Implement the base `canon observability-design` command interface in `crates/canon-cli/src/commands/observability_design.rs` mapping to `contracts/cli.md`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Proactive Telemetry Mapping (Priority: P1) 🎯 MVP

**Goal**: Operators need to define what needs tracing and at what level before code deployment.

**Independent Test**: Can be fully tested by providing an `architecture.md` document and validating that a `telemetry-plan.md` and `05-instrumentation-checklist.md` with appropriate tracing boundaries are generated.

### Implementation for User Story 1

- [x] T009 [P] [US1] Create unit tests for LLM boundary inference in `crates/canon-engine/tests/unit/test_observability_design.rs`
- [x] T010 [US1] Implement `evaluator.rs` containing the reasoning-heavy LLM inference logic in `crates/canon-engine/src/observability/evaluator.rs`
- [x] T011 [P] [US1] Implement `telemetry-plan.md` and checklist generators in `crates/canon-engine/src/observability/generators.rs`
- [x] T012 [US1] Integrate `evaluator` and `generators` to output plans in `crates/canon-cli/src/commands/observability_design.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently (MVP)

---

## Phase 4: User Story 2 - SLI/SLO and Alert Thresholds (Priority: P1)

**Goal**: Operators need concrete thresholds for system SLIs and first-responder playbooks.

**Independent Test**: Can be fully tested by checking if the generated `03-slo-alerts.md` and `04-runbook.md` include thresholds and generic If-This-Then-That action items for operators based on the inferred boundaries.

### Implementation for User Story 2

- [x] T013 [P] [US2] Create unit tests for SLO and Runbook stub generation in `crates/canon-engine/tests/unit/test_observability_design.rs`
- [x] T014 [US2] Implement SLI/SLO threshold generator in `crates/canon-engine/src/observability/generators.rs`
- [x] T015 [US2] Implement generic Markdown Runbook stub generator in `crates/canon-engine/src/observability/generators.rs`
- [x] T016 [US2] Wire SLO and Runbook artifact output logic in `crates/canon-cli/src/commands/observability_design.rs`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T017 [P] Implement interactive disambiguation logic (edge cases) for vague inputs and accompanying tests in `crates/canon-cli/src/commands/observability_design.rs`
- [x] T018 Verify output paths are correctly localized to the feature directory (not root)
- [x] T019 Record final independent validation results into `validation-report.md`
- [x] T020 Run the `quickstart.md` execution flow manually on a sample fixture to close out independent testing

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Integrates with US1 boundaries, but implementation logic can run concurrently.

### Parallel Opportunities

- Foundation models (T006, T007) can be implemented in parallel.
- Unit tests for US1 and US2 can be implemented in parallel.
- `evaluator.rs` LLM inference and `generators.rs` formatting logic can be parallelized.

---

## Parallel Example: User Story 1

```bash
# Launch test and generator stubs concurrently:
Task: "T009 [P] [US1] Create unit tests for LLM boundary inference"
Task: "T011 [P] [US1] Implement telemetry-plan.md and checklist generators"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently by passing an architecture document and checking if boundaries are inferred and `telemetry-plan.md` generated.

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → MVP!
3. Add User Story 2 → Generate SLOs and Runbooks → Test independently
4. Add Polish phase → Interactive disambiguation → Validate failure edge cases
