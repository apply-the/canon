# Tasks: Agent-Governed Onion-Layer PR Review

**Input**: Design documents from `/specs/074-pr-review-onion-orchestration/`

**Prerequisites**: plan.md (✅), spec.md (✅), research.md (✅), data-model.md (✅), contracts/cli.md (✅), quickstart.md (✅)

**Tests**: All 4 user stories are P1. Implement in dependency order: prepare → accept → finalize. Each story includes contract, integration, and unit tests.

**Organization**: Tasks grouped by Phase. Dependencies noted where ordering matters.

---

## Phase 0: Governance & Setup

- [x] T001 Declare execution mode `pr-review`, risk classification `Systemic-Impact`, and scope invariants (layer completion rules, finalize blocking, file-based handoff, empty review must be explained) in `specs/074-pr-review-onion-orchestration/plan.md` and `.canon/` initialization.
- [x] T002 Verify `canon-adapters` has `serde` dependency active; add if missing in `crates/canon-adapters/Cargo.toml`.

---

## Phase 1: Foundational — Onion Layer State Machine & Context Index

**Purpose**: Core data structures that ALL user stories depend on. Must be complete before any story work begins.

- [x] T003 [P] Create `LayerState` enum and `RunState` enum (14 states) in `crates/canon-engine/src/review/onion.rs`.
- [x] T004 [P] Create `LayerStatus` enum (`completed`, `skipped_with_reason`, `failed`) and `SkipRecord`/`FailureRecord` structs in `crates/canon-engine/src/review/onion.rs`.
- [x] T005 [P] Create `ContextIndexEntry` struct with `id`, `type`, `path`, `start_line`, `end_line`, `reason`, `risk`, `layer` fields in `crates/canon-engine/src/review/context.rs`.
- [x] T006 Implement `ContextIndex` builder: collect changed files from diff, classify by type (diff/file/related/test/doc), assign risk, assign layer in `crates/canon-engine/src/review/context.rs`.
- [x] T007 Implement `ContextIndex::to_tsv()` and `ContextIndex::to_json()` serialization in `crates/canon-engine/src/review/context.rs`.
- [x] T008 [P] Register `onion` and `context` modules in `crates/canon-engine/src/review.rs`.
- [x] T009 [P] Unit tests for `RunState` transitions, `LayerStatus` terminal state validation, `ContextIndex` builder in `tests/unit/pr_review_onion_unit.rs`.

**Checkpoint**: State machine and context index are available for prepare phase.

---

## Phase 2: User Story 1 — Onion-Layer Review Context Preparation (P1) 🎯

**Goal**: `canon pr-review prepare --base <base> --head <head>` produces the full context packet.

### Tests for US1

- [x] T010 [P] [US1] Contract test: `prepare` emits `run-state.json`, `review-brief.md`, `review-plan.md`, `context-index.tsv`, `context-index.json`, `changed-files.tsv`, `high-risk-files.tsv`, `relation-hints.tsv`, `diff.patch`, `reviewer-output.schema.json` in `tests/contract/pr_review_onion_contract.rs`.
- [x] T011 [P] [US1] Integration test: `prepare` on fixture repo with boundary/contract changes classifies high-risk surfaces correctly in `tests/integration/pr_review_onion_run.rs`.
- [x] T012 [P] [US1] Unit test: empty diff produces `run-state` with empty changed-files but valid context skeleton.

### Implementation for US1

