# Tasks: Supply Chain And Legacy Analysis Mode

**Input**: Design documents from `/specs/024-supply-chain-legacy/`
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

- [x] T001 Set Canon version `0.24.0` in `Cargo.toml`, `CHANGELOG.md`, `README.md`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-scope decisions in `specs/024-supply-chain-legacy/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/024-supply-chain-legacy/validation-report.md`
- [x] T004 Confirm the mode contracts in `specs/024-supply-chain-legacy/contracts/supply-chain-packet-shape.md` and `specs/024-supply-chain-legacy/contracts/scanner-intake-and-coverage-gap.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and authoring-surface setup

- [x] T005 Update agent context from `specs/024-supply-chain-legacy/plan.md` into `AGENTS.md`
- [x] T006 [P] Create the mode scaffolds in `defaults/methods/supply-chain-analysis.toml`, `defaults/embedded-skills/canon-supply-chain-analysis/skill-source.md`, and `.agents/skills/canon-supply-chain-analysis/SKILL.md`
- [x] T007 [P] Create authored-input scaffolds in `docs/templates/canon-input/supply-chain-analysis.md` and `docs/examples/canon-input/supply-chain-analysis-rust-workspace.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared runtime registration and compatibility work that all user
stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T008 [P] Add failing shared mode-registration coverage in `tests/integration/mode_profiles.rs` and `tests/contract/inspect_modes.rs`
- [x] T009 [P] Add failing initialization and release-surface coverage in `tests/integration/init_creates_canon.rs`, `tests/direct_runtime_coverage.rs`, and `tests/release_024_docs.rs`
- [x] T010 Extend the core mode registry and system-context rules in `crates/canon-engine/src/domain/mode.rs` and `crates/canon-engine/src/orchestrator/classifier.rs`
- [x] T011 Extend shared runtime integration in `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/orchestrator/service/execution.rs`, and `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [x] T012 Extend materialization and shared runtime persistence in `crates/canon-engine/src/persistence/store.rs`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T013 Capture foundational invariant and compatibility evidence in `specs/024-supply-chain-legacy/validation-report.md`

**Checkpoint**: Shared runtime registration, initialization, and release surfaces recognize the new mode

---

## Phase 3: User Story 1 - Run A Governed Supply-Chain Packet (Priority: P1) 🎯 MVP

**Goal**: Deliver the runtime `supply-chain-analysis` mode and its persisted packet

**Independent Test**: With a representative authored brief and repository
surface, Canon can run `supply-chain-analysis`, emit the expected artifacts
under `.canon/`, preserve recommendation-only posture, and publish the packet
to `docs/supply-chain/<RUN_ID>/`.

### Validation for User Story 1 (MANDATORY)

- [x] T014 [P] [US1] Add failing contract and renderer coverage in `tests/supply_chain_analysis_contract.rs`, `tests/contract/supply_chain_analysis_contract.rs`, and `tests/supply_chain_analysis_authoring_renderer.rs`
- [x] T015 [P] [US1] Add failing direct-runtime, run, and publish coverage in `tests/supply_chain_analysis_direct_runtime.rs`, `tests/supply_chain_analysis_run.rs`, and `tests/integration/supply_chain_analysis_run.rs`
- [x] T016 [US1] Record story-specific runtime decisions under `## User Story 1 Decisions` in `specs/024-supply-chain-legacy/decision-log.md`

### Implementation for User Story 1

- [x] T017 [P] [US1] Add the supply-chain method definition, artifact contract, and renderer in `defaults/methods/supply-chain-analysis.toml`, `crates/canon-engine/src/artifacts/contract.rs`, and `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T018 [P] [US1] Add gate evaluation and mode service implementation in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/service/mode_supply_chain_analysis.rs`
- [x] T019 [US1] Wire the new mode through runtime dispatch, summaries, canonical input binding, and publish mapping in `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, and `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T020 [US1] Capture runtime packet validation evidence in `specs/024-supply-chain-legacy/validation-report.md`

**Checkpoint**: `supply-chain-analysis` runs end to end and emits a reviewable recommendation-only packet

---

## Phase 4: User Story 2 - Clarify Posture And Govern Missing Tool Decisions (Priority: P2)

**Goal**: Deliver the clarification, missing-scanner, authoring, and discoverability surfaces for the new mode

**Independent Test**: A maintainer can provide posture answers, handle a
missing-scanner decision, validate mirrored-skill sync, and review the resulting
coverage-gap evidence without hand-editing runtime internals.

### Validation for User Story 2 (MANDATORY)

- [x] T021 [P] [US2] Add failing clarification and coverage-gap runtime coverage in `tests/supply_chain_analysis_direct_runtime.rs`, `tests/integration/supply_chain_analysis_run.rs`, and `tests/direct_runtime_coverage.rs`
- [x] T022 [P] [US2] Add failing authoring and docs coverage in `tests/supply_chain_analysis_authoring_docs.rs`, `tests/release_024_docs.rs`, `tests/skills_bootstrap.rs`, and `tests/integration/init_creates_canon.rs`
- [x] T023 [US2] Record story-specific clarification and authoring decisions under `## User Story 2 Decisions` in `specs/024-supply-chain-legacy/decision-log.md`

### Implementation for User Story 2

