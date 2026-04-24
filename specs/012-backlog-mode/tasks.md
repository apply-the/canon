# Tasks: Backlog Mode (Delivery Decomposition)

**Input**: Design documents from `/specs/012-backlog-mode/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/backlog-authored-input-contract.md`, `contracts/backlog-packet-contract.md`, `quickstart.md`

**Validation**: Layered validation is mandatory. Runtime, CLI, packet, documentation, skill, and publish evidence must be captured in `specs/012-backlog-mode/validation-report.md`.

**Organization**: Tasks are grouped by user story so successful backlog decomposition, closure-blocked behavior, and downstream handoff can be implemented and validated incrementally.

## Format: `[ID] [P?] [Story] Description`

## Constitution Alignment

- Governance artifacts stay current before implementation begins.
- No backlog runtime change lands before the artifacts that authorize closure gating, traceability, and validation ownership.
- Every user story includes executable validation tasks and closes with evidence capture.
- Independent review remains separate from generation and closes in the final verification phase.

## Phase 0: Governance & Artifacts

- [X] T001 Reconfirm execution mode, risk classification, scope boundaries, and invariants in `specs/012-backlog-mode/spec.md` and `specs/012-backlog-mode/plan.md`
- [X] T002 Refresh the decision and validation scaffolds in `specs/012-backlog-mode/decision-log.md` and `specs/012-backlog-mode/validation-report.md`
- [X] T003 Reconfirm authored-input and packet contracts in `specs/012-backlog-mode/contracts/backlog-authored-input-contract.md`, `specs/012-backlog-mode/contracts/backlog-packet-contract.md`, and `specs/012-backlog-mode/quickstart.md`
- [X] T004 Record independent review checkpoints, publish-path expectations, and evidence owners in `specs/012-backlog-mode/plan.md` and `specs/012-backlog-mode/validation-report.md`

---

## Phase 1: Setup (Shared Infrastructure)

- [X] T005 Create backlog contract and end-to-end test entrypoints in `tests/backlog_contract.rs` and `tests/backlog_run.rs`
- [X] T006 [P] Create backlog closure and evidence test entrypoints in `tests/backlog_closure_run.rs`, `tests/runtime_evidence_contract.rs`, and `tests/run_lookup.rs`
- [X] T007 [P] Scaffold backlog mode, method default, and skill files in `crates/canon-engine/src/modes/backlog.rs`, `defaults/methods/backlog.toml`, `defaults/embedded-skills/canon-backlog/skill-source.md`, and `.agents/skills/canon-backlog/SKILL.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared runtime plumbing that must exist before any backlog story can be delivered safely.

**⚠️ CRITICAL**: No user story work starts before these tasks are complete.

- [X] T008 Extend mode taxonomy and profile registration in `crates/canon-engine/src/domain/mode.rs` and `crates/canon-engine/src/modes.rs`
- [X] T009 [P] Add canonical backlog input auto-binding and CLI parsing coverage in `crates/canon-cli/src/app.rs` and `tests/cli_contract.rs`
- [X] T010 [P] Add backlog publish-directory and mode-visibility scaffolding in `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, and `tests/inspect_modes.rs`
- [X] T011 [P] Add backlog method defaults and gate-profile scaffolding in `defaults/methods/backlog.toml`, `defaults/policies/gates.toml`, and `crates/canon-engine/src/domain/mode.rs`
- [X] T012 [P] Extend persisted run context for backlog planning metadata in `crates/canon-engine/src/domain/run.rs`, `crates/canon-engine/src/persistence/manifests.rs`, and `crates/canon-engine/src/persistence/store.rs`
- [X] T013 [P] Add backlog orchestrator module scaffolding in `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/mode_backlog.rs`, and `crates/canon-engine/src/orchestrator/gatekeeper.rs`
- [X] T014 [P] Add backlog artifact and summary scaffolding in `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, and `crates/canon-engine/src/orchestrator/service/summarizers.rs`

**Checkpoint**: Backlog exists as a named mode with canonical input binding, publish routing, artifact scaffolding, and persisted planning context before any story-specific behavior is implemented.

---

## Phase 3: User Story 1 - Delivery Lead Produces a Governed Backlog Packet (Priority: P1) 🎯 MVP

**Goal**: Deliver `backlog` as a first-class mode that produces the full bounded planning packet from credible upstream inputs.

**Independent Test**: Run `canon run --mode backlog --system-context existing --risk bounded-impact --zone yellow --input canon-input/backlog`, then verify the run emits `backlog-overview.md`, `epic-tree.md`, `capability-to-epic-map.md`, `dependency-map.md`, `delivery-slices.md`, `sequencing-plan.md`, `acceptance-anchors.md`, and `planning-risks.md` without descending into task-level output.

### Validation for User Story 1

- [X] T015 [P] [US1] Add failing full-packet artifact contract assertions in `tests/backlog_contract.rs` and `tests/direct_runtime_coverage.rs`
- [X] T016 [P] [US1] Add failing successful-run lifecycle and CLI assertions in `tests/backlog_run.rs`, `tests/cli_contract.rs`, and `tests/invocation_cli_contract.rs`
- [X] T017 [US1] Record full-packet completeness, granularity, and authored-input decisions in `specs/012-backlog-mode/decision-log.md`

### Implementation for User Story 1

- [X] T018 [P] [US1] Implement backlog mode step sequence and successful-run profile in `crates/canon-engine/src/modes/backlog.rs`, `crates/canon-engine/src/domain/mode.rs`, and `defaults/methods/backlog.toml`
- [X] T019 [P] [US1] Implement full backlog artifact contract and markdown renderers in `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [X] T020 [US1] Implement authored-input loading and successful packet generation in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/service/mode_backlog.rs`
- [X] T021 [US1] Persist backlog planning context, source references, and authored priorities in `crates/canon-engine/src/domain/run.rs`, `crates/canon-engine/src/persistence/manifests.rs`, and `crates/canon-engine/src/persistence/store.rs`
- [X] T022 [US1] Surface full-packet summaries and primary-artifact actions in `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, and `crates/canon-cli/src/output.rs`
- [X] T023 [US1] Capture successful bounded backlog evidence in `specs/012-backlog-mode/validation-report.md`

