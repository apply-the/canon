# Tasks: Analysis Mode Expansion

**Input**: Design documents from `specs/006-analysis-expansion/`  
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md

**Validation**: Layered validation is mandatory. Executable test tasks verify
behavior, interfaces, and regressions. Independent review and evidence-capture
tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent
implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Execution mode set (`discovery`, `system-shaping`, `architecture`), risk
  (`bounded-impact`), scope, and invariants are recorded in `plan.md` §1.
- No implementation task appears before its authorizing governance artifact.
- Every user story includes validation tasks and evidence capture.
- Bounded-impact work in green/yellow does not require separate approval checkpoints; red-zone work still uses the existing Risk gate approval path.

---

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that permit implementation to start

- [X] T001 Verify execution mode set, risk classification, scope boundaries, and invariants are current in `specs/006-analysis-expansion/plan.md` §1 Governance Context
- [X] T002 Verify decision log entries PD-001 through PD-006 are current in `specs/006-analysis-expansion/decision-log.md`
- [X] T003 Verify validation report scaffold covers all planned validation checks in `specs/006-analysis-expansion/validation-report.md`

**Checkpoint**: Governance artifacts are current and authorize implementation

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Metadata alignment that unblocks all three modes

- [X] T004 [P] Update `defaults/methods/discovery.toml`: set `implementation_depth = "full"`, replace artifact names with spec-authoritative names (`problem-map.md`, `unknowns-and-assumptions.md`, `context-boundary.md`, `exploration-options.md`, `decision-pressure-points.md`)
- [X] T005 [P] Update `defaults/methods/system-shaping.toml`: set `implementation_depth = "full"`, replace artifact names with spec-authoritative names (`system-shape.md`, `architecture-outline.md`, `capability-map.md`, `delivery-options.md`, `risk-hotspots.md`)
- [X] T006 [P] Update `defaults/methods/architecture.toml`: set `implementation_depth = "full"`, replace artifact names with spec-authoritative names (`architecture-decisions.md`, `invariants.md`, `tradeoff-matrix.md`, `boundary-map.md`, `readiness-assessment.md`)
- [X] T007 Update `artifact_families` for `Discovery`, `System-Shaping`, and `Architecture` `ModeProfile` entries in `crates/canon-engine/src/domain/mode.rs` to use the spec-authoritative display names and set `implementation_depth` to `Full`

