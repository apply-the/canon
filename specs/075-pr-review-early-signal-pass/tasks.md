# Tasks: Early Signal Pass (First-Pass Risk Discovery)

**Input**: Design documents from `specs/075-pr-review-early-signal-pass/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/, quickstart.md

**Tests**: Contract-level integration tests for CLI behavior are included per the quickstart acceptance scenarios. Unit tests for deterministic rule functions are included per research.md decision 1.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Domain Types & Module Scaffolding)

**Purpose**: Define domain types, add `AwaitingReviewerOutput` run state, and create new module files before any executor logic exists

- [ ] T001 [P] Define `EarlySignalSeverity` enum, `EarlySignalFinding` struct, and `EarlySignalEvent`/`EarlySignalEventKind` enums with serde derives in `crates/canon-engine/src/domain/review.rs`
- [ ] T002 [P] Define `ReviewLayer`, `LayerExecutor`, `LayerStatus` enums, `LayerCoverageEntry`, and `CoverageAccounting` structs in `crates/canon-engine/src/domain/review.rs`
- [ ] T003 [P] Define the seven rule ID constants (`build.command.removed_file_reference`, `manifest.stale_dependency`, `manifest.schema_drift`, `reference.dangling_import`, `test.missing_for_changed_behavior`, `naming.drift`, `validation.failure`) in `crates/canon-engine/src/orchestrator/service/early_signal.rs`
- [ ] T004 [P] Add `AwaitingReviewerOutput` variant to the `RunState` enum in `crates/canon-engine/src/domain/run.rs`
- [ ] T005 [P] Add path helpers for `early-signal/`, `traces/`, and `layers/` directories under the pr-review run directory in `crates/canon-engine/src/persistence/layout.rs`
- [ ] T006 [P] Create stub module file `crates/canon-engine/src/orchestrator/service/early_signal.rs` with module-level doc comment and `use` declarations for the domain types from T001–T003
- [ ] T007 [P] Create stub module file `crates/canon-cli/src/output/early_signal.rs` with module-level doc comment
- [ ] T008 Wire the new modules into their parent module root files: register `pub mod early_signal` in `crates/canon-engine/src/orchestrator/service.rs`, register `pub mod early_signal` in `crates/canon-cli/src/output.rs` (or `crates/canon-cli/src/output/mod.rs` if it exists), register `pub mod review` in `crates/canon-engine/src/domain.rs`, and register `pub mod traces` in `crates/canon-engine/src/persistence.rs` (if T014 creates it under persistence). The codebase uses `service.rs`, `domain.rs`, and `persistence.rs` as module roots (Rust 2018+ convention), not `mod.rs`.

---

## Phase 2: Foundational (Early Signal Executor & Layer Generation)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented — the early signal check executor, layer directory generation, and persistence plumbing

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T009 Implement the early signal check executor function `execute_early_signal_pass(run_id, diff_context) -> Vec<EarlySignalFinding>` in `crates/canon-engine/src/orchestrator/service/early_signal.rs`. Wire each rule from T003 as a pure function returning `Vec<EarlySignalFinding>`. Include: `check_build_command_removed_file_reference`, `check_manifest_stale_dependency`, `check_manifest_schema_drift`, `check_reference_dangling_import`, `check_test_missing_for_changed_behavior`, `check_naming_drift`, `check_validation_failure`.
- [ ] T010 [P] Implement the file classification function `classify_files(changed_files) -> Vec<FileClassification>` in `crates/canon-engine/src/orchestrator/service/early_signal.rs`, producing risk classes (`high`, `medium`, `low`) with reasons. Reuse the existing `is_high_risk` logic from `mode_pr_review_prepare.rs`.
- [ ] T011 [P] Implement finding ID generation: a sequential counter producing `ES001`, `ES002`, etc. that is stable within a single prepare invocation across all output channels. In `crates/canon-engine/src/orchestrator/service/early_signal.rs`.
- [ ] T012 Implement the layer directory generation function `generate_layer_directories(run_dir) -> Result<()>` that creates `layers/01-early-signal/` through `layers/07-coverage-accounting/`, each with `instructions.md`, `required-context.tsv`, and an empty `output.md`. In `crates/canon-engine/src/orchestrator/service/mode_pr_review_prepare.rs`.
- [ ] T013 Implement `write_review_plan_md` to produce `review-plan.md` listing all 7 layers in order with their status (early signal: executed-by-Canon, layers 2-6: pending, layer 7: pending). In `crates/canon-engine/src/orchestrator/service/mode_pr_review_prepare.rs`.
- [ ] T014 Implement the event emission infrastructure: `emit_stdout_event(event)` that writes one JSON line to stdout when `--output json`, and `persist_trace_event(run_dir, event)` that appends one JSON line to `traces/early-signal.jsonl`. In `crates/canon-engine/src/orchestrator/service/early_signal.rs` (stdout) and a new trace helper in `crates/canon-engine/src/persistence/traces.rs`.
- [ ] T015 Implement the early signal artifact persistence: write `findings.tsv` (tab-separated), `findings.json` (valid JSON array), and `summary.md` (markdown with severity/bucket counts) under `early-signal/`. In `crates/canon-engine/src/orchestrator/service/mode_pr_review_prepare.rs`.
- [ ] T016 Implement `write_run_state` for `AwaitingReviewerOutput` after prepare completes. Update the existing `write_run_state` helper in `mode_pr_review_prepare.rs` to accept the new state variant.

**Checkpoint**: Foundation ready — early signal checks, layer directories, event infrastructure, and persistence are all available. User story implementation can now begin.

---

## Phase 3: User Story 1 — Reviewer receives early signal findings before deep review (Priority: P1) 🎯 MVP

**Goal**: When an agent invokes `canon pr-review prepare --base X --head Y`, Canon automatically executes the early signal pass and emits structured findings before the review plan. The agent does not need a separate subcommand.

**Independent Test**: Run `canon pr-review prepare` on a fixture PR with a deliberate broken build reference and stale manifest. Verify stdout JSON contains `early_signal.finding_detected` events with correct rule IDs and stable finding IDs before the review plan output. Verify `findings.json`, `findings.tsv`, `summary.md`, and `trace.jsonl` are all persisted with matching finding IDs.

### Implementation for User Story 1

- [ ] T017 [P] [US1] Add `--skip-early-signal`, `--skip-reason`, and `--output` flags to `PrReviewCommand::Prepare` in `crates/canon-cli/src/app.rs`. `--skip-early-signal` is a boolean flag; `--skip-reason` is a `String` required when `--skip-early-signal` is set. `--output` is the existing `OutputFormat` enum.
- [ ] T018 [P] [US1] Implement stdout JSON event rendering in `crates/canon-cli/src/output/early_signal.rs`: a function `render_early_signal_events(events: &[EarlySignalEvent], format: OutputFormat)` that emits one JSON line per event when format is `Json`, or a markdown summary when format is `Text`.
- [ ] T019 [US1] Wire the early signal pass into `EngineService::run_pr_review_prepare` in `crates/canon-engine/src/orchestrator/service/mode_pr_review_prepare.rs`: after diff collection but before the existing review plan logic, call `execute_early_signal_pass`. Emit lifecycle events (`started`, `file_classified`, `finding_detected`, `completed`) with all required fields per FR-018 and FR-019 (summary must include: total files classified, total findings, findings by severity, findings by bucket, high-risk files identified, suggested next layers, early signal status). When `--skip-early-signal` is requested, emit `early_signal.skipped` and persist all FR-013 metadata: skip reason, operator/agent source, and confidence impact.
- [ ] T020 [US1] Update `PrReviewCommand::Prepare` dispatch in `crates/canon-cli/src/commands/pr_review.rs` to pass `--skip-early-signal`, `--skip-reason`, and `--output` to the engine service. Render stdout events via the renderer from T018.
- [ ] T021 [US1] Implement `--skip-early-signal` validation: if the flag is set but `--skip-reason` is empty or missing, return a `CliError::InvalidInput` with a message per FR-015. In `crates/canon-cli/src/commands/pr_review.rs`.
- [ ] T022 [US1] Add unit tests for each of the seven early signal check rule functions in `crates/canon-engine/src/orchestrator/service/early_signal.rs` (in `#[cfg(test)]`): `check_reference_dangling_import`, `check_build_command_removed_file_reference`, `check_manifest_stale_dependency`, `check_manifest_schema_drift`, `check_test_missing_for_changed_behavior`, `check_naming_drift`, and `check_validation_failure`. Each test provides a fixture diff context and asserts the expected finding(s) or empty result.
- [ ] T023 [US1] Add unit tests for `EarlySignalEvent` and `EarlySignalFinding` serialization/deserialization round-trip in `crates/canon-engine/src/domain/review.rs` (in `#[cfg(test)]`), verifying all event kinds serialize to the schema defined in `contracts/events-schema.md`.
- [ ] T024 [US1] Add contract test for `prepare` default-on early signal pass in `tests/contract/pr_review_early_signal.rs`: create fixture repo with a dangling import, run `prepare --output json`, assert stdout contains `early_signal.finding_detected` with `rule_id: "reference.dangling_import"`, assert `findings.json` in run dir has matching finding IDs.
- [ ] T024a [US1] [P] Add contract test validating all 10 required fields (FR-018) in a `finding_detected` stdout JSON event: `run_id`, `rule_id`, `finding_id`, `severity`, `category`, `path`, `start_line`/`end_line` (optional), `evidence_context_ids`, `summary`, `suggested_layer`, `actionable_comment_candidate`. Parse the event from stdout and assert each field is present with expected type/value. In `tests/contract/pr_review_early_signal.rs`.
- [ ] T025 [US1] Add contract test for `prepare --skip-early-signal --skip-reason "..."` in `tests/contract/pr_review_early_signal.rs`: assert stdout contains `early_signal.skipped` with the reason, assert no `early_signal.finding_detected` events, assert persisted run state (e.g., `state.toml` in run dir) records `early_signal_status: "skipped_with_reason"`, and verify the skip reason, operator/agent source, and confidence impact fields are all present per FR-013.
- [ ] T025a [US1] [P] Add contract test validating trace JSONL content beyond JSON validity: assert trace includes at least one lifecycle event per type (started, completed or skipped or failed), at least one rule execution event, classification counts in the completed summary, and finding IDs that match stdout. In `tests/contract/pr_review_early_signal.rs`.
- [ ] T026 [US1] Add contract test for `prepare --skip-early-signal` without `--skip-reason` in `tests/contract/pr_review_early_signal.rs`: assert exit code 1 and error message about missing skip reason.
- [ ] T027 [US1] Add contract test for `prepare` layer directory generation in `tests/contract/pr_review_early_signal.rs`: assert all 7 layer directories exist with `instructions.md`, `required-context.tsv`, and `output.md`, and that `review-plan.md` lists all layers.
- [ ] T028 [US1] Add contract test for `prepare` trace persistence in `tests/contract/pr_review_early_signal.rs`: assert `traces/early-signal.jsonl` exists, each line is valid JSON, first event is `early_signal.started`, last event is `early_signal.completed`.
- [ ] T029 [US1] Add contract test for `prepare` finding artifact persistence in `tests/contract/pr_review_early_signal.rs`: assert `findings.tsv` is tab-separated, `findings.json` is valid JSON, `summary.md` contains severity/bucket counts, and finding IDs match across all three artifacts.
- [ ] T030 [US1] Add contract test for `prepare --output text` (default) in `tests/contract/pr_review_early_signal.rs`: assert stdout contains a human-readable markdown summary (not raw JSON), trace is still persisted.