**Checkpoint**: `backlog` is independently runnable as a full-packet planning mode and emits a distinct backlog artifact bundle when source inputs are credible.

---

## Phase 4: User Story 2 - Planner Is Blocked When Source Architecture Is Too Vague (Priority: P1)

**Goal**: Make backlog runs block or downgrade explicitly when source architecture is too vague, contradictory, or insufficiently bounded for credible decomposition.

**Independent Test**: Run `canon run --mode backlog ...` against a backlog packet with unresolved capabilities, contradictory priorities, or missing dependency boundaries, then verify the run blocks or downgrades with explicit closure findings and does not emit a misleading full decomposition packet.

### Validation for User Story 2

- [X] T024 [P] [US2] Add failing closure-assessment and risk-only packet assertions in `tests/backlog_contract.rs`, `tests/backlog_closure_run.rs`, and `tests/runtime_evidence_contract.rs`
- [X] T025 [P] [US2] Add failing blocked-or-downgraded summary and inspection assertions in `tests/backlog_closure_run.rs`, `tests/inspect_modes.rs`, and `tests/policy_and_traces.rs`
- [X] T026 [US2] Record closure-assessment, downgrade, and risk-only packet decisions in `specs/012-backlog-mode/decision-log.md`

### Implementation for User Story 2

- [X] T027 [P] [US2] Implement closure-assessment models and persisted findings in `crates/canon-engine/src/domain/run.rs`, `crates/canon-engine/src/persistence/manifests.rs`, and `crates/canon-engine/src/persistence/store.rs`
- [X] T028 [US2] Implement closure gating and downgrade semantics in `crates/canon-engine/src/orchestrator/gatekeeper.rs`, `crates/canon-engine/src/orchestrator/service.rs`, and `crates/canon-engine/src/orchestrator/service/mode_backlog.rs`
- [X] T029 [US2] Implement risk-only packet emission and closure-aware artifact selection in `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, and `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [X] T030 [US2] Surface closure findings through status, inspect, and evidence views in `crates/canon-engine/src/orchestrator/service/inspect.rs`, `crates/canon-engine/src/orchestrator/service.rs`, and `crates/canon-cli/src/output.rs`
- [X] T031 [US2] Capture closure-blocked backlog evidence and walkthrough notes in `specs/012-backlog-mode/validation-report.md` and `specs/012-backlog-mode/quickstart.md`

