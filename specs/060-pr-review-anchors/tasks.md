# Tasks: pr-review Optional Inline Anchors

**Input**: Design documents from `/specs/060-pr-review-anchors/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Validation**: Layered validation is mandatory. This slice changes the `pr-review`
review domain, rendered Conventional Comments, and reviewer guidance, so every
user story includes executable validation plus recorded evidence in
`specs/060-pr-review-anchors/validation-report.md`. Independent maintainer
review is required before closeout.

**Organization**: Tasks are grouped by user story to preserve independent
implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- Include exact file paths in descriptions

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls and evidence trails that authorize implementation.

- [X] T001 Confirm execution mode, risk classification, scope boundaries, and invariants remain aligned in `specs/060-pr-review-anchors/spec.md` and `specs/060-pr-review-anchors/plan.md`
- [X] T002 Update the implementation decision trail for anchor derivation and degradation in `specs/060-pr-review-anchors/decision-log.md`
- [X] T003 Create structural, logical, and independent-review evidence sections in `specs/060-pr-review-anchors/validation-report.md`
- [X] T004 Record independent maintainer review checkpoints and artifact evidence expectations in `specs/060-pr-review-anchors/validation-report.md` and `specs/060-pr-review-anchors/checklists/requirements.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare reusable validation scaffolding and anchor fixtures shared by all stories.

