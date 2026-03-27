# Tasks: Canon v0.1 Native CLI

**Input**: Design documents from `/Users/rt/workspace/apply-the/canon/specs/001-canon-spec/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Validation**: Layered validation is mandatory. Add executable tests before
mode-specific implementation, persist evidence in `validation-report.md`, and
keep independent review separate from generation.

**Organization**: Tasks are grouped into governance, setup, foundational
architecture, then user stories in priority order so each story remains
independently testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (`[US1]`, `[US2]`, `[US3]`)
- Include exact file paths in descriptions

## Constitution Alignment

- No implementation task starts before governance artifacts, risk controls, and
  validation ownership are captured.
- All twelve modes are modeled in code now; only `requirements`,
  `brownfield-change`, and `pr-review` receive full v0.1 execution depth.
- Mutating red-zone or systemic-impact execution remains recommendation-only.
- Every story includes executable validation tasks, evidence capture, and an
  independent review checkpoint.

## Phase 0: Governance & Artifacts

**Purpose**: Lock the implementation work to the approved architecture, risk
model, and validation plan before code lands.

- [X] T001 Record implementation kickoff scope, invariants, and MVP sequencing in `specs/001-canon-spec/decision-log.md`
- [X] T002 Update structural, logical, and independent review evidence sections in `specs/001-canon-spec/validation-report.md`
- [X] T003 Add CLI and runtime filesystem contract checkpoints to `specs/001-canon-spec/validation-report.md` and `specs/001-canon-spec/quickstart.md`
- [X] T004 Record systemic-impact approval and recommendation-only expectations for brownfield and review flows in `specs/001-canon-spec/decision-log.md`

---

## Phase 1: Setup (Repository Bootstrap and Rust Toolchain)

**Purpose**: Create the repository skeleton, Rust toolchain controls, and
default policy/method files that every later slice depends on.

- [X] T005 Create the root Rust workspace manifest and shared package metadata in `Cargo.toml`
- [X] T006 Create the pinned Rust toolchain and formatter configuration in `rust-toolchain.toml` and `rustfmt.toml`
- [X] T007 [P] Create the dependency and license policy scaffold in `deny.toml`
- [X] T008 [P] Create the fail-fast hook installer and pre-commit hook in `scripts/install-hooks.sh` and `.githooks/pre-commit`
- [X] T009 [P] Create crate manifests for the CLI, engine, and adapters in `crates/canon-cli/Cargo.toml`, `crates/canon-engine/Cargo.toml`, and `crates/canon-adapters/Cargo.toml`
- [X] T010 [P] Create the base source tree and entry files in `crates/canon-cli/src/main.rs`, `crates/canon-cli/src/app.rs`, `crates/canon-engine/src/lib.rs`, and `crates/canon-adapters/src/lib.rs`
- [X] T011 [P] Create built-in method and policy default files for all twelve modes in `defaults/methods/*.toml` and `defaults/policies/*.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared architecture that all modes and all stories rely
on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

### Validation and Contracts

- [X] T012 [P] Add failing CLI contract coverage for `init`, `status`, and `inspect` in `tests/contract/cli_contract.rs`
- [X] T013 [P] Add failing runtime filesystem contract coverage for `.canon` and run manifests in `tests/contract/runtime_filesystem.rs`
- [X] T014 [P] Add a failing integration test for `init` materializing `.canon` and defaults in `tests/integration/init_creates_canon.rs`

### Workspace and Module Structure

- [X] T015 Create the CLI command surface for `init`, `run`, `resume`, `status`, `approve`, `verify`, and `inspect` in `crates/canon-cli/src/app.rs` and `crates/canon-cli/src/commands/*.rs`
- [X] T016 Create the engine module registry and mode module stubs for all twelve modes in `crates/canon-engine/src/lib.rs` and `crates/canon-engine/src/modes/*.rs`

### Core Domain Types

- [X] T017 Create `Mode` and `ModeProfile` types for all twelve modes in `crates/canon-engine/src/domain/mode.rs`
- [X] T018 [P] Create `RiskClass`, `UsageZone`, `GateKind`, `GateStatus`, and `PolicySet` types in `crates/canon-engine/src/domain/policy.rs` and `crates/canon-engine/src/domain/gate.rs`
- [X] T019 [P] Create `ArtifactRequirement`, `ArtifactContract`, `ArtifactRecord`, `DecisionRecord`, `VerificationRecord`, and `ApprovalRecord` types in `crates/canon-engine/src/domain/artifact.rs`, `crates/canon-engine/src/domain/decision.rs`, `crates/canon-engine/src/domain/verification.rs`, and `crates/canon-engine/src/domain/approval.rs`
- [X] T020 Create `Run`, `RunContext`, `RunState`, `StopCondition`, `ExitCriteria`, and `StepDefinition` types in `crates/canon-engine/src/domain/run.rs` and `crates/canon-engine/src/domain/method.rs`

### Run Model and Persistence

- [X] T021 Create `.canon` layout definitions and atomic file helpers in `crates/canon-engine/src/persistence/layout.rs` and `crates/canon-engine/src/persistence/atomic.rs`
- [X] T022 Create run manifest, state, and links persistence in `crates/canon-engine/src/persistence/manifests.rs` and `crates/canon-engine/src/persistence/store.rs`
- [X] T023 Create artifact contract persistence and artifact link manifests in `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/manifest.rs`

### Artifact Model and Rendering

- [X] T024 Create Markdown, JSON, and YAML artifact renderers in `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/artifacts/json.rs`, and `crates/canon-engine/src/artifacts/yaml.rs`

### Policy and Gate Engine

- [X] T025 Create policy loading and typed merge logic for embedded defaults and local overrides in `crates/canon-engine/src/orchestrator/classifier.rs` and `crates/canon-engine/src/domain/policy.rs`
- [X] T026 Create the gatekeeper service and persisted `GateEvaluation` flow in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/persistence/store.rs`

### Adapter Abstraction and First Adapters

- [X] T027 Create adapter request, capability, side-effect, and trace types in `crates/canon-adapters/src/capability.rs` and `crates/canon-adapters/src/dispatcher.rs`
- [X] T028 [P] Implement the `FilesystemAdapter` and trace emission in `crates/canon-adapters/src/filesystem.rs` and `crates/canon-engine/src/persistence/store.rs`
- [X] T029 [P] Implement the `ShellAdapter` with read-only versus mutating capability separation in `crates/canon-adapters/src/shell.rs` and `crates/canon-adapters/src/dispatcher.rs`
- [X] T030 [P] Scaffold recommendation-only `CopilotCliAdapter` and optional `McpStdioAdapter` paths in `crates/canon-adapters/src/copilot_cli.rs` and `crates/canon-adapters/src/mcp_stdio.rs`

### Verification Hooks and CLI Skeleton

- [X] T031 Create critique, findings, summary, and verification runner scaffolding in `crates/canon-engine/src/review/critique.rs`, `crates/canon-engine/src/review/findings.rs`, `crates/canon-engine/src/review/summary.rs`, and `crates/canon-engine/src/orchestrator/verification_runner.rs`
- [X] T032 Wire the runnable CLI skeleton for `init`, `status`, and `inspect` to engine services in `crates/canon-cli/src/commands/init.rs`, `crates/canon-cli/src/commands/status.rs`, `crates/canon-cli/src/commands/inspect.rs`, and `crates/canon-cli/src/output.rs`

**Checkpoint**: The repository builds a runnable CLI skeleton, exposes all
twelve modes in typed form, persists `.canon` structures, and enforces the core
contracts needed for the first end-to-end mode.

---

## Phase 3: User Story 1 - Requirements Mode (Priority: P1) 🎯 MVP

**Goal**: Deliver one complete end-to-end governed requirements flow that
classifies risk and zone, persists a run, writes an artifact contract, emits the
required artifact set, and blocks on missing scope discipline.

**Independent Test**: Given a raw product idea and explicit constraints, `run
--mode requirements` creates a run folder, persists the artifact contract, emits
the six requirements artifacts, and refuses readiness if scope cuts or tradeoffs
are missing.

### Validation for User Story 1 (MANDATORY)

- [X] T033 [P] [US1] Add a failing end-to-end test for `run --mode requirements` in `tests/integration/requirements_run.rs`
- [X] T034 [P] [US1] Add requirements CLI and artifact contract snapshots in `tests/contract/requirements_contract.rs`
- [X] T035 [US1] Record the MVP acceptance evidence checklist for requirements mode in `specs/001-canon-spec/validation-report.md`

### Implementation for User Story 1

- [X] T036 [US1] Implement the requirements mode profile, step sequence, and artifact requirements in `crates/canon-engine/src/modes/requirements.rs` and `defaults/methods/requirements.toml`
- [X] T037 [US1] Implement renderers and minimum-section validators for `problem-statement.md`, `constraints.md`, `options.md`, `tradeoffs.md`, `scope-cuts.md`, and `decision-checklist.md` in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/artifacts/contract.rs`
- [X] T038 [US1] Implement run classification, artifact-contract creation, and requirements step orchestration in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/classifier.rs` (depends on T017-T026)
- [X] T039 [US1] Wire `run --mode requirements` and requirements artifact inspection in `crates/canon-cli/src/commands/run.rs` and `crates/canon-cli/src/commands/inspect.rs` (depends on T036-T038)
- [X] T040 [US1] Implement `Exploration`, `Risk`, and `ReleaseReadiness` gate checks for requirements artifacts in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/modes/requirements.rs` (depends on T036-T038)
- [X] T041 [US1] Implement self-critique and adversarial critique recording for requirements runs in `crates/canon-engine/src/orchestrator/verification_runner.rs` and `crates/canon-engine/src/review/critique.rs` (depends on T031 and T038)
- [X] T042 [US1] Record MVP completion evidence and decisions in `specs/001-canon-spec/validation-report.md` and `specs/001-canon-spec/decision-log.md`

**Checkpoint**: User Story 1 is independently runnable and produces a complete
requirements artifact bundle.

---

## Phase 4: User Story 2 - Brownfield Change Mode (Priority: P2)

**Goal**: Deliver the brownfield planning flow that makes preserved behavior,
change surface, approvals, and recommendation-only execution explicit before any
change plan is accepted.

**Independent Test**: Given a repository slice and change goal, `run --mode
brownfield-change` persists the run, emits the brownfield artifact contract,
blocks when `legacy-invariants.md` or `change-surface.md` are missing, and
requires approval evidence for systemic or red-zone work.

### Validation for User Story 2 (MANDATORY)

- [X] T043 [P] [US2] Add a failing end-to-end test for blocked and successful brownfield runs in `tests/integration/brownfield_run.rs`
- [X] T044 [P] [US2] Add approval and blocked-run CLI contract coverage in `tests/contract/brownfield_contract.rs`
- [X] T045 [US2] Record brownfield preservation and approval evidence checkpoints in `specs/001-canon-spec/validation-report.md`

### Implementation for User Story 2

- [X] T046 [US2] Implement the brownfield-change mode profile, step sequence, and artifact requirements in `crates/canon-engine/src/modes/brownfield_change.rs` and `defaults/methods/brownfield-change.toml`
- [X] T047 [US2] Implement renderers and validators for `system-slice.md`, `legacy-invariants.md`, `change-surface.md`, `implementation-plan.md`, `validation-strategy.md`, and `decision-record.md` in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/artifacts/contract.rs`
- [X] T048 [US2] Implement `BrownfieldPreservation`, `Architecture`, and `Risk` gate behavior for constrained change planning in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/modes/brownfield_change.rs` (depends on T046-T047)
- [X] T049 [US2] Implement blocked-run approval persistence and `approve` command handling in `crates/canon-engine/src/domain/approval.rs`, `crates/canon-engine/src/persistence/store.rs`, and `crates/canon-cli/src/commands/approve.rs` (depends on T022 and T048)
- [X] T050 [US2] Implement stale-context detection and `resume` or fork behavior for blocked runs in `crates/canon-engine/src/orchestrator/resume.rs` and `crates/canon-cli/src/commands/resume.rs` (depends on T020-T023)
- [X] T051 [US2] Enforce recommendation-only mutating adapter policy for red-zone or systemic brownfield runs in `crates/canon-adapters/src/dispatcher.rs` and `crates/canon-engine/src/orchestrator/classifier.rs` (depends on T027-T030 and T048)
- [X] T052 [US2] Record brownfield validation evidence, approval outcomes, and invariants in `specs/001-canon-spec/validation-report.md` and `specs/001-canon-spec/decision-log.md`

**Checkpoint**: User Story 2 is independently testable and showcases the
governed brownfield discipline.

---

## Phase 5: User Story 3 - PR Review Mode (Priority: P3)

**Goal**: Deliver the structured PR review flow that produces the full review
packet, maps findings to changed surfaces, and requires explicit disposition for
high-impact issues.

**Independent Test**: Given a branch or diff, `run --mode pr-review` emits the
full review artifact set, links findings to changed files, and blocks readiness
until unresolved high-impact findings receive a recorded disposition.

### Validation for User Story 3 (MANDATORY)

- [X] T053 [P] [US3] Add a failing end-to-end test for `run --mode pr-review` in `tests/integration/pr_review_run.rs`
- [X] T054 [P] [US3] Add review disposition exit-code and summary contract coverage in `tests/contract/pr_review_contract.rs`
- [X] T055 [US3] Record independent review and disposition checkpoints in `specs/001-canon-spec/validation-report.md`

### Implementation for User Story 3

- [X] T056 [US3] Implement the pr-review mode profile, step sequence, and artifact requirements in `crates/canon-engine/src/modes/pr_review.rs` and `defaults/methods/pr-review.toml`
- [X] T057 [US3] Implement diff ingestion and changed-surface collection through the shell adapter in `crates/canon-adapters/src/shell.rs` and `crates/canon-engine/src/orchestrator/service.rs` (depends on T029 and T056)
- [X] T058 [US3] Implement renderers and findings models for `pr-analysis.md`, `boundary-check.md`, `duplication-check.md`, `contract-drift.md`, `missing-tests.md`, `decision-impact.md`, and `review-summary.md` in `crates/canon-engine/src/review/findings.rs`, `crates/canon-engine/src/review/summary.rs`, and `crates/canon-engine/src/artifacts/markdown.rs`
- [X] T059 [US3] Implement `Architecture`, `ReviewDisposition`, and `ReleaseReadiness` gate logic for pr-review runs in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/modes/pr_review.rs` (depends on T056-T058)
- [X] T060 [US3] Wire `run --mode pr-review`, review artifact inspection, and reviewer-facing status output in `crates/canon-cli/src/commands/run.rs`, `crates/canon-cli/src/commands/status.rs`, and `crates/canon-cli/src/commands/inspect.rs` (depends on T056-T059)
- [X] T061 [US3] Record review evidence and final disposition handling in `crates/canon-engine/src/orchestrator/verification_runner.rs` and `specs/001-canon-spec/validation-report.md`

**Checkpoint**: User Story 3 is independently testable and produces the full
governed review packet.

---

## Final Phase: Verification & Compliance

**Purpose**: Finish repository quality gates, non-MVP mode coverage, and final
compliance evidence after the three deep modes are complete.

- [X] T062 [P] Add typed profile coverage for non-MVP modes and `inspect modes` assertions in `tests/integration/mode_profiles.rs` and `tests/contract/inspect_modes.rs`
- [X] T063 [P] Implement the GitHub Actions format, lint, test, and nextest workflow in `.github/workflows/ci.yml`
- [X] T064 [P] Implement MSRV verification, cargo-deny, permissive-license policy, and staged cross-platform build jobs in `.github/workflows/ci.yml`, `deny.toml`, and `rust-toolchain.toml`
- [X] T065 [P] Finalize fail-fast local quality gates in `.githooks/pre-commit` and `scripts/install-hooks.sh`
- [X] T066 Update user-facing bootstrap and mode guidance in `README.md` and `specs/001-canon-spec/quickstart.md`
- [X] T067 Record structural, logical, and independent review completion evidence in `specs/001-canon-spec/validation-report.md`
- [X] T068 Perform final adversarial review closeout and milestone decision logging in `specs/001-canon-spec/validation-report.md` and `specs/001-canon-spec/decision-log.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: No dependencies. MUST complete first.
- **Phase 1: Setup**: Depends on Phase 0 completion.
- **Phase 2: Foundational**: Depends on Phase 1 completion. BLOCKS all user stories.
- **Phase 3: User Story 1**: Depends on Phase 2 completion.
- **Phase 4: User Story 2**: Depends on Phase 3 completion for the shared run, approval, and verification flows used by brownfield execution.
- **Phase 5: User Story 3**: Depends on Phase 3 completion and should start after Phase 4 if the same team is finishing approval and resume behavior first.
- **Final Phase**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: First MVP. No dependency on other stories once Foundational completes.
- **User Story 2 (P2)**: Depends on the common run orchestration, gate engine, and verification hooks built for US1.
- **User Story 3 (P3)**: Depends on the common run orchestration plus the review and verification plumbing stabilized by US1 and US2.

### Within Each User Story

- Validation tests and contract checks MUST be written before mode-specific implementation.
- Artifact renderers and validators MUST exist before gate logic can pass.
- Gate logic MUST exist before CLI completion or approval behaviors are considered valid.
- Evidence capture and decision logging MUST happen before the story is declared done.

---

## Parallel Opportunities

- Setup tasks marked `[P]` can run in parallel after T005-T006 establish the workspace root.
- Foundational validation tasks T012-T014 can run in parallel.
- Domain tasks T018-T019 can run in parallel after T017 begins the mode model.
- Adapter tasks T028-T030 can run in parallel after T027 defines shared capability types.
- Within each user story, the `[P]` validation tasks can run in parallel before implementation starts.

---

## Parallel Example: User Story 1

```bash
# Validation work in parallel
Task: "T033 Add a failing end-to-end test for run --mode requirements in tests/integration/requirements_run.rs"
Task: "T034 Add requirements CLI and artifact contract snapshots in tests/contract/requirements_contract.rs"

# After the tests exist, split artifact and flow work
Task: "T036 Implement the requirements mode profile in crates/canon-engine/src/modes/requirements.rs and defaults/methods/requirements.toml"
Task: "T037 Implement requirements artifact renderers and validators in crates/canon-engine/src/artifacts/markdown.rs and crates/canon-engine/src/artifacts/contract.rs"
```

## Parallel Example: User Story 2

```bash
# Brownfield validation work in parallel
Task: "T043 Add a failing end-to-end test for brownfield runs in tests/integration/brownfield_run.rs"
Task: "T044 Add approval and blocked-run CLI contract coverage in tests/contract/brownfield_contract.rs"

# After the tests exist, split mode and approval work
Task: "T046 Implement the brownfield-change mode profile in crates/canon-engine/src/modes/brownfield_change.rs and defaults/methods/brownfield-change.toml"
Task: "T047 Implement brownfield artifact renderers and validators in crates/canon-engine/src/artifacts/markdown.rs and crates/canon-engine/src/artifacts/contract.rs"
```

## Parallel Example: User Story 3

```bash
# Review validation work in parallel
Task: "T053 Add a failing end-to-end test for pr-review in tests/integration/pr_review_run.rs"
Task: "T054 Add review disposition contract coverage in tests/contract/pr_review_contract.rs"

# After the tests exist, split diff ingestion and artifact rendering
Task: "T057 Implement diff ingestion through crates/canon-adapters/src/shell.rs and crates/canon-engine/src/orchestrator/service.rs"
Task: "T058 Implement review findings and renderers in crates/canon-engine/src/review/findings.rs, crates/canon-engine/src/review/summary.rs, and crates/canon-engine/src/artifacts/markdown.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0, Phase 1, and Phase 2.
2. Complete Phase 3 and stop at T042.
3. Run the requirements integration and contract tests.
4. Confirm `.canon/` persistence, artifact contract support, and the six
   requirements artifacts.
5. Update `specs/001-canon-spec/validation-report.md` before expanding
   scope.

### Incremental Delivery

1. Deliver the governed CLI skeleton and typed domain model.
2. Ship the complete requirements flow as the first usable milestone.
3. Add brownfield-change to demonstrate preserved invariants, approvals, and
   recommendation-only execution in high-risk zones.
4. Add pr-review to showcase structured outward-facing review artifacts.
5. Finish with CI, cross-platform verification, and compliance closeout.

### Suggested MVP Scope

- Stop after **Phase 3: User Story 1** for the first external milestone.
- Use **Phase 4** as the discipline showcase for governed engineering.
- Use **Phase 5** as the first compelling review-centric demonstration.
