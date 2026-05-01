# Tasks: Decision Alternatives, Pattern Choices, And Framework Evaluations

**Input**: Design documents from `/specs/028-decision-alternatives/`
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

- [x] T001 Set Canon version to `0.28.0` in `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-scope decisions and release-boundary notes in `specs/028-decision-alternatives/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/028-decision-alternatives/validation-report.md`
- [x] T004 Confirm decision-packet and release-alignment contracts in `specs/028-decision-alternatives/contracts/decision-packet-shapes.md` and `specs/028-decision-alternatives/contracts/release-alignment.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and release-guard setup

- [x] T005 Update agent context from `specs/028-decision-alternatives/plan.md` into `AGENTS.md`
- [x] T006 Create release-surface regression coverage in `tests/release_028_docs.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared markdown-artifact and contract behavior that all user
stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 [P] Create or update failing shared contract coverage for decision-support behavior in `tests/system_shaping_contract.rs`, `tests/architecture_contract.rs`, `tests/architecture_c4_contract.rs`, `tests/change_contract.rs`, `tests/implementation_contract.rs`, and `tests/migration_contract.rs`
- [x] T008 [P] Create or update failing shared renderer coverage for decision-analysis and evidence-gap behavior in `tests/system_shaping_authoring_renderer.rs`, `tests/architecture_c4_renderer.rs`, `tests/change_authoring_renderer.rs`, `tests/implementation_authoring_renderer.rs`, and `tests/migration_authoring_renderer.rs`
- [x] T009 Extend shared artifact contract extraction in `crates/canon-engine/src/artifacts/contract.rs`
- [x] T010 Extend shared markdown preservation and explicit gap handling in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T011 Capture foundational invariant, regression, and touched-Rust-file coverage targets in `specs/028-decision-alternatives/validation-report.md`

**Checkpoint**: Shared contract and renderer support are ready for story-level rollout

---

## Phase 3: User Story 1 - Compare Real Structural Alternatives (Priority: P1) 🎯 MVP

**Goal**: Deliver explicit decision-analysis support for `system-shaping`,
`change`, and the `architecture` regression anchor

**Independent Test**: With representative authored briefs, the emitted packets
for `system-shaping`, `change`, and `architecture` expose viable options,
tradeoffs, the selected direction, and rejected alternatives without relying on
chat history.

### Validation for User Story 1 (MANDATORY)

- [x] T012 [P] [US1] Add failing structural-decision docs coverage in `tests/system_shaping_domain_modeling_docs.rs`, `tests/architecture_decision_shape_docs.rs`, and `tests/change_authoring_docs.rs`
- [x] T013 [P] [US1] Add failing structural-decision run coverage in `tests/system_shaping_run.rs`, `tests/architecture_c4_run.rs`, and `tests/change_authoring_run.rs`
- [x] T014 [US1] Record story-specific structural-decision choices under `## User Story 1 Decisions` in `specs/028-decision-alternatives/decision-log.md`

### Implementation for User Story 1

- [x] T015 [P] [US1] Update structural decision guidance in `defaults/embedded-skills/canon-system-shaping/skill-source.md`, `defaults/embedded-skills/canon-architecture/skill-source.md`, and `defaults/embedded-skills/canon-change/skill-source.md`
- [x] T016 [P] [US1] Mirror structural decision guidance in `.agents/skills/canon-system-shaping/SKILL.md`, `.agents/skills/canon-architecture/SKILL.md`, and `.agents/skills/canon-change/SKILL.md`
- [x] T017 [P] [US1] Update structural decision templates and examples in `docs/templates/canon-input/system-shaping.md`, `docs/templates/canon-input/architecture.md`, `docs/templates/canon-input/change.md`, `docs/examples/canon-input/system-shaping-billing.md`, `docs/examples/canon-input/architecture-state-management.md`, and `docs/examples/canon-input/change-add-caching.md`
- [x] T018 [US1] Capture structural-decision validation evidence in `specs/028-decision-alternatives/validation-report.md`

**Checkpoint**: `system-shaping` and `change` packets preserve reviewable alternatives while `architecture` stays aligned as the regression baseline

---

## Phase 4: User Story 2 - Evaluate Concrete Stack And Migration Choices (Priority: P2)

**Goal**: Deliver explicit framework-evaluation support for `implementation`
and `migration`

**Independent Test**: With representative authored briefs, the emitted packets
for `implementation` and `migration` expose concrete options,
evidence-grounded tradeoffs, ecosystem-health posture, adoption burden, and
selected-direction rationale.

### Validation for User Story 2 (MANDATORY)

- [x] T019 [P] [US2] Add failing framework-evaluation docs coverage in `tests/implementation_authoring_docs.rs` and `tests/migration_authoring_docs.rs`
- [x] T020 [P] [US2] Add failing framework-evaluation run coverage in `tests/implementation_run.rs` and `tests/migration_run.rs`
- [x] T021 [US2] Record story-specific framework-evaluation choices under `## User Story 2 Decisions` in `specs/028-decision-alternatives/decision-log.md`

### Implementation for User Story 2

- [x] T022 [P] [US2] Update framework-evaluation guidance in `defaults/embedded-skills/canon-implementation/skill-source.md` and `defaults/embedded-skills/canon-migration/skill-source.md`
- [x] T023 [P] [US2] Mirror framework-evaluation guidance in `.agents/skills/canon-implementation/SKILL.md` and `.agents/skills/canon-migration/SKILL.md`
- [x] T024 [P] [US2] Update framework-evaluation templates and examples in `docs/templates/canon-input/implementation.md`, `docs/templates/canon-input/migration.md`, `docs/examples/canon-input/implementation-auth-session-revocation.md`, and `docs/examples/canon-input/migration-platform-consolidation.md`
- [x] T025 [US2] Capture framework-evaluation validation evidence in `specs/028-decision-alternatives/validation-report.md`

