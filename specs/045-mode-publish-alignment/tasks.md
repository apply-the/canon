# Tasks: Mode Publish Alignment

**Input**: Design documents from `/specs/045-mode-publish-alignment/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Validation**: Layered validation is mandatory. This feature requires focused runtime and assistant package tests, release-surface checks, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run --workspace --all-features`, and touched-file coverage review with `cargo llvm-cov`.

**Organization**: Tasks are grouped by user story so runtime publish alignment, assistant command-surface alignment, and versioned closeout remain independently reviewable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel when files do not overlap and dependencies are already satisfied.
- **[Story]**: Maps to `US1`, `US2`, or `US3` from `spec.md`.

## Constitution Alignment

- Every feature starts with explicit mode, risk, scope, invariants, and durable evidence artifacts.
- The requested version bump is the first executable implementation task.
- Every user story includes validation work and evidence capture before it is declared complete.
- Final closeout must include formatter, linter, regression, and coverage evidence.

## Phase 0: Version, Governance & Artifacts

**Purpose**: Advance the release line first and keep the feature packet current before behavior changes land.

- [x] T001 Upgrade the Canon release line from `0.44.0` to `0.45.0` in `Cargo.toml`, `Cargo.lock`, `README.md`, `CHANGELOG.md`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `tech-docs/guides/publishing-to-winget.md`, `tech-docs/guides/publishing-to-scoop.md`, `tests/integration/skills_bootstrap.rs`, and any directly affected release assertions
- [x] T002 Record current alignment decisions and validation intent in `specs/045-mode-publish-alignment/decision-log.md` and `specs/045-mode-publish-alignment/validation-report.md`
- [x] T003 Confirm `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/mode-publish-alignment-surface.md`, and `quickstart.md` remain coherent in `specs/045-mode-publish-alignment/`

---

## Phase 1: Validation Harness

**Purpose**: Create failing checks that expose the current runtime and assistant-surface drift before implementation.

