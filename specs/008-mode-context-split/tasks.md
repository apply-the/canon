# Tasks: Mode Context Split

**Input**: Design documents from `/specs/008-mode-context-split/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/mode-context-run-contract.md`, `quickstart.md`

**Validation**: Layered validation is mandatory. Runtime, CLI, documentation, skill, and coverage evidence must be captured in `specs/008-mode-context-split/validation-report.md`.

**Organization**: Tasks are grouped by user story so the renamed `change` workflow, the explicit `system_context` model, the public-surface cleanup, and the coverage recovery can be delivered incrementally.

## Format: `[ID] [P?] [Story] Description`

## Constitution Alignment

- Governance artifacts stay current before implementation begins.
- Systemic-impact work keeps independent validation and evidence capture separate from code generation.
- Every user story includes executable validation and closes with evidence updates.
- Final acceptance requires documentation truth, runtime truth, and coverage evidence to agree.

## Phase 0: Governance & Artifacts

- [X] T001 Reconfirm execution mode, risk classification, scope boundaries, and invariants in `specs/008-mode-context-split/spec.md` and `specs/008-mode-context-split/plan.md`
- [X] T002 Refresh decision and validation scaffolds in `specs/008-mode-context-split/decision-log.md` and `specs/008-mode-context-split/validation-report.md`
- [X] T003 Reconfirm CLI and runtime contract examples in `specs/008-mode-context-split/contracts/mode-context-run-contract.md` and `specs/008-mode-context-split/quickstart.md`
- [X] T004 Record independent review checkpoints and merge-owner expectations in `specs/008-mode-context-split/plan.md` and `specs/008-mode-context-split/validation-report.md`

---

## Phase 1: Setup (Shared Infrastructure)

- [X] T005 Create change-mode public surface scaffolds by copying `defaults/methods/brownfield-change.toml` to `defaults/methods/change.toml`, `defaults/embedded-skills/canon-brownfield/skill-source.md` to `defaults/embedded-skills/canon-change/skill-source.md`, and `.agents/skills/canon-brownfield/SKILL.md` to `.agents/skills/canon-change/SKILL.md`
- [X] T006 [P] Create change-mode test entrypoints by moving `tests/brownfield_contract.rs` to `tests/change_contract.rs`, `tests/brownfield_invocation_contract.rs` to `tests/change_invocation_contract.rs`, `tests/brownfield_run.rs` to `tests/change_run.rs`, `tests/brownfield_governed_execution.rs` to `tests/change_governed_execution.rs`, `tests/contract/brownfield_contract.rs` to `tests/contract/change_contract.rs`, `tests/contract/brownfield_invocation_contract.rs` to `tests/contract/change_invocation_contract.rs`, `tests/integration/brownfield_run.rs` to `tests/integration/change_run.rs`, and `tests/integration/brownfield_governed_execution.rs` to `tests/integration/change_governed_execution.rs`
- [X] T007 [P] Prepare shared runtime-hint rename touchpoints in `.agents/skills/canon-shared/scripts/check-runtime.sh`, `.agents/skills/canon-shared/scripts/check-runtime.ps1`, `defaults/embedded-skills/canon-shared/scripts/check-runtime.sh`, and `defaults/embedded-skills/canon-shared/scripts/check-runtime.ps1`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared runtime plumbing that must exist before any story-specific delivery can succeed.

- [X] T008 Extend the shared `SystemContext` model and persisted run-context fields in `crates/canon-engine/src/domain/run.rs`, `crates/canon-engine/src/persistence/manifests.rs`, and `crates/canon-engine/src/persistence/store.rs`
- [X] T009 [P] Add `--system-context` CLI plumbing in `crates/canon-cli/src/app.rs` and `crates/canon-cli/src/commands/run.rs`
- [X] T010 [P] Add shared mode/context validation helpers in `crates/canon-engine/src/orchestrator/classifier.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T011 [P] Thread `system_context` through status, inspect, and mode-result summaries in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-cli/src/output.rs`
- [X] T012 [P] Thread `system_context` through invocation and gate contexts in `crates/canon-engine/src/orchestrator/invocation.rs` and `crates/canon-engine/src/orchestrator/gatekeeper.rs`
- [X] T013 [P] Add semantic-split comments in `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/orchestrator/classifier.rs`, and `crates/canon-engine/src/orchestrator/gatekeeper.rs`

