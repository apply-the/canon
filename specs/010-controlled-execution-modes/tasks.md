# Tasks: Controlled Execution Modes (`implementation` and `refactor`)

**Input**: Design documents from `/specs/010-controlled-execution-modes/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/implementation-execution-contract.md`, `contracts/refactor-execution-contract.md`, `quickstart.md`

**Validation**: Layered validation is mandatory. Runtime, CLI, policy, documentation, skill, and publish evidence must be captured in `specs/010-controlled-execution-modes/validation-report.md`.

**Organization**: Tasks are grouped by user story so bounded `implementation`, preservation-first `refactor`, and recommendation-only fallback can be implemented and validated incrementally.

## Format: `[ID] [P?] [Story] Description`

## Constitution Alignment

- Governance artifacts stay current before implementation begins.
- No execution-heavy runtime change lands before the artifacts that authorize mutation, preservation, and validation ownership.
- Every user story includes executable validation tasks and closes with evidence capture.
- Independent review remains separate from generation and closes in the final verification phase.

## Phase 0: Governance & Artifacts

- [X] T001 Reconfirm execution mode, risk classification, scope boundaries, and invariants in `specs/010-controlled-execution-modes/spec.md` and `specs/010-controlled-execution-modes/plan.md`
- [X] T002 Refresh decision and validation scaffolds in `specs/010-controlled-execution-modes/decision-log.md` and `specs/010-controlled-execution-modes/validation-report.md`
- [X] T003 Reconfirm execution contracts and end-to-end scenarios in `specs/010-controlled-execution-modes/contracts/implementation-execution-contract.md`, `specs/010-controlled-execution-modes/contracts/refactor-execution-contract.md`, and `specs/010-controlled-execution-modes/quickstart.md`
- [X] T004 Record independent review checkpoints, approval gates, and evidence owners in `specs/010-controlled-execution-modes/plan.md` and `specs/010-controlled-execution-modes/validation-report.md`

---

## Phase 1: Setup (Shared Infrastructure)