- [x] T013 [US1] Implement `canon pr-review prepare` CLI sub-command in `crates/canon-cli/src/commands/pr_review.rs` (add `Prepare` variant to sub-command enum).
- [x] T014 [US1] Implement `mode_pr_review_prepare` orchestrator: collect diff via `ShellAdapter`, build `ContextIndex`, write all output files in `crates/canon-engine/src/orchestrator/service/mode_pr_review_prepare.rs`.
- [x] T015 [US1] Generate `review-brief.md` with base/head, mode, expected outcome in `mode_pr_review_prepare.rs`.
- [x] T016 [US1] Generate `review-plan.md` explaining the 5-layer onion sequence and progressive context discovery in `mode_pr_review_prepare.rs`.
- [x] T017 [US1] Generate `changed-files.tsv` and `high-risk-files.tsv` (classify boundary/contract/public/api/schema paths as high-risk) in `mode_pr_review_prepare.rs`.
- [x] T018 [US1] Generate `relation-hints.tsv`: extract changed symbol names from diff hunks, find callers via text search, find related tests/docs/examples by naming convention in `mode_pr_review_prepare.rs`.
- [x] T019 [US1] Generate per-layer `instructions.md` and `required-context.tsv` under `layers/` directory in `mode_pr_review_prepare.rs`.
- [x] T020 [US1] Set initial run state to `awaiting_diff_review` in `run-state.json`.
- [x] T021 [US1] Register `mode_pr_review_prepare` module in `crates/canon-engine/src/orchestrator/service.rs`.

**Checkpoint**: `prepare` command fully functional. Context indexes, instructions, and run state emitted.

---

## Phase 3: User Story 2 — LLM-Governed Semantic Review (P1)

**Goal**: Canon orchestrates layer transitions and records layer completion. The LLM writes per-layer `output.md`.

### Tests for US2

- [x] T022 [P] [US2] [SC-006] Unit test: advance run state through all 5 layers (`diff_review_recorded` → `whole_file_review_recorded` → ... → `test_review_recorded`) in layer state machine tests.
- [x] T023 [P] [US2] Unit test: layer skip with valid reason accepted; layer skip without reason rejected.

### Implementation for US2

- [x] T024 [US2] Implement `advance_layer` method: validate current state, accept new state, persist `run-state.json` in `crates/canon-engine/src/review/onion.rs`.
- [x] T025 [US2] Implement `record_layer_skip` and `record_layer_failure` methods writing skip/failure records in `crates/canon-engine/src/review/onion.rs`.
- [x] T026 [US2] Canon records layer completion by advancing state; no separate `accept` needed per layer — layer transitions happen in sequence driven by operator or agent.

**Checkpoint**: Layer state machine fully operational with skip/failure records.

---

## Phase 4: User Story 3 — Reviewer Output Acceptance And Validation (P1)

**Goal**: `canon pr-review accept` validates the final `reviewer-output.md` and produces `canonical-review-output.json`.

### Tests for US3

- [x] T027 [P] [US3] Unit test: valid reviewer output JSON passes all validation checks in `tests/unit/pr_review_onion_unit.rs`.
- [x] T028 [P] [US3] [SC-004] Unit test: invalid JSON returns `actionable_review_failed`.
- [x] T029 [P] [US3] Unit test: duplicate comment IDs rejected.
- [x] T030 [P] [US3] Unit test: invalid severity value rejected.
- [x] T031 [P] [US3] Unit test: line target not in diff downgraded to hunk-level.
- [x] T032 [P] [US3] [SC-008] Unit test: layer coverage incomplete (missing layer terminal state) blocks acceptance.
- [x] T033 [P] [US3] Contract test: valid fixture `reviewer-output.md` accepted, invalid fixture rejected in `tests/contract/pr_review_onion_contract.rs`.

### Implementation for US3

- [x] T034 [US3] Implement `canon pr-review accept` CLI sub-command in `crates/canon-cli/src/commands/pr_review.rs`.
- [x] T035 [US3] Implement `ValidateReviewerOutput` with schema validation, comment ID uniqueness check, severity vocabulary check, recommendation vocabulary check in `crates/canon-engine/src/review/validate.rs`.
- [x] T036 [US3] Implement path validation: verify paths exist in changed/related files; downgrade invalid line comments to hunk-level or global in `crates/canon-engine/src/review/validate.rs`.
- [x] T037 [US3] Implement layer coverage check: all required layers must have terminal state (`completed`/`skipped_with_reason`/`failed`) before acceptance in `crates/canon-engine/src/review/validate.rs`.
- [x] T038 [US3] Parse LLM-authored `reviewer-output.md` (Markdown with structured sections) into `ReviewerOutput` struct in `crates/canon-engine/src/review/validate.rs`.
- [x] T039 [US3] Compile validated `ReviewerOutput` into `canonical-review-output.json` in `mode_pr_review_accept.rs`.
- [x] T040 [US3] Implement `mode_pr_review_accept` orchestrator: run validation, persist results, set run state to `reviewer_output_accepted` or `reviewer_output_rejected` in `crates/canon-engine/src/orchestrator/service/mode_pr_review_accept.rs`.
- [x] T041 [US3] Register `validate` module and `mode_pr_review_accept` module.