- [x] T004 [P] Add failing `security-assessment` publish-state regressions in `tests/security_assessment_direct_runtime.rs` and any directly supporting publish tests under `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T005 [P] Add failing assistant publish command-surface drift assertions in `tests/assistant_plugin_packages.rs`
- [x] T006 Run the focused failing validations and record the expected drift in `specs/045-mode-publish-alignment/validation-report.md`

---

## Phase 2: Foundational Guardrails

**Purpose**: Keep the bounded scope and invariant checks explicit before story work begins.

- [x] T007 Record the bounded publish-alignment guardrails and untouched PRD/C4/ADR invariants in `specs/045-mode-publish-alignment/decision-log.md`
- [x] T008 Capture the expected release-surface and validation commands for this slice in `specs/045-mode-publish-alignment/validation-report.md`

**Checkpoint**: Validation harnesses fail for the known drift and the bounded scope is explicit.

---

## Phase 3: User Story 1 - Publish Security Assessment Packets Consistently (Priority: P1) 🎯 MVP

**Goal**: `security-assessment` publish behavior matches the documented operational publish posture for readable blocked or approval-gated packets.

**Independent Test**: `cargo test --test security_assessment_direct_runtime`

### Validation for User Story 1

- [x] T009 [US1] Record the runtime publishability decision for `security-assessment` in `specs/045-mode-publish-alignment/decision-log.md`
- [x] T010 [US1] Run `cargo test --test security_assessment_direct_runtime` and confirm the new publish-state regressions fail before the runtime fix lands

### Implementation for User Story 1

- [x] T011 [US1] Align non-`Completed` publish eligibility for `security-assessment` in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T012 [US1] Add or update focused publish-state coverage in `tests/security_assessment_direct_runtime.rs` and any touched publish unit tests in `crates/canon-engine/src/orchestrator/publish.rs`
- [x] T013 [US1] Record passing runtime evidence and invariant confirmation in `specs/045-mode-publish-alignment/validation-report.md`

**Checkpoint**: `security-assessment` now publishes from the documented readable operational states without widening other mode gates.

---

## Phase 4: User Story 2 - Keep Assistant Publish Guidance Honest (Priority: P2)

**Goal**: Assistant package metadata and prompt-pack examples reflect the real positional `canon publish <RUN_ID>` syntax.

**Independent Test**: `cargo test --test assistant_plugin_packages`

### Validation for User Story 2

- [x] T014 [US2] Record the assistant publish command-surface decision in `specs/045-mode-publish-alignment/decision-log.md`
- [x] T015 [US2] Run `cargo test --test assistant_plugin_packages` and confirm the new publish-syntax assertions fail before the metadata fix lands

### Implementation for User Story 2

- [x] T016 [US2] Update assistant publish command references in `assistant/commands/governed-methods.json` and `assistant/prompts/copilot-command-pack.md`
- [x] T017 [US2] Update any additional assistant-facing publish examples or tests touched by the drift in `tests/assistant_plugin_packages.rs` and related package docs if required
- [x] T018 [US2] Record passing assistant-surface validation evidence in `specs/045-mode-publish-alignment/validation-report.md`

**Checkpoint**: Assistant package surfaces no longer drift from the shipped CLI contract.

---

## Phase 5: User Story 3 - Close The Slice As A Versioned Release Surface Update (Priority: P3)

**Goal**: The repository release line and closeout artifacts advance cleanly to `0.45.0` with validation evidence.

**Independent Test**: Focused version-surface assertions plus repository closeout commands show only `0.45.0` on touched surfaces.

### Validation for User Story 3

- [x] T019 [US3] Record the `0.45.0` release-line decision and touched-surface scope in `specs/045-mode-publish-alignment/decision-log.md`
- [x] T020 [US3] Run focused version-surface checks against touched files and record any expected failures before closeout in `specs/045-mode-publish-alignment/validation-report.md`

### Implementation for User Story 3

- [x] T021 [US3] Update touched release-surface docs and assertions for `0.45.0` in `README.md`, `CHANGELOG.md`, `tech-docs/guides/publishing-to-winget.md`, `tech-docs/guides/publishing-to-scoop.md`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `tests/integration/skills_bootstrap.rs`, and any directly affected release checks
- [x] T022 [US3] Record version-surface closeout evidence in `specs/045-mode-publish-alignment/validation-report.md`

**Checkpoint**: The delivery line is `0.45.0` and touched release surfaces are aligned.

---

## Final Phase: Verification & Compliance

**Purpose**: Finish structural validation, regression closure, independent review, and coverage closeout.

- [x] T023 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/045-mode-publish-alignment/validation-report.md`
- [x] T024 [P] Run the focused logical validations plus `cargo nextest run --workspace --all-features`, then record results in `specs/045-mode-publish-alignment/validation-report.md`
- [x] T025 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, confirm touched-file coverage for modified Rust files is at or above 95%, and record the closeout in `specs/045-mode-publish-alignment/validation-report.md`
- [x] T026 Perform independent readback comparing documented publish semantics, assistant publish syntax, and runtime behavior in `specs/045-mode-publish-alignment/validation-report.md`
- [x] T027 Confirm invariants still hold, mark all tasks complete, and prepare the final commit message in `specs/045-mode-publish-alignment/tasks.md` and `specs/045-mode-publish-alignment/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Version, Governance & Artifacts**: Must complete first.
- **Phase 1: Validation Harness**: Depends on Phase 0.
- **Phase 2: Foundational Guardrails**: Depends on Phase 1 and locks the bounded scope.
- **Phase 3: User Story 1**: Depends on the foundational guardrails and delivers the MVP.
- **Phase 4: User Story 2**: Depends on Phase 1 but can proceed after the foundational guardrails.
- **Phase 5: User Story 3**: Depends on Phase 0 and the touched-surface audit.
- **Final Phase**: Depends on all user stories being complete.

### User Story Dependencies

- **US1**: No dependency on later stories.
- **US2**: Independent from US1 behaviorally, but both share the same closeout and versioned validation evidence.
- **US3**: Depends on knowing the final touched file set from US1 and US2.

### Parallel Opportunities

- T004 and T005 can be written in parallel.
- T023, T024, and T025 can run independently during final closeout once implementation is stable.

## Implementation Strategy

1. Bump the version first and lock the feature artifacts.
2. Write the failing runtime and assistant validations.
3. Deliver the MVP by aligning `security-assessment` publishability.
4. Fix assistant publish syntax drift.
5. Close the slice with `0.45.0` release surfaces, then finish formatter, linter, nextest, and coverage evidence.