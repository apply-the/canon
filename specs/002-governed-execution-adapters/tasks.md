# Tasks: Governed Execution Adapters

**Input**: Design documents from `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Validation**: Layered validation is mandatory. Add executable tests before
implementation, persist evidence in
`/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`,
and keep independent review separate from generation.

**Organization**: Tasks are grouped into governance, setup, foundational
runtime work, then user stories in priority order so each story is
independently testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (`[US1]`, `[US2]`, `[US3]`)
- Include exact file paths in descriptions

## Constitution Alignment

- No implementation task starts before governance artifacts, risk controls, and
  validation ownership are updated for this increment.
- Invocation governance is evaluated before execution, not after output exists.
- Denied, constrained, approval-gated, and executed invocations all become
  durable evidence under `.canon/`.
- Generation and validation paths remain separately recorded for consequential
  work.
- MCP-compatible tools remain modeled in domain and policy only; runtime MCP
  execution is explicitly excluded from this tranche.
- Any task path outside the approved plan structure is marked as `new file to
  be created intentionally`.

## Phase 0: Governance & Artifacts

**Purpose**: Lock the increment to the corrected risk model, baseline, and
evidence contract before code changes begin.

- [x] T001 Record the corrected `SystemicImpact` classification, baseline assumptions, and scope guardrails in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/decision-log.md`
- [x] T002 Update execution, validation, and independent review checkpoints in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`
- [x] T003 Freeze governed invocation and evidence inspection checkpoints in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/contracts/cli-contract.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/contracts/runtime-evidence-contract.md`
- [x] T004 Record the first-tranche exclusion of MCP runtime execution in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/research.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/decision-log.md`

---

## Phase 1: Setup (Governed Execution Scaffolding)

**Purpose**: Create the new files and module boundaries this increment depends
on without reopening the whole Canon architecture.

- [x] T005 Create execution-domain scaffolding in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/execution.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/mod.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/lib.rs`
- [x] T006 Create invocation and evidence orchestrator scaffolding in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/evidence.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/mod.rs`
- [x] T007 [P] Create invocation persistence scaffolding in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/invocations.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/traces.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/mod.rs`
- [x] T008 [P] Expand policy default scaffolding for governed invocation in `/Users/rt/workspace/apply-the/canon/defaults/policies/adapters.toml` and `/Users/rt/workspace/apply-the/canon/defaults/policies/verification.toml`
- [x] T009 [P] Seed CLI inspection scaffolding for invocation and evidence views in `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/commands/inspect.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/output.rs`

---

## Phase 2: Foundational (Blocking Invocation Layer)

**Purpose**: Build the shared invocation, policy, persistence, and evidence
layer that every story depends on.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

### Validation and Contracts

- [x] T010 [P] Add failing unit tests for invocation policy evaluation in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs`
- [x] T011 [P] Add failing unit tests for capability and trust-boundary classification in `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/capability.rs`
- [x] T012 [P] Add failing unit tests for generation and validation lineage separation in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/evidence.rs`
- [x] T013 [P] Add failing contract coverage for invocation manifests and `evidence.toml` in `/Users/rt/workspace/apply-the/canon/tests/contract/runtime_evidence_contract.rs` (new file to be created intentionally)
- [x] T014 [P] Add failing CLI contract coverage for `inspect invocations`, `inspect evidence`, and invocation-scoped approvals in `/Users/rt/workspace/apply-the/canon/tests/contract/invocation_cli_contract.rs` (new file to be created intentionally)

### Shared Runtime Implementation