**Checkpoint**: `accept` command validates and persists reviewer output.

---

## Phase 5: User Story 4 — Reviewer-Facing Artifact Rendering (P1)

**Goal**: `canon pr-review finalize` renders all Markdown and JSON artifacts following the templates.

### Tests for US4

- [x] T042 [P] [US4] [SC-001] [SC-004] Contract test: `finalize` emits all 7 primary artifacts + `manifest.toml` + `packet-metadata.json` in `tests/contract/pr_review_onion_contract.rs`.
- [x] T043 [P] [US4] [SC-001] Contract test: `01-review-summary.md` follows `review-summary-template.md` structure.
- [x] T044 [P] [US4] [SC-001] Contract test: `02-conventional-comments.md` groups file comments by severity and path, global comments at end.
- [x] T045 [P] [US4] [SC-002] Contract test: `03-github-comments.json` and `02-conventional-comments.md` share same comment IDs and counts.
- [x] T046 [P] [US4] [SC-003] Contract test: `06-review-report.md` follows `review-report-template.md` structure.
- [x] T047 [P] [US4] Contract test: no generated artifact contains unresolved `{{ placeholder }}` values.
- [x] T048 [P] [US4] [SC-008] Unit test: `Request changes` with 0 blocking comments and 0 must-fix findings is rejected; also cover `Approve + must-fix findings present` and `Approve + actionable_review_failed` as forbidden inconsistent states.
- [x] T049 [P] [US4] [SC-005] Integration test: stub reviewer output with 2 blocking + 3 non-blocking comments produces correct recommendation `Request changes` and matching artifact content in `tests/integration/pr_review_onion_run.rs`.

### Implementation for US4

- [x] T050 [US4] Implement `canon pr-review finalize` CLI sub-command in `crates/canon-cli/src/commands/pr_review.rs`.
- [x] T051 [US4] Implement `render_review_summary` following `review-summary-template.md` in `crates/canon-engine/src/review/render.rs`.
- [x] T052 [US4] Implement `render_conventional_comments` following `conventional-comments-template.md` (grouped by severity+path, sorted lexicographically, global at end) in `crates/canon-engine/src/review/render.rs`.
- [x] T053 [US4] Implement `render_github_comments_json` from canonical comment set in `crates/canon-engine/src/review/render.rs`.
- [x] T054 [US4] Implement `render_review_report` following `review-report-template.md` in `crates/canon-engine/src/review/render.rs`.
- [x] T055 [US4] Implement `derive_recommendation` logic: blocking findings → Request changes; non-blocking or partial coverage → Comment; no findings + sufficient coverage → Approve in `crates/canon-engine/src/review/render.rs`.
- [x] T056 [US4] [SC-007] Implement governance notes rendering as secondary artifact in `crates/canon-engine/src/review/render.rs`.
- [x] T057 [US4] Implement `review-findings.json` with normalized findings linking to comment IDs.
- [x] T058 [US4] Implement `missing-tests.md` with concrete test gap analysis.
- [x] T059 [US4] Implement `mode_pr_review_finalize` orchestrator: build canonical comment set, render all artifacts, write output files, set run state to `finalized` in `crates/canon-engine/src/orchestrator/service/mode_pr_review_finalize.rs`.
- [x] T060 [US4] Register `render` module and `mode_pr_review_finalize` module.

**Checkpoint**: `finalize` command renders all artifacts following templates.

---

## Phase 6: Stub Reviewer Adapter (Cross-Cutting)

**Purpose**: Deterministic stub for testing end-to-end without live LLM.