**Checkpoint**: At this point, User Story 1 should be fully functional — `canon pr-review prepare` runs the early signal pass by default, emits structured findings via stdout JSON and trace JSONL, persists all artifacts, and supports `--skip-early-signal` with validation.

---

## Phase 4: User Story 2 — Canon never stops after early signal pass alone (Priority: P1)

**Goal**: After the early signal pass completes (with or without findings), Canon must automatically continue to generate the remaining review plan and layer instructions. Finalization is gated on all layers being completed or deferred — layer 1 alone is never sufficient.

**Independent Test**: Run a review. After prepare completes, attempt finalize without accept — must fail. After accept with only layer 1 completed and layers 2-7 not deferred — must fail. After accept with all layers accounted for — must succeed.

### Implementation for User Story 2

- [ ] T031 [US2] Update `EngineService::run_pr_review_accept` in `crates/canon-engine/src/orchestrator/service/mode_pr_review_accept.rs` to validate that run state is `AwaitingReviewerOutput` before proceeding. Reject if state is anything else.
- [ ] T032 [US2] Implement layer output validation in `run_pr_review_accept`: for each layer 1-7, verify either a non-empty `output.md` exists with required sections, or a deferral with non-empty reason is recorded. Layer 1 (early signal) is pre-populated by Canon; layers 2-7 must have reviewer-written content. In `crates/canon-engine/src/orchestrator/service/mode_pr_review_accept.rs`.
- [ ] T033 [US2] Implement the `accept` rejection logic: if any required layer lacks both valid output and a deferral reason, produce structured errors referencing the layer path and missing requirement (FR-027, FR-028). Layer directories with only `instructions.md` but no reviewer `output.md` must NOT be accepted as completed.
- [ ] T034 [US2] Update `EngineService::run_pr_review_finalize` in `crates/canon-engine/src/orchestrator/service/mode_pr_review_finalize.rs` to gate on `RunState::ReviewerOutputAccepted` (or equivalent post-accept state). Reject finalization if the run has not been accepted.
- [ ] T035 [US2] Update `run_pr_review_finalize` to reject finalization when fewer than all 7 layers are accounted for (reviewed or deferred). Per FR-006, Canon must NOT allow finalization after layer 1 alone. In `crates/canon-engine/src/orchestrator/service/mode_pr_review_finalize.rs`.
- [ ] T035a [US2] [P] Add contract test validating ordered layer progression enforcement: `accept` rejects a state where a semantic layer (e.g., layer 3) has output but an earlier layer (e.g., layer 2) has neither output nor deferral, enforcing that all layers are accounted for in order (FR-007, FR-027). In `tests/contract/pr_review_early_signal.rs`.
- [ ] T036 [US2] Update the `accept` dispatch in `crates/canon-cli/src/commands/pr_review.rs` to surface validation errors from the engine with layer-level detail. When validation fails, print the specific layer paths and missing requirements.
- [ ] T037 [US2] Add contract test for `finalize` rejected without accept in `tests/contract/pr_review_early_signal.rs`: run prepare, then attempt finalize without accept, assert exit code 1 with error about run not in accepted state.
- [ ] T038 [US2] Add contract test for `accept` rejecting incomplete layer coverage in `tests/contract/pr_review_early_signal.rs`: run prepare, populate only layer 1 output (early signal), write empty outputs for layers 2-7 without deferral reasons, run accept, assert exit code 1 with errors about missing layer outputs.
- [ ] T039 [US2] Add contract test for `accept` succeeding with all layers accounted for (including deferrals) in `tests/contract/pr_review_early_signal.rs`: run prepare, populate valid outputs or deferral reasons for all layers, run accept, assert success.
- [ ] T040 [US2] Add contract test for `finalize` rejected when fewer than 7 layers are completed (e.g., only layers 1-3 done, 4-7 neither completed nor deferred) in `tests/contract/pr_review_early_signal.rs`.

