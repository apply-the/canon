# Tasks: PR Review Conventional Comments

**Input**: Design documents from `/specs/013-pr-review-comments/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/conventional-comments-artifact.md`, `quickstart.md`

**Validation**: Layered validation is mandatory. Contract, integration, publish/readability, and independent review evidence must be recorded in `specs/013-pr-review-comments/validation-report.md`.

**Organization**: Tasks are grouped by user story so Conventional Comments rendering, approval-safe behavior, and publish/docs cleanup can each be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

## Constitution Alignment

- Governance and decision artifacts stay current before implementation begins.
- No artifact-shape change lands before the invariants around review disposition, traceability, and host-agnostic output are explicitly recorded.
- Every user story includes validation and evidence capture.
- Independent validation remains separate from generation and closes in the final phase.

## Phase 0: Governance & Artifacts

- [x] T001 Reconfirm mode, risk, scope boundaries, invariants, and validation ownership in `specs/013-pr-review-comments/spec.md` and `specs/013-pr-review-comments/plan.md`
- [x] T002 Refresh the mapping, disposition, and publish/readability decisions in `specs/013-pr-review-comments/decision-log.md` and `specs/013-pr-review-comments/validation-report.md`

---

## Phase 1: Setup (Shared Infrastructure)

- [x] T003 Create a dedicated publish/readability integration test entrypoint in `tests/integration/pr_review_publish.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add the deterministic mapping and additive artifact contract that all user stories depend on.

**⚠️ CRITICAL**: No user story work starts before these tasks are complete.

- [x] T004 [P] Add failing artifact-contract coverage for `conventional-comments.md` in `tests/contract/pr_review_contract.rs`
- [x] T005 [P] Add failing mapping and renderer coverage in `crates/canon-engine/src/review/findings.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T006 Implement deterministic finding-to-comment-kind mapping helpers in `crates/canon-engine/src/review/findings.rs`
- [x] T007 Implement the additive `conventional-comments.md` artifact contract in `crates/canon-engine/src/artifacts/contract.rs`

**Checkpoint**: The runtime has a deterministic mapping layer and a declared additive artifact contract before packet-level implementation begins.

---

## Phase 3: User Story 1 - Reviewer Reads Conventional Comments Packet (Priority: P1) 🎯 MVP

**Goal**: Emit a Conventional Comments shaped artifact from persisted `pr-review` findings without losing reviewer-facing traceability.

**Independent Test**: Run `canon run --mode pr-review ...` against a diff with findings and verify the emitted packet includes `conventional-comments.md` with valid kinds and changed-surface traceability.

### Validation for User Story 1

- [x] T008 [P] [US1] Add failing packet and artifact assertions in `tests/pr_review_run.rs` and `tests/integration/pr_review_run.rs`
- [x] T009 [US1] Record artifact-shape and traceability decisions in `specs/013-pr-review-comments/decision-log.md`

### Implementation for User Story 1

