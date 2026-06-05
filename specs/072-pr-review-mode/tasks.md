# Tasks: Refactor Canon pr-review Into An Actionable Code Review Mode

**Input**: Design documents from `/specs/072-pr-review-mode/`

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/cli.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 0: Governance & Artifacts

**Purpose**: Governance gates required by the Bounded-Impact risk classification.

- [ ] T001 Ensure execution mode (`pr-review`) and risk classification (`Bounded-Impact`) are declared.
- [ ] T002 Ensure scope invariants (governance tracking persists; Approve decision blocked by findings) are verified before implementation.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure.

- [ ] T003 Ensure Cargo dependencies (`serde`, `serde_json`) are active for the `canon-engine` workspace.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data structures that MUST be complete before ANY user story can be implemented.

- [x] T004 [P] Create `ReviewFinding` and `GithubComment` strict `serde` structs in `crates/canon-engine/src/review/findings.rs`.
- [x] T005 [P] Create `MissingTest` and `ReviewCoverage` strict `serde` structs in `crates/canon-engine/src/review/findings.rs`.

**Checkpoint**: Data model foundations are ready for review generation.

---

## Phase 3: User Story 1 - Actionable PR Review Generation (Priority: P1) 🎯 MVP

**Goal**: Generate structured JSON and Markdown artifacts mapping exactly to diff lines, with explicit sampling logic for large diffs.

**Independent Test**: Provide a PR diff containing known syntax flaws; verify `github-comments.json` contains blocking items with exact line numbers, and `review-summary.md` renders the Decision correctly.

### Tests for User Story 1 ⚠️

- [x] T006 [P] [US1] Unit test for deterministic mapping of hallucinated line numbers to hunks in `crates/canon-engine/src/review/diff.rs`.
- [x] T007 [P] [US1] Integration test parsing a mock diff to extract `github-comments.json` matching the schema in `crates/canon-engine/tests/review_evaluator_tests.rs`.

### Implementation for User Story 1

- [x] T008 [US1] Implement deterministic diff mapping and parsing in `crates/canon-engine/src/review/diff.rs`.
- [x] T009 [US1] Implement LLM extraction logic using the new structs, including large-diff sampling thresholds (>20 files/500 lines) in `crates/canon-engine/src/review/evaluator.rs`.
- [x] T010 [P] [US1] Implement markdown generators for `review-summary.md`, `conventional-comments.md`, and `missing-tests.md` in `crates/canon-engine/src/review/generators.rs`.
- [-] T015 [US3] Add `.github/PULL_REQUEST_TEMPLATE.md` to map `conventional-comments.md` content via CI pipeline logic. (Skipped/Reverted: repository configuration modification is out of scope for the CLI tool)
- [x] T016 [US3] Update `defaults/methods/pr-review.toml` to enforce JSON extraction formatting guidelines.
- [x] T017 [US3] Update `defaults/embedded-skills/canon-pr-review/skill-source.md` with system prompt to guide LLM for outputting correct JSON structure.
- [x] T011 [US1] Update `canon pr-review` CLI orchestrator to output `review-findings.json`, `github-comments.json`, etc. in `crates/canon-cli/src/commands/pr_review.rs` (mapped to `crates/canon-engine/src/orchestrator/service/mode_pr_review.rs`).

**Checkpoint**: At this point, User Story 1 should be fully functional and emit exact line-matched reviews.

---

## Phase 4: User Story 2 - Governance Preservation (Priority: P2)

**Goal**: Ensure the legacy governance audits (evidence readiness, blocking decisions) remain intact as secondary outputs.

**Independent Test**: Provide a diff failing basic gating and ensure `state.toml` marks it as blocked, and `Approve` is not emitted.

### Tests for User Story 2 ⚠️

- [x] T012 [P] [US2] Engine test asserting `Decision::Approve` is NEVER returned if `blocking: true` is present in any finding in `crates/canon-engine/tests/review_evaluator_tests.rs`.
- [x] T018 [P] [US2] Engine test asserting `MissingTest` findings without an explicit `affected_behavior` mapping are rejected or flagged (SC-003).

### Implementation for User Story 2

- [x] T013 [P] [US2] Retain and pipe secondary governance artifacts (`state.toml`, `evidence.toml`) naturally handled by Canon.
- [x] T014 [US2] Enforce blocking logic in evaluator to map `blocking` true to a `Request changes` state in `crates/canon-engine/src/review/evaluator.rs`.

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently.

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories.

- [ ] T015 Run quickstart validation from `specs/072-pr-review-mode/quickstart.md` locally against a test branch.
- [ ] T016 Ensure `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes cleanly.
- [ ] T017 Ensure `cargo test` suite passes successfully with all new deterministic validation tests.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Can start immediately.
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories.
- **User Stories (Phase 3+)**: All depend on Foundational phase completion. User Story 1 (P1) should be implemented first, followed by User Story 2 (P2).
- **Polish (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2).
- **User Story 2 (P2)**: Integrates into the same modules as US1 but adds governance enforcement. Should start after US1 is fundamentally working.
