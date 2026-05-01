# Tasks: Structured External Publish Destinations

**Input**: Design documents from `/specs/029-publish-destinations/`
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

- Every feature MUST start with mode, risk, scope, and invariant artifact tasks.
- No implementation task may appear before the artifacts that authorize it.
- Every user story MUST include validation tasks and evidence capture.
- Systemic-impact work MUST include an independent review task separate from
  generation.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the publish-contract and release boundary that permit implementation to start

- [x] T001 Set Canon version to `0.29.0` in `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-scope decisions and publish-boundary notes in `specs/029-publish-destinations/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/029-publish-destinations/validation-report.md`
- [x] T004 Confirm the publish-destination, published-metadata, and release-alignment contracts in `specs/029-publish-destinations/contracts/publish-destination-contract.md`, `specs/029-publish-destinations/contracts/published-packet-metadata.md`, and `specs/029-publish-destinations/contracts/release-alignment.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and release-regression setup

- [x] T005 Update agent context from `specs/029-publish-destinations/plan.md` into `AGENTS.md`
- [x] T006 Create release-surface regression coverage in `tests/release_029_publish.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared publish-path behavior that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 [P] Create or update failing default-publish contract coverage in `crates/canon-engine/src/orchestrator/publish.rs`, `tests/contract/runtime_filesystem.rs`, and `tests/integration/run_lookup.rs`
- [x] T008 [P] Create or update failing publish-output coverage for default summaries and packet materialization in `crates/canon-cli/src/commands/publish.rs`, `tests/integration/pr_review_publish.rs`, `tests/integration/incident_publish.rs`, and `tests/integration/migration_publish.rs`
- [x] T009 Extend shared publish destination resolution, descriptor derivation, and summary reporting in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T010 Capture foundational invariants, focused publish targets, and touched-Rust-file coverage expectations in `specs/029-publish-destinations/validation-report.md`

**Checkpoint**: Shared publish destination support is ready for story-level rollout

---

## Phase 3: User Story 1 - Browse Published Packets By Meaningful Path (Priority: P1) 🎯 MVP

**Goal**: Deliver readable default publish destinations without changing override semantics or publish eligibility

**Independent Test**: Publish one completed run without `--to` and verify the packet lands under the correct family root using a date-prefixed descriptor path rather than a run-id-only directory.

### Validation for User Story 1 (MANDATORY)

- [x] T011 [P] [US1] Add failing structured-path coverage in `crates/canon-engine/src/orchestrator/publish.rs`, `tests/contract/runtime_filesystem.rs`, `tests/integration/pr_review_publish.rs`, and `tests/integration/run_lookup.rs`
- [x] T012 [US1] Record story-specific destination-shape decisions under `## User Story 1 Decisions` in `specs/029-publish-destinations/decision-log.md`

### Implementation for User Story 1

- [x] T013 [P] [US1] Implement date-prefixed descriptor destination resolution in `crates/canon-engine/src/orchestrator/publish.rs` and reuse persisted descriptor inputs from `crates/canon-engine/src/persistence/manifests.rs`
- [x] T014 [US1] Update publish command expectations and default-path integration assertions in `crates/canon-cli/src/commands/publish.rs`, `tests/integration/pr_review_publish.rs`, `tests/integration/incident_publish.rs`, `tests/integration/migration_publish.rs`, and `tests/integration/run_lookup.rs`
- [x] T015 [US1] Capture structured-destination validation evidence in `specs/029-publish-destinations/validation-report.md`

**Checkpoint**: Default publish paths are human-browsable and independently validated

---

## Phase 4: User Story 2 - Recover Traceability From Published Metadata (Priority: P2)

**Goal**: Deliver published-packet metadata that keeps canonical run identity and lineage recoverable outside `.canon/`

**Independent Test**: Publish one packet and verify the published output includes durable metadata for run id, mode, risk, zone, publish timestamp, descriptor, destination, and source artifact lineage.

### Validation for User Story 2 (MANDATORY)

- [x] T016 [P] [US2] Add failing metadata traceability coverage in `crates/canon-engine/src/orchestrator/publish.rs`, `tests/contract/runtime_filesystem.rs`, `tests/integration/incident_publish.rs`, and `tests/integration/migration_publish.rs`
- [x] T017 [US2] Record story-specific metadata decisions under `## User Story 2 Decisions` in `specs/029-publish-destinations/decision-log.md`

### Implementation for User Story 2

- [x] T018 [P] [US2] Implement published-packet metadata materialization and summary enrichment in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T019 [US2] Update publish integration assertions for metadata lineage in `tests/contract/runtime_filesystem.rs`, `tests/integration/pr_review_publish.rs`, `tests/integration/incident_publish.rs`, and `tests/integration/migration_publish.rs`
- [x] T020 [US2] Capture metadata-traceability validation evidence in `specs/029-publish-destinations/validation-report.md`

