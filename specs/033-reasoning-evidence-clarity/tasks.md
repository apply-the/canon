# Tasks: Cross-Mode Reasoning Evidence And Clarity Expansion

**Input**: Design documents from `/specs/033-reasoning-evidence-clarity/`
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

**Purpose**: Establish the controls, release boundary, and planning artifacts
that permit implementation to start

- [x] T001 Set Canon version to `0.33.0` in `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-scope decisions and full-feature notes in `specs/033-reasoning-evidence-clarity/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/033-reasoning-evidence-clarity/validation-report.md`
- [x] T004 Confirm clarity, reasoning-evidence, and release-alignment contracts in `specs/033-reasoning-evidence-clarity/contracts/clarity-and-reasoning-contract.md` and `specs/033-reasoning-evidence-clarity/contracts/release-alignment.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and release-guard setup

- [x] T005 Update agent context from `specs/033-reasoning-evidence-clarity/plan.md` into `AGENTS.md`
- [x] T006 Create or extend release-alignment bootstrap coverage for `0.33.0` in `tests/integration/skills_bootstrap.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared clarity, summary, and renderer behavior that all user
stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 [P] Create or extend failing clarity-inspection coverage in `tests/inspect_clarity.rs` and `tests/contract/inspect_clarity.rs`
- [x] T008 [P] Create or extend failing shared packet-posture coverage in `tests/backlog_contract.rs`, `tests/backlog_run.rs`, `tests/backlog_closure_run.rs`, `tests/review_contract.rs`, `tests/review_run.rs`, `tests/verification_contract.rs`, `tests/verification_run.rs`, `tests/pr_review_contract.rs`, and `tests/pr_review_run.rs`
- [x] T009 [P] Create or extend failing shared docs and renderer coverage in `tests/review_authoring_docs.rs`, `tests/review_authoring_renderer.rs`, `tests/verification_authoring_docs.rs`, `tests/verification_authoring_renderer.rs`, and the impacted `*_authoring_docs.rs` mode tests
- [x] T010 Extend shared clarity dispatch and reasoning-signal helpers in `crates/canon-engine/src/orchestrator/service/inspect.rs` and `crates/canon-engine/src/orchestrator/service/clarity.rs`
- [x] T011 Extend shared summary, readiness, and blocker posture in `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/orchestrator/gatekeeper.rs`, and `crates/canon-cli/src/output.rs`
- [x] T012 Extend shared artifact-contract and renderer fallback behavior in `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T013 Capture foundational invariant, coverage, and touched-Rust-file validation expectations in `specs/033-reasoning-evidence-clarity/validation-report.md`

**Checkpoint**: Shared runtime posture is ready for story-level rollout across
clarity, packet summaries, and fallback tightening

---

## Phase 3: User Story 1 - Inspect Clarity Across Governed Modes (Priority: P1) 🎯 MVP

**Goal**: Expand clarity inspection and reasoning signals across the remaining
file-backed governed modes

**Independent Test**: Representative authored briefs for the targeted
file-backed modes return structured clarity output with non-empty reasoning
signals and missing-context or materially-closed posture where applicable.

### Validation for User Story 1 (MANDATORY)

- [x] T014 [P] [US1] Add failing clarity behavior coverage for planning and change-family modes in `tests/inspect_clarity.rs` and `tests/contract/inspect_clarity.rs`
- [x] T015 [P] [US1] Add failing clarity behavior coverage for execution, incident, assessment, and migration families in `tests/inspect_clarity.rs` and `tests/contract/inspect_clarity.rs`
- [x] T016 [US1] Record clarity-expansion decisions under `## User Story 1 Decisions` in `specs/033-reasoning-evidence-clarity/decision-log.md`

### Implementation for User Story 1

- [x] T017 [US1] Implement file-backed clarity support and reasoning signals in `crates/canon-engine/src/orchestrator/service/inspect.rs` and `crates/canon-engine/src/orchestrator/service/clarity.rs`
- [x] T018 [P] [US1] Update `canon-inspect-clarity` guidance in `defaults/embedded-skills/canon-inspect-clarity/skill-source.md` and `.agents/skills/canon-inspect-clarity/SKILL.md`
- [x] T019 [US1] Capture clarity-expansion validation evidence in `specs/033-reasoning-evidence-clarity/validation-report.md`