**Checkpoint**: Closure-limited backlog runs remain explicit, durable, and non-misleading, with risk-focused output instead of false decomposition precision.

---

## Phase 5: User Story 3 - Implementation Lead Reuses Backlog Slices as Bounded Handoff (Priority: P2)

**Goal**: Keep backlog packets publishable and reusable so downstream implementation work can consume slices and dependencies without losing rationale or source traceability.

**Independent Test**: Publish a completed backlog run, inspect the packet under `docs/planning/<RUN_ID>/`, and confirm a downstream reader can identify source links, dependencies, sequencing context, and acceptance anchors for a selected slice without relying on hidden runtime state.

### Validation for User Story 3

- [X] T032 [P] [US3] Add failing downstream handoff, publish, and lookup assertions in `tests/backlog_run.rs`, `tests/run_lookup.rs`, and `tests/render_next_steps.rs`
- [X] T033 [P] [US3] Add failing mode-availability and skill-surface assertions in `tests/mode_profiles.rs`, `tests/skills_bootstrap.rs`, and `tests/inspect_modes.rs`
- [X] T034 [US3] Record publish-path, downstream handoff, and skill-surface decisions in `specs/012-backlog-mode/decision-log.md`

### Implementation for User Story 3

- [X] T035 [P] [US3] Implement backlog publish routing and lookup compatibility in `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/orchestrator/service.rs`, and `crates/canon-engine/src/domain/mode.rs`
- [X] T036 [US3] Implement downstream handoff trace surfaces in `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, and `crates/canon-engine/src/orchestrator/service/inspect.rs`
- [X] T037 [P] [US3] Update backlog mode guidance in `README.md`, `MODE_GUIDE.md`, and `NEXT_FEATURES.md`
- [X] T038 [P] [US3] Add backlog embedded and materialized skill surfaces in `defaults/embedded-skills/canon-backlog/skill-source.md`, `.agents/skills/canon-backlog/SKILL.md`, and `defaults/embedded-skills/canon-shared/references/skill-index.md`
- [X] T039 [US3] Capture downstream handoff and publish evidence in `specs/012-backlog-mode/validation-report.md`

**Checkpoint**: Backlog packets publish cleanly to `docs/planning/<RUN_ID>/`, remain inspectable through existing surfaces, and preserve enough traceability for later `implementation` work.

---

## Final Phase: Verification & Compliance

- [ ] T040 [P] Run structural validation with `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `bash scripts/validate-canon-skills.sh`; record results in `specs/012-backlog-mode/validation-report.md`
- [ ] T041 [P] Run logical validation with `cargo test`, `cargo nextest run`, and backlog-specific suites; record results in `specs/012-backlog-mode/validation-report.md`
- [ ] T042 [P] Run successful and closure-blocked quickstart walkthroughs with `canon run`, `canon inspect`, and `canon publish`; update `specs/012-backlog-mode/quickstart.md` and `specs/012-backlog-mode/validation-report.md`
- [ ] T043 Perform independent review of full-packet credibility, closure-limited outputs, and downstream handoff readiness in `specs/012-backlog-mode/validation-report.md`
- [ ] T044 Confirm invariants, decision links, and story-level evidence closure in `specs/012-backlog-mode/validation-report.md` and `specs/012-backlog-mode/decision-log.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: no dependencies; must complete first.
- **Phase 1: Setup**: depends on Phase 0.
- **Phase 2: Foundational**: depends on Phase 1 and blocks all user stories.
- **Phase 3: US1**: depends on Phase 2.
- **Phase 4: US2**: depends on Phase 2 and should merge after US1 establishes the successful backlog packet path.
- **Phase 5: US3**: depends on US1 and US2 because downstream handoff relies on both full-packet outputs and closure-aware backlog semantics.
- **Final Phase**: depends on every desired user story being complete.

### User Story Completion Order

- **US1 (P1)**: MVP. Establishes successful backlog packet generation from credible bounded inputs.
- **US2 (P1)**: Adds honest closure gating, downgrade behavior, and risk-only packet handling for non-credible inputs.
- **US3 (P2)**: Adds downstream handoff readiness, publish compatibility, and backlog skill/docs surfaces.

### Within Each User Story

- Validation tasks must land before the implementation they constrain.
- Decision-log updates must happen before affected runtime behavior is declared complete.
- Artifact contracts, gate logic, and persistence changes must exist before docs and skills claim backlog mode is available.
- Evidence capture is required before a story reaches its checkpoint.

---

## Parallel Examples

### User Story 1

```bash
# Validation can start in parallel:
T015 Add failing full-packet artifact contract assertions
T016 Add failing successful-run lifecycle and CLI assertions