**Checkpoint**: Shared context plumbing exists and no user story starts before the new runtime contract is in place.

---

## Phase 3: User Story 1 - Start an Explicit Existing-System Change (Priority: P1) 🎯 MVP

**Goal**: Deliver the renamed `change` mode for existing systems while preserving the old bounded-change semantics.

**Independent Test**: Run `canon run --mode change --system-context existing ... --input canon-input/change.md` and verify renamed artifacts, persisted context, and preserved invariant or change-surface gating.

### Validation for User Story 1

- [X] T014 [P] [US1] Add failing change-mode contract and integration coverage in `tests/contract/change_contract.rs`, `tests/contract/change_invocation_contract.rs`, `tests/integration/change_run.rs`, and `tests/integration/change_governed_execution.rs`
- [X] T015 [US1] Record preserved-behavior and change-surface decisions in `specs/008-mode-context-split/decision-log.md`

### Implementation for User Story 1

- [X] T016 [US1] Replace brownfield mode parsing and profile binding with change in `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/modes/change.rs`, `crates/canon-engine/src/modes.rs`, `defaults/methods/change.toml`, and `crates/canon-engine/src/persistence/store.rs`
- [X] T017 [US1] Rebind change canonical inputs and artifact namespaces in `crates/canon-engine/src/persistence/layout.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T018 [US1] Preserve existing-system change gates and reject `change + new` in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T019 [US1] Update change-mode adapter prompting and mode-result summaries in `crates/canon-adapters/src/copilot_cli.rs` and `crates/canon-cli/src/output.rs`
- [X] T020 [US1] Capture change-mode validation evidence in `specs/008-mode-context-split/validation-report.md`

**Checkpoint**: The renamed `change` workflow is runnable for existing systems and preserves the prior bounded-change behavior.

---

## Phase 4: User Story 2 - Use the Same Mode Model Across Context-Aware Workflows (Priority: P2)

**Goal**: Enforce the required-versus-optional `system_context` matrix across the whole mode catalog and keep persisted context visible and truthful.

**Independent Test**: Start required-context modes without `--system-context` and observe fast failure; start optional-context modes without the flag and observe successful runs with no invented default in `context.toml`.

### Validation for User Story 2

- [X] T021 [P] [US2] Add failing context-matrix and persistence tests in `tests/contract/architecture_contract.rs`, `tests/contract/system_shaping_contract.rs`, `tests/contract/cli_contract.rs`, `tests/contract/inspect_modes.rs`, `tests/contract/runtime_filesystem.rs`, and `tests/mode_profiles.rs`
- [X] T022 [US2] Record required-versus-optional context decisions in `specs/008-mode-context-split/decision-log.md`

### Implementation for User Story 2

- [X] T023 [US2] Enforce required-context modes and legacy-name rejection in `crates/canon-engine/src/orchestrator/classifier.rs` and `crates/canon-cli/src/commands/run.rs`
- [X] T024 [US2] Persist optional and explicit `system_context` in `crates/canon-engine/src/domain/run.rs`, `crates/canon-engine/src/persistence/store.rs`, and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T025 [US2] Surface `system_context` through status, inspect, and evidence outputs in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-cli/src/output.rs`
- [X] T026 [P] [US2] Update required-context method manifests in `defaults/methods/architecture.toml`, `defaults/methods/system-shaping.toml`, `defaults/methods/implementation.toml`, `defaults/methods/refactor.toml`, `defaults/methods/migration.toml`, and `defaults/methods/incident.toml`
- [X] T027 [US2] Capture context-matrix validation evidence in `specs/008-mode-context-split/validation-report.md`

**Checkpoint**: The entire mode catalog obeys the explicit context matrix and persists `system_context` truthfully.

---

## Phase 5: User Story 3 - Read Consistent Runtime and Documentation Surfaces (Priority: P3)