- [X] T005 Create implementation-mode contract and integration test entrypoints in `tests/contract/implementation_contract.rs` and `tests/integration/implementation_run.rs`
- [X] T006 [P] Create refactor-mode contract and integration test entrypoints in `tests/contract/refactor_contract.rs`, `tests/integration/refactor_run.rs`, and `tests/integration/refactor_preservation_run.rs`
- [X] T007 [P] Prepare shared runtime-hint and validator touchpoints in `defaults/embedded-skills/canon-shared/scripts/check-runtime.sh`, `defaults/embedded-skills/canon-shared/scripts/check-runtime.ps1`, `.agents/skills/canon-shared/scripts/check-runtime.sh`, `.agents/skills/canon-shared/scripts/check-runtime.ps1`, and `scripts/validate-canon-skills.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared runtime plumbing that must exist before any execution-heavy story can be delivered safely.

**⚠️ CRITICAL**: No user story work starts before these tasks are complete.

- [X] T008 Extend mode-specific execution metadata in `crates/canon-engine/src/domain/run.rs`, `crates/canon-engine/src/domain/execution.rs`, and `crates/canon-engine/src/persistence/manifests.rs`
- [X] T009 [P] Persist and resume mode-specific execution metadata in `crates/canon-engine/src/persistence/store.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T010 [P] Add canonical `canon-input/implementation.*` and `canon-input/refactor.*` auto-binding in `crates/canon-cli/src/app.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T011 [P] Add shared mutation-bounds and recommendation-only constraint plumbing in `crates/canon-engine/src/domain/policy.rs`, `crates/canon-engine/src/orchestrator/invocation.rs`, and `defaults/policies/adapters.toml`
- [X] T012 [P] Thread execution-posture summaries through `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-cli/src/output.rs`
- [X] T013 [P] Prepare shared mode-profile and embedded-store scaffolds in `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/persistence/store.rs`, `defaults/methods/implementation.toml`, and `defaults/methods/refactor.toml`

**Checkpoint**: Shared execution metadata, input binding, and posture plumbing exist before any mode-specific gates or artifacts are promoted.

---

## Phase 3: User Story 1 - Bounded Implementation Execution (Priority: P1) 🎯 MVP

**Goal**: Deliver `implementation` as a governed bounded-execution mode for existing codebases with explicit mutation bounds and task mapping.

**Independent Test**: Run `canon run --mode implementation --system-context existing --risk bounded-impact --zone yellow --input canon-input/implementation.md`, verify the run blocks without explicit bounds, verify emitted artifacts and context metadata when bounds exist, and confirm publish works through the existing surfaces.

### Validation for User Story 1

- [X] T014 [P] [US1] Add failing implementation artifact-contract and authored-input tests in `tests/contract/implementation_contract.rs` and `tests/direct_runtime_coverage.rs`
- [X] T015 [P] [US1] Add failing implementation lifecycle and invocation-policy tests in `tests/integration/implementation_run.rs` and `tests/policy_and_traces.rs`
- [X] T016 [US1] Record implementation task-mapping, mutation-bounds, and safety-net decisions in `specs/010-controlled-execution-modes/decision-log.md`

### Implementation for User Story 1

- [X] T017 [P] [US1] Replace the generic implementation artifact contract in `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [X] T018 [P] [US1] Promote the implementation mode profile and step sequence in `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/modes/implementation.rs`, and `defaults/methods/implementation.toml`
- [X] T019 [US1] Implement implementation gate evaluation and readiness enforcement in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T020 [US1] Wire implementation task mapping, mutation-bounds enforcement, and artifact emission in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/invocation.rs`
- [X] T021 [US1] Update implementation-mode runtime guidance in `README.md`, `MODE_GUIDE.md`, `defaults/embedded-skills/canon-implementation/skill-source.md`, and `.agents/skills/canon-implementation/SKILL.md`
- [X] T022 [US1] Capture implementation validation evidence in `specs/010-controlled-execution-modes/validation-report.md`

**Checkpoint**: `implementation` is independently runnable as a bounded execution mode and publishes a distinct implementation artifact bundle.

---

## Phase 4: User Story 2 - Preservation-First Refactor Execution (Priority: P1)

**Goal**: Deliver `refactor` as a governed structural-improvement mode that preserves externally meaningful behavior and blocks undeclared feature addition.

**Independent Test**: Run `canon run --mode refactor --system-context existing --risk bounded-impact --zone yellow --input canon-input/refactor.md`, verify the run blocks without preserved-behavior inputs or safety-net evidence, verify the preservation artifact bundle on success, and confirm drift/no-feature-addition failures are blocking.

### Validation for User Story 2

- [X] T023 [P] [US2] Add failing refactor artifact-contract and authored-input tests in `tests/contract/refactor_contract.rs` and `tests/direct_runtime_coverage.rs`
- [X] T024 [P] [US2] Add failing refactor preservation and drift tests in `tests/integration/refactor_run.rs` and `tests/integration/refactor_preservation_run.rs`
- [X] T025 [US2] Record refactor preservation, allowed-exception, and no-feature-addition decisions in `specs/010-controlled-execution-modes/decision-log.md`

### Implementation for User Story 2

- [X] T026 [P] [US2] Replace the generic refactor artifact contract in `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [X] T027 [P] [US2] Promote the refactor mode profile and step sequence in `crates/canon-engine/src/domain/mode.rs`, `crates/canon-engine/src/modes/refactor.rs`, and `defaults/methods/refactor.toml`
- [X] T028 [US2] Implement refactor gate evaluation and preservation enforcement in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T029 [US2] Wire refactor scope enforcement, regression evidence, and no-feature-addition emission in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/invocation.rs`
- [X] T030 [US2] Update refactor-mode runtime guidance in `README.md`, `MODE_GUIDE.md`, `defaults/embedded-skills/canon-refactor/skill-source.md`, and `.agents/skills/canon-refactor/SKILL.md`
- [X] T031 [US2] Capture refactor validation evidence in `specs/010-controlled-execution-modes/validation-report.md`

**Checkpoint**: `refactor` is independently runnable with preservation gates, drift blocking, and a distinct refactor artifact bundle.

---

## Phase 5: User Story 3 - Recommendation-Only Fallback for High-Risk Execution (Priority: P2)

**Goal**: Make high-risk or under-evidenced `implementation` and `refactor` requests degrade explicitly to recommendation-only without breaking run lookup, inspect, status, or publish compatibility.

**Independent Test**: Run `implementation` and `refactor` requests in red-zone/systemic-impact or missing-safety-net scenarios, confirm no consequential mutation occurs, confirm `recommendation_only = true` is surfaced through summaries and inspection, and confirm publish still routes through the existing directories.

### Validation for User Story 3

- [X] T032 [P] [US3] Add failing recommendation-only policy and publish tests in `tests/policy_and_traces.rs`, `tests/integration/implementation_run.rs`, and `tests/integration/refactor_run.rs`
- [X] T033 [P] [US3] Add failing status, inspect, and CLI posture tests in `tests/cli_contract.rs`, `tests/invocation_cli_contract.rs`, and `tests/inspect_modes.rs`
- [X] T034 [US3] Record recommendation-only posture, approval boundaries, and publish-label decisions in `specs/010-controlled-execution-modes/decision-log.md`

### Implementation for User Story 3

- [X] T035 [P] [US3] Add implementation/refactor constraint profiles and risk-zone fallbacks in `defaults/policies/adapters.toml` and `crates/canon-engine/src/orchestrator/invocation.rs`
- [X] T036 [US3] Surface recommendation-only posture through status, inspect, and evidence summaries in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-cli/src/output.rs`
- [X] T037 [US3] Preserve publish and run-lookup compatibility for recommendation-only execution in `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/persistence/manifests.rs`, and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T038 [US3] Capture recommendation-only validation evidence in `specs/010-controlled-execution-modes/validation-report.md`

