# Tasks: Pragmatic C4 Architecture Packets And Visual Artifacts

**Input**: Design documents from `/specs/042-visual-artifact-generation/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`

**Validation**: Layered validation is mandatory. This feature requires focused Rust contract, renderer, run, and publish tests; `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo nextest run`; and a coverage review for modified Rust files using `cargo llvm-cov`.

**Organization**: Tasks are grouped by user story so the consolidated architecture packet, machine-readable visual artifacts, and pragmatic C4 depth rules remain independently testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel when tasks touch different files and have no dependency on incomplete work.
- **[Story]**: Maps the task to `US1`, `US2`, or `US3` from `spec.md`.

## Constitution Alignment

- Every feature starts with explicit governance, decision, and validation artifacts.
- No implementation task appears before the artifacts that authorize it.
- Every user story includes validation tasks and evidence capture.
- Coverage review for touched Rust files is required before closeout.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls that authorize implementation.

- [x] T001 Record execution mode, risk classification, scope boundaries, and invariants in `specs/042-visual-artifact-generation/spec.md` and `specs/042-visual-artifact-generation/plan.md`
- [x] T002 Create the decision log and validation scaffold in `specs/042-visual-artifact-generation/decision-log.md` and `specs/042-visual-artifact-generation/validation-report.md`
- [x] T003 Capture research, data model, quickstart, and packet contract artifacts in `specs/042-visual-artifact-generation/research.md`, `specs/042-visual-artifact-generation/data-model.md`, `specs/042-visual-artifact-generation/quickstart.md`, and `specs/042-visual-artifact-generation/contracts/architecture-visual-packet.md`
- [x] T004 Update agent context from the plan in `AGENTS.md`

---

## Phase 1: Setup

**Purpose**: Lock the release line and implementation surfaces before the packet redesign.