**Checkpoint**: File-backed governed modes expose pre-run clarity and
reasoning signals instead of unsupported-target failures

---

## Phase 4: User Story 2 - Read Honest Reasoning In Emitted Packets (Priority: P2)

**Goal**: Tighten packet posture so shallow reasoning and generic fallback
content are exposed honestly across summaries and affected artifacts

**Independent Test**: Representative planning, review, and analysis packets
either preserve real reasoning evidence or surface explicit incompleteness,
with no generic fallback prose that looks like authored reasoning.

### Validation for User Story 2 (MANDATORY)

- [x] T020 [P] [US2] Add failing fallback-tightening coverage in `tests/backlog_contract.rs`, `tests/backlog_run.rs`, and `tests/backlog_closure_run.rs`
- [x] T021 [P] [US2] Add failing review-family honesty coverage in `tests/review_contract.rs`, `tests/review_run.rs`, `tests/verification_contract.rs`, `tests/verification_run.rs`, `tests/pr_review_contract.rs`, and `tests/pr_review_run.rs`
- [x] T022 [US2] Record packet-posture and fallback decisions under `## User Story 2 Decisions` in `specs/033-reasoning-evidence-clarity/decision-log.md`

### Implementation for User Story 2

- [x] T023 [US2] Implement packet-summary and blocker-posture changes in `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/orchestrator/gatekeeper.rs`, and `crates/canon-cli/src/output.rs`
- [x] T024 [US2] Tighten fallback-heavy artifact rendering in `crates/canon-engine/src/artifacts/markdown.rs` and any required headings in `crates/canon-engine/src/artifacts/contract.rs`
- [x] T025 [US2] Capture packet-posture validation evidence in `specs/033-reasoning-evidence-clarity/validation-report.md`

**Checkpoint**: Affected packets and summaries now distinguish grounded
reasoning from shallow or closure-limited output

---

## Phase 5: User Story 3 - Ship 0.33.0 With Synchronized Authoring Surfaces And Validation (Priority: P3)

**Goal**: Align mirrored skills, templates, examples, docs, changelog, and
release surfaces with the new reasoning contract

**Independent Test**: A maintainer can inspect the updated skills, templates,
docs, and release surfaces and see the same `0.33.0` reasoning-evidence
contract described everywhere without runtime drift.

### Validation for User Story 3 (MANDATORY)

- [x] T026 [P] [US3] Validate skill-sync and release alignment without prose-coupled repository-doc tests by using `tests/integration/skills_bootstrap.rs`, `/bin/bash scripts/validate-canon-skills.sh`, and the impacted runtime-backed `*_authoring_docs.rs` mode tests
- [x] T027 [US3] Record release and authoring-surface decisions under `## User Story 3 Decisions` in `specs/033-reasoning-evidence-clarity/decision-log.md`

### Implementation for User Story 3

- [x] T028 [P] [US3] Update shared embedded reasoning-contract guidance in `defaults/embedded-skills/canon-inspect-clarity/skill-source.md` and `defaults/embedded-skills/canon-shared/references/output-shapes.md`
- [x] T029 [P] [US3] Mirror the shared reasoning-contract guidance in `.agents/skills/canon-inspect-clarity/SKILL.md` and `.agents/skills/canon-shared/references/output-shapes.md`
- [x] T030 [P] [US3] Review `docs/templates/canon-input/` and `docs/examples/canon-input/` for the impacted mode families and record when the existing H2 contracts already satisfy the `0.33.0` reasoning contract without further edits
- [x] T031 [US3] Update impacted docs and changelog closeout in `README.md`, `ROADMAP.md`, `docs/guides/modes.md`, `docs/guides/publishing-to-winget.md`, `docs/guides/publishing-to-scoop.md`, and `CHANGELOG.md`, keeping `0.33.0` and the delivered reasoning contract synchronized
- [x] T032 [US3] Capture docs, skill-sync, and release-alignment validation evidence in `specs/033-reasoning-evidence-clarity/validation-report.md`

