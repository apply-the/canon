# Tasks: Requirements PRD Publishing And Chat Publish Skill

**Input**: Design documents from `/specs/041-prd-publish-chat/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Validation**: Layered validation is mandatory. This feature requires focused Rust tests for requirements rendering and publish output, repo skill validation, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and targeted or full test execution sufficient to validate the touched surfaces.

**Organization**: Tasks are grouped by user story so the consolidated PRD artifact, chat publish skill, and publish UX docs remain independently testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel when the tasks touch different files and have no dependency on incomplete work.
- **[Story]**: Maps the task to `US1`, `US2`, or `US3` from `spec.md`.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that authorize implementation.

- [x] T001 Record execution mode, risk, scope, invariants, and story boundaries in `specs/041-prd-publish-chat/spec.md` and `specs/041-prd-publish-chat/plan.md`
- [x] T002 Update feature decisions in `specs/041-prd-publish-chat/decision-log.md`
- [x] T003 Capture the validation structure and expected evidence in `specs/041-prd-publish-chat/validation-report.md`
- [x] T004 Record the publish surface contract in `specs/041-prd-publish-chat/contracts/requirements-publish-surface.md`

---

## Phase 1: Setup

**Purpose**: Lock the implementation and validation surface.

- [x] T005 Create the implementation task ledger in `specs/041-prd-publish-chat/tasks.md`
- [x] T006 Identify the requirements artifact and publish anchors in `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/service/mode_requirements.rs`, and `crates/canon-cli/src/commands/publish.rs`
- [x] T007 Identify the closest regression anchors in `tests/contract/requirements_contract.rs`, `tests/requirements_authoring_renderer.rs`, and `scripts/validate-canon-skills.sh`

---

## Phase 2: Foundational

**Purpose**: Bound compatibility and release expectations before code changes.

**⚠️ CRITICAL**: No user story work begins until artifact and publish invariants are explicit.

- [x] T008 Audit the current requirements artifact set and publish metadata expectations in `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/orchestrator/publish.rs`, and `crates/canon-cli/src/commands/publish.rs`
- [x] T009 Audit version and release-surface anchors in `Cargo.toml`, `CHANGELOG.md`, `README.md`, and `tech-docs/guides/modes.md`

**Checkpoint**: Compatibility, release, and skill-validation constraints are explicit.

---

## Phase 3: User Story 1 - Publish A Readable PRD Packet (Priority: P1) 🎯 MVP

**Goal**: Add a consolidated `prd.md` to published requirements packets without breaking the existing file set.

**Independent Test**: A completed requirements run publishes `prd.md` alongside the existing sectional files and metadata at both default and override destinations.

### Validation for User Story 1 (MANDATORY)

- [x] T010 [P] [US1] Write failing requirements renderer assertions for `prd.md` in `tests/requirements_authoring_renderer.rs`
- [x] T011 [P] [US1] Write failing requirements contract assertions for `prd.md` in `tests/contract/requirements_contract.rs`
- [x] T012 [P] [US1] Extend publish command tests for default and override destinations with `prd.md` in `crates/canon-cli/src/commands/publish.rs`
- [x] T013 [US1] Record PRD artifact decisions in `specs/041-prd-publish-chat/decision-log.md`

### Implementation for User Story 1

- [x] T014 [P] [US1] Add the additive `prd.md` requirements artifact to `crates/canon-engine/src/artifacts/contract.rs`
- [x] T015 [US1] Implement consolidated PRD rendering in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T016 [US1] Align requirements artifact persistence expectations in `crates/canon-engine/src/orchestrator/service/mode_requirements.rs` if needed for the new contract artifact
- [x] T017 [US1] Record User Story 1 validation evidence in `specs/041-prd-publish-chat/validation-report.md`

**Checkpoint**: Requirements publish emits one readable PRD while preserving the sectional packet.

---

## Phase 4: User Story 2 - Invoke Publish From Chat (Priority: P2)

**Goal**: Make publish discoverable and usable from repo-local Copilot or Codex skills.

**Independent Test**: Skill validation passes with a new publish skill that accurately describes `canon publish <RUN_ID>` and preserves gate honesty.

### Validation for User Story 2 (MANDATORY)

- [x] T018 [P] [US2] Add or extend skill bootstrap assertions for `canon-publish` in `tests/integration/skills_bootstrap.rs`
- [x] T019 [US2] Record chat publish guidance decisions in `specs/041-prd-publish-chat/decision-log.md`

### Implementation for User Story 2

- [x] T020 [P] [US2] Add the repo-local skill in `.agents/skills/canon-publish/SKILL.md`
- [x] T021 [P] [US2] Mirror the embedded skill source in `defaults/embedded-skills/canon-publish/skill-source.md`
- [x] T022 [US2] Update any shared skill references needed for discoverability in `.agents/skills/canon-shared/` and `defaults/embedded-skills/canon-shared/` if required by the existing skill conventions
- [x] T023 [US2] Record User Story 2 validation evidence in `specs/041-prd-publish-chat/validation-report.md`

**Checkpoint**: Chat-first users have an explicit publish skill that maps to the real CLI surface.

---

## Phase 5: User Story 3 - Understand The Publish UX Up Front (Priority: P3)

**Goal**: Clarify where artifacts live before and after publish, surface the consolidated PRD, and align the release line.

**Independent Test**: Docs and release surfaces make the `.canon/artifacts` versus published destination distinction explicit and advertise the new `prd.md` output.

### Validation for User Story 3 (MANDATORY)

- [x] T024 [P] [US3] Add or extend doc or release-surface guardrail assertions in `tests/release_040_governance_runtime_framing.rs` or a new focused test file for the requirements publish UX
- [x] T025 [US3] Record publish UX and release-alignment decisions in `specs/041-prd-publish-chat/decision-log.md`

### Implementation for User Story 3

- [x] T026 [P] [US3] Update the release line in `Cargo.toml` and any lockfile or shared compatibility surfaces that inherit the workspace version
- [x] T027 [P] [US3] Update `README.md` with the clearer publish UX and consolidated PRD guidance
- [x] T028 [P] [US3] Update `tech-docs/guides/modes.md` with requirements publish expectations and artifact visibility
- [x] T029 [P] [US3] Update `CHANGELOG.md` and `ROADMAP.md` for the delivered feature and follow-on positioning
- [x] T030 [US3] Record User Story 3 validation evidence in `specs/041-prd-publish-chat/validation-report.md`

**Checkpoint**: Users can understand and discover the publish flow and PRD output without source-code spelunking.

---

## Final Phase: Verification & Compliance

**Purpose**: Finish validation, skill verification, and closeout evidence.

- [x] T031 [P] Run focused Rust tests for `tests/requirements_authoring_renderer.rs`, `tests/contract/requirements_contract.rs`, `tests/integration/skills_bootstrap.rs`, and the touched publish command tests, then record results in `specs/041-prd-publish-chat/validation-report.md`
- [x] T032 [P] Run `/bin/bash scripts/validate-canon-skills.sh` and record the result in `specs/041-prd-publish-chat/validation-report.md`
- [x] T033 [P] Run `cargo fmt --check` and record the result in `specs/041-prd-publish-chat/validation-report.md`
- [x] T034 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and record the result in `specs/041-prd-publish-chat/validation-report.md`
- [x] T035 [P] Run a broader regression command for the touched workspace if needed and record the result in `specs/041-prd-publish-chat/validation-report.md`
- [x] T036 Perform an independent readback of the published requirements packet, skill wording, and updated docs against `specs/041-prd-publish-chat/spec.md`
- [x] T037 Confirm invariants still hold and close the feature evidence in `specs/041-prd-publish-chat/validation-report.md`
- [x] T038 Prepare the final commit message for the delivered `041` feature and include it in the closeout handoff

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: No dependencies. Must complete first.
- **Phase 1: Setup**: Depends on Phase 0 completion.
- **Phase 2: Foundational**: Depends on Phase 1 completion and blocks all user-story work.
- **Phase 3: User Story 1**: Starts after Foundational and provides the MVP user value.
- **Phase 4: User Story 2**: Starts after Foundational and depends on US1 only for stable publish terminology.
- **Phase 5: User Story 3**: Starts after Foundational and after the PRD artifact name and skill wording are stable.
- **Final Phase**: Depends on all user stories being complete.

### User Story Dependencies

- **US1**: No dependency on later stories.
- **US2**: Depends on US1 only for stable PRD or publish terminology.
- **US3**: Depends on US1 and US2 because docs and release surfaces must reflect the final packet and skill behavior.

### Within Each User Story

- Validation tasks happen before implementation tasks.
- Decision log updates happen before or alongside the first dependent code or doc changes.
- Evidence capture happens before the story is declared complete.

### Parallel Opportunities

- `T010`, `T011`, and `T012` can run in parallel before the US1 implementation tasks.
- `T020` and `T021` can run in parallel after `T018` and `T019`.
- `T026` through `T029` can run in parallel once the feature behavior is stable.
- `T031` through `T035` can be launched independently once implementation is complete, subject to workspace capacity.

---

## Parallel Example: User Story 1

```bash
# Write the failing PRD coverage first:
Task: "Write failing requirements renderer assertions for prd.md in tests/requirements_authoring_renderer.rs"
Task: "Write failing requirements contract assertions for prd.md in tests/contract/requirements_contract.rs"
Task: "Extend publish command tests for default and override destinations with prd.md in crates/canon-cli/src/commands/publish.rs"

# Then land the additive contract and renderer work:
Task: "Add the additive prd.md requirements artifact to crates/canon-engine/src/artifacts/contract.rs"
Task: "Implement consolidated PRD rendering in crates/canon-engine/src/artifacts/markdown.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0 through Phase 2.
2. Complete User Story 1.
3. Validate that published requirements packets contain both sectional files and `prd.md`.

### Incremental Delivery

1. Finish governance artifacts and compatibility audits.
2. Deliver US1 to close the primary PRD usability gap.
3. Deliver US2 to expose publish in chat-first workflows.
4. Deliver US3 to align release and docs surfaces.
5. Finish with validation, skill verification, and independent readback.

### Parallel Team Strategy

1. One contributor can handle the PRD artifact contract and renderer tests.
2. One contributor can handle skill packaging and skill bootstrap coverage.
3. One contributor can handle docs, changelog, versioning, and release-surface guardrails after behavior stabilizes.

## Notes

- The user explicitly requested end-to-end delivery including tests, docs, lint cleanup, version bump, and a final commit message.
- If no dedicated release-surface guardrail exists for the new publish UX, create a focused Rust test instead of relying on manual doc review alone.
- Keep the decision log and validation report current as tasks are completed.