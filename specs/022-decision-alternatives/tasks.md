# Tasks: Decision Alternatives, Pattern Choices, And Framework Evaluations

**Input**: Design documents from `/specs/022-decision-alternatives/`
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

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls and release boundary that permit
implementation to start

- [X] T001 Set Canon version to `0.22.0` in `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [X] T002 Update implementation-scope decisions and release-boundary notes in `specs/022-decision-alternatives/decision-log.md`
- [X] T003 Update planned structural, logical, and independent validation checkpoints in `specs/022-decision-alternatives/validation-report.md`
- [X] T004 Confirm decision-packet and persona-boundary contracts in `specs/022-decision-alternatives/contracts/decision-packet-shapes.md` and `specs/022-decision-alternatives/contracts/persona-completion.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and release-guard setup

- [X] T005 Update agent context from `specs/022-decision-alternatives/plan.md` into `AGENTS.md`
- [X] T006 Create release-surface regression coverage in `tests/release_022_docs.rs`
- [X] T007 [P] Prepare missing authored-input scaffolds in `docs/templates/canon-input/migration.md`, `docs/templates/canon-input/incident.md`, and `docs/examples/canon-input/migration-platform-consolidation.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared markdown-artifact and contract behavior that all user
stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T008 [P] Create or update failing shared contract coverage for decision-alternatives behavior in `tests/system_shaping_contract.rs`, `tests/architecture_contract.rs`, `tests/change_contract.rs`, `tests/implementation_contract.rs`, and `tests/migration_contract.rs`
- [X] T009 [P] Create or update failing shared renderer coverage for decision-alternatives sections in `tests/system_shaping_authoring_renderer.rs`, `tests/change_authoring_renderer.rs`, `tests/implementation_authoring_renderer.rs`, and `tests/migration_authoring_renderer.rs`
- [X] T010 Extend shared artifact contract extraction in `crates/canon-engine/src/artifacts/contract.rs`
- [X] T011 Extend shared markdown preservation and explicit gap handling in `crates/canon-engine/src/artifacts/markdown.rs`
- [X] T012 Capture foundational invariant and regression evidence in `specs/022-decision-alternatives/validation-report.md`

**Checkpoint**: Shared contract and renderer support are ready for story-level rollout

---

## Phase 3: User Story 1 - Compare Real Structural Alternatives (Priority: P1) 🎯 MVP

**Goal**: Deliver explicit decision-alternatives support for `system-shaping`,
`architecture`, and `change`

**Independent Test**: With representative authored briefs, the emitted packets
for `system-shaping`, `architecture`, and `change` expose viable options,
tradeoffs, the selected direction, and rejected alternatives without relying on
chat history.

### Validation for User Story 1 (MANDATORY)

- [X] T013 [P] [US1] Add failing structural-decision docs coverage in `tests/system_shaping_domain_modeling_docs.rs`, `tests/architecture_decision_shape_docs.rs`, and `tests/change_authoring_docs.rs`
- [X] T014 [P] [US1] Add failing structural-decision run coverage in `tests/system_shaping_run.rs`, `tests/architecture_c4_run.rs`, and `tests/change_authoring_run.rs`
- [X] T015 [US1] Record story-specific structural-decision choices under `## User Story 1 Decisions` in `specs/022-decision-alternatives/decision-log.md`

### Implementation for User Story 1