**Checkpoint**: At this point, User Stories 1 AND 2 should both work. Prepare runs the early signal pass + generates the full review plan. Accept validates layer completeness. Finalize gates on proper coverage.

---

## Phase 5: User Story 3 — Coverage accounting forces honest enumeration (Priority: P2)

**Goal**: When a review reaches finalization, Canon produces a coverage accounting that honestly lists which layers were reviewed and which were deferred with explicit reasons. The early signal status (completed or skipped) is included, and a skipped early signal reduces the overall confidence assessment.

**Independent Test**: Run a review on a complex PR. Explicitly defer layer 5 (logical stress). Finalize. Verify coverage accounting lists layer 5 as deferred with the stated reason, the overall confidence is reduced, and the early signal status is included.

### Implementation for User Story 3

- [ ] T041 [US3] Implement `compile_coverage_accounting` in `crates/canon-engine/src/orchestrator/service/mode_pr_review_finalize.rs`: iterate all 7 layers, read each layer's status from the accept-validated state, build `CoverageAccounting` with one `LayerCoverageEntry` per layer. For deferred layers, extract the deferral reason from the layer's output metadata.
- [ ] T042 [US3] Implement the early signal status inclusion in coverage accounting: if early signal was skipped, include the skip reason, source, and confidence impact in the accounting. If skipped, reduce `overall_confidence` by at least one level and ensure the final report does NOT imply full early-risk coverage (FR-014). In `crates/canon-engine/src/orchestrator/service/mode_pr_review_finalize.rs`.
- [ ] T043 [US3] Implement deferral reason validation at `finalize`: reject any deferral that has an empty or missing reason string (FR-009). T032 already handles accept-side deferral validation; this task covers only the finalize-side recheck. In `crates/canon-engine/src/orchestrator/service/mode_pr_review_finalize.rs`.
- [ ] T044 [US3] Render the `CoverageAccounting` as both a `coverage-accounting.md` markdown artifact and within the final review summary/report. In `crates/canon-engine/src/orchestrator/service/mode_pr_review_finalize.rs`.
- [ ] T045 [US3] Update the `Finalize` dispatch in `crates/canon-cli/src/commands/pr_review.rs` to print a summary of coverage accounting (reviewed vs deferred layers, overall confidence, early signal status) after successful finalization.
- [ ] T046 [US3] Add contract test for coverage accounting with a deferred layer in `tests/contract/pr_review_early_signal.rs`: run prepare, populate outputs for layers 1-4 and 6-7, write a deferral for layer 5 with reason "edge-case analysis deferred — time budget", accept, finalize, assert `coverage-accounting.md` lists layer 5 as deferred with the reason.
- [ ] T047 [US3] Add contract test for deferred layer without reason rejected at accept in `tests/contract/pr_review_early_signal.rs`: run prepare, populate layer 5 deferral with empty reason, run accept, assert exit code 1 with error about missing deferral reason.
- [ ] T047a [US3] [P] Add contract test for the "diff too large" edge case: fixture with a large diff where the agent defers layers 5 (logical stress) and 6 (tests) with explicit reasons. Verify `accept` validates the remaining layers and `finalize` records the deferrals in coverage accounting with their reasons. In `tests/contract/pr_review_early_signal.rs`.
- [ ] T048 [US3] Add contract test for early signal skipped reducing confidence in `tests/contract/pr_review_early_signal.rs`: run prepare with `--skip-early-signal --skip-reason "debug"`, populate all layers, accept, finalize, assert `coverage-accounting.md` includes `early_signal_status: "skipped_with_reason"` and overall confidence is `low` or `medium` (not `high`), and final report does NOT claim early-risk coverage was achieved.
- [ ] T049 [US3] Add contract test for full coverage (all 7 layers reviewed, no deferrals, early signal completed) producing `high` confidence in `tests/contract/pr_review_early_signal.rs`.