**Checkpoint**: `implementation` and `migration` packets preserve concrete
stack choices, evidence posture, and adoption consequences

---

## Phase 5: User Story 3 - Ship 0.28.0 With Aligned Release Surfaces (Priority: P3)

**Goal**: Make the `0.28.0` version, runtime compatibility, and release
boundary explicit and testable

**Independent Test**: A maintainer can inspect version surfaces, runtime
compatibility references, and release-facing docs and confirm they describe the
same `0.28.0` slice without reading chat history.

### Validation for User Story 3 (MANDATORY)

- [x] T026 [P] [US3] Add failing release-surface and runtime-compatibility coverage in `tests/release_028_docs.rs` and `tests/skills_bootstrap.rs`
- [x] T027 [US3] Record story-specific release-alignment choices under `## User Story 3 Decisions` in `specs/028-decision-alternatives/decision-log.md`

### Implementation for User Story 3

- [x] T028 [US3] Update version surfaces in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T029 [US3] Capture release-alignment validation evidence and touched-Rust-file coverage expectations in `specs/028-decision-alternatives/validation-report.md`

**Checkpoint**: The runtime and release-version surfaces align cleanly at `0.28.0`

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and documentation closeout

- [x] T030 [P] Run `/bin/bash scripts/validate-canon-skills.sh` to verify embedded skill sources and mirrored `.agents/skills/` files stay synchronized, then record results in `specs/028-decision-alternatives/validation-report.md`
- [x] T031 Update impacted docs and changelog closeout in `README.md`, `AGENTS.md`, `docs/guides/modes.md`, `CHANGELOG.md`, `ROADMAP.md`, `docs/templates/canon-input/`, and `docs/examples/canon-input/`, and verify all release-facing `0.28.0` references are synchronized
- [x] T032 [P] Run the targeted feature suite for `tests/system_shaping_contract.rs`, `tests/system_shaping_domain_modeling_docs.rs`, `tests/system_shaping_authoring_renderer.rs`, `tests/system_shaping_run.rs`, `tests/architecture_contract.rs`, `tests/architecture_c4_contract.rs`, `tests/architecture_c4_docs.rs`, `tests/architecture_decision_shape_docs.rs`, `tests/architecture_c4_renderer.rs`, `tests/architecture_c4_run.rs`, `tests/change_contract.rs`, `tests/change_authoring_docs.rs`, `tests/change_authoring_renderer.rs`, `tests/change_authoring_run.rs`, `tests/implementation_contract.rs`, `tests/implementation_authoring_docs.rs`, `tests/implementation_authoring_renderer.rs`, `tests/implementation_run.rs`, `tests/migration_contract.rs`, `tests/migration_authoring_docs.rs`, `tests/migration_authoring_renderer.rs`, `tests/migration_run.rs`, `tests/release_028_docs.rs`, and `tests/skills_bootstrap.rs`, then record results in `specs/028-decision-alternatives/validation-report.md`
- [x] T033 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and document coverage for every modified or newly created Rust file in `specs/028-decision-alternatives/validation-report.md`
- [x] T034 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/028-decision-alternatives/validation-report.md`
- [x] T035 [P] Run `cargo nextest run --workspace --all-features` and record results in `specs/028-decision-alternatives/validation-report.md`
- [x] T036 Perform independent review of invariants, evidence honesty, and final diff in `specs/028-decision-alternatives/validation-report.md`
- [x] T037 Confirm invariants still hold and close the final validation state in `specs/028-decision-alternatives/validation-report.md`

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
- **User Story 3 (P3)**: Can start after Foundational. Release alignment must not weaken the runtime behavior delivered by earlier stories.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- Skills before mirrored skills.
- Templates before examples when both change in the same story.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- Phase 0 tasks after T001 can run in parallel where they touch different planning artifacts.
- T007 and T008 can run in parallel before T009 and T010.
- Within each user story, skill, mirror, and docs/example tasks marked [P] can run in parallel.
- Final validation tasks T030, T032, T033, T034, and T035 can run in parallel once implementation is stable.

---

## Parallel Example: User Story 1

```bash
# Launch the structural docs and run regressions in parallel:
Task: "Add failing structural-decision docs coverage in tests/system_shaping_domain_modeling_docs.rs, tests/architecture_decision_shape_docs.rs, and tests/change_authoring_docs.rs"
Task: "Add failing structural-decision run coverage in tests/system_shaping_run.rs, tests/architecture_c4_run.rs, and tests/change_authoring_run.rs"

# Launch skill, mirror, and docs/example work in parallel:
Task: "Update structural decision guidance in defaults/embedded-skills/canon-system-shaping/skill-source.md, defaults/embedded-skills/canon-architecture/skill-source.md, and defaults/embedded-skills/canon-change/skill-source.md"
Task: "Mirror structural decision guidance in .agents/skills/canon-system-shaping/SKILL.md, .agents/skills/canon-architecture/SKILL.md, and .agents/skills/canon-change/SKILL.md"
Task: "Update structural decision templates and examples in docs/templates/canon-input/system-shaping.md, docs/templates/canon-input/architecture.md, docs/templates/canon-input/change.md, docs/examples/canon-input/system-shaping-billing.md, docs/examples/canon-input/architecture-state-management.md, and docs/examples/canon-input/change-add-caching.md"
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
- `T031` is intentionally the impacted docs plus changelog closeout task as requested
- Each user story should be independently completable and validated
- Keep the decision log and validation report current as work progresses