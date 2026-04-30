# Tasks: Distribution Channels Beyond GitHub Releases

**Input**: Design documents from `/specs/025-distribution-channels/`
**Prerequisites**: plan.md (required), spec.md (required for user stories),
research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks
whenever behavior, interfaces, or regressions must be checked. Independent
review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent
implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature starts with current governance artifacts and validation owners.
- No release workflow mutation starts before metadata, invariants, and evidence
  checkpoints are current.
- Each user story includes failing or focused validation tasks before
  implementation tasks.
- Independent review and evidence capture close the feature after executable
  validation.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that permit implementation to start.

- [x] T001 Update implementation-phase decisions in `specs/025-distribution-channels/decision-log.md`
- [x] T002 Update validation checkpoints and independent-review expectations in `specs/025-distribution-channels/validation-report.md`
- [x] T003 [P] Refresh Homebrew release walkthrough expectations in `specs/025-distribution-channels/quickstart.md`
- [x] T004 Confirm scope boundaries and invariants remain current in `specs/025-distribution-channels/spec.md` and `specs/025-distribution-channels/plan.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the scaffolding required by the distribution-channel slice.

- [x] T005 Update the workspace release version to `0.25.0` in `Cargo.toml`
- [x] T006 [P] Create the Homebrew template scaffold in `packaging/homebrew/canon.rb.tpl`
- [x] T007 [P] Create the focused release-surface test scaffold in `tests/release_025_distribution.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared release-metadata foundation that every user story
depends on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [x] T008 [P] Add failing distribution-metadata contract assertions in `tests/release_025_distribution.rs`
- [x] T009 Implement verified metadata emission in `scripts/release/write-distribution-metadata.sh`
- [x] T010 Extend release-bundle verification for metadata artifacts in `scripts/release/verify-release-surface.sh`
- [x] T011 Wire distribution metadata generation into `.github/workflows/release.yml`
- [x] T012 Update foundational release-contract notes in `specs/025-distribution-channels/decision-log.md` and `specs/025-distribution-channels/validation-report.md`

**Checkpoint**: The canonical release bundle now produces validated
channel-neutral metadata that all channel work can consume.

---

## Phase 3: User Story 1 - Install Canon Through Homebrew (Priority: P1) 🎯 MVP

**Goal**: Deliver a real Homebrew install path for macOS and Linux users while
keeping the direct archive fallback visible.

**Independent Test**: From a valid release metadata artifact, render the
Homebrew formula, validate the platform mappings, and confirm the install docs
describe the Homebrew path alongside the direct-download fallback.

### Validation for User Story 1 (MANDATORY)

- [x] T013 [P] [US1] Add failing Homebrew formula platform-mapping and smoke-test assertions in `tests/release_025_distribution.rs`
- [x] T014 [US1] Record Homebrew install-surface and fallback decisions in `specs/025-distribution-channels/decision-log.md`

### Implementation for User Story 1

- [x] T015 [US1] Complete the generated Homebrew formula template in `packaging/homebrew/canon.rb.tpl`
- [x] T016 [US1] Implement formula rendering from distribution metadata in `scripts/release/render-homebrew-formula.sh`
- [x] T017 [US1] Update Homebrew plus direct-download install guidance in `README.md`
- [x] T018 [US1] Capture User Story 1 evidence in `specs/025-distribution-channels/validation-report.md`

**Checkpoint**: User Story 1 is independently functional and documented.

---

## Phase 4: User Story 2 - Publish A Canonical Homebrew Update From The Release Flow (Priority: P2)

**Goal**: Let maintainers generate and publish or export the Homebrew update as
part of the release flow instead of editing formula data manually.

**Independent Test**: From a verified release bundle, the workflow emits both a
metadata artifact and a formula artifact, and preserves an artifact-only
fallback when tap publication is not configured.

### Validation for User Story 2 (MANDATORY)

- [x] T019 [P] [US2] Add failing workflow and artifact-only fallback assertions in `tests/release_025_distribution.rs`
- [x] T020 [US2] Record tap-publication and artifact-only fallback decisions in `specs/025-distribution-channels/decision-log.md`

### Implementation for User Story 2

- [x] T021 [US2] Wire formula artifact generation and upload into `.github/workflows/release.yml`
- [x] T022 [P] [US2] Implement tap synchronization and fallback handling in `scripts/release/sync-homebrew-tap.sh`
- [x] T023 [US2] Update release notes and release verification expectations for distribution artifacts in `.github/release-notes-template.md` and `scripts/release/verify-release-surface.sh`
- [x] T024 [US2] Capture User Story 2 evidence in `specs/025-distribution-channels/validation-report.md`