**Checkpoint**: All three user stories should now be independently functional. Coverage accounting forces honest enumeration, deferrals require explicit reasons, and early signal status affects confidence.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Replace deprecated terminology, add integration tests, validate against quickstart.md

- [ ] T050 [P] Audit and replace all "quick wins" terminology with "early signal pass" in code comments, doc strings, CLI output, and user-facing messages across the codebase (FR-002). Use `grep_search` to locate occurrences first. Prefer the single canonical term "early signal pass"; use "first-pass risk discovery" only as a parenthetical qualifier.
- [ ] T051 [P] Add finding deduplication by canonical location reference in `execute_early_signal_pass`: when multiple rules produce findings for the same file+line, keep the earliest (highest) severity (FR-010). In `crates/canon-engine/src/orchestrator/service/early_signal.rs`.
- [ ] T052 Implement the `early_signal.failed` event path: when a check rule encounters a non-recoverable error (e.g., shell timeout), emit `early_signal.failed` with the error, rule_id, and partial findings count, then persist to trace. Return a soft error that still allows the review plan to be generated. In `crates/canon-engine/src/orchestrator/service/early_signal.rs`.
- [ ] T053 [P] Add the `--output json` flag to Accept and Finalize `PrReviewCommand` variants in `crates/canon-cli/src/app.rs` for consistency with Prepare. Update dispatch in `crates/canon-cli/src/commands/pr_review.rs` to respect the format.
- [ ] T054 Run `cargo fmt` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` across the workspace. Fix any lint errors. Also audit all new and modified files for magic string/number literals in production code (outside tests): extract reusable literals into named `const` items or typed enums per the Rust Language Rules.
- [ ] T055 Run the full quickstart.md validation sequence (sections 1-9) using the built binary, verifying all expected outcomes match. Document any deviations.
- [ ] T056 Run `cargo test --workspace` and `cargo nextest run` to ensure all new and existing tests pass. Run `cargo deny check licenses advisories bans sources` to verify no dependency issues.
- [ ] T057 [P] Add a performance assertion test for SC-001: verify that the early signal pass completes in ≤30 seconds for a fixture PR with 50 changed files. Instrument `execute_early_signal_pass` with a duration measurement and assert the wall-clock time. In `crates/canon-engine/src/orchestrator/service/early_signal.rs` (timing) and `tests/contract/pr_review_early_signal.rs` (assertion).
- [ ] T058 [P] Extend `tests/integration/pr_review_prepare.rs` (or create if absent) with integration tests for early signal in the full prepare flow: early signal runs by default inside prepare, layer directories are generated, review-plan.md is created, and the run state transitions to `AwaitingReviewerOutput` (FR-024, FR-025, FR-026).
- [ ] T059 [P] Add a contract test for a PR with zero changed files (spec edge case): verify the early signal pass completes with zero findings, all 7 layer directories are generated, layers 4-6 (related-context, logical-stress, tests) produce instructions reflecting "nothing to review", and the review workflow proceeds without errors. In `tests/contract/pr_review_early_signal.rs`.

---

## Final Phase: Release, Quality, And Verification

**Purpose**: Version bump, documentation synchronization, quality gate enforcement, coverage target validation.

- [ ] T060 Bump workspace version from `0.72.2` to `0.73.0` in `Cargo.toml` (feature: early signal pass as first review layer). Also update any internal crate version references that follow the workspace version.
- [ ] T061 Update `CHANGELOG.md` with a new `[0.73.0]` entry documenting the early signal pass feature: new `--skip-early-signal` flag, 7-layer review plan in `prepare`, stdout JSON events, persisted trace and findings artifacts.
- [ ] T062 Update `README.md` and any affected `docs/` or `tech-docs/` markdown files referencing the PR review workflow to reflect the early signal pass terminology and the prepare → accept → finalize flow.
- [ ] T063 Run `./scripts/update-docs-versions.sh` to synchronize version references across docs after the `Cargo.toml` bump.
- [ ] T064 Run `cargo fmt` across the workspace.
- [ ] T065 Run `scripts/clippy.sh` and fix all warnings. Verify `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes with zero errors.
- [ ] T066 Run `scripts/test.sh` and fix any failing tests. Verify `cargo nextest run --workspace --all-features` passes.
- [ ] T067 Run `scripts/coverage.sh` and confirm at least 95% line coverage for every modified or created Rust source file. If any file falls below 95%, add targeted tests or document the exclusion in the coverage report. Use `scripts/common/coverage/intersect_patch_coverage.py` to verify no uncovered patch lines.
- [ ] T068 Run `scripts/check-rust-no-panic.sh` (or equivalent grep: `rg 'unwrap\(|expect\(|panic!|todo!|unimplemented!|unreachable!' crates/ --include '*.rs' --glob '!*main.rs' --glob '!*tests*'`) and verify no panic-prone patterns were introduced in production code outside `main.rs` and `#[cfg(test)]`.
- [ ] T069 [P] If this feature touches assistant plugin metadata or `.agents/skills/`, run `scripts/validate-assistant-plugins.sh`. (Skip if no assistant assets were modified.)
- [ ] T070 Re-run the `specs/075-pr-review-early-signal-pass/quickstart.md` validation sequence after all quality fixes (T064–T069) are applied, confirming all expected outcomes still match.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion (T001–T008) — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational completion (T009–T016)
- **US2 (Phase 4)**: Depends on US1 completion (T017–T030) — needs prepare to work before accept/finalize gating
- **US3 (Phase 5)**: Depends on US2 completion (T031–T040) — needs accept validation before coverage accounting
- **Polish (Phase 6)**: Depends on all user stories being complete
- **Release & Quality (Final Phase)**: Depends on Polish completion

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) — No dependencies on other stories. This is the MVP.
- **User Story 2 (P1)**: Depends on US1 (needs prepare output to validate accept/finalize). Can partially overlap with US1 contract tests.
- **User Story 3 (P2)**: Depends on US2 (needs accept gating to validate coverage accounting). Can partially overlap with US2 contract tests.

