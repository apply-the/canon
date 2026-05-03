# Tasks: Governance Runtime Framing

**Input**: Design documents from `/specs/040-governance-runtime-framing/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Validation**: Layered validation is mandatory. This feature requires Rust guardrail tests for documentation and release-surface drift, manual readback against the spec, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo nextest run`, and `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`.

**Organization**: Tasks are grouped by user story so the product-framing docs, governance adapter guide, and release-surface alignment remain independently testable even though the user requested one end-to-end delivery.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel when the tasks touch different files and have no dependency on incomplete work.
- **[Story]**: Maps the task to `US1`, `US2`, or `US3` from `spec.md`.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that authorize implementation.

- [x] T001 Record execution mode, risk, scope, invariants, and story boundaries in `specs/040-governance-runtime-framing/spec.md` and `specs/040-governance-runtime-framing/plan.md`
- [x] T002 Update feature decisions in `specs/040-governance-runtime-framing/decision-log.md`
- [x] T003 Capture the validation structure and expected evidence in `specs/040-governance-runtime-framing/validation-report.md`
- [x] T004 Record the governance adapter documentation contract in `specs/040-governance-runtime-framing/contracts/governance-adapter-doc-surface.md`

---

## Phase 1: Setup

**Purpose**: Lock the implementation surface and guardrail locations.

- [x] T005 Create the implementation task ledger in `specs/040-governance-runtime-framing/tasks.md`
- [x] T006 Identify the release-surface guardrail anchor points in `tests/release_036_release_provenance_integrity.rs`, `tests/integration/skills_bootstrap.rs`, and the adapter contract anchors in `tests/contract/governance_cli.rs`
- [x] T007 Reserve the new documentation guardrail test files `tests/governance_runtime_framing_docs.rs` and `tests/release_040_governance_runtime_framing.rs`

---

## Phase 2: Foundational

**Purpose**: Establish shared invariants and release targets before editing user-facing docs.

**⚠️ CRITICAL**: No user story work begins until the shared framing and version targets are bounded.

- [x] T008 Audit existing `0.39.0` and delivered `039` references in `Cargo.toml`, `Cargo.lock`, runtime-compatibility references, `README.md`, `CHANGELOG.md`, `ROADMAP.md`, and the release publishing guides
- [x] T009 Audit existing governance adapter wording in `README.md`, `docs/guides/modes.md`, `crates/canon-cli/src/commands/governance.rs`, and `tests/integration/governance_adapter_surface.rs` to preserve contract semantics while changing framing

**Checkpoint**: Shared release and adapter invariants are explicit.

---

## Phase 3: User Story 1 - Clarify Canon's Product Identity (Priority: P1) 🎯 MVP

**Goal**: Make the opening docs state clearly that Canon is the governed packet runtime for AI-assisted engineering and show the simplest human-driven happy path.

**Independent Test**: `tests/governance_runtime_framing_docs.rs` fails until `README.md` and `docs/guides/getting-started.md` expose the new runtime framing, explicit non-goals, and a happy path that includes `inspect clarity`.

### Validation for User Story 1 (MANDATORY)

- [x] T010 [P] [US1] Write failing README and getting-started framing assertions in `tests/governance_runtime_framing_docs.rs`
- [x] T011 [US1] Record wording and boundary decisions for the opening docs in `specs/040-governance-runtime-framing/decision-log.md`

### Implementation for User Story 1

- [x] T012 [P] [US1] Reframe Canon's opening identity, non-goals, and delivery line in `README.md`
- [x] T013 [US1] Update the human-driven happy path and product description in `docs/guides/getting-started.md`
- [x] T014 [US1] Record User Story 1 validation evidence in `specs/040-governance-runtime-framing/validation-report.md`

**Checkpoint**: A first-time reader can identify Canon's role and shortest correct human path from the opening docs alone.

---

## Phase 4: User Story 2 - Document The Governance Adapter As The Machine Boundary (Priority: P2)

**Goal**: Give orchestrator maintainers one dedicated integration guide that explains the governance adapter commands, stable fields, examples, and human-vs-machine boundary.

**Independent Test**: `tests/governance_runtime_framing_docs.rs` fails until the new integration guide and linked docs cover the three adapter commands, stable fields, required examples, and the rule for when to use `canon governance`.

### Validation for User Story 2 (MANDATORY)

- [x] T015 [P] [US2] Write failing governance adapter documentation assertions in `tests/governance_runtime_framing_docs.rs`
- [x] T016 [US2] Record adapter-boundary decisions in `specs/040-governance-runtime-framing/decision-log.md`

### Implementation for User Story 2

- [x] T017 [P] [US2] Create the dedicated governance adapter guide in `docs/integration/governance-adapter.md`
- [x] T018 [US2] Update machine-facing boundary guidance and cross-links in `docs/guides/modes.md`
- [x] T019 [US2] Add governance adapter cross-links and boundary wording in `README.md`
- [x] T020 [US2] Record User Story 2 validation evidence in `specs/040-governance-runtime-framing/validation-report.md`

**Checkpoint**: An external tool maintainer can integrate Canon from one machine-facing guide without scraping human CLI prose.

---

## Phase 5: User Story 3 - Ship A Coherent Public Release Surface (Priority: P3)

**Goal**: Align the new framing with the `0.40.0` release surfaces, changelog, roadmap, and automated drift guardrails.

**Independent Test**: `tests/release_040_governance_runtime_framing.rs` fails until workspace versions, README delivery line, changelog, roadmap, and governance adapter docs all advertise the delivered `040` state coherently.