- [X] T016 [P] [US1] Update structural decision guidance in `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `defaults/embedded-skills/canon-architecture/skill-source.md`, and `defaults/embedded-skills/canon-change/skill-source.md`
- [X] T017 [P] [US1] Mirror structural decision guidance in `.agents/skills/canon-system-shaping/SKILL.md`, `.agents/skills/canon-architecture/SKILL.md`, and `.agents/skills/canon-change/SKILL.md`
- [X] T018 [P] [US1] Update structural decision templates in `docs/templates/canon-input/system-shaping.md`, `docs/templates/canon-input/architecture.md`, and `docs/templates/canon-input/change.md`
- [X] T019 [P] [US1] Update structural decision examples in `docs/examples/canon-input/system-shaping-billing.md`, `docs/examples/canon-input/architecture-state-management.md`, and `docs/examples/canon-input/change-add-caching.md`
- [X] T020 [US1] Capture structural decision validation evidence in `specs/022-decision-alternatives/validation-report.md`

**Checkpoint**: `system-shaping`, `architecture`, and `change` packets preserve
reviewable alternatives and explicit why-not reasoning

---

## Phase 4: User Story 2 - Evaluate Concrete Stack And Migration Choices (Priority: P2)

**Goal**: Deliver explicit framework-evaluation support for `implementation`
and `migration`

**Independent Test**: With representative authored briefs, the emitted packets
for `implementation` and `migration` expose concrete options,
ecosystem-health reasoning, adoption burden, and selected-direction rationale.

### Validation for User Story 2 (MANDATORY)

- [X] T021 [P] [US2] Add failing framework-evaluation docs coverage in `tests/implementation_authoring_docs.rs` and `tests/migration_authoring_docs.rs`
- [X] T022 [P] [US2] Add failing framework-evaluation run coverage in `tests/implementation_run.rs` and `tests/migration_run.rs`
- [X] T023 [US2] Record story-specific framework-evaluation choices under `## User Story 2 Decisions` in `specs/022-decision-alternatives/decision-log.md`

### Implementation for User Story 2

- [X] T024 [P] [US2] Update framework-evaluation guidance in `defaults/embedded-skills/canon-implementation/skill-source.md` and `defaults/embedded-skills/canon-migration/skill-source.md`
- [X] T025 [P] [US2] Mirror framework-evaluation guidance in `.agents/skills/canon-implementation/SKILL.md` and `.agents/skills/canon-migration/SKILL.md`
- [X] T026 [P] [US2] Update framework-evaluation templates in `docs/templates/canon-input/implementation.md` and `docs/templates/canon-input/migration.md`
- [X] T027 [P] [US2] Update framework-evaluation examples in `docs/examples/canon-input/implementation-auth-session-revocation.md` and `docs/examples/canon-input/migration-platform-consolidation.md`
- [X] T028 [US2] Capture framework-evaluation validation evidence in `specs/022-decision-alternatives/validation-report.md`

**Checkpoint**: `implementation` and `migration` packets preserve concrete
stack choices and adoption consequences

---

## Phase 5: User Story 3 - Make Persona And Release Scope Explicit For 0.22.0 (Priority: P3)

**Goal**: Complete adjacent persona guidance and make the `0.22.0` release
surface explicit

**Independent Test**: A maintainer can identify the bounded personas for
`review`, `pr-review`, `verification`, and `incident`, confirm the `0.22.0`
release surfaces, and see the remaining roadmap candidates without reading chat
history.

### Validation for User Story 3 (MANDATORY)

- [X] T029 [P] [US3] Add failing persona-guidance docs coverage in `tests/review_authoring_docs.rs`, `tests/verification_authoring_docs.rs`, `tests/incident_authoring_docs.rs`, and `tests/pr_review_docs.rs`
- [X] T030 [US3] Record persona-completion and release-surface decisions under `## User Story 3 Decisions` in `specs/022-decision-alternatives/decision-log.md`

### Implementation for User Story 3

- [X] T031 [P] [US3] Update persona guidance in `defaults/embedded-skills/canon-review/skill-source.md`, `defaults/embedded-skills/canon-pr-review/skill-source.md`, `defaults/embedded-skills/canon-verification/skill-source.md`, and `defaults/embedded-skills/canon-incident/skill-source.md`
- [X] T032 [P] [US3] Mirror persona guidance in `.agents/skills/canon-review/SKILL.md`, `.agents/skills/canon-pr-review/SKILL.md`, `.agents/skills/canon-verification/SKILL.md`, and `.agents/skills/canon-incident/SKILL.md`
- [X] T033 [US3] Update release and persona regression checks in `tests/release_022_docs.rs`, `tests/pr_review_docs.rs`, `tests/review_authoring_docs.rs`, `tests/verification_authoring_docs.rs`, and `tests/incident_authoring_docs.rs`
- [X] T034 [US3] Capture persona and release-surface validation evidence in `specs/022-decision-alternatives/validation-report.md`, including an explicit check that persona text stays advisory and avoids authority-claiming language

**Checkpoint**: Adjacent persona coverage is explicit and the `0.22.0` release
surface is discoverable

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and documentation closeout