### Within Each User Story

- CLI flags (T017–T018) before engine wiring (T019–T020)
- Engine wiring before contract tests (T024–T030)
- Unit tests (T022–T023) can run in parallel with CLI work
- Contract tests within a story can run in parallel once implementation is complete

### Parallel Opportunities

- All Setup tasks marked [P] (T001–T007) can run in parallel
- T009 and T010 (executor + classifier) can be developed in parallel once T001–T003 are done
- T017 and T018 (CLI flags + rendering) can run in parallel after T001–T003
- T022 and T023 (unit tests) can run in parallel with T019–T021 (engine wiring)
- T024–T030 (contract tests) can all run in parallel after T019–T021
- T037–T040 (US2 contract tests) can run in parallel after T031–T036
- T046–T049 (US3 contract tests) can run in parallel after T041–T045
- T050, T051, T053 (Polish tasks) can run in parallel

---

## Parallel Example: User Story 1

```bash
# After Foundational phase (T009–T016) completes, these can proceed in parallel:

# Developer A: CLI flags and rendering
Task T017: Add --skip-early-signal, --skip-reason, --output to app.rs
Task T018: Implement stdout JSON renderer in output/early_signal.rs

# Developer B: Engine wiring
Task T019: Wire early signal into run_pr_review_prepare
Task T020: Update CLI dispatch in commands/pr_review.rs
Task T021: Implement skip-without-reason validation

# Developer C: Unit tests
Task T022: Unit tests for check rules in early_signal.rs
Task T023: Unit tests for event serialization in review.rs

# After T019–T021 complete, all contract tests (T024–T030) can run in parallel
```

