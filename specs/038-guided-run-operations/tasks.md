# Tasks: Guided Run Operations And Review Experience

**Input**: Design documents from `/specs/038-guided-run-operations/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Validation**: Layered validation is mandatory. Every behavior change must be
covered by focused executable checks before full-suite closeout.

**Organization**: Tasks are grouped by user story for traceability, but feature
`038` remains one macrofeature to be delivered whole rather than shipped as
separate slices.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- Every task includes exact file paths

## Phase 0: Governance & Artifacts

**Purpose**: Keep the execution authority artifacts current before mutation

- [ ] T001 Refresh mode, risk, scope, invariants, and release-alignment obligations in `specs/038-guided-run-operations/spec.md` and `specs/038-guided-run-operations/plan.md` if implementation scope moves
- [ ] T002 Keep accepted decisions and tradeoffs current in `specs/038-guided-run-operations/decision-log.md`
- [ ] T003 Keep evidence buckets current in `specs/038-guided-run-operations/validation-report.md`
- [ ] T004 Reconfirm the operator-guidance and render-next-steps contracts in `specs/038-guided-run-operations/contracts/operator-guidance.md` and `specs/038-guided-run-operations/contracts/render-next-steps.md` before code lands

---

## Phase 1: Foundational Guidance Surface

**Purpose**: Establish the shared guidance derivation that all renderers will consume

**⚠️ CRITICAL**: No user-story implementation should drift outside this shared contract

- [ ] T005 Define the shared operator-guidance data flow in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/service/next_action.rs` so recommended next step and ordered possible actions come from the same Canon-backed facts
- [ ] T006 Update action-chip derivation in `crates/canon-engine/src/orchestrator/service/summarizers.rs` so chip fallback text mirrors the same valid operator actions and stale approve or resume chips disappear
- [ ] T007 [P] Add or tighten foundational helper tests in `crates/canon-engine/src/orchestrator/service/tests.rs`, `crates/canon-engine/src/orchestrator/service/next_action.rs`, and `crates/canon-engine/src/orchestrator/service/summarizers.rs` for completed, blocked, approval-gated, and resumable states

**Checkpoint**: One shared operator-guidance contract exists and has focused engine-level tests

---

## Phase 2: User Story 1 - Understand Run State Immediately (Priority: P1) 🎯 MVP

**Goal**: Make `run` and `status` read as one coherent operator summary with result, blockers, possible actions, and one honest next step

**Independent Test**: A reviewer can inspect completed, blocked, and gated run or status output and determine the readable packet, blockers, and next step without running another command just to understand state

### Validation for User Story 1 (MANDATORY)

- [ ] T008 [P] [US1] Add failing renderer expectations in `crates/canon-cli/src/output.rs` for `Possible Actions:` and the new recommended-next-step behavior
- [ ] T009 [P] [US1] Add or tighten focused summary-contract assertions in `crates/canon-engine/src/orchestrator/service/tests.rs` for result-first completed runs and blocker-first blocked runs
- [ ] T010 [US1] Record any guidance-ordering decision changes in `specs/038-guided-run-operations/decision-log.md`

### Implementation for User Story 1

