# Tasks: Standard ADR Publish Artifacts

**Input**: Design documents from `/specs/043-standard-adr-publish/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Validation**: Layered validation is mandatory. This feature requires focused Rust CLI, publish, contract, and integration tests; `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo nextest run`; and touched-file coverage review with `cargo llvm-cov` until every new or modified Rust source file reaches at least 95% line coverage.

**Organization**: Tasks are grouped by user story so architecture default ADR export, change or migration opt-in ADR export, and registry/documentation integrity remain independently testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel when tasks touch different files and have no dependency on incomplete work.
- **[Story]**: Maps the task to `US1`, `US2`, or `US3` from `spec.md`.

## Constitution Alignment

- Every feature starts with explicit governance, decision, and validation artifacts.
- No implementation task appears before the artifacts that authorize it.
- Every user story includes validation tasks and evidence capture.
- The requested version bump is the first implementation task.
- The requested coverage closeout, formatting validation, and lint validation are the last implementation tasks.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that authorize implementation.

- [x] T001 Record execution mode, risk classification, scope boundaries, and invariants in `specs/043-standard-adr-publish/spec.md` and `specs/043-standard-adr-publish/plan.md`
- [x] T002 Create the decision log and validation scaffold in `specs/043-standard-adr-publish/decision-log.md` and `specs/043-standard-adr-publish/validation-report.md`
- [x] T003 Create the supporting design artifacts in `specs/043-standard-adr-publish/research.md`, `specs/043-standard-adr-publish/data-model.md`, `specs/043-standard-adr-publish/quickstart.md`, and `specs/043-standard-adr-publish/contracts/adr-publish-surface.md`
- [x] T004 Update agent context from the plan in `AGENTS.md`

---

## Phase 1: Setup

**Purpose**: Lock the release line before behavior changes begin.

- [x] T005 Update the `0.43.0` release line across `Cargo.toml`, `Cargo.lock`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `README.md`, `CHANGELOG.md`, `tech-docs/guides/publishing-to-winget.md`, `tech-docs/guides/publishing-to-scoop.md`, `tests/integration/skills_bootstrap.rs`, and related release-surface assertions under `tests/`

---

## Phase 2: Foundational

**Purpose**: Add the publish surface and ADR registry primitives that every story depends on.

**⚠️ CRITICAL**: No user story work begins until the CLI and engine can distinguish default, opt-in, and unsupported ADR publication.

- [x] T006 [P] Extend the publish command surface and request wiring for ADR export in `crates/canon-cli/src/app.rs`, `crates/canon-cli/src/commands/publish.rs`, and any engine service call sites they require
- [x] T007 [P] Implement ADR policy evaluation, registry destination rules, and numbering helpers in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T008 [P] Add or update shared publish metadata and summary handling for ADR file reporting in `crates/canon-engine/src/orchestrator/publish.rs` and `crates/canon-cli/src/commands/publish.rs`
- [x] T009 Record foundational publish-surface decisions in `specs/043-standard-adr-publish/decision-log.md`

**Checkpoint**: Publish can distinguish supported, opt-in, and unsupported ADR export behavior without altering existing packet authority.

---

## Phase 3: User Story 1 - Publish A Standard Architecture ADR (Priority: P1) 🎯 MVP

**Goal**: `architecture` publishes emit one standard ADR entry by default.

**Independent Test**: A publishable `architecture` run creates a normal packet publish plus one `tech-docs/adr/ADR-XXXX-<slug>.md` file with the required standard sections and source traceability.

### Validation for User Story 1 (MANDATORY)

- [x] T010 [P] [US1] Write failing architecture ADR export tests in `tests/architecture_adr_publish.rs`
- [x] T011 [P] [US1] Add failing CLI publish summary assertions for ADR output in `crates/canon-cli/src/commands/publish.rs`
- [x] T012 [US1] Record architecture ADR mapping decisions in `specs/043-standard-adr-publish/decision-log.md`

### Implementation for User Story 1

- [x] T013 [P] [US1] Implement architecture ADR section synthesis and file emission in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T014 [P] [US1] Surface generated ADR files in publish summaries and packet metadata handling in `crates/canon-engine/src/orchestrator/publish.rs` and `crates/canon-cli/src/commands/publish.rs`
- [x] T015 [US1] Update architecture ADR documentation in `README.md` and `tech-docs/guides/modes.md`
- [x] T016 [US1] Capture User Story 1 validation evidence in `specs/043-standard-adr-publish/validation-report.md`

**Checkpoint**: `architecture` is the canonical ADR-producing mode and works end-to-end on its own.

---

## Phase 4: User Story 2 - Opt Durable Change Or Migration Decisions Into The ADR Register (Priority: P2)

**Goal**: `change` and `migration` can export ADRs only when the operator explicitly requests it.

**Independent Test**: Publishing a valid `change` or `migration` run without `--adr` creates no ADR file, while publishing the same run with `--adr` creates exactly one ADR entry.

### Validation for User Story 2 (MANDATORY)

- [x] T017 [P] [US2] Write failing opt-in ADR export tests for `change` and `migration` in `tests/change_migration_adr_publish.rs`
- [x] T018 [P] [US2] Add failing CLI parsing tests for the ADR export flag in `crates/canon-cli/src/app.rs`
- [x] T019 [US2] Record opt-in ADR export decisions in `specs/043-standard-adr-publish/decision-log.md`

### Implementation for User Story 2

- [x] T020 [P] [US2] Implement `--adr` parsing and publish request wiring in `crates/canon-cli/src/app.rs`, `crates/canon-cli/src/commands/publish.rs`, and any affected engine service interfaces
- [x] T021 [US2] Implement `change` and `migration` opt-in ADR synthesis paths in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T022 [P] [US2] Update ADR export skill guidance in `.agents/skills/canon-change/SKILL.md`, `.agents/skills/canon-migration/SKILL.md`, `defaults/embedded-skills/canon-change/skill-source.md`, and `defaults/embedded-skills/canon-migration/skill-source.md`
- [x] T023 [US2] Update user-facing ADR publish guidance for `change` and `migration` in `README.md` and `tech-docs/guides/modes.md`
- [x] T024 [US2] Capture User Story 2 validation evidence in `specs/043-standard-adr-publish/validation-report.md`

**Checkpoint**: Tactical decision packets can enter the ADR registry only when explicitly requested.

---

## Phase 5: User Story 3 - Keep ADR Publication Honest, Bounded, And Documented (Priority: P3)

**Goal**: Registry numbering, unsupported-mode rejection, and documentation stay coherent and regression-tested.

**Independent Test**: Existing ADR history does not collide with new entries, unsupported modes reject `--adr`, and the docs describe the same behavior the code implements.

### Validation for User Story 3 (MANDATORY)

- [x] T025 [P] [US3] Write failing ADR numbering and collision tests in `tests/adr_publish_registry.rs`
- [x] T026 [P] [US3] Write failing unsupported-mode ADR rejection tests in `tests/unsupported_mode_adr_publish.rs`
- [x] T027 [P] [US3] Add failing documentation or release-surface assertions in `tests/release_040_governance_runtime_framing.rs`, `tests/integration/skills_bootstrap.rs`, and related release verification tests under `tests/`
- [x] T028 [US3] Record registry-boundary and honesty decisions in `specs/043-standard-adr-publish/decision-log.md`

### Implementation for User Story 3

- [x] T029 [P] [US3] Implement registry numbering, fixed `tech-docs/adr/` destination behavior, and unsupported-mode rejection in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T030 [P] [US3] Update ADR registry documentation and examples in `README.md`, `tech-docs/guides/modes.md`, `ROADMAP.md`, `.agents/skills/canon-architecture/SKILL.md`, and `defaults/embedded-skills/canon-architecture/skill-source.md`
- [x] T031 [US3] Update release notes and related release-surface fixtures in `CHANGELOG.md`, `tech-docs/guides/publishing-to-winget.md`, `tech-docs/guides/publishing-to-scoop.md`, and affected tests under `tests/`
- [x] T032 [US3] Capture User Story 3 validation evidence in `specs/043-standard-adr-publish/validation-report.md`

**Checkpoint**: ADR publication rules are stable, documented, and enforced for supported and unsupported modes.

---

## Final Phase: Verification & Compliance

**Purpose**: Finish validation, coverage closeout, and commit preparation.

- [x] T033 [P] Run the focused ADR publish regression suite and record results in `specs/043-standard-adr-publish/validation-report.md`
- [x] T034 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/043-standard-adr-publish/validation-report.md`
- [x] T035 [P] Run `cargo nextest run` and `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, then add targeted tests until every new or modified Rust source file touched by the feature is at or above 95% line coverage and record the closeout in `specs/043-standard-adr-publish/validation-report.md`
- [x] T036 Perform an independent readback comparing generated ADRs to their source packets and record findings in `specs/043-standard-adr-publish/validation-report.md`
- [x] T037 Confirm invariants still hold, close the validation report, and prepare the final commit message in `specs/043-standard-adr-publish/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: Completed and must remain current.
- **Phase 1: Setup**: Depends on Phase 0 completion and must happen before any behavior change.
- **Phase 2: Foundational**: Depends on Phase 1 and blocks all user stories.
- **Phase 3: User Story 1**: Starts after Foundational and delivers the MVP.
- **Phase 4: User Story 2**: Starts after User Story 1 establishes the base ADR synthesis path.
- **Phase 5: User Story 3**: Starts after User Stories 1 and 2 stabilize the supported-mode behavior.
- **Final Phase**: Depends on all user stories being complete.