- [x] T024 [P] [US2] Implement clarification intake, scanner decision recording, and coverage-gap packet behavior in `crates/canon-engine/src/orchestrator/service/mode_supply_chain_analysis.rs` and `crates/canon-engine/src/orchestrator/service/clarity.rs`
- [x] T025 [P] [US2] Author the supply-chain skill surfaces in `defaults/embedded-skills/canon-supply-chain-analysis/skill-source.md` and `.agents/skills/canon-supply-chain-analysis/SKILL.md`
- [x] T026 [P] [US2] Add shared compatibility and runtime helper updates in `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/scripts/check-runtime.sh`, and `.agents/skills/canon-shared/scripts/check-runtime.sh`
- [x] T027 [P] [US2] Add mode guidance and examples in `docs/guides/modes.md`, `docs/templates/canon-input/supply-chain-analysis.md`, and `docs/examples/canon-input/supply-chain-analysis-rust-workspace.md`
- [x] T028 [US2] Capture authoring and clarification validation evidence in `specs/024-supply-chain-legacy/validation-report.md`

**Checkpoint**: The new mode is discoverable, authorable, and honest about missing-tool coverage

---

## Phase 5: User Story 3 - Ship 0.24.0 With Coverage And Quality Gates Closed (Priority: P3)

**Goal**: Close the release boundary with version synchronization, release-surface updates, and validation-ready evidence

**Independent Test**: A maintainer can inspect release-facing files, the task
plan, and the validation report and confirm the feature is staged for `0.24.0`
with explicit closeout expectations.

### Validation for User Story 3 (MANDATORY)

- [x] T029 [P] [US3] Add failing release-surface and compatibility regression coverage in `tests/release_024_docs.rs`, `tests/contract/inspect_modes.rs`, `tests/integration/mode_profiles.rs`, `tests/direct_runtime_coverage.rs`, and `tests/skills_bootstrap.rs`
- [x] T030 [US3] Record story-specific release decisions under `## User Story 3 Decisions` in `specs/024-supply-chain-legacy/decision-log.md`

### Implementation for User Story 3

- [x] T031 [P] [US3] Update release-facing docs and version surfaces in `Cargo.toml`, `CHANGELOG.md`, `README.md`, `AGENTS.md`, and `ROADMAP.md`
- [x] T032 [US3] Capture release-surface validation evidence in `specs/024-supply-chain-legacy/validation-report.md`

**Checkpoint**: The `0.24.0` release surface is synchronized and traceable

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and repository-quality closeout

- [x] T033 [P] Run the focused supply-chain contract, renderer, runtime, publish, and authoring test suite and record results in `specs/024-supply-chain-legacy/validation-report.md`
- [x] T034 [P] Run shared regression checks for mode discovery, initialization, and skill materialization and record results in `specs/024-supply-chain-legacy/validation-report.md`
- [x] T035 [P] Run `/bin/bash scripts/validate-canon-skills.sh` and record results in `specs/024-supply-chain-legacy/validation-report.md`
- [x] T036 Perform independent review of recommendation-only posture, coverage-gap honesty, and final diff in `specs/024-supply-chain-legacy/validation-report.md`
- [ ] T037 Guarantee at least 85% line coverage for every Rust file added or modified by this feature, update any remaining `docs/`, `docs/examples/`, and `ROADMAP.md` surfaces touched by the closeout, run `cargo fmt` plus `cargo fmt --check`, run `cargo clippy --workspace --all-targets --all-features -- -D warnings`, resolve warnings or errors in touched files, and record the clean final closeout in `specs/024-supply-chain-legacy/validation-report.md`

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
- **User Story 2 (P2)**: Can start after Foundational. Reuses the new runtime mode but remains independently testable through clarification and authoring surfaces.
- **User Story 3 (P3)**: Can start after Foundational. Depends on the mode and docs surfaces being stable enough for release closure.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- Runtime contracts before service wiring.
- Skills before mirrored-skill validation.
- Templates before examples.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- Phase 0 tasks after T001 can run in parallel where they touch different planning artifacts.
- T008 and T009 can run in parallel before T010 through T012.
- Within User Story 1, contract and runtime coverage can run in parallel, and contract plus gate or service implementation can run in parallel once the shared mode registry exists.
- Within User Story 2, clarification runtime work, skill authoring, shared helper updates, and docs updates marked [P] can run in parallel.
- Final validation tasks T033 through T035 can run in parallel once implementation is stable, but T037 must be the last execution task.

---

## Parallel Example: User Story 2

```bash
# Launch failing validation in parallel:
Task: "Add failing clarification and coverage-gap runtime coverage in tests/supply_chain_analysis_direct_runtime.rs, tests/integration/supply_chain_analysis_run.rs, and tests/direct_runtime_coverage.rs"
Task: "Add failing authoring and docs coverage in tests/supply_chain_analysis_authoring_docs.rs, tests/release_024_docs.rs, tests/skills_bootstrap.rs, and tests/integration/init_creates_canon.rs"

# Launch compatible implementation slices in parallel after the runtime packet exists:
Task: "Author the supply-chain skill surfaces in defaults/embedded-skills/canon-supply-chain-analysis/skill-source.md and .agents/skills/canon-supply-chain-analysis/SKILL.md"
Task: "Add mode guidance and examples in docs/guides/modes.md, docs/templates/canon-input/supply-chain-analysis.md, and docs/examples/canon-input/supply-chain-analysis-rust-workspace.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm the new mode runs and emits the expected packet before expanding to clarification and release surfaces.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Finish with coverage growth, docs/examples/roadmap closeout, `cargo fmt`, and lint remediation.

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together.
2. Once Foundational is done:
   - Developer A: User Story 1 runtime mode work.
   - Developer B: User Story 2 clarification, skill, and docs work.
   - Developer C: User Story 3 release-surface and validation-closeout work.
3. Each story closes only after its evidence is recorded.

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- `T001` is intentionally the version bump to `0.24.0` as requested
- `T037` is intentionally the final task and explicitly includes high coverage,
  docs/examples/roadmap updates, `cargo fmt`, and clippy clean-up as requested
- Each user story should be independently completable and validated
- Keep the decision log and validation report current as work progresses