- [x] T010 [US1] Implement `conventional-comments.md` rendering in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T011 [US1] Wire additive packet emission and mode-result expectations in `crates/canon-engine/src/orchestrator/service/mode_pr_review.rs` and `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [x] T012 [US1] Capture User Story 1 validation evidence in `specs/013-pr-review-comments/validation-report.md`

**Checkpoint**: `pr-review` emits a readable Conventional Comments artifact and the feature has an independently testable MVP.

---

## Phase 4: User Story 2 - Approval Workflow Keeps Existing Semantics (Priority: P2)

**Goal**: Keep `review-summary.md` primary and preserve unresolved must-fix disposition behavior while the new artifact is present.

**Independent Test**: Run `pr-review` on a high-impact diff and verify the run still waits for explicit disposition while exposing the new artifact.

### Validation for User Story 2

- [x] T013 [P] [US2] Add failing approval-preservation assertions in `tests/pr_review_run.rs` and `tests/contract/pr_review_contract.rs`
- [x] T014 [US2] Record disposition-preservation decisions in `specs/013-pr-review-comments/decision-log.md`

### Implementation for User Story 2

- [x] T015 [US2] Preserve `review-summary.md` as the primary status/next-step artifact while exposing the additive artifact in `crates/canon-engine/src/orchestrator/service/summarizers.rs` and `crates/canon-cli/src/output.rs`
- [x] T016 [US2] Capture User Story 2 validation evidence in `specs/013-pr-review-comments/validation-report.md`

**Checkpoint**: Approval-gated `pr-review` behavior remains unchanged and explicitly validated.

---

## Phase 5: User Story 3 - Published Packet Is Readable Outside Canon (Priority: P3)

**Goal**: Publish a readable Conventional Comments artifact and update docs/skills/roadmap to reflect the delivered slice.

**Independent Test**: Publish a completed `pr-review` run and verify `docs/reviews/prs/<RUN_ID>/conventional-comments.md` is readable without hidden runtime state.

### Validation for User Story 3

- [x] T017 [P] [US3] Add failing publish/readability assertions in `tests/integration/pr_review_publish.rs`
- [x] T018 [US3] Record publish-surface and roadmap-cleanup decisions in `specs/013-pr-review-comments/decision-log.md`

### Implementation for User Story 3

- [x] T019 [US3] Update user-facing `pr-review` guidance in `README.md`, `defaults/embedded-skills/canon-pr-review/skill-source.md`, and `.agents/skills/canon-pr-review/SKILL.md`
- [x] T020 [US3] Update `NEXT_FEATURES.md` to move delivered `pr-review` Conventional Comments work out of the roadmap and leave the remaining architecture C4 slice explicit
- [x] T021 [US3] Capture User Story 3 publish/readability evidence in `specs/013-pr-review-comments/validation-report.md`

**Checkpoint**: The Conventional Comments slice is shipped, documented, and removed from the roadmap as delivered work.

---

## Final Phase: Verification & Compliance

- [x] T022 [P] Run structural validation with `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`; record results in `specs/013-pr-review-comments/validation-report.md`
- [x] T023 [P] Run logical validation with targeted `pr-review` suites, including `tests/contract/pr_review_contract.rs`, `tests/pr_review_run.rs`, `tests/integration/pr_review_run.rs`, and `tests/integration/pr_review_publish.rs`; record results in `specs/013-pr-review-comments/validation-report.md`
- [x] T024 Perform independent packet review for Conventional Comments readability and close `specs/013-pr-review-comments/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: no dependencies; must complete first.
- **Phase 1: Setup**: depends on Phase 0.
- **Phase 2: Foundational**: depends on Phase 1 and blocks all user stories.
- **Phase 3: US1**: depends on Phase 2.
- **Phase 4: US2**: depends on US1 because summary/approval preservation relies on the additive artifact being present.
- **Phase 5: US3**: depends on US1 and US2 because docs and roadmap cleanup should only happen once the artifact and approval semantics are validated.
- **Final Phase**: depends on all desired user stories being complete.

### User Story Completion Order

- **US1 (P1)**: MVP. Emit the Conventional Comments artifact.
- **US2 (P2)**: Preserve existing approval and summary semantics.
- **US3 (P3)**: Publish the new artifact and update docs/roadmap.

### Within Each User Story

- Validation tasks land before implementation tasks.
- Decision-log updates happen before the affected behavior is declared complete.
- Evidence capture is required before the story checkpoint closes.

---

## Parallel Examples

### Foundational Phase

```bash
# Validation-first work can start in parallel:
T004 Add failing artifact-contract coverage
T005 Add failing mapping and renderer coverage
```

### User Story 1

```bash
# Story-level verification can start in parallel with decision recording:
T008 Add failing packet assertions
T009 Record artifact-shape decisions
```

### User Story 3

```bash
# Publish coverage and roadmap/docs planning can proceed in parallel:
T017 Add failing publish/readability assertions
T018 Record publish-surface and roadmap-cleanup decisions
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 0, Phase 1, and Phase 2.
2. Deliver US1 and validate the additive artifact end to end.
3. Stop and confirm the emitted packet is readable before changing docs or roadmap surfaces.

### Incremental Delivery

1. Add deterministic mapping and artifact contract.
2. Emit the Conventional Comments artifact.
3. Prove approval semantics are unchanged.
4. Update docs/skills and clean the delivered roadmap slice.
5. Finish with structural, logical, and independent validation.

## Notes

- Total tasks: 24
- User-story task counts: US1 = 5, US2 = 4, US3 = 5
- Parallel opportunities identified: foundational validation, story-level failing checks, and publish/docs planning
- Suggested MVP scope: through Phase 3 (US1) only