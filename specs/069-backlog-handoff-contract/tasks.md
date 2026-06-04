# Tasks: Backlog Handoff Contract

**Input**: Design documents from `/specs/069-backlog-handoff-contract/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`,
`contracts/backlog-execution-handoff-contract.md`, `quickstart.md`

**Tests**: Test tasks are required. Add or refine focused regressions first,
confirm the relevant assertion fails before changing implementation, then close
the regression with the smallest coherent runtime and artifact change.

**Organization**: Tasks are grouped by user story so stable slice identity,
handoff-availability semantics, and publish/docs/skills alignment can be
validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it targets different files and has no
  dependency on incomplete work
- **[Story]**: Maps a task to a user story for traceability
- Every task includes repository-relative file paths

## Phase 0: Governance & Artifact Alignment

**Purpose**: Lock the implementation evidence plan and sample packet fixtures
before touching runtime behavior.

- [x] T001 Refine implementation evidence targets and reviewer checkpoints in `specs/069-backlog-handoff-contract/decision-log.md` and `specs/069-backlog-handoff-contract/validation-report.md`
- [x] T002 [P] Add sample handoff-capable and handoff-unavailable authored fixtures in `tests/fixtures/backlog-handoff/README.md`

---

## Phase 1: Setup

**Purpose**: Establish failing packet-shape, publish-surface, and skill-surface
regressions before runtime changes.

- [x] T003 [P] Add failing packet-contract regressions for stable `slice_id` propagation and `execution-handoff.md` packet shapes in `tests/backlog_contract.rs`
- [x] T004 [P] Add failing integration regressions for handoff-capable backlog packets in `tests/backlog_run.rs`
- [x] T005 [P] Add failing integration regressions for handoff-unavailable and closure-limited packets in `tests/backlog_closure_run.rs` and `tests/run_lookup.rs`
- [x] T006 [P] Add failing skill and publish-surface regressions for backlog handoff wording in `tests/skills_bootstrap.rs` and `tests/integration/skills_bootstrap.rs`

---

## Phase 2: Foundational

**Purpose**: Add the shared data and contract scaffolding that all user stories
depend on.

**Critical**: Complete this phase before user-story implementation.

- [x] T007 Extend backlog runtime context for stable slice identity, handoff availability, and selected handoff slice metadata in `crates/canon-engine/src/domain/run.rs` and `crates/canon-engine/src/orchestrator/gatekeeper/context.rs`
- [x] T008 Extend backlog artifact contract evaluation for additive handoff semantics in `crates/canon-engine/src/artifacts/contract.rs`
- [x] T009 Add shared rendering helpers for stable slice identifiers and handoff visibility in `crates/canon-engine/src/artifacts/markdown/delivery/backlog.rs`

**Checkpoint**: The runtime has typed scaffolding for stable slice identity and
handoff availability, but no behavior is yet allowed to overclaim readiness.

---

## Phase 3: User Story 1 - Downstream runtime receives a governed handoff slice (Priority: P1)

**Goal**: Emit stable `slice_id` values across backlog packet artifacts and add
`execution-handoff.md` when a slice is truly credible for downstream execution
handoff.

**Independent Test**: Run backlog mode from bounded upstream artifacts and
confirm the resulting packet emits stable slice IDs plus a handoff artifact
naming the first admissible slice and its evidence.

### Implementation

- [x] T010 [US1] Emit stable `slice_id` values across `delivery-slices.md`, `dependency-map.md`, `sequencing-plan.md`, and `acceptance-anchors.md` in `crates/canon-engine/src/artifacts/markdown/delivery/backlog.rs`
- [x] T011 [US1] Implement handoff selection and `execution-handoff.md` generation in `crates/canon-engine/src/orchestrator/service/mode_backlog.rs`
- [x] T012 [US1] Preserve publish and lookup compatibility for handoff-capable packets in `crates/canon-engine/src/orchestrator/publish.rs` and `tests/integration/run_lookup.rs`
- [x] T013 [US1] Run the focused US1 regression set in `tests/backlog_contract.rs` and `tests/backlog_run.rs`

**Checkpoint**: A handoff-capable full backlog packet exposes one governed
execution-admissible slice without drifting into task generation.

---

## Phase 4: User Story 2 - Planner sees explicit handoff unavailability instead of false readiness (Priority: P1)