**Checkpoint**: Maintainers can generate the Homebrew update from the release
flow with a durable fallback path.

---

## Phase 5: User Story 3 - Verify Release And Install Surfaces Stay Consistent (Priority: P3)

**Goal**: Keep metadata, formula, docs, and roadmap references aligned so later
distribution channels can reuse the same contract safely.

**Independent Test**: Compare the release metadata artifact, rendered formula,
install docs, and roadmap references and confirm they stay aligned with the
verified release bundle.

### Validation for User Story 3 (MANDATORY)

- [x] T025 [P] [US3] Add failing metadata, formula, and docs consistency assertions in `tests/release_025_distribution.rs`
- [x] T026 [US3] Record future-channel compatibility decisions in `specs/025-distribution-channels/decision-log.md`

### Implementation for User Story 3

- [x] T027 [US3] Preserve channel-neutral metadata examples and invariants in `specs/025-distribution-channels/contracts/release-metadata.md` and `scripts/release/write-distribution-metadata.sh`
- [x] T028 [US3] Update roadmap and release-surface references in `ROADMAP.md` and `README.md`
- [x] T029 [US3] Capture User Story 3 evidence and reviewer notes in `specs/025-distribution-channels/validation-report.md`

**Checkpoint**: Metadata, formula, docs, and roadmap references stay aligned.

---

## Final Phase: Verification & Compliance

**Purpose**: Execute cross-cutting validation, document outcomes, and close the
feature safely.

- [x] T030 [P] Run focused release-distribution tests and record results in `specs/025-distribution-channels/validation-report.md`
- [x] T031 [P] Run shell syntax checks for `scripts/release/write-distribution-metadata.sh`, `scripts/release/render-homebrew-formula.sh`, and `scripts/release/sync-homebrew-tap.sh` and record results in `specs/025-distribution-channels/validation-report.md`
- [x] T032 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` and record results in `specs/025-distribution-channels/validation-report.md`
- [x] T033 Perform `brew install --formula` smoke validation when Homebrew is available, otherwise record the explicit environment gap, and close independent artifact review in `specs/025-distribution-channels/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. Must complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational completion. This is the MVP.
- **User Story 2 (Phase 4)**: Depends on Foundational completion and reuses the US1 formula surface.
- **User Story 3 (Phase 5)**: Depends on Foundational completion and validates the full surface after US1 and US2 land.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Starts after metadata generation is in place.
- **User Story 2 (P2)**: Reuses the formula surface delivered by US1 and the metadata emitted in Phase 2.
- **User Story 3 (P3)**: Validates consistency across the artifacts created by US1 and US2.

### Within Each User Story

- Validation tasks happen before implementation tasks.
- Decision-log updates happen before any behavior that depends on those
  decisions is finalized.
- Generated artifact logic is implemented before docs and evidence capture.
- Evidence capture happens before the story is considered complete.

### Parallel Opportunities

- T003 can run in parallel with T001-T002 once governance edits begin.
- T006 and T007 can run in parallel during Setup.
- T008 can run before T009-T011 and then be revisited as implementation lands.
- T013, T019, and T025 are independent validation updates to the same focused
  release test file and should be handled sequentially, but each can run in
  parallel with its story's decision-log task.
- T022 can run in parallel with T021 because it targets a separate script while
  the workflow wiring lands.
- T030-T032 can run in parallel during closeout.

---

## Parallel Example: User Story 2

```bash
# After foundational metadata work is complete:
Task: "Wire formula artifact generation and upload into .github/workflows/release.yml"
Task: "Implement tap synchronization and fallback handling in scripts/release/sync-homebrew-tap.sh"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational metadata generation and verification.
4. Complete Phase 3: User Story 1.
5. **Stop and validate**: prove the generated formula and install docs are
   independently correct before starting tap publication work.

### Incremental Delivery

1. Land governance, setup, and foundational metadata work.
2. Add Homebrew install delivery (US1) and validate it independently.
3. Add release-flow publication and fallback behavior (US2) and validate it.
4. Add cross-surface consistency and future-channel readiness (US3).
5. Finish with the Verification & Compliance phase.

### Parallel Team Strategy

With multiple developers:

1. One developer owns release metadata generation and verification.
2. One developer owns formula template and install-doc updates.
3. One developer owns tap synchronization and final release workflow wiring
   after the metadata contract stabilizes.

---

## Notes

- Keep the distribution metadata contract channel-neutral even while only
  shipping Homebrew behavior.
- Do not replace the direct-download install path.
- Do not start tap publication work before the metadata and formula surfaces
  validate locally.
- Keep decision-log and validation-report entries current as tasks close.