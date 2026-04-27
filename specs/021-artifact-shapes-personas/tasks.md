# Tasks: Industry-Standard Artifact Shapes With Personas

**Input**: Design documents from `/specs/021-artifact-shapes-personas/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks
whenever behavior, interfaces, or regressions must be checked. Independent
review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story so each increment can be
implemented, validated, and audited independently.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that permit implementation to start.

- [x] T001 Record execution mode, risk classification, scope boundaries, and invariants in `specs/021-artifact-shapes-personas/spec.md` and `specs/021-artifact-shapes-personas/plan.md`
- [x] T002 Update first-slice decisions and deferrals in `specs/021-artifact-shapes-personas/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/021-artifact-shapes-personas/validation-report.md`
- [x] T004 Confirm persona and shape boundary contracts in `specs/021-artifact-shapes-personas/contracts/first-slice-artifact-shapes.md` and `specs/021-artifact-shapes-personas/contracts/persona-boundaries.md`

---

## Phase 1: Setup

**Purpose**: Prepare shared implementation context before code changes.

- [x] T005 Update agent context from `specs/021-artifact-shapes-personas/plan.md` into `AGENTS.md`
- [x] T006 Prepare implementation and evidence flow notes in `specs/021-artifact-shapes-personas/quickstart.md`

---

## Phase 2: Foundational

**Purpose**: Shared prerequisites that all user stories depend on.

**⚠️ CRITICAL**: No user story work starts until this phase is complete.

- [x] T007 [P] Review first-slice artifact requirement coverage in `tests/requirements_authoring_renderer.rs` and `tests/architecture_c4_renderer.rs`
- [x] T008 [P] Review change and missing-body coverage in `tests/change_authoring_renderer.rs` and `tests/architecture_c4_run.rs`
- [x] T009 Add regression guard notes for non-targeted modes in `specs/021-artifact-shapes-personas/validation-report.md`

**Checkpoint**: Existing runtime coverage is confirmed and evidence scaffolding is ready.

---

## Phase 3: User Story 1 - Ship Shaped Authoring For High-Leverage Modes (Priority: P1) 🎯 MVP

**Goal**: Deliver persona-aware industry-standard shaping for `requirements`, `architecture`, and `change`.

**Independent Test**: Representative authored briefs for the three targeted modes produce packets that read as PRD, C4 plus ADR, and ADR-style change artifacts without losing Canon contract fidelity.

### Validation for User Story 1 (MANDATORY)

- [x] T010 [P] [US1] Reconfirm requirements PRD-shape preservation in `tests/requirements_authoring_renderer.rs`
- [x] T011 [P] [US1] Reconfirm architecture and change shaped packet preservation in `tests/architecture_c4_renderer.rs` and `tests/change_authoring_renderer.rs`
- [x] T012 [US1] Record story-specific shape and persona decisions in `specs/021-artifact-shapes-personas/decision-log.md`

### Implementation for User Story 1

- [x] T013 [P] [US1] Update requirements shape and persona guidance in `defaults/embedded-skills/canon-requirements/skill-source.md` and `.agents/skills/canon-requirements/SKILL.md`
- [x] T014 [P] [US1] Update architecture shape and persona guidance in `defaults/embedded-skills/canon-architecture/skill-source.md` and `.agents/skills/canon-architecture/SKILL.md`
- [x] T015 [P] [US1] Update change shape and persona guidance in `defaults/embedded-skills/canon-change/skill-source.md` and `.agents/skills/canon-change/SKILL.md`
- [x] T016 [US1] Record the no-runtime-change decision for first-slice shape preservation in `specs/021-artifact-shapes-personas/decision-log.md`
- [x] T017 [US1] Record first-slice shape-fit evidence for `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/artifacts/contract.rs` in `specs/021-artifact-shapes-personas/validation-report.md`
- [x] T018 [US1] Capture US1 validation evidence in `specs/021-artifact-shapes-personas/validation-report.md`

**Checkpoint**: `requirements`, `architecture`, and `change` each emit the intended shaped packet with the intended bounded persona.

---

## Phase 4: User Story 2 - Keep Personas Bounded By Canon Governance (Priority: P2)

**Goal**: Ensure persona guidance never hides missing authored content or weakens Canon honesty semantics.

**Independent Test**: Negative-path validation still surfaces explicit gap markers and unchanged evidence posture when persona-aware guidance is active.

### Validation for User Story 2 (MANDATORY)

- [x] T019 [P] [US2] Reconfirm missing-authored-body coverage in `tests/requirements_authoring_renderer.rs`, `tests/architecture_c4_renderer.rs`, and `tests/change_authoring_renderer.rs`
- [x] T020 [US2] Record persona-boundary decisions in `specs/021-artifact-shapes-personas/decision-log.md`

### Implementation for User Story 2

- [x] T021 [US2] Tighten persona-boundary wording in `defaults/embedded-skills/canon-requirements/skill-source.md`, `defaults/embedded-skills/canon-architecture/skill-source.md`, and `defaults/embedded-skills/canon-change/skill-source.md`
- [x] T022 [US2] Mirror persona-boundary wording in `.agents/skills/canon-requirements/SKILL.md`, `.agents/skills/canon-architecture/SKILL.md`, and `.agents/skills/canon-change/SKILL.md`
- [x] T023 [US2] Record preserved negative-path honesty behavior for `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/artifacts/contract.rs` in `specs/021-artifact-shapes-personas/validation-report.md`
- [x] T024 [US2] Capture US2 validation evidence in `specs/021-artifact-shapes-personas/validation-report.md`

**Checkpoint**: Persona-aware packets remain bounded by the same missing-gap, evidence, and approval semantics as before.

---

## Phase 5: User Story 3 - Make Persona And Shape Mapping Discoverable (Priority: P3)

**Goal**: Make the first-slice mapping explicit in roadmap and operator-facing docs.

**Independent Test**: A maintainer can identify targeted modes, intended personas, intended shapes, and deferred modes by reading repository artifacts alone.

### Validation for User Story 3 (MANDATORY)

- [x] T025 [P] [US3] Add discoverability evidence checkpoints in `specs/021-artifact-shapes-personas/validation-report.md`
- [x] T026 [US3] Record documentation-scope decisions in `specs/021-artifact-shapes-personas/decision-log.md`

### Implementation for User Story 3

- [x] T027 [P] [US3] Update roadmap mapping in `ROADMAP.md`
- [x] T028 [P] [US3] Update operator mode guidance in `docs/guides/modes.md`
- [x] T029 [US3] Capture US3 validation evidence in `specs/021-artifact-shapes-personas/validation-report.md`

**Checkpoint**: The first-slice persona and shape mapping is discoverable outside chat history.

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, closeout, and independent review.

- [x] T030 [P] Run skill synchronization validation with `scripts/validate-canon-skills.sh` and record results in `specs/021-artifact-shapes-personas/validation-report.md`
- [x] T031 [P] Run focused first-slice tests for `tests/requirements_authoring_docs.rs`, `tests/requirements_authoring_renderer.rs`, `tests/requirements_authoring_run.rs`, `tests/architecture_c4_docs.rs`, `tests/architecture_decision_shape_docs.rs`, `tests/architecture_c4_renderer.rs`, `tests/architecture_c4_run.rs`, `tests/change_authoring_docs.rs`, `tests/change_authoring_renderer.rs`, and `tests/change_authoring_run.rs` and record results in `specs/021-artifact-shapes-personas/validation-report.md`
- [x] T032 Run `cargo fmt` and `cargo test`, then finalize closeout notes in `specs/021-artifact-shapes-personas/validation-report.md`
- [x] T033 Perform independent review of invariants and final diff in `specs/021-artifact-shapes-personas/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: No dependencies. MUST complete first.
- **Phase 1: Setup**: Depends on Phase 0.
- **Phase 2: Foundational**: Depends on Phase 1. BLOCKS all user stories.
- **Phase 3: User Story 1**: Depends on Phase 2.
- **Phase 4: User Story 2**: Depends on User Story 1 because it tightens and proves the same touched surfaces.
- **Phase 5: User Story 3**: Depends on User Story 1 so docs reflect the implemented mapping.
- **Final Phase**: Depends on all selected user stories being complete.