**Checkpoint**: Runtime and authoring surfaces agree on the delivered
reasoning-evidence posture at `0.33.0`

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, coverage, independent review, and closeout

- [x] T033 [P] Run `/bin/bash scripts/validate-canon-skills.sh` and record results in `specs/033-reasoning-evidence-clarity/validation-report.md`
- [x] T034 [P] Run the targeted feature suite for `tests/inspect_clarity.rs`, `tests/contract/inspect_clarity.rs`, `tests/integration/skills_bootstrap.rs`, `tests/backlog_contract.rs`, `tests/backlog_run.rs`, `tests/backlog_closure_run.rs`, `tests/review_contract.rs`, `tests/review_run.rs`, `tests/review_authoring_docs.rs`, `tests/review_authoring_renderer.rs`, `tests/verification_contract.rs`, `tests/verification_run.rs`, `tests/verification_authoring_docs.rs`, `tests/verification_authoring_renderer.rs`, `tests/pr_review_contract.rs`, `tests/pr_review_run.rs`, and `tests/pr_review_docs.rs`, then record results in `specs/033-reasoning-evidence-clarity/validation-report.md`
- [x] T035 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and document coverage for every modified or newly created Rust file in `specs/033-reasoning-evidence-clarity/validation-report.md`
- [x] T036 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/033-reasoning-evidence-clarity/validation-report.md`
- [x] T037 [P] Run `cargo nextest run --workspace --all-features` and record results in `specs/033-reasoning-evidence-clarity/validation-report.md`
- [x] T038 Perform independent review of invariants, honesty markers, release alignment, and final diff in `specs/033-reasoning-evidence-clarity/validation-report.md`
- [x] T039 Confirm invariants still hold and close the final validation state in `specs/033-reasoning-evidence-clarity/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the clarity-expansion MVP.
- **User Story 2 (P2)**: Can start after Foundational. Reuses the shared runtime surfaces from Phase 2 and remains independently testable through packet posture.
- **User Story 3 (P3)**: Depends on the implemented runtime contract from User Stories 1 and 2 so the synchronized skills, docs, and release surfaces describe the delivered behavior accurately.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- Runtime code before mirrored skills when behavior changes first define the contract.
- Embedded skills before mirrored `.agents` copies.
- Templates before examples when both change in the same story.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- Phase 0 tasks after T001 can run in parallel where they touch different planning artifacts.
- T007, T008, and T009 can run in parallel before the shared engine changes in T010 through T012.
- Within US3, embedded skills, mirrored skills, and docs review can run in parallel once the runtime contract is stable.
- Final validation tasks T033 through T037 can run in parallel once implementation is stable.

---

## Parallel Example: User Story 1

```bash
# Launch clarity coverage for different behavior families in parallel:
Task: "Add failing clarity behavior coverage for planning and change-family modes in tests/inspect_clarity.rs and tests/contract/inspect_clarity.rs"
Task: "Add failing clarity behavior coverage for execution, incident, assessment, and migration families in tests/inspect_clarity.rs and tests/contract/inspect_clarity.rs"

# Launch guidance sync after runtime behavior is stable:
Task: "Update canon-inspect-clarity guidance in defaults/embedded-skills/canon-inspect-clarity/skill-source.md and .agents/skills/canon-inspect-clarity/SKILL.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm clarity expansion works for representative file-backed modes and update `validation-report.md`.

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
   - Developer C: User Story 3 after runtime contract checkpoints are stable.
3. Each story closes only after its evidence is recorded.

---

## Notes

- [P] tasks = different files, no dependencies.
- [Story] labels map tasks to user stories for traceability.
- `T001` is intentionally the version bump as requested.
- `T031` is intentionally the impacted docs plus changelog closeout task as requested.
- `T035` is intentionally the coverage task for modified or created Rust files.
- `T036` is intentionally the explicit `cargo fmt` plus `cargo clippy` task.
- Do not reintroduce brittle repository-doc content tests for `README.md`, `ROADMAP.md`, `docs/guides/`, or `CHANGELOG.md`.
- Each user story should be independently completable and validated.
- Keep the decision log and validation report current as work progresses.