- [ ] T005 Update the `0.42.0` release line across `Cargo.toml`, `Cargo.lock`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `README.md`, `CHANGELOG.md`, `tech-docs/guides/publishing-to-winget.md`, `tech-docs/guides/publishing-to-scoop.md`, `tests/release_036_release_provenance_integrity.rs`, `tests/release_040_governance_runtime_framing.rs`, and `tests/integration/skills_bootstrap.rs`
- [ ] T006 Audit the current architecture artifact and publish anchors in `crates/canon-engine/src/domain/artifact.rs`, `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, and `crates/canon-engine/src/orchestrator/publish.rs`

---

## Phase 2: Foundational

**Purpose**: Add the contract and persistence capabilities that every story depends on.

**⚠️ CRITICAL**: No user story work begins until optional artifact support and packet-manifest plumbing are bounded.

- [ ] T007 [P] Extend the artifact domain model for optional or capability-dependent outputs in `crates/canon-engine/src/domain/artifact.rs`
- [ ] T008 [P] Update architecture artifact contract helpers and serialization in `crates/canon-engine/src/artifacts/contract.rs` and `tests/contract/snapshots/`
- [ ] T009 Update persisted-artifact and publish loading logic for optional architecture outputs in `crates/canon-engine/src/orchestrator/publish.rs` and any adjacent persistence code under `crates/canon-engine/src/`
- [ ] T010 Record foundational contract decisions in `specs/042-visual-artifact-generation/decision-log.md`

**Checkpoint**: Architecture packets can declare required and optional outputs without forcing every view or render target to exist.

---

## Phase 3: User Story 1 - Read One Primary Architecture Packet (Priority: P1) 🎯 MVP

**Goal**: Publish one primary architecture handoff document that centers System Context, Container, and Deployment coverage.

**Independent Test**: A completed architecture run publishes `architecture-overview.md` plus the required pragmatic view coverage and supporting artifacts without forcing reviewers to read every file individually.

### Validation for User Story 1 (MANDATORY)

- [ ] T011 [P] [US1] Write failing contract coverage for the required primary architecture packet artifacts in `tests/contract/architecture_c4_contract.rs` or a new adjacent contract test file
- [ ] T012 [P] [US1] Write failing renderer assertions for `architecture-overview.md` and deployment coverage behavior in `tests/architecture_c4_renderer.rs`
- [ ] T013 [P] [US1] Write failing run or publish assertions for the primary packet output in `tests/architecture_c4_run.rs` and `crates/canon-cli/src/commands/publish.rs`
- [ ] T014 [US1] Record story-specific packet-shape decisions in `specs/042-visual-artifact-generation/decision-log.md`

### Implementation for User Story 1

- [ ] T015 [P] [US1] Add `architecture-overview.md` and deployment coverage requirements to `crates/canon-engine/src/artifacts/contract.rs`
- [ ] T016 [US1] Implement `architecture-overview.md` and deployment coverage rendering in `crates/canon-engine/src/artifacts/markdown.rs`
- [ ] T017 [US1] Align architecture run persistence and publish expectations for the primary overview packet in `crates/canon-engine/src/` and `crates/canon-cli/src/commands/publish.rs`
- [ ] T018 [US1] Capture User Story 1 validation evidence in `specs/042-visual-artifact-generation/validation-report.md`

**Checkpoint**: Architecture publish exposes one primary overview packet with pragmatic baseline coverage.

---

## Phase 4: User Story 2 - Keep The Packet Machine-Readable (Priority: P2)

**Goal**: Emit Mermaid view sources, a view manifest, and optional rendered assets without weakening packet honesty.

**Independent Test**: A published architecture packet includes Mermaid sources and a manifest for included views, and it records omitted or unsupported rendered assets explicitly.

### Validation for User Story 2 (MANDATORY)

- [ ] T019 [P] [US2] Write failing tests for Mermaid source artifacts and view-manifest output in `tests/architecture_c4_renderer.rs`, `tests/architecture_c4_run.rs`, and any new focused manifest test
- [ ] T020 [P] [US2] Write failing publish assertions for machine-readable diagram artifacts in `crates/canon-cli/src/commands/publish.rs` or a new publish-surface test
- [ ] T021 [US2] Record visual-artifact and manifest decisions in `specs/042-visual-artifact-generation/decision-log.md`

### Implementation for User Story 2

- [ ] T022 [P] [US2] Add Mermaid source and manifest artifact requirements in `crates/canon-engine/src/artifacts/contract.rs`
- [ ] T023 [US2] Implement Mermaid rendering, manifest generation, and capability notes in `crates/canon-engine/src/artifacts/markdown.rs` and adjacent architecture packet code under `crates/canon-engine/src/`
- [ ] T024 [US2] Add optional SVG or PNG render output handling where supported in `crates/canon-engine/src/` and publish integration paths in `crates/canon-engine/src/orchestrator/publish.rs`
- [ ] T025 [US2] Capture User Story 2 validation evidence in `specs/042-visual-artifact-generation/validation-report.md`

**Checkpoint**: The packet remains machine-readable and explicit about supported render formats.

---

## Phase 5: User Story 3 - Generate Only The C4 Depth That Helps (Priority: P3)

**Goal**: Make Component and Dynamic views justification-driven and align docs around the pragmatic C4 default.

**Independent Test**: Simple briefs omit deeper views cleanly while complex briefs can justify them, and the docs explain the default `System Context + Container + Deployment` posture.

### Validation for User Story 3 (MANDATORY)

- [ ] T026 [P] [US3] Write failing tests for view-selection and omission behavior in `tests/architecture_c4_renderer.rs` and `tests/architecture_c4_run.rs`
- [ ] T027 [P] [US3] Add or extend release-surface assertions for the new architecture publish guidance in `tests/release_040_governance_runtime_framing.rs`
- [ ] T028 [US3] Record pragmatic-depth and documentation decisions in `specs/042-visual-artifact-generation/decision-log.md`

### Implementation for User Story 3

- [ ] T029 [P] [US3] Implement pragmatic view-selection rules for component and dynamic artifacts in `crates/canon-engine/src/artifacts/markdown.rs` and adjacent architecture-mode runtime code under `crates/canon-engine/src/`
- [ ] T030 [P] [US3] Update architecture packet documentation in `README.md`, `tech-docs/guides/modes.md`, and `ROADMAP.md`
- [ ] T031 [US3] Update release notes in `CHANGELOG.md` and any related release verification surfaces under `tests/` or `scripts/release/`
- [ ] T032 [US3] Capture User Story 3 validation evidence in `specs/042-visual-artifact-generation/validation-report.md`

**Checkpoint**: Pragmatic C4 depth is encoded in behavior, docs, and release surfaces.

---

## Final Phase: Verification & Compliance

**Purpose**: Finish validation, coverage review, and closeout.

- [ ] T033 [P] Run focused Rust tests for the touched architecture, publish, and release surfaces and record results in `specs/042-visual-artifact-generation/validation-report.md`
- [ ] T034 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/042-visual-artifact-generation/validation-report.md`
- [ ] T035 [P] Run `cargo nextest run` and `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, then review coverage for every modified Rust file and record the closeout in `specs/042-visual-artifact-generation/validation-report.md`
- [ ] T036 Perform an independent readback of the emitted architecture packet against the smart-grid demo under `/Users/rt/Downloads/smart-grid-canon-demo/.canon/artifacts/` and record findings in `specs/042-visual-artifact-generation/validation-report.md`
- [ ] T037 Confirm invariants still hold, close the validation report, and prepare the final commit message in `specs/042-visual-artifact-generation/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: No dependencies. Completed.
- **Phase 1: Setup**: Depends on Phase 0 completion.
- **Phase 2: Foundational**: Depends on Phase 1 and blocks all user stories.
- **Phase 3: User Story 1**: Starts after Foundational and provides the MVP user value.
- **Phase 4: User Story 2**: Starts after User Story 1 stabilizes the primary packet shape.
- **Phase 5: User Story 3**: Starts after the packet and manifest shapes are stable enough to document and gate depth pragmatically.
- **Final Phase**: Depends on all user stories being complete.

