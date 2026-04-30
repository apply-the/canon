# Tasks: System Assessment Mode

**Input**: Design documents from `/specs/027-system-assessment-mode/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks
whenever behavior, interfaces, or regressions must be checked. Independent
review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent
implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this belongs to (e.g., [US1], [US2], [US3])
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

- [x] T001 Set Canon version `0.26.0` in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-scope decisions in `specs/027-system-assessment-mode/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/027-system-assessment-mode/validation-report.md`
- [x] T004 Confirm the assessment contracts in `specs/027-system-assessment-mode/contracts/system-assessment-packet.md` and `specs/027-system-assessment-mode/contracts/system-assessment-publish.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and assessment-surface setup

- [x] T005 Update agent context from `specs/027-system-assessment-mode/plan.md` into `AGENTS.md`
- [x] T006 [P] Create the mode scaffolds in `defaults/methods/system-assessment.toml`, `defaults/embedded-skills/canon-system-assessment/skill-source.md`, and `.agents/skills/canon-system-assessment/SKILL.md`
- [x] T007 [P] Create authored-input scaffolds in `docs/templates/canon-input/system-assessment.md` and `docs/examples/canon-input/system-assessment-commerce-platform.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared runtime registration and compatibility work that all user
stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T008 [P] Add failing shared mode-registration coverage in `tests/integration/mode_profiles.rs`, `tests/contract/inspect_modes.rs`, and `tests/integration/init_creates_canon.rs`
- [x] T009 [P] Add failing shared runtime-surface coverage in `tests/release_027_system_assessment_mode.rs`, `tests/integration/skills_bootstrap.rs`, and `tests/release_022_docs.rs`
- [x] T010 Extend the core mode registry and system-context rules in `crates/canon-engine/src/domain/mode.rs` and `crates/canon-engine/src/orchestrator/classifier.rs`
- [x] T011 Extend shared runtime integration in `crates/canon-engine/src/persistence/store.rs`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, and `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [x] T012 Capture foundational registration and compatibility evidence in `specs/027-system-assessment-mode/validation-report.md`

**Checkpoint**: Shared runtime registration, publishing, and initialization surfaces recognize the new mode

---

## Phase 3: User Story 1 - Assess An Existing System As-Is (Priority: P1) 🎯 MVP

**Goal**: Deliver the runtime `system-assessment` mode and its persisted as-is packet

**Independent Test**: With a representative authored brief, Canon can run
`system-assessment`, emit the expected artifacts under `.canon/`, preserve
missing-body honesty, and summarize the result as an as-is packet.

### Validation for User Story 1 (MANDATORY)

- [x] T013 [P] [US1] Add failing contract and renderer coverage in `tests/system_assessment_contract.rs`, `tests/contract/system_assessment_contract.rs`, and `tests/system_assessment_authoring_renderer.rs`
- [x] T014 [P] [US1] Add failing run and publish coverage in `tests/system_assessment_run.rs` and `tests/integration/system_assessment_run.rs`
- [x] T015 [US1] Record story-specific runtime decisions under `## User Story 1 Decisions` in `specs/027-system-assessment-mode/decision-log.md`

### Implementation for User Story 1

- [x] T016 [P] [US1] Add the assessment artifact contract and renderer in `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T017 [P] [US1] Add gate evaluation and mode service implementation in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/service/mode_system_assessment.rs`
- [x] T018 [US1] Wire the new mode through runtime dispatch and summaries in `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, and `crates/canon-engine/src/orchestrator/service/tests.rs`
- [x] T019 [US1] Capture runtime packet validation evidence in `specs/027-system-assessment-mode/validation-report.md`

**Checkpoint**: `system-assessment` runs end to end and emits a reviewable as-is packet

---

## Phase 4: User Story 2 - Author, Publish, And Reuse The Assessment Packet (Priority: P2)

**Goal**: Deliver aligned authoring, compatibility, and publish surfaces for the new mode

**Independent Test**: A maintainer can author the packet via the new skill and
template, validate mirrored-skill sync, and publish the packet to the dedicated
assessment docs location.

### Validation for User Story 2 (MANDATORY)

- [x] T020 [P] [US2] Add failing authoring and docs coverage in `tests/system_assessment_authoring_docs.rs`, `tests/release_027_system_assessment_mode.rs`, `tests/integration/init_creates_canon.rs`, and `tests/integration/skills_bootstrap.rs`
- [x] T021 [US2] Record story-specific authoring and publish decisions under `## User Story 2 Decisions` in `specs/027-system-assessment-mode/decision-log.md`

### Implementation for User Story 2

- [x] T022 [P] [US2] Add shared compatibility and runtime helper updates in `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/scripts/check-runtime.sh`, and `.agents/skills/canon-shared/scripts/check-runtime.sh`
- [x] T023 [P] [US2] Author the assessment skill surfaces in `defaults/embedded-skills/canon-system-assessment/skill-source.md` and `.agents/skills/canon-system-assessment/SKILL.md`
- [x] T024 [P] [US2] Add mode guidance and authored-input examples in `docs/guides/modes.md`, `docs/templates/canon-input/system-assessment.md`, and `docs/examples/canon-input/system-assessment-commerce-platform.md`
- [x] T025 [US2] Capture authoring and publish validation evidence in `specs/027-system-assessment-mode/validation-report.md`