- [x] T015 Implement `ExecutionAdapterDescriptor`, `InvocationRequest`, `InvocationPolicyDecision`, `InvocationTrace`, `ToolOutcome`, `GenerationPath`, `ValidationPath`, and `EvidenceBundle` in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/execution.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/mod.rs`
- [x] T016 [P] Extend run, approval, and verification types for invocation evidence linkage in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/run.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/approval.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/verification.rs`
- [x] T017 [P] Extend adapter capability typing, trust boundaries, orientations, and mutability classes in `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/capability.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/dispatcher.rs`
- [x] T018 Implement adapter and verification policy schema updates plus constraint defaults in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/policy.rs`, `/Users/rt/workspace/apply-the/canon/defaults/policies/adapters.toml`, and `/Users/rt/workspace/apply-the/canon/defaults/policies/verification.toml`
- [x] T019 Implement policy loading and parsing for invocation rules and constraint profiles in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/store.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/policy.rs`
- [x] T020 Implement invocation policy evaluation logic and classifier wiring in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/classifier.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs`
- [x] T021 Implement per-invocation request, decision, and attempt persistence in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/invocations.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/layout.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/manifests.rs`
- [x] T022 Implement summary-first trace event append and read helpers in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/traces.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/store.rs`
- [x] T023 Implement run-level evidence bundle persistence and link management in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/evidence.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/store.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/manifests.rs`
- [x] T024 Implement invocation request normalization and preflight validation in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/execution.rs`
- [x] T025 Implement invocation policy checks and constraint application in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/classifier.rs`
- [x] T026 Implement adapter dispatch orchestration for filesystem, shell, and Copilot CLI in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/service.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/filesystem.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/shell.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/copilot_cli.rs`
- [x] T027 Implement invocation outcome mapping and persistence hooks in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/evidence.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/store.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/traces.rs`
- [x] T028 Implement invocation-scoped approval targets and stale resume invalidation in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/approval.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/resume.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/commands/approve.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/commands/resume.rs`
- [x] T029 Implement CLI inspection plumbing for invocation and evidence views in `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/commands/inspect.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/output.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/app.rs`
- [x] T030 Implement artifact provenance fields and execution-derived artifact links in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/artifact.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/artifacts/manifest.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/store.rs`
- [x] T031 Keep `McpStdio` contract-only by wiring explicit runtime denial or disabled policy in `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/mcp_stdio.rs`, `/Users/rt/workspace/apply-the/canon/defaults/policies/adapters.toml`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs`

**Checkpoint**: Canon can evaluate invocation policy before execution, persist
request and evidence records under `.canon/`, expose inspection scaffolding,
and keep MCP runtime explicitly out of scope.

---

## Phase 3: User Story 1 - Govern a Real External Invocation (Priority: P1) 🎯 MVP

**Goal**: Deliver governed external invocation for `requirements` mode so Canon
authorizes real repository and AI-assisted actions before execution and derives
artifacts from durable evidence.

**Independent Test**: Given a `requirements` run with repository context and a
bounded AI-assisted step, Canon persists invocation requests, decisions,
attempts, and evidence-linked artifacts, and records denied requests when
policy blocks a capability.

### Validation for User Story 1 (MANDATORY)

- [x] T032 [P] [US1] Add a failing integration test for governed `requirements` invocations in `/Users/rt/workspace/apply-the/canon/tests/integration/requirements_governed_invocation.rs` (new file to be created intentionally)
- [x] T033 [P] [US1] Add failing contract coverage for requirements evidence-derived artifacts in `/Users/rt/workspace/apply-the/canon/tests/contract/requirements_evidence_contract.rs` (new file to be created intentionally)
- [x] T034 [US1] Record US1 acceptance and evidence checkpoints in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`

### Implementation for User Story 1

- [x] T035 [US1] Wire governed context, generation, and critique requests into `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/modes/requirements.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/service.rs`
- [x] T036 [US1] Implement summary-first Copilot CLI generation and critique execution in `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/copilot_cli.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs`
- [x] T037 [US1] Derive requirements artifacts and provenance from invocation evidence in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/artifacts/markdown.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/artifacts/contract.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/modes/requirements.rs`
- [x] T038 [US1] Extend requirements gate evaluation to consume denied invocations and evidence completeness in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/gatekeeper.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/modes/requirements.rs`
- [x] T039 [US1] Expose requirements run summaries with invocation counts and evidence references in `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/commands/run.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/commands/status.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/output.rs`
- [x] T040 [US1] Capture US1 decisions and validation evidence in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/decision-log.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`

**Checkpoint**: User Story 1 is independently testable and proves governed
external invocation, `inspect invocations`, `inspect evidence`,
invocation-scoped approvals, trace persistence, and evidence-derived artifacts
in `requirements` mode.

---

## Phase 4: User Story 2 - Keep Generation and Validation Separate (Priority: P2)

**Goal**: Deliver `brownfield-change` with governed repository-context
consumption, bounded analysis or generation, and explicit generation versus
validation path separation for consequential work.

**Independent Test**: Given a brownfield run that analyzes repository context
and produces a consequential recommendation, Canon records generation and
validation paths separately, blocks insufficient independence, and keeps risky
mutation recommendation-only or approval-gated.

### Validation for User Story 2 (MANDATORY)

- [x] T041 [P] [US2] Add a failing integration test for brownfield governed repository analysis and validation separation in `/Users/rt/workspace/apply-the/canon/tests/integration/brownfield_governed_execution.rs` (new file to be created intentionally)
- [x] T042 [P] [US2] Add failing contract coverage for brownfield approval-gated invocations and recommendation-only outcomes in `/Users/rt/workspace/apply-the/canon/tests/contract/brownfield_invocation_contract.rs` (new file to be created intentionally)
- [x] T043 [US2] Record US2 validation-independence and approval checkpoints in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`

### Implementation for User Story 2