**Checkpoint**: Published packets keep traceability explicit without depending on run-id-only paths

---

## Phase 5: User Story 3 - Ship 0.29.0 With Aligned Release Surfaces And Validation (Priority: P3)

**Goal**: Make the `0.29.0` version, release docs, changelog, and final quality gates explicit and testable

**Independent Test**: A maintainer can inspect the release surfaces, task list, and validation report and confirm `0.29.0` alignment plus explicit tasks and evidence for version bump, docs/changelog, coverage, `cargo clippy`, and `cargo fmt`.

### Validation for User Story 3 (MANDATORY)

- [x] T021 [P] [US3] Add failing release-surface and runtime-compatibility coverage in `tests/release_029_publish.rs` and `tests/skills_bootstrap.rs`
- [x] T022 [US3] Record story-specific release-alignment decisions under `## User Story 3 Decisions` in `specs/029-publish-destinations/decision-log.md`

### Implementation for User Story 3

- [x] T023 [US3] Update version surfaces in `Cargo.toml`, `Cargo.lock`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T024 [US3] Update impacted docs and changelog closeout in `README.md`, `ROADMAP.md`, `docs/guides/modes.md`, `CHANGELOG.md`, and any publish-facing guidance touched by the feature
- [x] T025 [US3] Capture release-alignment validation evidence and touched-Rust-file coverage expectations in `specs/029-publish-destinations/validation-report.md`

**Checkpoint**: Runtime behavior and release-facing `0.29.0` surfaces align cleanly

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and documentation closeout

- [x] T026 [P] Run the focused publish suite for `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-cli/src/commands/publish.rs`, `tests/contract/runtime_filesystem.rs`, `tests/integration/run_lookup.rs`, `tests/integration/pr_review_publish.rs`, `tests/integration/incident_publish.rs`, `tests/integration/migration_publish.rs`, `tests/release_029_publish.rs`, and `tests/skills_bootstrap.rs`, then record results in `specs/029-publish-destinations/validation-report.md`
- [x] T027 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and document coverage for every modified or newly created Rust file in `specs/029-publish-destinations/validation-report.md`
- [x] T028 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/029-publish-destinations/validation-report.md`
- [x] T029 [P] Run `cargo nextest run --workspace --all-features` and record results in `specs/029-publish-destinations/validation-report.md`
- [x] T030 Perform independent review of publish invariants, traceability honesty, and final diff in `specs/029-publish-destinations/validation-report.md`
- [x] T031 Confirm invariants still hold and close the final validation state in `specs/029-publish-destinations/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the MVP.
- **User Story 2 (P2)**: Can start after Foundational. Reuses the shared publish-path support from Phase 2 but remains independently testable.
- **User Story 3 (P3)**: Can start after Foundational. Release alignment must not weaken the runtime behavior delivered by earlier stories.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- Shared runtime behavior before story-specific assertions.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- Phase 0 tasks after T001 can run in parallel where they touch different planning artifacts.
- T007 and T008 can run in parallel before T009.
- Within each user story, validation and docs tasks marked [P] can run in parallel when they touch different files.
- Final validation tasks T026, T027, T028, and T029 can run in parallel once implementation is stable.

---

## Parallel Example: User Story 1

```bash
# Launch the default-path checks in parallel:
Task: "Add failing structured-path coverage in crates/canon-engine/src/orchestrator/publish.rs, tests/contract/runtime_filesystem.rs, tests/integration/pr_review_publish.rs, and tests/integration/run_lookup.rs"
Task: "Record story-specific destination-shape decisions in specs/029-publish-destinations/decision-log.md"

# Launch runtime and integration updates in parallel where file ownership differs:
Task: "Implement date-prefixed descriptor destination resolution in crates/canon-engine/src/orchestrator/publish.rs"
Task: "Update publish integration assertions in tests/integration/pr_review_publish.rs, tests/integration/incident_publish.rs, tests/integration/migration_publish.rs, and tests/integration/run_lookup.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm the structured default publish path works independently and update `validation-report.md`.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Add User Story 1 and validate independently.
3. Add User Story 2 and validate independently.
4. Add User Story 3 and validate independently.
5. Finish with Verification & Compliance and repository closeout.

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together.
2. Once Foundational is done:
   - Developer A: User Story 1.
   - Developer B: User Story 2.
   - Developer C: User Story 3.
3. Each story closes only after its evidence is recorded.

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] labels map tasks to user stories for traceability
- `T001` is intentionally the version bump as requested
- `T024` is intentionally the impacted docs plus changelog closeout task as requested
- Each user story should be independently completable and validated
- Keep the decision log and validation report current as work progresses