**Checkpoint**: High-risk or under-evidenced execution-heavy runs remain explicitly recommendation-only while all existing lookup and publish surfaces continue to work.

---

## Final Phase: Verification & Compliance

- [X] T039 [P] Update shared runtime-hint and validator surfaces for promoted modes in `defaults/embedded-skills/canon-shared/scripts/check-runtime.sh`, `defaults/embedded-skills/canon-shared/scripts/check-runtime.ps1`, `.agents/skills/canon-shared/scripts/check-runtime.sh`, `.agents/skills/canon-shared/scripts/check-runtime.ps1`, `defaults/embedded-skills/canon-shared/references/skill-index.md`, and `scripts/validate-canon-skills.sh`
- [X] T040 [P] Update non-regression mode/profile coverage in `tests/integration/mode_profiles.rs`, `tests/contract/inspect_modes.rs`, and `tests/direct_runtime_coverage.rs`
- [X] T041 [P] Run structural validation with `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `scripts/validate-canon-skills.sh`; record results in `specs/010-controlled-execution-modes/validation-report.md`
- [X] T042 [P] Run logical validation with `cargo test`, `cargo nextest run`, and targeted implementation/refactor suites; record results in `specs/010-controlled-execution-modes/validation-report.md`
- [X] T043 [P] Run quickstart and publish walk-through validation with `canon run`, `canon inspect`, and `canon publish`; record results in `specs/010-controlled-execution-modes/quickstart.md` and `specs/010-controlled-execution-modes/validation-report.md`
- [ ] T044 Perform independent review of artifact distinctness, recommendation-only labeling, and backward compatibility in `specs/010-controlled-execution-modes/validation-report.md`
- [X] T045 Confirm invariants, changed surfaces, and evidence links in `specs/010-controlled-execution-modes/validation-report.md` and `specs/010-controlled-execution-modes/decision-log.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: no dependencies; must complete first.
- **Phase 1: Setup**: depends on Phase 0.
- **Phase 2: Foundational**: depends on Phase 1 and blocks all user stories.
- **Phase 3: US1**: depends on Phase 2.
- **Phase 4: US2**: depends on Phase 2 and should merge after US1 establishes the shared execution contract pattern in runtime artifacts and gatekeeping.
- **Phase 5: US3**: depends on US1 and US2 because recommendation-only posture must apply consistently across both promoted modes.
- **Final Phase**: depends on every desired user story being complete.