**Checkpoint**: Methods TOML and mode profiles are aligned with the spec

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core engine plumbing that MUST be complete before any user story
can deliver a runnable mode

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T008 Add `Mode::Discovery` match arm to `contract_for_mode()` in `crates/canon-engine/src/artifacts/contract.rs` — 5 `ArtifactRequirement` entries with required sections and gate bindings per spec Artifact Contracts table (Discovery)
- [X] T009 Add `Mode::SystemShaping` match arm to `contract_for_mode()` in `crates/canon-engine/src/artifacts/contract.rs` — 5 `ArtifactRequirement` entries with required sections and gate bindings per spec Artifact Contracts table (System-Shaping)
- [X] T010 Add `Mode::Architecture` match arm to `contract_for_mode()` in `crates/canon-engine/src/artifacts/contract.rs` — 5 `ArtifactRequirement` entries with required sections and gate bindings per spec Artifact Contracts table (Architecture)
- [X] T011 Add `DiscoveryGateContext` struct to `crates/canon-engine/src/orchestrator/gatekeeper.rs` with fields: `owner`, `risk`, `zone`, `approvals`, `evidence_complete`
- [X] T012 Add `SystemShapingGateContext` struct to `crates/canon-engine/src/orchestrator/gatekeeper.rs` with fields: `owner`, `risk`, `zone`, `approvals`, `evidence_complete`
- [X] T013 Add `ArchitectureGateContext` struct to `crates/canon-engine/src/orchestrator/gatekeeper.rs` with fields: `owner`, `risk`, `zone`, `approvals`, `evidence_complete`
- [X] T014 Implement `evaluate_discovery_gates()` in `crates/canon-engine/src/orchestrator/gatekeeper.rs` returning `[Exploration, Risk, ReleaseReadiness]` — Exploration gate checks `problem-map.md` + `context-boundary.md` for problem domain boundedness
- [X] T015 Implement `evaluate_system_shaping_gates()` in `crates/canon-engine/src/orchestrator/gatekeeper.rs` returning `[Exploration, Architecture, Risk, ReleaseReadiness]` — Architecture gate checks `system-shape.md`, `architecture-outline.md`, `capability-map.md`
- [X] T016 Implement `evaluate_architecture_gates()` in `crates/canon-engine/src/orchestrator/gatekeeper.rs` returning `[Exploration, Architecture, Risk, ReleaseReadiness]` — Architecture gate checks `architecture-decisions.md`, `invariants.md`, `tradeoff-matrix.md`
- [X] T017 [P] Expand `crates/canon-engine/src/modes/discovery.rs` from stub: add `STEP_SEQUENCE`, `REQUIRED_GATES` (`[Exploration, Risk, ReleaseReadiness]`), `GOVERNED_CAPABILITIES` (`[ReadRepository, GenerateContent, CritiqueContent]`)
- [X] T018 [P] Expand `crates/canon-engine/src/modes/system_shaping.rs` from stub: add `STEP_SEQUENCE`, `REQUIRED_GATES` (`[Exploration, Architecture, Risk, ReleaseReadiness]`), `GOVERNED_CAPABILITIES` (`[ReadRepository, GenerateContent, CritiqueContent]`)
- [X] T019 [P] Expand `crates/canon-engine/src/modes/architecture.rs` from stub: add `STEP_SEQUENCE`, `REQUIRED_GATES` (`[Exploration, Architecture, Risk, ReleaseReadiness]`), `GOVERNED_CAPABILITIES` (`[ReadRepository, GenerateContent, CritiqueContent]`)
- [X] T020 Run `cargo check --workspace` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` to confirm all foundational additions compile cleanly

**Checkpoint**: Foundation ready — artifact contracts, gate contexts, gate evaluation functions, and mode constants are in place

---

## Phase 3: User Story 1 — Problem Exploration (discovery) (Priority: P1) 🎯 MVP

**Goal**: Users can run `canon run --mode discovery` end-to-end with governed
artifact emission, gate evaluation, and evidence persistence.

**Independent Test**: Execute `canon run --mode discovery` with partial context.
The system emits 5 discovery artifacts, persists gates and evidence, and
supports the full inspect/approve/resume lifecycle.

### Validation for US1 (MANDATORY)

- [X] T021 [P] [US1] Write contract test in `tests/discovery_contract.rs`: verify `contract_for_mode(Mode::Discovery)` returns 5 `ArtifactRequirement` entries with correct file names, required sections, and gate bindings matching the spec
- [X] T022 [P] [US1] Write contract test in `tests/discovery_contract.rs`: verify `evaluate_discovery_gates()` returns `Blocked` when `problem-map.md` or `context-boundary.md` are missing (Exploration gate blocks on unbounded problem domain)
- [X] T023 [P] [US1] Write contract test in `tests/discovery_contract.rs`: verify `evaluate_discovery_gates()` returns `NeedsApproval` when risk is `SystemicImpact` in `Red` zone
- [X] T024 [P] [US1] Write contract test in `tests/discovery_contract.rs`: verify discovery run completes without critique invocation (critique is optional per PD-003)

### Implementation for US1

- [X] T025 [US1] Implement `render_discovery_artifact()` in `crates/canon-engine/src/orchestrator/service.rs` — renders evidence-backed content for each of the 5 discovery artifacts with their required section headers
- [X] T026 [US1] Implement `run_discovery()` in `crates/canon-engine/src/orchestrator/service.rs` following the common runtime flow: context capture → generation → optional critique → build paths → render artifacts → evaluate gates → derive state → build evidence → persist
- [X] T027 [US1] Add `Mode::Discovery => self.run_discovery(&store, request, policy_set)` dispatch arm to the `run()` match in `crates/canon-engine/src/orchestrator/service.rs`
- [X] T028 [US1] Write integration test in `tests/discovery_run.rs`: end-to-end `canon run --mode discovery` emits 5 artifacts under `.canon/artifacts/{run_id}/discovery/`, persists `run.toml`, `state.toml`, `artifact-contract.toml`, gates under `gates/`, and `evidence.toml`
- [X] T029 [US1] Write integration test in `tests/discovery_run.rs`: verify `canon status --run {run_id}` returns correct state after a discovery run
- [X] T030 [US1] Capture US1 validation evidence (test results, artifact inspection) in `specs/006-analysis-expansion/validation-report.md`

**Checkpoint**: Discovery mode is fully functional and independently validated

---

## Phase 4: User Story 2 — New-System Shaping (`system-shaping`) (Priority: P2)

**Goal**: Users can run `canon run --mode system-shaping` end-to-end with mandatory
critique, governed artifact emission, gate evaluation, and evidence persistence.

**Independent Test**: Execute a `system-shaping` run from bounded requirements.
Canon creates system shape artifacts passing governed critique before emission.

### Validation for US2 (MANDATORY)

- [X] T031 [P] [US2] Write contract test in `tests/system_shaping_contract.rs`: verify `contract_for_mode(Mode::SystemShaping)` returns 5 `ArtifactRequirement` entries with correct file names, required sections, and gate bindings matching the spec
- [X] T032 [P] [US2] Write contract test in `tests/system_shaping_contract.rs`: verify `evaluate_system_shaping_gates()` returns `Blocked` when Architecture gate artifacts are missing (`system-shape.md`, `architecture-outline.md`, `capability-map.md`)
- [X] T033 [P] [US2] Write contract test in `tests/system_shaping_contract.rs`: verify system-shaping run includes mandatory critique evidence in the evidence bundle

### Implementation for US2

- [X] T034 [US2] Implement `render_system_shaping_artifact()` in `crates/canon-engine/src/orchestrator/service.rs` — renders evidence-backed content for each of the 5 system-shaping artifacts with their required section headers
- [X] T035 [US2] Implement `run_system_shaping()` in `crates/canon-engine/src/orchestrator/service.rs` following the common runtime flow with mandatory critique: context capture → generation → critique (mandatory) → build paths → render artifacts → evaluate gates → derive state → build evidence → persist
- [X] T036 [US2] Add `Mode::SystemShaping => self.run_system_shaping(&store, request, policy_set)` dispatch arm to the `run()` match in `crates/canon-engine/src/orchestrator/service.rs`
- [X] T037 [US2] Write integration test in `tests/system_shaping_run.rs`: end-to-end `canon run --mode system-shaping` emits 5 artifacts under `.canon/artifacts/{run_id}/system-shaping/`, persists run manifests, gates, and evidence
- [X] T038 [US2] Write integration test in `tests/system_shaping_run.rs`: verify Architecture gate blocks when insufficient context is supplied
- [X] T039 [US2] Capture US2 validation evidence in `specs/006-analysis-expansion/validation-report.md`

**Checkpoint**: System-shaping mode is fully functional and independently validated

---

## Phase 5: User Story 3 — Architecture Decision Flow (architecture) (Priority: P2)

**Goal**: Users can run `canon run --mode architecture` end-to-end with
mandatory severe critique, governed artifact emission, gate evaluation,
evidence persistence, and approval gating for systemic-impact runs.

**Independent Test**: Provide multiple architecture options. Canon performs
governed challenge and emits `architecture-decisions.md` and
`tradeoff-matrix.md` traceable to critique evidence.

### Validation for US3 (MANDATORY)

- [X] T040 [P] [US3] Write contract test in `tests/architecture_contract.rs`: verify `contract_for_mode(Mode::Architecture)` returns 5 `ArtifactRequirement` entries with correct file names, required sections, and gate bindings matching the spec
- [X] T041 [P] [US3] Write contract test in `tests/architecture_contract.rs`: verify `evaluate_architecture_gates()` returns `Blocked` when Architecture gate artifacts are missing (`architecture-decisions.md`, `invariants.md`, `tradeoff-matrix.md`)
- [X] T042 [P] [US3] Write contract test in `tests/architecture_contract.rs`: verify `evaluate_architecture_gates()` returns `NeedsApproval` for both systemic-impact architecture runs and bounded-impact red-zone architecture runs, proving Risk gate `SystemicImpact OR Red zone` approval semantics

### Implementation for US3

- [X] T043 [US3] Implement `render_architecture_artifact()` in `crates/canon-engine/src/orchestrator/service.rs` — renders evidence-backed content for each of the 5 architecture artifacts with their required section headers
- [X] T044 [US3] Implement `run_architecture()` in `crates/canon-engine/src/orchestrator/service.rs` following the common runtime flow with mandatory severe critique: context capture → generation → critique (mandatory, severe) → build paths → render artifacts → evaluate gates → derive state → build evidence → persist
- [X] T045 [US3] Add `Mode::Architecture => self.run_architecture(&store, request, policy_set)` dispatch arm to the `run()` match in `crates/canon-engine/src/orchestrator/service.rs`
- [X] T046 [US3] Write integration test in `tests/architecture_run.rs`: end-to-end `canon run --mode architecture` emits 5 artifacts under `.canon/artifacts/{run_id}/architecture/`, persists run manifests, gates, and evidence
- [X] T047 [US3] Write integration test in `tests/architecture_run.rs`: verify both systemic-impact architecture runs and bounded-impact red-zone architecture runs enter `AwaitingApproval` state with exit code 3
- [X] T048 [US3] Capture US3 validation evidence in `specs/006-analysis-expansion/validation-report.md`

**Checkpoint**: Architecture mode is fully functional and independently validated

---

## Phase 6: Skill Transition

**Purpose**: Update embedded skills and Codex-facing skills to reflect full
runtime support. Skills MUST NOT claim runnability until the runtime is verified.

- [X] T049 [P] Rewrite `defaults/embedded-skills/canon-discovery/skill-source.md` from modeled-only disclosure to runnable wrapper following `canon-requirements` pattern — include `canon run --mode discovery` invocation and full run/inspect/approve/resume lifecycle
- [X] T050 [P] Rewrite `defaults/embedded-skills/canon-system-shaping/skill-source.md` from modeled-only disclosure to runnable wrapper following `canon-requirements` pattern — include `canon run --mode system-shaping` invocation and full run/inspect/approve/resume lifecycle
- [X] T051 [P] Rewrite `defaults/embedded-skills/canon-architecture/skill-source.md` from modeled-only disclosure to runnable wrapper following `canon-requirements` pattern — include `canon run --mode architecture` invocation and full run/inspect/approve/resume lifecycle
- [X] T052 [P] Update `.agents/skills/canon-discovery/SKILL.md` description to reflect full support state and add runnable workflow instructions
- [X] T053 [P] Update `.agents/skills/canon-system-shaping/SKILL.md` description to reflect full support state and add runnable workflow instructions
- [X] T054 [P] Update `.agents/skills/canon-architecture/SKILL.md` description to reflect full support state and add runnable workflow instructions

**Checkpoint**: Skills truthfully expose all three modes as runnable

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, regression, and closeout

- [X] T055 Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` — record results in `specs/006-analysis-expansion/validation-report.md`
- [x] T056 Run `cargo test` and `cargo nextest run`, then generate and record an overall coverage report in `specs/006-analysis-expansion/validation-report.md` showing total coverage is at least 85% and no regressions were introduced in `requirements`, `brownfield-change`, or `pr-review`
- [X] T057 Run `cargo deny check licenses advisories bans sources` — record results in `specs/006-analysis-expansion/validation-report.md`
- [X] T058 [P] Verify artifact name consistency: confirm artifact file names match across `contract.rs`, `mode.rs` artifact_families, `defaults/methods/*.toml`, and spec artifact contracts tables
- [X] T059 [P] Verify gate profile consistency: confirm gate_profile in `mode.rs` matches gates evaluated in each gatekeeper function and `REQUIRED_GATES` in mode files
- [X] T060 Perform independent review: compare generated artifacts from a test run of each mode against the spec artifact contracts to verify content is evidence-derived and not generic boilerplate — record findings in `specs/006-analysis-expansion/validation-report.md`
- [x] T061 Confirm invariants still hold (analysis-only, no second orchestrator, evidence persists, existing modes unbroken) and close `specs/006-analysis-expansion/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Phase 1 completion. BLOCKS all user stories.
- **User Stories (Phases 3-5)**: Depend on Phase 2 completion.
- **Skill Transition (Phase 6)**: Depends on Phases 3-5 completion (runtime must be verified before skills claim runnability).
- **Verification & Compliance (Final Phase)**: Depends on all prior phases.

### User Story Dependencies

- **US1 — Discovery (P1)**: Can start after Phase 2. No dependencies on other stories.
- **US2 — System-Shaping (P2)**: Can start after Phase 2. No dependencies on US1.
- **US3 — Architecture (P2)**: Can start after Phase 2. No dependencies on US1 or US2.

All three user stories are structurally independent — they modify different
mode files, add different gate evaluation functions, and create separate test
files. They CAN run in parallel after Phase 2 if staffing allows.

### Within Each User Story

- Contract tests (validation) MUST be written before orchestration implementation
- Artifact rendering MUST be implemented before orchestration method
- Orchestration method MUST be implemented before dispatch arm
- Integration tests MUST run after dispatch arm is added
- Evidence capture MUST happen after integration tests pass

---

## Parallel Opportunities

### Phase 1 (all parallel)

```
T004: Update discovery.toml
T005: Update system-shaping.toml    (parallel — different files)
T006: Update architecture.toml
```

### Phase 2 (partially parallel)

```
T008-T010: Artifact contracts        (sequential — same file contract.rs)
T011-T013: Gate context structs      (sequential or single-batch edit — same file gatekeeper.rs)
T017-T019: Mode file expansions      (parallel — different files)
T014-T016: Gate evaluation functions  (after T011-T013; sequential — depend on struct definitions)
```

### Phases 3-5 (fully parallel across stories)

```
Story 1 — discovery:    T021-T030    (parallel with US2 and US3)
Story 2 — system-shaping: T031-T039  (parallel with US1 and US3)
Story 3 — architecture: T040-T048    (parallel with US1 and US2)
```

### Phase 6 (all parallel)

```
T049-T054: All skill rewrites        (parallel — different files)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts
2. Complete Phase 1: Setup
3. Complete Phase 2: Foundational
4. Complete Phase 3: User Story 1 — Discovery
5. **STOP AND VALIDATE**: Confirm discovery run works end-to-end and update `validation-report.md`

### Incremental Delivery

1. Complete Governance + Setup + Foundational
2. Add Discovery (US1) → Validate independently → `canon run --mode discovery` works
3. Add System-Shaping (US2) → Validate independently → `canon run --mode system-shaping` works
4. Add Architecture (US3) → Validate independently → `canon run --mode architecture` works
5. Update skills (Phase 6) → Only after all three modes are verified
6. Finish with Verification & Compliance

### Parallel Team Strategy

With multiple developers after Phase 2:

1. Developer A: US1 — Discovery (T021-T030)
2. Developer B: US2 — System-Shaping (T031-T039)
3. Developer C: US3 — Architecture (T040-T048)
4. Each story closes only after its evidence is recorded in `validation-report.md`

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- Each user story is independently completable and validated
- Contract/validation tests MUST be written before orchestration implementation
- The decision log and validation report must stay current as work progresses
- Critique is optional for discovery (PD-003) but mandatory for system-shaping and architecture
- Approval is required when risk is `SystemicImpact` or zone is `Red`; US3 validation explicitly proves that `OR` semantics (PD-004)
- Skill updates in Phase 6 MUST NOT happen before the runtime is verified — fabricating runnable claims violates the skill invariant