- [x] T061 [P] Extend `StubReviewerAdapter` with per-layer findings: add `with_findings_for_layer(layer, findings)` builder method in `crates/canon-adapters/src/reviewer_stub.rs`.
- [x] T062 [P] Add `StubReviewerAdapter::full_onion_review(changed_files)` that returns findings across all 5 layers in `crates/canon-adapters/src/reviewer_stub.rs`.
- [x] T063 [SC-005] Integration test: full prepare→accept→finalize with stub reviewer producing non-empty comments end-to-end in `tests/integration/pr_review_onion_run.rs`.

**Checkpoint**: Stub adapter supports full onion review simulation.

---

## Phase 7: Optional Helper Commands (Deferred)

**Purpose**: Progressive context retrieval commands for LLM agent use.

**Unblocking criteria**: Depends on Phase 5 US4 (finalize pipeline stable). These commands are SHOULD (FR-041), not MUST. Safe to defer to a follow-up slice after the core prepare→accept→finalize pipeline is complete and tested.

- [x] T064 [P] Implement `canon pr-review context --list` in `crates/canon-cli/src/commands/pr_review.rs`.
- [x] T065 [P] Implement `canon pr-review context --show <id> [--range <start>..<end>]` in `crates/canon-cli/src/commands/pr_review.rs`.
- [x] T066 [P] Implement `canon pr-review context --related <id>` in `crates/canon-cli/src/commands/pr_review.rs`.
- [x] T067 [P] Implement `canon pr-review context --tests <id>` in `crates/canon-cli/src/commands/pr_review.rs`.
- [x] T068 [P] Implement `canon pr-review context --explain <id>` in `crates/canon-cli/src/commands/pr_review.rs`.

---

## Phase 8: Polish & Cross-Cutting

**Purpose**: Integration with existing pr-review infrastructure, cleanup, and final adjustments.

- [x] T069 Update `mode_pr_review.rs` to delegate to `prepare`/`accept`/`finalize` phased workflow; deprecate old single-run path.
- [x] T070 Update `gatekeeper` for new review-report.md sections in `crates/canon-engine/src/orchestrator/gatekeeper/`.
- [x] T071 Update `summarizers/governance.rs` for new primary artifact titles.
- [x] T072 Run quickstart validation from `specs/074-pr-review-onion-orchestration/quickstart.md`.
- [x] T073 Ensure all modified Rust files reference context IDs for traceability.
- [x] T074 [P] Add status projection for `pr-review` run state (layer progress, actionable review status) in `crates/canon-engine/src/orchestrator/service/summarizers/governance.rs` and `crates/canon-cli/src/output.rs`.
- [x] T075 [P] Add inspect projection for `pr-review` layer details (per-layer status, skip/failure records, coverage) in `crates/canon-cli/src/commands/inspect.rs` or inspect module.
- [x] T076 [P] Ensure `RefinementRoundCompleted`-equivalent trace events are emitted for each layer transition in `mode_pr_review.rs`.

---

## Final Phase: Release, Quality, And Verification

- [x] T077 Update `Cargo.toml` version from `0.71.1` to `0.72.2` (feature release, minor bump, reset patch) in `Cargo.toml`.
- [x] T078 Update `CHANGELOG.md` with `074-pr-review-onion-orchestration` highlights.
- [x] T079 Update `docs/`, `tech-docs/`, and `roadmap/` markdown references affected by this feature.
- [x] T080 Run `./scripts/update-docs-versions.sh`.
- [x] T081 Run `cargo fmt`.
- [x] T082 Run `scripts/clippy.sh` and fix all warnings.
- [x] T083 Run `scripts/test.sh` and fix failing tests.
- [x] T084 Run `scripts/coverage.sh` and confirm at least 95% coverage for every modified or created Rust file.
- [x] T085 Run `scripts/check-rust-no-panic.sh`.
- [x] T086 Run `./scripts/validate-assistant-plugins.sh` if assistant plugin metadata was touched.
- [x] T087 Verify no local filesystem paths are committed (manual check; `scripts/check-no-local-paths.sh` not present in Canon repo — add as blocking task if path leaks are found).