## Parallel Example: User Story 2

```bash
# After US1 completes:

# Developer A: Accept validation
Task T031: Gate accept on AwaitingReviewerOutput state
Task T032: Implement layer output validation
Task T033: Implement accept rejection logic

# Developer B: Finalize gating
Task T034: Gate finalize on accepted state
Task T035: Reject finalize when <7 layers accounted

# Developer C: CLI dispatch
Task T036: Update CLI dispatch for accept errors

# After all implementation tasks, contract tests T037–T040 run in parallel
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T008) — domain types and module scaffolding
2. Complete Phase 2: Foundational (T009–T016) — executor, layer dirs, events, persistence
3. Complete Phase 3: User Story 1 (T017–T030) — CLI flags, engine wiring, unit + contract tests
4. **STOP and VALIDATE**: Run quickstart.md sections 1-6. Verify early signal findings are emitted, artifacts are persisted, layer directories exist.
5. Deploy/demo if ready — `canon pr-review prepare` now includes the early signal pass by default.

### Incremental Delivery

1. **MVP (US1)**: `prepare` runs early signal pass, emits findings with validated field contracts, generates layer plan → agent receives findings before deep review (18 tasks)
2. **+US2**: `accept` validates layer completeness with ordered progression enforcement, `finalize` gates on all layers → Canon never stops after layer 1 (11 tasks)
3. **+US3**: Coverage accounting with honest enumeration, deferral reasons, diff-too-large scenarios, confidence impact → Complete review governance (10 tasks)
4. **+Polish**: Terminology cleanup, deduplication, failure handling, quickstart validation → Production-ready (10 tasks)
5. **+Release**: Version bump, changelog, docs sync, quality gates → Shippable (11 tasks)

### Suggested MVP Scope

- Deliver User Story 1 (Phase 3) as the MVP
- This gives the reviewer immediate early signal findings with traceable artifacts
- US2 and US3 can follow in subsequent increments
- The full 7-layer review flow remains governed but layers 2-7 can use placeholder deferrals during early adoption

---

## Summary

| Phase | User Story | Task Count | Key Artifacts |
|---|---|---|---|
| Phase 1: Setup | — | 8 | Domain types, RunState, path helpers, module stubs |
| Phase 2: Foundational | — | 8 | Executor, classifier, layer dirs, events, persistence |
| Phase 3: US1 (P1) 🎯 | Early signal findings | 18 | CLI flags, engine wiring, unit tests, 10 contract tests |
| Phase 4: US2 (P1) | Never stops after layer 1 | 11 | Accept validation, finalize gating, ordered-progression test, 5 contract tests |
| Phase 5: US3 (P2) | Coverage accounting | 10 | Accounting logic, deferral enforcement, diff-too-large test, 5 contract tests |
| Phase 6: Polish | — | 10 | Terminology rename, dedup, quickstart, perf test, integration test, zero-change test |
| Final Phase: Release & Quality | — | 11 | Version bump, changelog, docs sync, fmt, clippy, test, coverage, no-panic, quickstart |
| **Total** | | **74** | |

### Independent Test Criteria

- **US1**: Run `prepare` on fixture PR with dangling import → stdout JSON has `finding_detected`, all 4 artifacts match, `--skip-early-signal` works/validates
- **US2**: `finalize` before `accept` → rejected; `accept` with incomplete layers → rejected; `accept` with all layers completed/deferred → success; `finalize` after `accept` → success
- **US3**: Deferred layer with reason → accepted and recorded; deferred layer without reason → rejected; skipped early signal → confidence reduced; full coverage → high confidence