### User Story Dependencies

- **US1**: No dependency on later stories.
- **US2**: Depends on US1 because opt-in export reuses the base ADR synthesis and summary path.
- **US3**: Depends on US1 and US2 because numbering, unsupported-mode rejection, and docs need the final supported-mode behavior.

### Within Each User Story

- Validation tasks happen before implementation tasks.
- Decision log updates happen before or alongside the first dependent code changes.
- Evidence capture happens before the story is declared complete.

### Parallel Opportunities

- `T006`, `T007`, and `T008` can run in parallel after the version bump.
- `T010` and `T011` can run in parallel before US1 implementation.
- `T017` and `T018` can run in parallel before US2 implementation.
- `T024`, `T025`, and `T026` can run in parallel before US3 implementation.
- `T032`, `T033`, and `T034` can run independently once implementation is complete.

---

## Parallel Example: User Story 1

```bash
# Write the failing checks first:
Task: "Write failing architecture ADR export tests in tests/architecture_adr_publish.rs"
Task: "Add failing CLI publish summary assertions for ADR output in crates/canon-cli/src/commands/publish.rs"

# Then land the engine and CLI behavior:
Task: "Implement architecture ADR section synthesis and file emission in crates/canon-engine/src/orchestrator/publish.rs"
Task: "Surface generated ADR files in publish summaries and packet metadata handling in crates/canon-engine/src/orchestrator/publish.rs and crates/canon-cli/src/commands/publish.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete the version bump and foundational publish-surface work.
2. Deliver automatic ADR export for `architecture`.
3. Validate the story independently through focused tests and publish readback.

### Incremental Delivery

1. Finish governance artifacts, version bump, and publish primitives.
2. Deliver US1 to establish the canonical ADR-producing mode.
3. Deliver US2 to make tactical decision export explicit and bounded.
4. Deliver US3 to harden numbering, unsupported-mode behavior, and docs.
5. Finish with coverage closeout, formatting, lint, and independent review.

### Parallel Team Strategy

1. One contributor can handle CLI argument and summary wiring.
2. One contributor can handle engine-side ADR synthesis and numbering.
3. One contributor can handle docs, release surfaces, and closeout validation once the behavior stabilizes.