# Tasks: Governance Adapter Surface For External Orchestrators

**Input**: Design documents from `/specs/035-governance-adapter-surface/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks whenever behavior, interfaces, or regressions must be checked. Independent review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with mode, risk, scope, and invariant artifact tasks.
- No implementation task may appear before the artifacts that authorize it.
- Every user story MUST include validation tasks and evidence capture.
- Systemic-impact work MUST include an independent review task separate from generation.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls, release boundary, and contract artifacts that permit implementation to start

- [x] T001 Set Canon version to `0.35.0` in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-scope decisions and the >95% touched-file coverage gate in `specs/035-governance-adapter-surface/decision-log.md`
- [x] T003 Update planned structural, logical, independent, and coverage checkpoints in `specs/035-governance-adapter-surface/validation-report.md`
- [x] T004 Confirm the machine-facing adapter contract in `specs/035-governance-adapter-surface/contracts/governance-adapter-contract.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and test harness setup

- [x] T005 Update agent context from `specs/035-governance-adapter-surface/plan.md` into `AGENTS.md`
- [x] T006 Create governance adapter test scaffolding in `tests/contract/governance_cli.rs` and `tests/integration/governance_adapter_surface.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: CLI routing, request or response normalization, and shared invariants that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 [P] Add failing CLI routing and schema-compat coverage in `tests/contract/governance_cli.rs`
- [x] T008 [P] Add failing response-invariant coverage for readiness, approval, and canonical refs in `tests/integration/governance_adapter_surface.rs`
- [x] T009 [P] Add failing release-alignment coverage for `0.35.0` runtime compatibility refs in `tests/integration/skills_bootstrap.rs`
- [x] T010 Implement the additive governance CLI namespace in `crates/canon-cli/src/app.rs`, `crates/canon-cli/src/commands.rs`, and `crates/canon-cli/src/commands/governance.rs`
- [x] T011 Implement shared request parsing, response serialization, and invariant normalization in `crates/canon-cli/src/commands/governance.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [x] T012 Capture foundational invariants and the >95% coverage requirement in `specs/035-governance-adapter-surface/validation-report.md`

**Checkpoint**: The governance command surface exists and can normalize machine-facing outcomes before story-specific behavior lands

---

## Phase 3: User Story 1 - Start A Governed Attempt From An External Orchestrator (Priority: P1) 🎯 MVP

**Goal**: Start governed Canon work from a machine-facing command and return blocked or successful domain outcomes without protocol ambiguity

**Independent Test**: A consumer can send a start request and receive either a blocked reason-coded outcome or a successful run reference without reading `.canon/` internals.

### Validation for User Story 1 (MANDATORY)

- [x] T013 [P] [US1] Add failing start-path coverage for blocked missing-context and successful run materialization in `tests/contract/governance_cli.rs`
- [x] T014 [US1] Record start-operation decisions under `## User Story 1 Decisions` in `specs/035-governance-adapter-surface/decision-log.md`

### Implementation for User Story 1

- [x] T015 [US1] Implement start request mapping and blocked-domain `reason_code` handling in `crates/canon-cli/src/commands/governance.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [x] T016 [P] [US1] Implement machine-facing start response projection and exit-code handling in `crates/canon-cli/src/app.rs`, `crates/canon-cli/src/commands.rs`, and `crates/canon-cli/src/commands/governance.rs`
- [x] T017 [US1] Capture start-operation validation evidence in `specs/035-governance-adapter-surface/validation-report.md`

**Checkpoint**: External consumers can start governed work and branch safely on blocked versus successful start outcomes

---

## Phase 4: User Story 2 - Refresh A Governed Attempt And Trust Packet Readiness (Priority: P2)

**Goal**: Refresh an existing governed run and project packet readiness, packet refs, and approval posture without contradictory state

**Independent Test**: A consumer can refresh a governed run and distinguish reusable, incomplete, rejected, approval-gated, and stale-run outcomes from one machine-facing response.

### Validation for User Story 2 (MANDATORY)

- [x] T018 [P] [US2] Add failing refresh-path coverage for reusable, incomplete, rejected, and stale-run outcomes in `tests/contract/governance_cli.rs` and `tests/integration/governance_adapter_surface.rs`
- [x] T019 [US2] Record refresh and packet-projection decisions under `## User Story 2 Decisions` in `specs/035-governance-adapter-surface/decision-log.md`

### Implementation for User Story 2

- [x] T020 [US2] Implement refresh packet projection and stale-run handling in `crates/canon-cli/src/commands/governance.rs`, `crates/canon-engine/src/orchestrator/service.rs`, and `crates/canon-engine/src/persistence/store.rs`
- [x] T021 [P] [US2] Implement strict `governed_ready`, `awaiting_approval`, and canonical workspace-relative ref normalization in `crates/canon-cli/src/commands/governance.rs` and `crates/canon-engine/src/persistence/store.rs`
- [x] T022 [US2] Capture refresh and readiness validation evidence in `specs/035-governance-adapter-surface/validation-report.md`

**Checkpoint**: Refresh responses are idempotent, semantically consistent, and safe for downstream control flow

---

## Phase 5: User Story 3 - Discover Compatibility Before Binding To Canon (Priority: P3)

**Goal**: Publish capabilities, supported schema versions, and exact vocabularies before live governance operations begin

**Independent Test**: A consumer can inspect capabilities and decide whether the current Canon binary satisfies the expected contract without attempting start or refresh.

### Validation for User Story 3 (MANDATORY)