- [x] T044 [US2] Wire repository-context invocation requests into `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/modes/brownfield_change.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/service.rs`
- [x] T045 [US2] Enforce bounded repository scope and mutability constraints in `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/filesystem.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/shell.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/invocation.rs`
- [x] T046 [US2] Implement generation-path, validation-path, and independence assessment recording in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/execution.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/evidence.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/verification.rs`
- [x] T047 [US2] Attach validation-tool outcomes to brownfield validation paths in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/modes/brownfield_change.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/verification_runner.rs`
- [x] T048 [US2] Extend brownfield gates to block insufficiently independent validation and stale approvals in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/gatekeeper.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/modes/brownfield_change.rs`
- [x] T049 [US2] Capture US2 decisions, approval rationale, and validation evidence in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/decision-log.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`

**Checkpoint**: User Story 2 is independently testable and proves that
consequential brownfield work cannot pass without separately recorded
validation evidence.

---

## Phase 5: User Story 3 - Preserve Evidence of Work in Motion (Priority: P3)

**Goal**: Deliver `pr-review` with real diff inspection, preserved invocation
evidence, and reviewer-facing inspection of what was attempted, allowed,
denied, reviewed, and challenged.

**Independent Test**: Given a `pr-review` run over a real diff, Canon persists
inspection and critique invocations, derives the review packet from evidence,
and exposes those records through inspection commands without replaying tool
execution.

### Validation for User Story 3 (MANDATORY)

- [x] T050 [P] [US3] Add a failing integration test for pr-review invocation evidence and reviewer inspection in `/Users/rt/workspace/apply-the/canon/tests/integration/pr_review_evidence.rs` (new file to be created intentionally)
- [x] T051 [P] [US3] Add failing CLI and contract coverage for pr-review evidence inspection in `/Users/rt/workspace/apply-the/canon/tests/contract/pr_review_evidence_contract.rs` (new file to be created intentionally)
- [x] T052 [US3] Record US3 review-evidence and disposition checkpoints in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`

### Implementation for User Story 3

- [x] T053 [US3] Wire governed diff inspection and critique requests into `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/modes/pr_review.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/service.rs`
- [x] T054 [US3] Extend shell diff ingestion to retain summary-first payload references per request in `/Users/rt/workspace/apply-the/canon/crates/canon-adapters/src/shell.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/persistence/invocations.rs`
- [x] T055 [US3] Derive pr-review findings, review summary, and provenance links from invocation evidence in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/review/findings.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/review/summary.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/artifacts/markdown.rs`
- [x] T056 [US3] Extend pr-review gates and disposition handling to consume denied invocations, approvals, and evidence bundles in `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/orchestrator/gatekeeper.rs` and `/Users/rt/workspace/apply-the/canon/crates/canon-engine/src/domain/approval.rs`
- [x] T057 [US3] Surface reviewer-facing invocation and evidence inspection in `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/commands/status.rs`, `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/commands/inspect.rs`, and `/Users/rt/workspace/apply-the/canon/crates/canon-cli/src/output.rs`
- [x] T058 [US3] Capture US3 evidence and review closeout in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/decision-log.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`

**Checkpoint**: User Story 3 is independently testable and lets reviewers
inspect durable evidence of work in motion.

---

## Final Phase: Verification & Compliance

**Purpose**: Close the increment with structural validation, logical
verification, contract alignment, and independent review.

- [x] T059 Run structural validation for invocation manifests, evidence bundle schema, and CLI contracts in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`
- [x] T060 Run logical integration validation for the delivered US1 `requirements` slice, denied requests, approval-gated requests, and resume flows in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`
- [x] T061 Perform independent review of lineage rules, constraint enforcement, and MCP runtime exclusion in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/decision-log.md`
- [x] T062 Update operator guidance for governed execution and evidence inspection in `/Users/rt/workspace/apply-the/canon/README.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/quickstart.md`
- [x] T063 Confirm the implemented CLI and runtime evidence behavior still matches `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/contracts/cli-contract.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/contracts/runtime-evidence-contract.md`
- [x] T064 Record the final milestone decision and closeout evidence in `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/decision-log.md` and `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: No dependencies. MUST complete first.
- **Phase 1: Setup**: Depends on Phase 0 completion.
- **Phase 2: Foundational**: Depends on Phase 1 completion. BLOCKS all user stories.
- **Phase 3: User Story 1**: Depends on Phase 2 completion.
- **Phase 4: User Story 2**: Depends on Phase 3 completion and the US1 stability checkpoint being satisfied.
- **Phase 5: User Story 3**: Depends on Phase 3 completion and the US1 stability checkpoint being satisfied.
- **Final Phase**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: First MVP. No dependency on other stories once Foundational completes.
- **User Story 2 (P2)**: MUST NOT start until US1 has proven `inspect invocations`, `inspect evidence`, invocation-scoped approvals, trace persistence, and end-to-end `requirements` stability.
- **User Story 3 (P3)**: MUST NOT start until US1 has proven `inspect invocations`, `inspect evidence`, invocation-scoped approvals, trace persistence, and end-to-end `requirements` stability.

### Within Each User Story

- Failing validation or contract checks MUST be written before story-specific implementation.
- Decision and validation artifacts MUST be updated before the story is considered complete.
- Invocation wiring MUST precede artifact derivation and gate enforcement.
- Gate logic MUST precede reviewer-facing or operator-facing closeout.

---

## Parallel Opportunities

- Setup tasks T007-T009 can run in parallel after T005-T006 establish the new module boundaries.
- Foundational validation tasks T010-T014 can run in parallel.
- Domain and adapter typing tasks T016-T017 can run in parallel after T015 begins the execution model.
- Story validation tasks marked `[P]` can run in parallel before implementation starts.
- US2 and US3 may be staffed in parallel only after the US1 stability checkpoint is complete.

---

## Parallel Example: User Story 1

```bash
# Validation work in parallel
Task: "T032 Add a failing integration test for governed requirements invocations in tests/integration/requirements_governed_invocation.rs"
Task: "T033 Add failing contract coverage for requirements evidence-derived artifacts in tests/contract/requirements_evidence_contract.rs"

