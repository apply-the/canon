# Tasks: Output Quality Gates

**Input**: Design documents from `/specs/034-output-quality-gates/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. Add executable test tasks whenever behavior, interfaces, or regressions must be checked. Independent review and evidence-capture tasks are always required.

**Organization**: Tasks are grouped by user story to enable independent implementation, validation, and auditability for each increment.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Constitution Alignment

- Every feature MUST start with mode, risk, scope, and invariant artifact tasks.
- No implementation task may appear before the artifacts that authorize it.
- Every user story MUST include validation tasks and evidence capture.
- Systemic-impact work MUST include an independent review task separate from generation.

## Phase 0: Governance & Artifacts

**Purpose**: Establish the controls, release boundary, and contracts that permit implementation to start

- [x] T001 Set Canon version to `0.34.0` in `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- [x] T002 Update implementation-scope decisions and unsliced delivery notes in `specs/034-output-quality-gates/decision-log.md`
- [x] T003 Update planned structural, logical, and independent validation checkpoints in `specs/034-output-quality-gates/validation-report.md`
- [x] T004 Confirm shared output-quality posture and release-alignment expectations in `specs/034-output-quality-gates/contracts/output-quality-contract.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared scaffolding and release-guard setup

- [x] T005 Update agent context from `specs/034-output-quality-gates/plan.md` into `AGENTS.md`
- [x] T006 Create or extend release-alignment bootstrap coverage for `0.34.0` in `tests/integration/skills_bootstrap.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared quality assessment, inspect transport, and posture plumbing that all user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T007 [P] Add failing shared output-quality assessment coverage in `crates/canon-engine/src/orchestrator/service/clarity.rs` and `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [x] T008 [P] Add failing inspect contract coverage for output-quality posture in `tests/contract/inspect_clarity.rs`
- [x] T009 [P] Add failing summary and fallback-honesty coverage in `crates/canon-engine/src/artifacts/markdown.rs` and `tests/contract/review_contract.rs`
- [x] T010 Implement shared output-quality assessment and downgrade helpers in `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/clarity.rs`, and `crates/canon-engine/src/orchestrator/service/context_parse.rs`
- [x] T011 Implement inspect-surface transport for output-quality posture in `crates/canon-engine/src/orchestrator/service/inspect.rs` and `crates/canon-cli/src/output.rs`
- [x] T012 Implement shared summary and renderer posture mapping in `crates/canon-engine/src/orchestrator/service/summarizers.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T013 Capture foundational invariants and touched-Rust-file coverage expectations in `specs/034-output-quality-gates/validation-report.md`

**Checkpoint**: Shared quality posture exists in the engine and can be consumed by story-level behavior

---

## Phase 3: User Story 1 - Inspect Output Quality Before Trusting A Packet (Priority: P1) 🎯 MVP

**Goal**: Expose explicit output-quality posture with evidence and downgrade reasons in inspect-facing surfaces

**Independent Test**: Representative weak, strong, and materially closed authored inputs return explicit posture, evidence signals, and downgrade reasons without forcing a full packet read.

### Validation for User Story 1 (MANDATORY)

- [x] T014 [P] [US1] Add failing inspect behavior coverage for weak, strong, and materially closed authored packets in `tests/contract/inspect_clarity.rs` and `crates/canon-engine/src/orchestrator/service/clarity.rs`
- [x] T015 [US1] Record inspect-quality decisions under `## User Story 1 Decisions` in `specs/034-output-quality-gates/decision-log.md`

### Implementation for User Story 1

- [x] T016 [US1] Implement inspect-visible output-quality posture in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/service/inspect.rs`
- [x] T017 [P] [US1] Update shared embedded inspect guidance in `defaults/embedded-skills/canon-inspect-clarity/skill-source.md` and `defaults/embedded-skills/canon-shared/references/output-shapes.md`
- [x] T018 [P] [US1] Mirror inspect guidance in `.agents/skills/canon-inspect-clarity/SKILL.md` and `.agents/skills/canon-shared/references/output-shapes.md`
- [x] T019 [US1] Capture inspect-quality validation evidence in `specs/034-output-quality-gates/validation-report.md`

**Checkpoint**: Maintainers can see packet quality posture before trusting the packet as downstream input

---

## Phase 4: User Story 2 - Read Honest Quality Posture In Summaries And Artifacts (Priority: P2)

**Goal**: Make runtime summaries and fallback-heavy artifacts reflect the same shared quality posture without overstating shallow content

**Independent Test**: Representative targeted packets downgrade clearly when support is weak and only surface stronger posture when evidence is explicit.

### Validation for User Story 2 (MANDATORY)

- [x] T020 [P] [US2] Add failing summary-posture and fallback-honesty coverage in `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, and `tests/contract/review_contract.rs`
- [x] T021 [US2] Record summary and artifact posture decisions under `## User Story 2 Decisions` in `specs/034-output-quality-gates/decision-log.md`

### Implementation for User Story 2

- [x] T022 [US2] Implement summary posture changes in `crates/canon-engine/src/orchestrator/service/summarizers.rs` and `crates/canon-cli/src/output.rs`
- [x] T023 [US2] Tighten fallback-heavy artifact posture in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T024 [US2] Capture summary and artifact validation evidence in `specs/034-output-quality-gates/validation-report.md`

**Checkpoint**: Readers can distinguish structurally complete from materially useful and publishable output directly in summaries and targeted artifacts

---