- [x] T023 [P] [US3] Add failing capabilities coverage for schema versions, operations, modes, and exact vocabularies in `tests/contract/governance_cli.rs`
- [x] T024 [US3] Record capabilities and compatibility decisions under `## User Story 3 Decisions` in `specs/035-governance-adapter-surface/decision-log.md`

### Implementation for User Story 3

- [x] T025 [US3] Implement the capabilities response in `crates/canon-cli/src/commands/governance.rs` and `crates/canon-cli/src/app.rs`
- [x] T026 [P] [US3] Align runtime compatibility references in `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml` and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T027 [US3] Capture capabilities validation evidence in `specs/035-governance-adapter-surface/validation-report.md`

**Checkpoint**: Consumers can negotiate compatibility from Canon's published machine-facing metadata

---

## Phase 6: User Story 4 - Ship A Trustworthy Consumer Contract In 0.35.0 (Priority: P4)

**Goal**: Align release-facing docs, changelog, roadmap, and validation evidence around the shipped `0.35.0` adapter surface

**Independent Test**: A maintainer can inspect Canon's release-facing surfaces and validation artifacts and see one coherent `0.35.0` contract story.

### Validation for User Story 4 (MANDATORY)

- [x] T028 [P] [US4] Validate release-alignment expectations in `tests/integration/skills_bootstrap.rs` and `specs/035-governance-adapter-surface/validation-report.md`
- [x] T029 [US4] Record release and closeout decisions under `## User Story 4 Decisions` in `specs/035-governance-adapter-surface/decision-log.md`

### Implementation for User Story 4

- [x] T030 [US4] Update impacted docs and changelog in `README.md`, `docs/guides/modes.md`, and `CHANGELOG.md`
- [x] T031 [US4] Clean roadmap continuity in `ROADMAP.md`
- [x] T032 [US4] Capture docs, roadmap, and release-alignment validation evidence in `specs/035-governance-adapter-surface/validation-report.md`

**Checkpoint**: Runtime and release-facing surfaces agree on the shipped machine-facing governance contract

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, coverage, independent review, and closeout

- [x] T033 [P] Run the targeted governance surface suite for `tests/contract/governance_cli.rs`, `tests/integration/governance_adapter_surface.rs`, and `tests/integration/skills_bootstrap.rs`, then record results in `specs/035-governance-adapter-surface/validation-report.md`
- [x] T034 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and document line coverage above 95% for every modified or newly created Rust source file in `specs/035-governance-adapter-surface/validation-report.md`
- [x] T035 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/035-governance-adapter-surface/validation-report.md`
- [x] T036 [P] Run `cargo nextest run` and record results in `specs/035-governance-adapter-surface/validation-report.md`
- [x] T037 Perform independent review of invariants, contract semantics, consumer smoke evidence, and final diff in `specs/035-governance-adapter-surface/validation-report.md`
- [x] T038 Confirm invariants still hold and close the final validation state in `specs/035-governance-adapter-surface/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the start-operation MVP.
- **User Story 2 (P2)**: Depends on the command surface from Foundational and reuses the request or response normalization seams from US1.
- **User Story 3 (P3)**: Depends on the additive command surface from Foundational but remains independently testable once the namespace exists.
- **User Story 4 (P4)**: Depends on the implemented runtime contract from US1 through US3 so release surfaces describe shipped behavior accurately.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- CLI parsing and contract tests before runtime projection adjustments.
- Runtime normalization before release-facing wording updates.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- Phase 0 tasks after T001 can run in parallel where they touch different planning artifacts.
- T007, T008, and T009 can run in parallel before the shared implementation tasks T010 and T011.
- Within US2, refresh projection and canonical-ref normalization can proceed in parallel once the failing tests exist.
- Final validation tasks T033 through T036 can run in parallel once implementation is stable.

---

## Parallel Example: User Story 2

```bash
# Launch refresh behavior checks in parallel:
Task: "Add failing refresh-path coverage for reusable, incomplete, rejected, and stale-run outcomes in tests/contract/governance_cli.rs and tests/integration/governance_adapter_surface.rs"

# Launch implementation work in parallel after the failing checks exist:
Task: "Implement refresh packet projection and stale-run handling in crates/canon-cli/src/commands/governance.rs, crates/canon-engine/src/orchestrator/service.rs, and crates/canon-engine/src/persistence/store.rs"
Task: "Implement strict governed_ready, awaiting_approval, and canonical workspace-relative ref normalization in crates/canon-cli/src/commands/governance.rs and crates/canon-engine/src/persistence/store.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm a consumer can start governed work and branch correctly on blocked versus successful outcomes.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Add User Story 4 and validate independently.
6. Finish with Verification & Compliance.

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together.
2. Once Foundational is done:
	- Developer A: User Story 1.
	- Developer B: User Story 2.
	- Developer C: User Story 3.
3. User Story 4 starts after runtime contract checkpoints are stable.
4. Each story closes only after its evidence is recorded.

---

## Notes

- [P] tasks = different files, no dependencies.
- [Story] labels map tasks to user stories for traceability.
- `T001` is intentionally the version bump task as requested.
- `T030` is intentionally the impacted docs plus changelog task as requested.
- `T031` is intentionally the roadmap cleanup task.
- `T034` is intentionally the coverage task and carries the >95% touched-file threshold.
- `T035` is intentionally the explicit `cargo fmt` plus `cargo clippy` task.
- Keep the decision log and validation report current as work progresses.