**Checkpoint**: The new mode is discoverable, authorable, and publishable through the documented surfaces

---

## Phase 5: User Story 3 - Ship 0.26.0 With Coverage And Quality Gates Closed (Priority: P3)

**Goal**: Close the release boundary with docs, coverage growth, and workspace quality gates

**Independent Test**: A maintainer can review the versioned release surface,
run the targeted validation suite, and confirm the evidence is recorded in the
validation report.

### Validation for User Story 3 (MANDATORY)

- [x] T026 [P] [US3] Add failing release and compatibility regression coverage in `tests/release_027_system_assessment_mode.rs`, `tests/contract/inspect_modes.rs`, `tests/system_assessment_authoring_docs.rs`, and `tests/integration/init_creates_canon.rs`
- [x] T027 [US3] Record story-specific release decisions under `## User Story 3 Decisions` in `specs/027-system-assessment-mode/decision-log.md`

### Implementation for User Story 3

- [x] T028 [US3] Update release-facing runtime references in `crates/canon-cli/src/output.rs`, `README.md`, and `AGENTS.md`
- [x] T029 [US3] Capture release-surface validation evidence in `specs/027-system-assessment-mode/validation-report.md`

**Checkpoint**: The `0.26.0` release surface is synchronized and the feature remains traceable

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and release-quality closeout

- [x] T030 [P] Run the targeted `system-assessment` test suite and record results in `specs/027-system-assessment-mode/validation-report.md`
- [x] T031 [P] Run `/bin/bash scripts/validate-canon-skills.sh` and record results in `specs/027-system-assessment-mode/validation-report.md`
- [x] T032 [P] Perform independent review of as-is posture, invariants, and publish-path integrity in `specs/027-system-assessment-mode/validation-report.md`
- [x] T033 Finalize coverage in `tests/system_assessment_contract.rs`, `tests/system_assessment_authoring_renderer.rs`, `tests/system_assessment_authoring_docs.rs`, `tests/system_assessment_run.rs`, `tests/release_027_system_assessment_mode.rs`, `tests/contract/inspect_modes.rs`, `tests/integration/init_creates_canon.rs`, and `tests/integration/skills_bootstrap.rs`; update `ROADMAP.md`, `README.md`, `CHANGELOG.md`, and `specs/027-system-assessment-mode/validation-report.md`; run `cargo fmt`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, resolving warning or error findings in touched files

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
- **User Story 2 (P2)**: Can start after Foundational. Reuses the new runtime mode but remains independently testable through authoring and publish surfaces.
- **User Story 3 (P3)**: Can start after Foundational. Depends on the mode and docs surfaces being stable enough for release closure.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- Runtime contracts before mode-service wiring.
- Shared compatibility before mirrored-skill sync checks.
- Templates before examples.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- `T006` and `T007` can run in parallel after Phase 0.
- `T008` and `T009` can run in parallel before `T010` and `T011`.
- Within User Story 1, contract or renderer coverage and run coverage can run in parallel, and contract or gate/service implementation can run in parallel once the shared mode registry exists.
- Within User Story 2, shared compatibility, skill, and docs updates marked `[P]` can run in parallel.
- Final validation tasks `T030` through `T032` can run in parallel once implementation is stable, but `T033` is intentionally last.

---

## Parallel Example: User Story 1

```bash
# Launch failing runtime coverage in parallel:
Task: "Add failing contract and renderer coverage in tests/system_assessment_contract.rs, tests/contract/system_assessment_contract.rs, and tests/system_assessment_authoring_renderer.rs"
Task: "Add failing run and publish coverage in tests/system_assessment_run.rs and tests/integration/system_assessment_run.rs"

# Launch compatible implementation slices in parallel after the shared mode registry lands:
Task: "Add the assessment artifact contract and renderer in crates/canon-engine/src/artifacts/contract.rs and crates/canon-engine/src/artifacts/markdown.rs"
Task: "Add gate evaluation and mode service implementation in crates/canon-engine/src/orchestrator/gatekeeper.rs and crates/canon-engine/src/orchestrator/service/mode_system_assessment.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm the new mode runs and emits the expected as-is packet before expanding into authoring and release surfaces.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Finish with targeted tests, skill validation, independent review, and the requested release-closeout task.

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together.
2. Once Foundational is done:
   - Developer A: User Story 1 runtime mode work.
   - Developer B: User Story 2 skill and docs work.
   - Developer C: User Story 3 release-surface and closeout work.
3. Each story closes only after its evidence is recorded.

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- `T001` intentionally bumps the next release to `0.26.0`
- `T033` is intentionally the final closeout task and includes the requested coverage, roadmap/docs/changelog, formatting, and clippy work
- Keep the decision log and validation report current as work progresses