- [X] T005 Create contract-validation scaffolding for anchor shape and rendered comment expectations in `tests/contract/pr_review_anchor_contract.rs`
- [X] T006 [P] Create diff fixture inputs for line, span, cross-surface, and stale evidence in `tests/fixtures/pr_review_anchor_line.diff`, `tests/fixtures/pr_review_anchor_span.diff`, `tests/fixtures/pr_review_anchor_cross_surface.diff`, and `tests/fixtures/pr_review_anchor_stale.diff`
- [X] T007 [P] Add packet-sample fixture helpers and Conventional Comment artifact assertions in `tests/integration/pr_review_run.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared domain and diff-evidence primitives that MUST exist before any story work.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T008 Extend the typed anchor model and serde invariants on review findings in `crates/canon-engine/src/review/findings.rs`
- [X] T009 [P] Preserve zero-context diff capture and packet input plumbing for anchor derivation in `crates/canon-adapters/src/shell.rs` and `crates/canon-engine/src/orchestrator/service/mode_pr_review.rs`
- [X] T010 [P] Add shared contiguous-interval parsing and anchor-eligibility helpers in `crates/canon-engine/src/review/findings.rs`
- [X] T011 Add shared contract assertions for anchor-plus-scope coexistence in `tests/contract/pr_review_anchor_contract.rs` and `tests/integration/pr_review_run.rs`

**Checkpoint**: Foundation ready. The diff-backed pipeline can carry optional anchors without weakening scope or no-fabrication invariants.

---

## Phase 3: User Story 1 - Reviewer Sees Precise Anchors When Evidence Exists (Priority: P1) 🎯 MVP

**Goal**: Emit visible line and span anchors for findings backed by one concrete changed surface and one contiguous diff interval.

**Independent Test**: Run `pr-review` against fixture diffs that produce one line anchor and one span anchor, then verify the rendered Conventional Comments artifact includes explicit scope plus visible anchor text for both findings.

### Validation for User Story 1

- [X] T012 [P] [US1] Add failing unit coverage for single-line and contiguous-span anchor derivation in `crates/canon-engine/src/review/findings.rs`
- [X] T013 [P] [US1] Add failing integration coverage for anchored Conventional Comments output in `tests/integration/pr_review_run.rs`
- [X] T014 [US1] Record positive anchor derivation and rendering decisions in `specs/060-pr-review-anchors/decision-log.md`

### Implementation for User Story 1

- [X] T015 [US1] Populate `ReviewFinding` anchors from one-surface contiguous diff hunks in `crates/canon-engine/src/review/findings.rs`
- [X] T016 [US1] Render `surface:start` and `surface:start-end` anchor text alongside explicit scope in `crates/canon-engine/src/artifacts/markdown/governance.rs`
- [X] T017 [US1] Align positive line and span examples in `specs/060-pr-review-anchors/contracts/conventional-comment-anchor-contract.md` and `specs/060-pr-review-anchors/data-model.md`
- [X] T018 [US1] Capture anchored packet evidence and reviewer notes in `specs/060-pr-review-anchors/validation-report.md`

**Checkpoint**: User Story 1 should deliver visible, host-agnostic anchors for evidence-backed findings without removing explicit scope.

---

## Phase 4: User Story 2 - Reviewer Gets Honest Degradation When Precision Is Missing (Priority: P2)

**Goal**: Omit anchors whenever evidence is stale, ambiguous, cross-surface, or otherwise insufficient, while preserving scope-only comments and unchanged readiness behavior.

**Independent Test**: Run `pr-review` against cross-surface, disjoint, empty-surface, and stale-coordinate fixtures, then verify the emitted comments keep explicit scope, omit inline anchors, and leave review summary behavior unchanged.

### Validation for User Story 2

- [X] T019 [P] [US2] Add failing unit coverage for cross-surface, disjoint, empty-surface, and stale-anchor rejection in `crates/canon-engine/src/review/findings.rs`
- [X] T020 [P] [US2] Add failing contract and integration coverage for degraded scope-only comments and historical/imported packets lacking anchor evidence in `tests/contract/pr_review_anchor_contract.rs` and `tests/integration/pr_review_run.rs`
- [X] T021 [US2] Record no-fabrication and degradation decisions in `specs/060-pr-review-anchors/decision-log.md`

### Implementation for User Story 2

- [X] T022 [US2] Omit anchors for ambiguous, stale, cross-surface, disjoint, and historical no-evidence packets in `crates/canon-engine/src/review/findings.rs`
- [X] T023 [US2] Preserve unchanged review summary and readiness behavior in `crates/canon-engine/src/review/summary.rs` and `tests/integration/pr_review_run.rs`
- [X] T024 [US2] Align degradation rules and rejection conditions in `specs/060-pr-review-anchors/contracts/conventional-comment-anchor-contract.md` and `specs/060-pr-review-anchors/data-model.md`
- [X] T025 [US2] Capture degraded and historical-packet evidence plus invariant checks in `specs/060-pr-review-anchors/validation-report.md`

**Checkpoint**: User Story 2 should prove that Canon never fabricates inline precision and that scope-only output remains trustworthy when anchors are unavailable.

---

## Phase 5: User Story 3 - Published Packet Stays Portable Outside the Code Host (Priority: P3)

**Goal**: Keep anchored and unanchored Conventional Comments readable in published packets without host-specific syntax or tooling.

**Independent Test**: Publish a packet with anchored and degraded comments, then confirm a second reader can understand the scope, target surface, and anchor bounds from the artifact text alone.

### Validation for User Story 3

- [X] T026 [P] [US3] Add failing readability coverage for host-agnostic anchor text in `tests/contract/pr_review_anchor_contract.rs` and `tests/integration/pr_review_run.rs`
- [X] T027 [US3] Record published-packet readability decisions in `specs/060-pr-review-anchors/decision-log.md`

### Implementation for User Story 3

- [X] T028 [US3] Refine evidence-posture and Conventional Comment wording for anchored versus scope-only output in `crates/canon-engine/src/artifacts/markdown/governance.rs`
- [X] T029 [P] [US3] Update reviewer guidance mirrors for anchored and degraded comments in `.agents/skills/canon-pr-review/SKILL.md` and `defaults/embedded-skills/canon-pr-review/skill-source.md`
- [X] T030 [US3] Capture published packet samples and independent reader notes in `specs/060-pr-review-anchors/validation-report.md`

**Checkpoint**: User Story 3 should make published review artifacts self-explanatory for anchored and unanchored comments alike.

---

## Final Phase: Verification & Compliance

**Purpose**: Complete cross-cutting validation, independent review, and invariant closeout.

- [X] T031 [P] Run structural validation for `crates/canon-engine/src/review/findings.rs`, `crates/canon-engine/src/artifacts/markdown/governance.rs`, and `tests/contract/pr_review_anchor_contract.rs`, and record results in `specs/060-pr-review-anchors/validation-report.md`
- [X] T032 [P] Run focused and workspace Rust test validation for `crates/canon-engine/src/review/findings.rs`, `crates/canon-engine/src/review/summary.rs`, `tests/contract/pr_review_anchor_contract.rs`, and `tests/integration/pr_review_run.rs`, and record results in `specs/060-pr-review-anchors/validation-report.md`
- [X] T033 [P] Run coverage validation with `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` for `crates/canon-engine/src/review/findings.rs`, `crates/canon-engine/src/artifacts/markdown/governance.rs`, and `tests/integration/pr_review_run.rs`, and record results in `lcov.info` and `specs/060-pr-review-anchors/validation-report.md`
- [X] T034 [P] Update repository docs for anchored versus scope-only `pr-review` behavior in `README.md` and `tech-docs/guides/modes.md`
- [X] T035 [P] Update companion wiki guidance for anchored versus scope-only `pr-review` behavior in `../canon.wiki/Canon-Modes.md`, `../canon.wiki/Example-Flow-Code-Review.md`, and `../canon.wiki/Reference.md`
- [X] T036 Bump the Canon workspace version and release notes in `Cargo.toml`, `Cargo.lock`, and `CHANGELOG.md`
- [X] T037 Perform independent maintainer packet review against `specs/060-pr-review-anchors/contracts/conventional-comment-anchor-contract.md` and close findings in `specs/060-pr-review-anchors/validation-report.md`
- [X] T038 Confirm scope, no-fabrication, and primary-artifact invariants in `specs/060-pr-review-anchors/quickstart.md` and `specs/060-pr-review-anchors/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies and MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user stories.
- **User Stories (Phases 3-5)**: Depend on Foundational completion.
- **Verification & Compliance (Final Phase)**: Depends on all selected user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Starts after Foundational and is the MVP.
- **User Story 2 (P2)**: Starts after User Story 1's anchor model is stable because degradation rules build on the same typed anchor and renderer surfaces.
- **User Story 3 (P3)**: Starts after User Stories 1 and 2 because packet readability depends on both positive anchor rendering and honest degradation behavior.