**Goal**: Preserve honest planning packets when no slice is handoff-ready and
withhold handoff artifacts for closure-limited or contradictory packets.

**Independent Test**: Run backlog mode against closure-limited input and
handoff-unavailable full packets, then confirm Canon withholds the handoff
artifact and explains why.

### Implementation

- [x] T014 [US2] Implement handoff-unavailable versus withheld-for-closure semantics in `crates/canon-engine/src/orchestrator/service/mode_backlog.rs`, `crates/canon-engine/src/artifacts/markdown/delivery/backlog.rs`, and `crates/canon-engine/src/orchestrator/gatekeeper/context.rs`
- [x] T015 [US2] Align CLI and inspect summaries with handoff-availability semantics in `crates/canon-cli/src/output.rs` and `crates/canon-cli/src/app.rs`
- [x] T016 [US2] Run the focused US2 regression set in `tests/backlog_closure_run.rs` and `tests/run_lookup.rs`

**Checkpoint**: Full planning packets and closure-limited packets remain
distinct, and neither can imply downstream execution readiness when evidence is
missing.

---

## Phase 5: User Story 3 - Published and inspect surfaces preserve the handoff contract (Priority: P2)

**Goal**: Keep docs, skills, publish surfaces, and canonical backlog-contract
docs aligned with the delivered handoff behavior.

**Independent Test**: Publish handoff-capable and handoff-unavailable packets
and confirm docs, skills, and validation surfaces distinguish them truthfully
without inventing execution authority.

### Implementation

- [x] T017 [P] [US3] Update operator documentation for backlog handoff behavior in `README.md`, `tech-docs/guides/modes.md`, and `tech-docs/guides/getting-started.md`
- [x] T018 [P] [US3] Update embedded and repo-local backlog skills in `defaults/embedded-skills/canon-backlog/skill-source.md` and `.agents/skills/canon-backlog/SKILL.md`
- [x] T019 [US3] Reconcile canonical backlog-contract artifacts with the new handoff contract in `specs/012-backlog-mode/contracts/backlog-packet-contract.md`, `specs/012-backlog-mode/decision-log.md`, and `specs/012-backlog-mode/validation-report.md`
- [x] T020 [US3] Run surface validation with `tests/skills_bootstrap.rs`, `tests/integration/skills_bootstrap.rs`, and `bash scripts/validate-canon-skills.sh`

**Checkpoint**: Human-facing and downstream-facing surfaces describe the same
handoff truth as the runtime.

---

## Final Phase: Verification & Compliance

**Purpose**: Close formatting, linting, regression, and evidence requirements.

- [x] T021 Run `cargo fmt` and verify formatting with `cargo fmt --check`
- [x] T022 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and fix every reported issue
- [x] T023 Run focused regressions with `cargo test --test backlog_contract --test backlog_run --test backlog_closure_run --test run_lookup --test skills_bootstrap`
- [x] T024 Run `cargo nextest run`
- [x] T025 Record validation evidence, sample packet links, and independent review notes in `specs/069-backlog-handoff-contract/validation-report.md`

---

## Dependencies and Execution Order

### Phase Dependencies

- **Phase 0**: Starts immediately.
- **Phase 1**: Depends on Phase 0 and establishes failing regressions.
- **Phase 2**: Depends on Phase 1 and blocks all user-story work.
- **Phase 3 (US1)**: Depends on Phase 2 and is the MVP.
- **Phase 4 (US2)**: Depends on Phase 3 so handoff-unavailable semantics reuse
  the final stable slice identity and handoff-selection model.
- **Phase 5 (US3)**: Depends on Phases 3 and 4 because docs and skills must
  describe the final runtime truth.
- **Final Phase**: Depends on all selected stories.

### Parallel Opportunities

- T002 can run in parallel with T001.
- T003 through T006 can run in parallel.
- T017 and T018 can run in parallel.

## Implementation Strategy

### MVP First

1. Complete governance alignment and failing regressions.
2. Complete the foundational typed contract for slice identity and handoff
   availability.
3. Deliver US1 so Canon can emit one governed handoff slice.
4. Add US2 honesty semantics before updating docs and skills.
5. Finish with publish/docs/skills alignment and full verification.

### Quality Rule

The feature is complete only when Canon preserves the planning-only backlog
boundary, emits stable `slice_id` values consistently, distinguishes
handoff-available from handoff-unavailable packets truthfully, and passes
formatting, linting, regression, and independent review evidence checks.