- [ ] T011 [US1] Implement ordered possible-action and recommended-next-step assembly in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/service/next_action.rs`
- [ ] T012 [US1] Update `crates/canon-cli/src/output.rs` to render `## Result`, `## Blockers`, `## Recommended Next Step`, and `Possible Actions:` coherently for both `RunSummary` and `StatusSummary`
- [ ] T013 [US1] Ensure `ModeResultSummary.primary_artifact_action` and action-chip fallback text stay aligned with the new possible-action ordering in `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [ ] T014 [US1] Capture US1 validation notes and output examples in `specs/038-guided-run-operations/validation-report.md`

**Checkpoint**: Run and status summaries are independently readable and correctly guide the next move

---

## Phase 3: User Story 2 - Review And Remediate Governed Runs Coherently (Priority: P2)

**Goal**: Align chips, shared next-step scripts, and governed-flow regressions around the same review-first operator story

**Independent Test**: For approval-gated, approved, resumed, and blocked flows, CLI summaries, helper scripts, and chip fallback text all recommend the same ordered actions

### Validation for User Story 2 (MANDATORY)

- [ ] T015 [P] [US2] Add failing regression coverage in `tests/render_next_steps.rs` for any new or changed next-step profiles and wording
- [ ] T016 [P] [US2] Add failing governed-flow assertions in `tests/integration/implementation_run.rs` for approval-gated, approved, and resumed guidance alignment
- [ ] T017 [US2] Record state-transition or chip-contract decisions in `specs/038-guided-run-operations/decision-log.md`

### Implementation for User Story 2

- [ ] T018 [US2] Update `.agents/skills/canon-shared/scripts/render-next-steps.sh` and `.agents/skills/canon-shared/scripts/render-next-steps.ps1` to match the runtime operator-guidance ordering
- [ ] T019 [US2] Sync the shipped helper copies in `defaults/embedded-skills/canon-shared/scripts/render-next-steps.sh` and `defaults/embedded-skills/canon-shared/scripts/render-next-steps.ps1`
- [ ] T020 [US2] Update shared guidance references in `.agents/skills/canon-shared/references/output-shapes.md` and `defaults/embedded-skills/canon-shared/references/output-shapes.md` so docs match the delivered runtime contract
- [ ] T021 [US2] Tighten runtime or integration behavior in `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/next_action.rs`, and `crates/canon-engine/src/orchestrator/service/summarizers.rs` until governed approval, blocker, and resume flows stay aligned end-to-end
- [ ] T022 [US2] Capture US2 validation evidence in `specs/038-guided-run-operations/validation-report.md`

**Checkpoint**: Governed review, approval, and resume flows tell the same story across runtime, CLI, and shared helpers

---

## Phase 4: User Story 3 - Keep Release, Docs, And Roadmap Aligned With 038 (Priority: P3)

**Goal**: Ship the operator-guidance slice with matching docs, roadmap, changelog, and explicit `0.37.0` alignment

**Independent Test**: Release-alignment checks and human review show one coherent `0.37.0` story for guided run operations

### Validation for User Story 3 (MANDATORY)

- [ ] T023 [P] [US3] Add or update release-surface assertions in `tests/release_036_release_provenance_integrity.rs` for the delivered `038` roadmap and operator-guidance story
- [ ] T024 [US3] Record documentation and release-surface decisions in `specs/038-guided-run-operations/decision-log.md`

### Implementation for User Story 3

- [ ] T025 [US3] Update `README.md` and any adjacent operator-facing docs touched by the new run or status guidance contract
- [ ] T026 [US3] Update `CHANGELOG.md` with the delivered guided run operations and review experience behavior
- [ ] T027 [US3] Keep explicit `0.37.0` alignment in `Cargo.toml`, `Cargo.lock`, and any version-surface checks even if no version increment is required
- [ ] T028 [US3] Remove delivered feature `038` from `ROADMAP.md` and leave only the remaining future macrofeature(s) after this slice
- [ ] T029 [US3] Capture US3 validation evidence in `specs/038-guided-run-operations/validation-report.md`

**Checkpoint**: Runtime behavior and repository guidance are aligned for the delivered slice

---

## Final Phase: Verification & Compliance

**Purpose**: Close the slice with focused evidence, workspace hygiene, and final review

- [ ] T030 [P] Run the focused engine, CLI, renderer, and integration checks from `quickstart.md` and record results in `specs/038-guided-run-operations/validation-report.md`
- [ ] T031 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and record coverage implications for touched Rust files in `specs/038-guided-run-operations/validation-report.md`
- [ ] T032 [P] Run `cargo fmt --check` and fix any formatting fallout in touched files
- [ ] T033 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and resolve any issues introduced by this slice
- [ ] T034 Run `cargo nextest run` and record full-suite closeout evidence in `specs/038-guided-run-operations/validation-report.md`
- [ ] T035 Perform an independent review of operator-guidance honesty, confirm invariants still hold, and close `specs/038-guided-run-operations/validation-report.md`
- [ ] T036 Prepare the final commit message for the delivered `038` slice and record it with the closeout notes

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0**: Must remain current before mutation and closeout.
- **Phase 1**: Must land before renderer, script, and doc work diverges.
- **Phase 2**: Depends on Phase 1.
- **Phase 3**: Depends on Phase 1 and the core runtime guidance contract.
- **Phase 4**: Depends on the delivered runtime and helper behavior.
- **Final Phase**: Depends on all story work being complete.

### User Story Dependencies

- **US1 (P1)**: Starts after foundational guidance work and establishes the MVP operator summary.
- **US2 (P2)**: Depends on the US1 guidance contract so helper scripts and chips can mirror it accurately.
- **US3 (P3)**: Depends on the delivered runtime contract so docs and roadmap describe the real behavior.

### Parallel Opportunities

- T007 can run in parallel across engine test files once the shared guidance design is chosen.
- T008 and T009 can run in parallel as failing checks for US1.
- T015 and T016 can run in parallel as failing checks for US2.
- T018 and T019 can run in parallel once the target wording is settled.
- T030 through T033 can run as separate closeout commands once code changes are stable.

## Implementation Strategy

### Whole-Feature Delivery

1. Lock the shared operator-guidance derivation in engine code.
2. Make CLI markdown render the same state, next-step, and possible-action story.
3. Align helper scripts and skill references to the same wording.
4. Finish with docs, roadmap, changelog, version alignment, coverage, lint, format, and full regression closeout.

## Notes

- Keep the first executable validation focused on the touched operator-guidance slice before widening to workspace-wide checks.
- Do not add a new run-state family, new approval semantics, or a new runtime subsystem while delivering this feature.
- Treat roadmap cleanup, `0.37.0` alignment, coverage, `cargo fmt`, and `cargo clippy` as part of the feature definition, not optional cleanup.