### Within Each User Story

- Validation tasks and failing checks happen before implementation tasks.
- Decision and evidence updates happen before a story is declared complete.
- Skill source changes happen before or alongside mirrored skill changes.
- Renderer and contract validation happens before story evidence is closed.

### Parallel Opportunities

- T007 and T008 can run in parallel.
- T010 and T011 can run in parallel.
- T013, T014, and T015 can run in parallel.
- T027 and T028 can run in parallel.
- T030 and T031 can run in parallel after implementation is complete.

---

## Parallel Example: User Story 1

```bash
# Reconfirm first-slice behavior in parallel:
T010: requirements docs, renderer, and run confirmation
T011: architecture and change docs, renderer, and run confirmation

# Update first-slice skill pairs in parallel:
T013: requirements skill source and mirror
T014: architecture skill source and mirror
T015: change skill source and mirror
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phases 0, 1, and 2.
2. Complete User Story 1.
3. Stop and validate first-slice shaped packets before expanding scope.

### Incremental Delivery

1. Ship User Story 1 for the first-slice mapping.
2. Add User Story 2 to prove persona boundaries and honesty guarantees.
3. Add User Story 3 to publish the mapping clearly in repository docs.
4. Finish with verification, formatting, testing, and independent review.

### Parallel Team Strategy

1. One maintainer drives shared renderer/contract validation and evidence review.
2. One maintainer can update skill source and mirrors in parallel.
3. One maintainer can update roadmap and mode guidance once User Story 1 stabilizes.