# After tests exist, split adapter and artifact work
Task: "T036 Implement summary-first Copilot CLI generation and critique execution in crates/canon-adapters/src/copilot_cli.rs and crates/canon-engine/src/orchestrator/invocation.rs"
Task: "T037 Derive requirements artifacts and provenance from invocation evidence in crates/canon-engine/src/artifacts/markdown.rs, crates/canon-engine/src/artifacts/contract.rs, and crates/canon-engine/src/modes/requirements.rs"
```

## Parallel Example: User Story 2

```bash
# Validation work in parallel
Task: "T041 Add a failing integration test for brownfield governed repository analysis and validation separation in tests/integration/brownfield_governed_execution.rs"
Task: "T042 Add failing contract coverage for brownfield approval-gated invocations and recommendation-only outcomes in tests/contract/brownfield_invocation_contract.rs"

# After tests exist, split scope enforcement and evidence modeling
Task: "T045 Enforce bounded repository scope and mutability constraints in crates/canon-adapters/src/filesystem.rs, crates/canon-adapters/src/shell.rs, and crates/canon-engine/src/orchestrator/invocation.rs"
Task: "T046 Implement generation-path, validation-path, and independence assessment recording in crates/canon-engine/src/domain/execution.rs, crates/canon-engine/src/orchestrator/evidence.rs, and crates/canon-engine/src/domain/verification.rs"
```

## Parallel Example: User Story 3

```bash
# Validation work in parallel
Task: "T050 Add a failing integration test for pr-review invocation evidence and reviewer inspection in tests/integration/pr_review_evidence.rs"
Task: "T051 Add failing CLI and contract coverage for pr-review evidence inspection in tests/contract/pr_review_evidence_contract.rs"

# After tests exist, split diff capture and review derivation
Task: "T054 Extend shell diff ingestion to retain summary-first payload references per request in crates/canon-adapters/src/shell.rs and crates/canon-engine/src/persistence/invocations.rs"
Task: "T055 Derive pr-review findings, review summary, and provenance links from invocation evidence in crates/canon-engine/src/review/findings.rs, crates/canon-engine/src/review/summary.rs, and crates/canon-engine/src/artifacts/markdown.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0, Phase 1, and Phase 2.
2. Complete Phase 3 and stop at T040.
3. Run the requirements governed invocation integration and contract tests.
4. Confirm `.canon/` persistence now includes invocation manifests, trace persistence, `inspect invocations`, `inspect evidence`, and invocation-scoped approval behavior.
5. Update `/Users/rt/workspace/apply-the/canon/specs/002-governed-execution-adapters/validation-report.md` before broadening scope.

### Incremental Delivery

1. Deliver the shared invocation layer, evidence persistence, and inspection surfaces.
2. Ship `requirements` as the first governed execution slice.
3. Do not start `brownfield-change` or `pr-review` until the US1 stability checkpoint is complete.
4. Add `brownfield-change` to prove repository-context governance and validation separation.
5. Add `pr-review` to prove reviewer-facing evidence preservation.
6. Finish with structural validation, independent review, and operator guidance.

### Suggested MVP Scope

- Phase 0
- Phase 1
- Phase 2
- Phase 3 (US1) only

MCP runtime execution is intentionally excluded from the MVP and from the full
task list for this tranche.

---

## Notes

- `[P]` tasks = different files, no dependencies
- `[US1]`, `[US2]`, and `[US3]` preserve story traceability
- Every story has explicit test, implementation, and evidence-capture tasks
- No task introduces a generic plugin runtime or arbitrary adapter marketplace
- No task re-centers Canon on documents instead of invocation governance plus durable evidence