### Validation for User Story 3 (MANDATORY)

- [x] T021 [P] [US3] Write failing release-surface assertions in `tests/release_040_governance_runtime_framing.rs`, then align the existing release and skills guardrails in `tests/release_036_release_provenance_integrity.rs` and `tests/integration/skills_bootstrap.rs`
- [x] T022 [US3] Record release-alignment decisions in `specs/040-governance-runtime-framing/decision-log.md`

### Implementation for User Story 3

- [x] T023 [P] [US3] Bump the workspace release line to `0.40.0` in `Cargo.toml`, `Cargo.lock`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, and `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- [x] T024 [P] [US3] Update the delivered `040` changelog entry in `CHANGELOG.md`
- [x] T025 [P] [US3] Clean the delivered-roadmap surface in `ROADMAP.md`
- [x] T026 [US3] Align remaining release-facing references in `README.md`, `docs/integration/governance-adapter.md`, `docs/guides/publishing-to-winget.md`, and `docs/guides/publishing-to-scoop.md`
- [x] T027 [US3] Record User Story 3 validation evidence in `specs/040-governance-runtime-framing/validation-report.md`

**Checkpoint**: The repo advertises one coherent delivered `0.40.0` feature state with no stale roadmap drift.

---

## Final Phase: Verification & Compliance

**Purpose**: Finish validation, coverage, formatting, and independent review.

- [x] T028 [P] Run focused guardrail tests for `tests/governance_runtime_framing_docs.rs`, `tests/release_040_governance_runtime_framing.rs`, `tests/release_036_release_provenance_integrity.rs`, `tests/integration/skills_bootstrap.rs`, and `tests/contract/governance_cli.rs`, then record results in `specs/040-governance-runtime-framing/validation-report.md`
- [x] T029 [P] Run `cargo fmt --check` and record the result in `specs/040-governance-runtime-framing/validation-report.md`
- [x] T030 [P] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and record the result in `specs/040-governance-runtime-framing/validation-report.md`
- [x] T031 [P] Run `cargo nextest run` and record the result in `specs/040-governance-runtime-framing/validation-report.md`
- [x] T032 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, note the final `crates/canon-cli/src/commands/governance.rs` coverage, and record the `llvm-cov` export limitation for test crates in `specs/040-governance-runtime-framing/validation-report.md`
- [x] T033 Perform an independent readback of `README.md`, `docs/guides/getting-started.md`, `docs/guides/modes.md`, `docs/integration/governance-adapter.md`, `CHANGELOG.md`, and `ROADMAP.md` against `specs/040-governance-runtime-framing/spec.md`
- [x] T034 Confirm invariants still hold and close the feature evidence in `specs/040-governance-runtime-framing/validation-report.md`
- [x] T035 Prepare the final commit message for the delivered `040` feature and include it in the closeout handoff

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: No dependencies. Must complete first.
- **Phase 1: Setup**: Depends on Phase 0 completion.
- **Phase 2: Foundational**: Depends on Phase 1 completion and blocks all user-story work.
- **Phase 3: User Story 1**: Starts after Foundational and provides the MVP framing.
- **Phase 4: User Story 2**: Starts after Foundational and depends on the US1 framing language remaining stable.
- **Phase 5: User Story 3**: Starts after Foundational and after the major docs wording is settled, because it aligns release surfaces to the final framing.
- **Final Phase**: Depends on all user stories being complete.

### User Story Dependencies

- **US1**: No dependency on later stories.
- **US2**: Depends on US1 only for shared product-language consistency in `README.md`.
- **US3**: Depends on US1 and US2 because release surfaces must reflect the final framing and new integration guide.

### Within Each User Story

- Validation tasks happen before implementation tasks.
- Decision log updates happen before or alongside the first wording changes that depend on them.
- Evidence capture happens before the story is declared complete.

### Parallel Opportunities

- `T012` and `T013` can run in parallel after `T010` and `T011`.
- `T017` and `T018` can run in parallel after `T015` and `T016`.
- `T023`, `T024`, and `T025` can run in parallel after `T021` and `T022`.
- `T028` through `T032` can be launched independently once implementation is complete, subject to workspace capacity.

---

## Parallel Example: User Story 2

```bash
# Launch the failing doc assertions before implementation:
Task: "Write failing governance adapter documentation assertions in tests/governance_runtime_framing_docs.rs"

# Then implement independent documentation surfaces in parallel:
Task: "Create the dedicated governance adapter guide in docs/integration/governance-adapter.md"
Task: "Update machine-facing boundary guidance and cross-links in docs/guides/modes.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0 through Phase 2.
2. Complete User Story 1.
3. Validate `tests/governance_runtime_framing_docs.rs` against the opening docs.

### Incremental Delivery

1. Finish governance artifacts and shared audits.
2. Deliver US1 to lock the product identity.
3. Deliver US2 to make the adapter boundary explicit.
4. Deliver US3 to align release surfaces and drift guardrails.
5. Finish with formatting, clippy, nextest, coverage, and independent readback.

### Parallel Team Strategy

1. One contributor can handle README and getting-started updates for US1.
2. One contributor can handle the new integration guide and mode-guide links for US2.
3. One contributor can handle release surfaces and release guardrail tests for US3 after framing is stable.

## Notes

- The user explicitly requested no slice split, so the implementation must complete every phase in this file before closeout.
- Version bump, docs or changelog work, roadmap cleanup, coverage, clippy, and formatting are explicit tasks by design.
- The final user handoff must include a proposed commit message after all tasks are complete.