### Within Each User Story

- Validation tasks MUST fail before implementation when the behavior is executable.
- Decision-log updates happen before story sign-off.
- Domain-model changes happen before renderer or guidance changes.
- Evidence capture is required before the story is declared complete.

### Parallel Opportunities

- Setup and Foundational tasks marked `[P]` can run in parallel.
- Validation tasks within each user story marked `[P]` can run in parallel.
- After Foundational completion, document alignment tasks can run in parallel with code changes when they touch different files.

---

## Parallel Example: User Story 1

```bash
# Launch anchor derivation and rendered-output checks together:
Task: "Add failing unit coverage for single-line and contiguous-span anchor derivation in crates/canon-engine/src/review/findings.rs"
Task: "Add failing integration coverage for anchored Conventional Comments output in tests/integration/pr_review_run.rs"
```

## Parallel Example: User Story 2

```bash
# Launch degradation checks together:
Task: "Add failing unit coverage for cross-surface, disjoint, empty-surface, and stale-anchor rejection in crates/canon-engine/src/review/findings.rs"
Task: "Add failing contract and integration coverage for degraded scope-only comments in tests/contract/pr_review_anchor_contract.rs and tests/integration/pr_review_run.rs"
```

## Parallel Example: User Story 3

```bash
# Launch readability validation and guidance updates together:
Task: "Add failing readability coverage for host-agnostic anchor text in tests/contract/pr_review_anchor_contract.rs and tests/integration/pr_review_run.rs"
Task: "Update reviewer guidance mirrors for anchored and degraded comments in .agents/skills/canon-pr-review/SKILL.md and defaults/embedded-skills/canon-pr-review/skill-source.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. Stop and validate anchored output before proceeding.

### Incremental Delivery

1. Deliver US1 for positive line/span anchors.
2. Add US2 for honest degradation and readiness-preserving fallback.
3. Add US3 for published-packet readability and reviewer guidance.
4. Finish with independent review and final invariant closeout.

### Parallel Team Strategy

1. One contributor stabilizes the typed anchor model and diff parsing in `crates/canon-engine/src/review/findings.rs`.
2. A second contributor prepares contract and integration validation in `tests/contract/pr_review_anchor_contract.rs` and `tests/integration/pr_review_run.rs`.
3. A third contributor can update reviewer guidance mirrors once renderer wording stabilizes.

---

## Notes

- `[P]` tasks touch different files and do not depend on unfinished work.
- `[US#]` labels keep traceability back to the feature specification.
- Every story remains independently testable with explicit evidence capture.
- Do not weaken the existing explicit scope contract or fabricate inline precision.