### User Story Completion Order

- **US1 (P1)**: MVP. Establishes bounded `implementation` execution with explicit mutation bounds and task mapping.
- **US2 (P1)**: Adds preservation-first `refactor` execution with distinct drift and no-feature-addition semantics.
- **US3 (P2)**: Adds recommendation-only fallback and backward-compatible status/inspect/publish behavior for high-risk execution.

### Within Each User Story

- Validation tasks must land before the implementation they constrain.
- Decision-log updates must happen before the affected runtime behavior is declared complete.
- Artifact contracts and gate logic must exist before docs and skill promotion claim the mode is runnable.
- Evidence capture is required before a story reaches its checkpoint.

---

## Parallel Examples

### User Story 1

```bash
# Validation can start in parallel:
T014 Add failing implementation contract tests
T015 Add failing implementation lifecycle tests

# After the decisions are recorded, different-file runtime work can proceed in parallel:
T017 Replace the implementation artifact contract
T018 Promote the implementation mode profile and step sequence
```

### User Story 2

```bash
# Preservation and drift coverage can start in parallel:
T023 Add failing refactor contract tests
T024 Add failing refactor preservation and drift tests

# Once the preservation rules are recorded, different-file runtime work can proceed in parallel:
T026 Replace the refactor artifact contract
T027 Promote the refactor mode profile and step sequence
```

### User Story 3

```bash
# Policy and CLI coverage can start in parallel:
T032 Add failing recommendation-only policy and publish tests
T033 Add failing status, inspect, and CLI posture tests

# After posture decisions are recorded, these can proceed in parallel:
T035 Add implementation/refactor constraint profiles and fallbacks
T037 Preserve publish and run-lookup compatibility
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 0, Phase 1, and Phase 2.
2. Deliver US1 and validate bounded `implementation` execution end to end.
3. Stop and confirm the emitted implementation artifact bundle, task mapping, and mutation-bounds behavior before promoting `refactor`.

### Incremental Delivery

1. US1 establishes bounded `implementation` execution.
2. US2 adds preservation-first `refactor` execution.
3. US3 adds explicit recommendation-only fallback and compatible publish/inspect summaries.
4. Final verification updates shared hints, validates skills, and closes the evidence package.

### Parallel Team Strategy

1. One engineer handles shared execution metadata, context persistence, and invocation constraints in `domain/`, `persistence/`, and `orchestrator/invocation.rs`.
2. One engineer handles `implementation` contracts, gates, and integration tests.
3. One engineer handles `refactor` contracts, gates, and integration tests.
4. A docs/skills engineer updates runtime hints, skill wrappers, validators, and public docs once the runtime paths are stable.

---

## Notes

- Total tasks: 45
- User-story task counts: US1 = 9, US2 = 9, US3 = 7
- Parallel opportunities identified: setup scaffolds, foundational plumbing, validation-first story work, artifact-contract versus mode-profile updates, and final verification commands
- Suggested MVP scope: Phase 0 through Phase 3 (US1) only
- Independent test criteria:
  - US1: bounded `implementation` run blocks without bounds and succeeds with the implementation artifact bundle
  - US2: `refactor` run blocks without preservation evidence and succeeds with the refactor artifact bundle
  - US3: red-zone or under-evidenced execution remains recommendation-only while lookup, inspect, and publish continue to work
- Validation evidence paths: `specs/010-controlled-execution-modes/decision-log.md`, `specs/010-controlled-execution-modes/validation-report.md`, `specs/010-controlled-execution-modes/contracts/implementation-execution-contract.md`, `specs/010-controlled-execution-modes/contracts/refactor-execution-contract.md`, and `specs/010-controlled-execution-modes/quickstart.md`
- Format validation target: every task above uses a checkbox, sequential task ID, optional `[P]`, required story label for user-story tasks, and exact file paths