**Goal**: Remove `brownfield` and `greenfield` vocabulary from public-facing Canon surfaces while keeping docs, skills, validators, and runtime hints aligned.

**Independent Test**: Read the updated docs and run the shared skill validators; confirm that canonical inputs, mode names, and user-facing hints all point to `change` and the two-axis model.

### Validation for User Story 3

- [X] T028 [P] [US3] Add public-surface regression checks in `tests/contract/cli_contract.rs`, `tests/contract/inspect_modes.rs`, `scripts/validate-canon-skills.sh`, and `scripts/validate-canon-skills.ps1`
- [X] T029 [US3] Record public API rename decisions in `specs/008-mode-context-split/decision-log.md`

### Implementation for User Story 3

- [X] T030 [US3] Update `README.md`, `MODE_GUIDE.md`, and `NEXT_FEATURES.md` to explain the two-axis model and the `change` naming
- [X] T031 [P] [US3] Replace brownfield skill entry points with change in `.agents/skills/canon-brownfield/SKILL.md`, `.agents/skills/canon-change/SKILL.md`, `defaults/embedded-skills/canon-brownfield/skill-source.md`, and `defaults/embedded-skills/canon-change/skill-source.md`
- [X] T032 [P] [US3] Remove brownfield input hints and validator expectations from `.agents/skills/canon-shared/scripts/check-runtime.sh`, `.agents/skills/canon-shared/scripts/check-runtime.ps1`, `defaults/embedded-skills/canon-shared/scripts/check-runtime.sh`, `defaults/embedded-skills/canon-shared/scripts/check-runtime.ps1`, `scripts/validate-canon-skills.sh`, `scripts/validate-canon-skills.ps1`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, and `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- [X] T033 [US3] Capture documentation and public-surface evidence in `specs/008-mode-context-split/validation-report.md`

**Checkpoint**: Public docs, skills, and validators describe the same mode model as the runtime.

---

## Phase 6: User Story 4 - Trust the Refactor Through Regression Coverage (Priority: P4)

**Goal**: Recover patch coverage across the touched runtime, gate, artifact, adapter, and CLI surfaces.

**Independent Test**: Run targeted change-mode suites and `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`; confirm the touched patch clears the agreed threshold and the listed hotspots are directly exercised.

### Validation for User Story 4

- [X] T034 [P] [US4] Add failing hotspot tests for mode-result and renderer branches in `crates/canon-cli/src/output.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, and `tests/contract/cli_contract.rs`
- [X] T035 [US4] Record coverage hotspots and acceptance threshold in `specs/008-mode-context-split/validation-report.md`

### Implementation for User Story 4

- [X] T036 [P] [US4] Expand service and gatekeeper regression tests in `tests/integration/change_run.rs`, `tests/integration/change_governed_execution.rs`, and `tests/contract/change_contract.rs`
- [X] T037 [P] [US4] Expand classifier and persistence regression tests in `tests/contract/change_invocation_contract.rs`, `tests/contract/runtime_evidence_contract.rs`, `tests/contract/runtime_filesystem.rs`, and `tests/contract/inspect_modes.rs`
- [X] T038 [P] [US4] Expand adapter and inspect-output regression tests in `crates/canon-adapters/src/copilot_cli.rs`, `tests/contract/cli_contract.rs`, and `tests/contract/invocation_cli_contract.rs`
- [X] T039 [US4] Capture coverage delta and hotspot results in `specs/008-mode-context-split/validation-report.md`

**Checkpoint**: Targeted regression coverage closes the patch-risk gap called out in the feature spec.

---

## Final Phase: Verification & Compliance