# After the decisions are recorded, different-file runtime work can proceed in parallel:
T018 Implement backlog mode step sequence and successful-run profile
T019 Implement full backlog artifact contract and markdown renderers
```

### User Story 2

```bash
# Closure-focused validation can start in parallel:
T024 Add failing closure-assessment and risk-only packet assertions
T025 Add failing blocked-or-downgraded summary and inspection assertions

# Once closure semantics are recorded, different-file work can proceed in parallel:
T027 Implement closure-assessment models and persisted findings
T029 Implement risk-only packet emission and closure-aware artifact selection
```

### User Story 3

```bash
# Publish and availability coverage can start in parallel:
T032 Add failing downstream handoff, publish, and lookup assertions
T033 Add failing mode-availability and skill-surface assertions

# After handoff decisions are recorded, these can proceed in parallel:
T035 Implement backlog publish routing and lookup compatibility
T038 Add backlog embedded and materialized skill surfaces
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 0, Phase 1, and Phase 2.
2. Deliver US1 and validate successful backlog packet generation end to end.
3. Stop and confirm the full eight-artifact packet, traceability surfaces, and non-task granularity before enabling closure downgrade behavior.

### Incremental Delivery

1. US1 establishes backlog as a successful full-packet mode.
2. US2 adds explicit block-or-downgrade behavior for insufficiently closed upstream inputs.
3. US3 adds downstream handoff readiness, publish compatibility, and discoverable skill/docs surfaces.
4. Final verification closes structural, logical, and independent review evidence.

### Parallel Team Strategy

1. One engineer handles mode taxonomy, method defaults, and CLI input binding.
2. One engineer handles backlog artifacts, summaries, and publish routing.
3. One engineer handles closure assessment, persistence, and gatekeeping.
4. A docs/skills engineer updates README, mode guidance, and backlog skill surfaces once runtime behavior is stable.

---

## Notes

- Total tasks: 44
- User-story task counts: US1 = 9, US2 = 8, US3 = 8
- Parallel opportunities identified: test scaffolds, foundational runtime scaffolding, validation-first story work, artifact-contract versus persistence work, and docs/skills versus publish routing in US3
- Suggested MVP scope: Phase 0 through Phase 3 (US1) only
- Independent test criteria:
  - US1: a bounded backlog run emits the full eight-artifact packet without task-level output
  - US2: an insufficiently closed backlog run blocks or downgrades with explicit closure findings and no misleading full packet
  - US3: a published backlog packet under `docs/planning/<RUN_ID>/` preserves source links, dependencies, sequencing context, and acceptance anchors for downstream execution planning
- Validation evidence paths: `specs/012-backlog-mode/decision-log.md`, `specs/012-backlog-mode/validation-report.md`, `specs/012-backlog-mode/contracts/backlog-authored-input-contract.md`, `specs/012-backlog-mode/contracts/backlog-packet-contract.md`, and `specs/012-backlog-mode/quickstart.md`
- Format validation target: every task above uses a checkbox, sequential task ID, optional `[P]`, required story label for user-story tasks, and exact file paths