- [X] T035 [P] Run `/bin/bash scripts/validate-canon-skills.sh` to verify embedded skill sources and mirrored `.agents/skills/` files stay synchronized, then record results in `specs/022-decision-alternatives/validation-report.md`
- [X] T036 [P] Run the targeted feature suite for `tests/system_shaping_contract.rs`, `tests/system_shaping_authoring_renderer.rs`, `tests/system_shaping_run.rs`, `tests/architecture_c4_docs.rs`, `tests/architecture_decision_shape_docs.rs`, `tests/architecture_c4_run.rs`, `tests/change_authoring_docs.rs`, `tests/change_authoring_renderer.rs`, `tests/change_authoring_run.rs`, `tests/implementation_authoring_docs.rs`, `tests/implementation_authoring_renderer.rs`, `tests/implementation_run.rs`, `tests/migration_authoring_docs.rs`, `tests/migration_authoring_renderer.rs`, `tests/migration_run.rs`, `tests/review_authoring_docs.rs`, `tests/pr_review_docs.rs`, `tests/verification_authoring_docs.rs`, and `tests/incident_authoring_docs.rs`, including an explicit architecture regression check that existing C4 and ADR behavior remains intact, then record results in `specs/022-decision-alternatives/validation-report.md`
- [ ] T037 [P] Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo nextest run`, then record results in `specs/022-decision-alternatives/validation-report.md`
- [X] T038 Perform independent review of persona boundaries, invariants, and final diff in `specs/022-decision-alternatives/validation-report.md`
- [X] T039 Update docs, examples, and roadmap closeout in `README.md`, `AGENTS.md`, `docs/guides/modes.md`, `CHANGELOG.md`, `ROADMAP.md`, `docs/templates/canon-input/`, and `docs/examples/canon-input/`, and verify all release-facing `0.22.0` references are synchronized

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the MVP.
- **User Story 2 (P2)**: Can start after Foundational. Reuses the shared packet preservation logic from Phase 2 but remains independently testable.
- **User Story 3 (P3)**: Can start after Foundational. Guidance-only persona completion and release-surface work must not block runtime-targeted packet behavior.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- Skills before mirrored skills.
- Templates before examples.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- Phase 0 tasks after T001 can run in parallel where they touch different planning artifacts.
- T008 and T009 can run in parallel before T010 and T011.
- Within each user story, skill, mirror, template, and example updates marked [P] can run in parallel.
- Final validation tasks T035, T036, and T037 can run in parallel once implementation is stable.

---

## Parallel Example: User Story 1

```bash
# Launch the structural docs and run regressions in parallel:
Task: "Add failing structural-decision docs coverage in tests/system_shaping_domain_modeling_docs.rs, tests/architecture_decision_shape_docs.rs, and tests/change_authoring_docs.rs"
Task: "Add failing structural-decision run coverage in tests/system_shaping_run.rs, tests/architecture_c4_run.rs, and tests/change_authoring_run.rs"

# Launch skill, mirror, template, and example work in parallel:
Task: "Update structural decision guidance in defaults/embedded-skills/canon-system-shaping/skill-source.md, defaults/embedded-skills/canon-architecture/skill-source.md, and defaults/embedded-skills/canon-change/skill-source.md"
Task: "Mirror structural decision guidance in .agents/skills/canon-system-shaping/SKILL.md, .agents/skills/canon-architecture/SKILL.md, and .agents/skills/canon-change/SKILL.md"
Task: "Update structural decision templates in docs/templates/canon-input/system-shaping.md, docs/templates/canon-input/architecture.md, and docs/templates/canon-input/change.md"
Task: "Update structural decision examples in docs/examples/canon-input/system-shaping-billing.md, docs/examples/canon-input/architecture-state-management.md, and docs/examples/canon-input/change-add-caching.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm structural decision packets are independently reviewable and update `validation-report.md`.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Finish with Verification & Compliance and repository closeout.

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together.
2. Once Foundational is done:
   - Developer A: User Story 1.
   - Developer B: User Story 2.
   - Developer C: User Story 3.
3. Each story closes only after its evidence is recorded.

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- `T001` is intentionally the version bump as requested
- `T039` is intentionally the docs/examples/ROADMAP closeout as requested
- Each user story should be independently completable and validated
- Keep the decision log and validation report current as work progresses