- [X] T040 [P] Run structural validation with `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `/bin/bash scripts/validate-canon-skills.sh`, and `pwsh -File scripts/validate-canon-skills.ps1`; record results in `specs/008-mode-context-split/validation-report.md`
- [X] T041 [P] Run logical validation with targeted change-mode suites and `cargo nextest run`; record results in `specs/008-mode-context-split/validation-report.md`
- [X] T042 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and record coverage evidence in `lcov.info` and `specs/008-mode-context-split/validation-report.md`
- [ ] T043 Perform independent review of the semantic split, documentation truth, and coverage evidence in `specs/008-mode-context-split/validation-report.md`
- [X] T044 Confirm invariants still hold, list modified files, and close `specs/008-mode-context-split/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: no dependencies; must complete first.
- **Phase 1: Setup**: depends on Phase 0.
- **Phase 2: Foundational**: depends on Phase 1 and blocks all user stories.
- **Phase 3: US1**: depends on Phase 2.
- **Phase 4: US2**: depends on Phase 2 and should merge after US1 establishes the renamed `change` surface.
- **Phase 5: US3**: depends on US1 and US2 so public docs reflect shipped runtime truth.
- **Phase 6: US4**: depends on US1 through US3 because coverage must measure the final semantic surface.
- **Final Phase**: depends on every desired user story being complete.

### User Story Completion Order

- **US1 (P1)**: MVP. Establishes the renamed `change` workflow for existing systems.
- **US2 (P2)**: Extends the explicit context contract across the rest of the mode catalog.
- **US3 (P3)**: Cleans up the public API, documentation, skill wrappers, and validators.
- **US4 (P4)**: Recovers patch coverage and closes the validation evidence gap.

### Within Each User Story

- Validation tasks must land before the implementation they constrain.
- Decision-log updates must happen before the affected runtime behavior is declared complete.
- Runtime plumbing comes before public-surface polish.
- Evidence capture is required before a story reaches its checkpoint.

---

## Parallel Examples

### User Story 1

```bash
# Validation and decision capture can start together:
T014 Add failing change-mode coverage
T015 Record preserved-behavior decisions

# After foundational plumbing, these can run in parallel:
T016 Replace mode parsing and profile binding
T019 Update adapter prompting and result summaries
```

### User Story 2

```bash
# Start matrix coverage and decisions together:
T021 Add failing context-matrix tests
T022 Record context decisions

# Once the validator behavior is defined, these can run in parallel:
T024 Persist optional and explicit system_context
T026 Update required-context method manifests
```

### User Story 3

```bash
# Public docs and shared skill surfaces can move in parallel:
T030 Update README.md, MODE_GUIDE.md, and NEXT_FEATURES.md
T031 Replace brownfield skill entry points with change
T032 Remove brownfield hints and validator expectations
```

### User Story 4

```bash
# Coverage expansion can split by hotspot family:
T036 Expand service and gatekeeper regression tests
T037 Expand classifier and persistence regression tests
T038 Expand adapter and inspect-output regression tests
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 0, Phase 1, and Phase 2.
2. Deliver US1 and validate `change + existing` end to end.
3. Stop and confirm the renamed change workflow is independently correct before broadening the mode matrix.

### Incremental Delivery

1. US1 establishes the renamed bounded-change runtime.
2. US2 expands the explicit context contract across the rest of the mode catalog.
3. US3 removes legacy public naming from docs, skills, and validators.
4. US4 recovers coverage and closes hotspot-specific validation evidence.
5. Final verification confirms runtime truth, documentation truth, and coverage truth all agree.

### Parallel Team Strategy

1. One engineer handles foundational plumbing in `domain/`, `classifier.rs`, and persistence.
2. One engineer handles change-mode runtime and artifact work in `service.rs`, `gatekeeper.rs`, `contract.rs`, and `markdown.rs`.
3. One engineer handles public docs, skills, validators, and runtime-hint cleanup.
4. Coverage-focused test expansion can fan out across the change, matrix, and output hotspots once the renamed runtime stabilizes.

---

## Notes

- Total tasks: 44
- User-story task counts: US1 = 7, US2 = 7, US3 = 6, US4 = 6
- Suggested MVP scope: Phase 0 through Phase 3 (US1) only
- Evidence paths: `specs/008-mode-context-split/decision-log.md`, `specs/008-mode-context-split/validation-report.md`, `specs/008-mode-context-split/contracts/mode-context-run-contract.md`, `specs/008-mode-context-split/quickstart.md`, `lcov.info`
- All tasks above follow the required checklist format: checkbox, task ID, optional `[P]`, required story label for story tasks, and exact file paths