## Phase 5: User Story 3 - Ship 0.34.0 With Synchronized Authoring Surfaces And Clean Roadmap (Priority: P3)

**Goal**: Align release-facing surfaces, docs, skills, roadmap, and validation around the delivered `0.34.0` output-quality contract

**Independent Test**: A maintainer can inspect the release-facing files and validation evidence and see one coherent `0.34.0` story with explicit output-quality posture and a cleaned roadmap.

### Validation for User Story 3 (MANDATORY)

- [x] T025 [P] [US3] Validate skill-sync and release alignment with `tests/integration/skills_bootstrap.rs`, `/bin/bash scripts/validate-canon-skills.sh`, and `specs/034-output-quality-gates/validation-report.md`
- [x] T026 [US3] Record release and roadmap decisions under `## User Story 3 Decisions` in `specs/034-output-quality-gates/decision-log.md`

### Implementation for User Story 3

- [x] T027 [P] [US3] Review and update impacted input guidance in `docs/templates/canon-input/` and `docs/examples/canon-input/`, recording unchanged-but-still-valid surfaces in `specs/034-output-quality-gates/validation-report.md`
- [x] T028 [US3] Update impacted docs and changelog in `README.md`, `docs/guides/modes.md`, `docs/guides/publishing-to-winget.md`, `docs/guides/publishing-to-scoop.md`, and `CHANGELOG.md`
- [x] T029 [US3] Clean roadmap continuity in `ROADMAP.md`
- [x] T030 [US3] Capture docs, roadmap, and release-alignment validation evidence in `specs/034-output-quality-gates/validation-report.md`

**Checkpoint**: Runtime and authoring surfaces agree on the delivered output-quality posture at `0.34.0`

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, coverage, independent review, and closeout

- [x] T031 [P] Run `/bin/bash scripts/validate-canon-skills.sh` and record results in `specs/034-output-quality-gates/validation-report.md`
- [x] T032 [P] Run the targeted feature suite for `tests/contract/inspect_clarity.rs`, `tests/contract/review_contract.rs`, `tests/integration/skills_bootstrap.rs`, and focused `cargo test -p canon-engine` quality-posture coverage, then record results in `specs/034-output-quality-gates/validation-report.md`
- [x] T033 [P] Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and document coverage for every modified or newly created Rust file in `specs/034-output-quality-gates/validation-report.md`
- [x] T034 [P] Run `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, then record results in `specs/034-output-quality-gates/validation-report.md`
- [x] T035 [P] Run `cargo nextest run --workspace --all-features` and record results in `specs/034-output-quality-gates/validation-report.md`
- [x] T036 Perform independent review of invariants, output-quality posture, release alignment, and final diff in `specs/034-output-quality-gates/validation-report.md`
- [x] T037 Confirm invariants still hold and close the final validation state in `specs/034-output-quality-gates/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. MUST complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion. BLOCKS all user stories.
- **User Stories (Phase 3+)**: Depend on Foundational phase completion.
- **Verification & Compliance (Final Phase)**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational. Establishes the inspect-facing MVP.
- **User Story 2 (P2)**: Can start after Foundational. Reuses the shared quality posture from Phase 2 and remains independently testable through summaries and artifacts.
- **User Story 3 (P3)**: Depends on the implemented runtime contract from User Stories 1 and 2 so docs and roadmap describe delivered behavior accurately.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant changes MUST be recorded before affected code or docs land.
- Shared engine structures before inspect transport.
- Embedded skills before mirrored `.agents` copies when both change.
- Docs plus changelog before roadmap cleanup closeout.
- Evidence capture before the story is declared complete.

### Parallel Opportunities

- Phase 0 tasks after T001 can run in parallel where they touch different planning artifacts.
- T007, T008, and T009 can run in parallel before the shared engine changes in T010 through T012.
- Within US1, embedded and mirrored skill updates can run in parallel once the inspect contract is stable.
- Final validation tasks T031 through T035 can run in parallel once implementation is stable.

---

## Parallel Example: User Story 1

```bash
# Launch weak and strong inspect coverage preparation in parallel:
Task: "Add failing inspect behavior coverage for weak, strong, and materially closed authored packets in tests/contract/inspect_clarity.rs and crates/canon-engine/src/orchestrator/service/clarity.rs"

# Launch guidance sync after the inspect contract is stable:
Task: "Update shared embedded inspect guidance in defaults/embedded-skills/canon-inspect-clarity/skill-source.md and defaults/embedded-skills/canon-shared/references/output-shapes.md"
Task: "Mirror inspect guidance in .agents/skills/canon-inspect-clarity/SKILL.md and .agents/skills/canon-shared/references/output-shapes.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm inspect-facing output-quality posture works for representative weak, strong, and materially closed inputs.

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
   - Developer C: User Story 3 after runtime posture checkpoints are stable.
3. Each story closes only after its evidence is recorded.

---

## Notes

- [P] tasks = different files, no dependencies.
- [Story] labels map tasks to user stories for traceability.
- `T001` is intentionally the version bump task as requested.
- `T028` is intentionally the impacted docs plus changelog task as requested.
- `T029` is intentionally the roadmap cleanup task as requested.
- `T033` is intentionally the coverage task for modified or created Rust files.
- `T034` is intentionally the explicit `cargo fmt` plus `cargo clippy` task.
- Do not reintroduce brittle repository-doc content tests for `README.md`, `ROADMAP.md`, `docs/guides/`, or `CHANGELOG.md`.
- Each user story should be independently completable and validated.
- Keep the decision log and validation report current as work progresses.