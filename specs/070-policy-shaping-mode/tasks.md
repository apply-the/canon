# Tasks: Policy Shaping Mode

**Input**: Design documents from `/specs/070-policy-shaping-mode/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/cli.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Phase 0: Governance & Artifacts

**Purpose**: Formal execution declarations and baseline artifacts

- [X] T001 Ensure `policy-shaping` mode is registered in execution workflow
- [X] T002 Ensure `Systemic Impact` risk level is explicitly logged before proceeding

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T003 Create directory `crates/canon-engine/src/policy/`
- [X] T004 Create directory `crates/canon-cli/src/commands/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Setup typed serde data models for `DraftPolicy` and `ImpactReport` in `crates/canon-engine/src/policy/models.rs`
- [X] T006 [P] Setup typed serde data models for `MigrationPlan` and `PolicyDiff` in `crates/canon-engine/src/policy/models.rs`
- [X] T007 Configure LLM skills execution boundary in `canon-engine` to allow invoking `.agents/skills`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Proposing a new constitution rule (Priority: P1) 🎯 MVP

**Goal**: Establish safe, enforceable rules while quantifying their impact across the codebase.

**Independent Test**: Can be fully tested by creating a `draft-policy.md`, executing the `policy-shaping` mode, and observing the generated impact report.

### Tests for User Story 1

- [X] T008 [P] [US1] Integration test for CLI dry-run and full evaluation in `crates/canon-cli/tests/integration/test_policy_shaping.rs`
- [X] T009 [P] [US1] Unit tests for report pagination and module grouping in `crates/canon-engine/tests/unit/test_policy_report.rs`

### Implementation for User Story 1

- [X] T010 [P] [US1] Implement `evaluator.rs` in `crates/canon-engine/src/policy/evaluator.rs` to orchestrate `.agents/skills`
- [X] T011 [P] [US1] Implement `report.rs` in `crates/canon-engine/src/policy/report.rs` to group violations by directory
- [X] T012 [P] [US1] Implement migration generation in `crates/canon-engine/src/policy/migration.rs` to output `04-migration.md`
- [X] T013 [P] [US1] Implement diff generation in `crates/canon-engine/src/policy/diff.rs` to output `policy-diff.md`
- [X] T014 [P] [US1] Author the `canon-policy-shaping` skill prompt in `.agents/skills/canon-policy-shaping/SKILL.md`
- [X] T015 [US1] Implement the Canon CLI command parser for `policy-shaping` in `crates/canon-cli/src/commands/policy_shaping.rs`
- [X] T016 [US1] Integrate `evaluator`, `report`, `migration`, and `diff` modules into the CLI handler in `crates/canon-cli/src/commands/policy_shaping.rs`
- [X] T017 [US1] Enforce explicit `Systemic Impact` broad-impact approval gate in `crates/canon-cli/src/commands/policy_shaping.rs`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T018 [P] Run quickstart.md validation to ensure documentation aligns with CLI behavior
- [X] T019 Check all new `canon-engine/src/policy/` structs and enums to ensure they use `serde` derives
- [X] T020 Audit all new methods for panic-prone calls (`unwrap`, `expect`) and replace with `Result` types

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Integration test for CLI dry-run and full evaluation"
Task: "Unit tests for report pagination and module grouping"

# Launch internal engine implementations together:
Task: "Implement evaluator.rs"
Task: "Implement report.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready
