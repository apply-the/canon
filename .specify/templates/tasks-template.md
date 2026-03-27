---
description: "Task list template for feature implementation"
---

# Tasks: [FEATURE NAME]

**Input**: Design documents from `/specs/[###-feature-name]/`
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
- High-risk and critical work MUST include an independent review task separate
  from generation.

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/`
- Paths shown below assume single project. Adjust based on `plan.md` structure.

<!--
  ============================================================================
  IMPORTANT: The tasks below are SAMPLE TASKS for illustration purposes only.

  The /speckit.tasks command MUST replace these with actual tasks based on:
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - Governance context and Constitution Check from plan.md
  - Feature requirements from plan.md
  - Entities from data-model.md
  - Endpoints from contracts/

  Tasks MUST be organized by user story so each story can be:
  - Implemented independently
  - Validated independently
  - Delivered as an MVP increment

  DO NOT keep these sample tasks in the generated tasks.md file.
  ============================================================================
-->

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that permit implementation to start

- [ ] T001 Record execution mode, risk classification, scope boundaries, and invariants in `specs/[###-feature-name]/spec.md` and `specs/[###-feature-name]/plan.md`
- [ ] T002 Create or update the decision log in `specs/[###-feature-name]/decision-log.md`
- [ ] T003 Create the validation report scaffold in `specs/[###-feature-name]/validation-report.md`
- [ ] T004 Capture required reviewers and approval gates for the declared risk level

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T005 Create project structure per implementation plan
- [ ] T006 Initialize [language] project with [framework] dependencies
- [ ] T007 [P] Configure linting and formatting tools

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can
be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

Examples of foundational tasks (adjust based on your project):

- [ ] T008 Setup database schema and migrations framework
- [ ] T009 [P] Implement authentication and authorization framework
- [ ] T010 [P] Setup API routing and middleware structure
- [ ] T011 Create base models or entities that all stories depend on
- [ ] T012 Configure error handling and logging infrastructure
- [ ] T013 Setup environment configuration management
- [ ] T014 Encode shared invariant guards or policy checks in the relevant runtime layer

**Checkpoint**: Foundation ready and governance artifacts remain current

---

## Phase 3: User Story 1 - [Title] (Priority: P1) 🎯 MVP

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Validation for User Story 1 (MANDATORY)

- [ ] T015 [P] [US1] Write failing verification for [behavior] in `tests/[location]/test_[name].py` or `contracts/[name]`
- [ ] T016 [US1] Record story-specific decisions or invariant updates in `specs/[###-feature-name]/decision-log.md`

### Implementation for User Story 1

- [ ] T017 [P] [US1] Create [Entity1] model in `src/models/[entity1].py`
- [ ] T018 [P] [US1] Create [Entity2] model in `src/models/[entity2].py`
- [ ] T019 [US1] Implement [Service] in `src/services/[service].py` (depends on T017, T018)
- [ ] T020 [US1] Implement [endpoint/feature] in `src/[location]/[file].py`
- [ ] T021 [US1] Add validation and error handling
- [ ] T022 [US1] Capture validation evidence and reviewer notes in `specs/[###-feature-name]/validation-report.md`

**Checkpoint**: User Story 1 is fully functional and independently validated

---

## Phase 4: User Story 2 - [Title] (Priority: P2)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Validation for User Story 2 (MANDATORY)

- [ ] T023 [P] [US2] Write failing verification for [behavior] in `tests/[location]/test_[name].py` or `contracts/[name]`
- [ ] T024 [US2] Record story-specific decisions or invariant updates in `specs/[###-feature-name]/decision-log.md`

### Implementation for User Story 2

- [ ] T025 [P] [US2] Create [Entity] model in `src/models/[entity].py`
- [ ] T026 [US2] Implement [Service] in `src/services/[service].py`
- [ ] T027 [US2] Implement [endpoint/feature] in `src/[location]/[file].py`
- [ ] T028 [US2] Integrate with User Story 1 components if needed
- [ ] T029 [US2] Capture validation evidence and reviewer notes in `specs/[###-feature-name]/validation-report.md`

**Checkpoint**: User Stories 1 and 2 both work independently

---

## Phase 5: User Story 3 - [Title] (Priority: P3)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: [How to verify this story works on its own]

### Validation for User Story 3 (MANDATORY)

- [ ] T030 [P] [US3] Write failing verification for [behavior] in `tests/[location]/test_[name].py` or `contracts/[name]`
- [ ] T031 [US3] Record story-specific decisions or invariant updates in `specs/[###-feature-name]/decision-log.md`

### Implementation for User Story 3

- [ ] T032 [P] [US3] Create [Entity] model in `src/models/[entity].py`
- [ ] T033 [US3] Implement [Service] in `src/services/[service].py`
- [ ] T034 [US3] Implement [endpoint/feature] in `src/[location]/[file].py`
- [ ] T035 [US3] Capture validation evidence and reviewer notes in `specs/[###-feature-name]/validation-report.md`

**Checkpoint**: All user stories are independently functional

---

[Add more user story phases as needed, following the same pattern]

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, documentation, and closeout

- [ ] TXXX [P] Run structural validation and record results in `specs/[###-feature-name]/validation-report.md`
- [ ] TXXX [P] Run logical and integration validation and record results in `specs/[###-feature-name]/validation-report.md`
- [ ] TXXX Perform independent review or adversarial validation and record findings in `specs/[###-feature-name]/validation-report.md`
- [ ] TXXX Update README, quickstart, or operational guidance artifacts
- [ ] TXXX Confirm invariants still hold and close the validation report

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user
  stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user
  stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. No dependencies on other
  stories.
- **User Story 2 (P2)**: Can start after Foundational. May integrate with US1
  but should remain independently testable.
- **User Story 3 (P3)**: Can start after Foundational. May integrate with
  earlier stories but should remain independently testable.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation
- Decision or invariant changes MUST be recorded before affected code lands
- Models before services
- Services before endpoints
- Evidence capture before the story is declared complete

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- Foundational tasks marked [P] can run in parallel after Phase 1
- Once Foundational completes, separate user stories can run in parallel if
  staffing allows
- Validation tasks for a story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch the failing verification and contract preparation in parallel:
Task: "Write failing verification for [behavior] in tests/[location]/test_[name].py"
Task: "Update any story-specific contract in contracts/[name]"

# Launch independent model work in parallel:
Task: "Create [Entity1] model in src/models/[entity1].py"
Task: "Create [Entity2] model in src/models/[entity2].py"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts
2. Complete Phase 1: Setup
3. Complete Phase 2: Foundational
4. Complete Phase 3: User Story 1
5. **STOP and VALIDATE**: Confirm User Story 1 independently and update
   `validation-report.md`

### Incremental Delivery

1. Complete Governance + Setup + Foundational
2. Add User Story 1 → Validate independently → Demo or ship
3. Add User Story 2 → Validate independently → Demo or ship
4. Add User Story 3 → Validate independently → Demo or ship
5. Finish with the Verification & Compliance phase

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Each story closes only after its evidence is recorded

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- Each user story should be independently completable and validated
- Verify checks fail before implementation when executable verification applies
- Keep the decision log and validation report current as work progresses
- Avoid vague tasks, hidden scope expansion, and cross-story dependencies that
  break independence