### User Story Dependencies

- **US1**: No dependency on later stories.
- **US2**: Depends on US1 because Mermaid sources and manifests need stable primary packet anchors.
- **US3**: Depends on US1 and US2 because pragmatic view depth and docs must reflect the final packet shape.

### Within Each User Story

- Validation tasks happen before implementation tasks.
- Decision log updates happen before or alongside the first dependent code changes.
- Evidence capture happens before the story is declared complete.

### Parallel Opportunities

- `T007` and `T008` can run in parallel once Phase 1 is complete.
- `T011`, `T012`, and `T013` can run in parallel before US1 implementation.
- `T019` and `T020` can run in parallel before US2 implementation.
- `T026` and `T027` can run in parallel before US3 implementation.
- `T033`, `T034`, and `T035` can run independently once implementation is complete.

---

## Parallel Example: User Story 1

```bash
# Write the failing packet checks first:
Task: "Write failing contract coverage for the required primary architecture packet artifacts in tests/contract/architecture_c4_contract.rs"
Task: "Write failing renderer assertions for architecture-overview.md and deployment coverage behavior in tests/architecture_c4_renderer.rs"
Task: "Write failing run/publish assertions for the primary packet output in tests/architecture_c4_run.rs and crates/canon-cli/src/commands/publish.rs"

# Then land the contract and renderer work:
Task: "Add architecture-overview.md and deployment coverage requirements to crates/canon-engine/src/artifacts/contract.rs"
Task: "Implement architecture-overview.md and deployment coverage rendering in crates/canon-engine/src/artifacts/markdown.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Setup and Foundational phases.
2. Deliver the primary architecture overview packet and deployment coverage.
3. Validate the story independently through focused tests and publish readback.

### Incremental Delivery

1. Finish governance, version bump, and optional artifact plumbing.
2. Deliver US1 to fix the human-readability problem first.
3. Deliver US2 to add machine-readable visual artifacts and render capability notes.
4. Deliver US3 to enforce pragmatic depth and align docs.
5. Finish with full validation, coverage review, and closeout.

### Parallel Team Strategy

1. One contributor can handle contract and persistence model changes.
2. One contributor can handle renderer and manifest generation.
3. One contributor can handle docs, release surfaces, and validation closeout after the packet shape stabilizes.