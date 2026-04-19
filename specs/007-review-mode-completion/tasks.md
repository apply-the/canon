# Tasks: Review Mode Completion

**Input**: Design documents from `/specs/007-review-mode-completion/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/review-verification-run.md

**Validation**: Layered validation is mandatory. Engine and CLI tests must lock runtime behavior; repo gates and a separate validation report close out the feature.

**Organization**: Tasks are grouped by user story so `review`, `verification`, and cross-surface compatibility can be delivered incrementally.

## Format: `[ID] [P?] [Story] Description`

## Constitution Alignment

- Governance artifacts exist before implementation.
- Systemic-impact work keeps decision logging and validation evidence current.
- Each user story includes executable validation and evidence capture.

## Phase 0: Governance & Artifacts

- [x] T001 Record execution mode, risk classification, scope boundaries, and invariants in `specs/007-review-mode-completion/spec.md` and `specs/007-review-mode-completion/plan.md`
- [x] T002 Create `specs/007-review-mode-completion/decision-log.md` and `specs/007-review-mode-completion/validation-report.md`
- [x] T003 Create `specs/007-review-mode-completion/contracts/review-verification-run.md` and `specs/007-review-mode-completion/quickstart.md`
- [x] T004 Record validation ownership and approval gates in `specs/007-review-mode-completion/plan.md`

---

## Phase 1: Setup (Shared Infrastructure)

- [x] T005 Update mode metadata in `crates/canon-engine/src/modes/review.rs` and `crates/canon-engine/src/modes/verification.rs`
- [x] T006 [P] Expand artifact contracts in `crates/canon-engine/src/artifacts/contract.rs`
- [x] T007 [P] Add review and verification renderers in `crates/canon-engine/src/artifacts/markdown.rs`
- [x] T008 [P] Add review and verification gate evaluation in `crates/canon-engine/src/orchestrator/gatekeeper.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

- [x] T009 Add shared runtime helpers and dispatch in `crates/canon-engine/src/orchestrator/service.rs`
- [x] T010 [P] Extend adapter prompts in `crates/canon-adapters/src/copilot_cli.rs`
- [x] T011 [P] Extend verification record handling in `crates/canon-engine/src/orchestrator/verification_runner.rs`
- [x] T012 [P] Extend mode-result summaries and refresh-state handling in `crates/canon-engine/src/orchestrator/service.rs`

---

## Phase 3: User Story 1 - Review Non-PR Package (Priority: P1) 🎯 MVP

**Goal**: Deliver a runnable `review` mode with durable packet, disposition semantics, and status/inspect compatibility.

**Independent Test**: Run `canon run --mode review ... --input <authored-review-input>` and verify a real run, artifact bundle, and coherent mode result.

### Validation for User Story 1

- [x] T013 [P] [US1] Add failing review-mode tests in `tests/contract/cli_contract.rs`, `tests/integration/review_run.rs`, and engine unit coverage where needed
- [x] T014 [US1] Record review-mode decisions in `specs/007-review-mode-completion/decision-log.md`

### Implementation for User Story 1

- [x] T015 [US1] Implement `review` run flow in `crates/canon-engine/src/orchestrator/service.rs`
- [x] T016 [US1] Implement review artifact rendering and disposition semantics in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/orchestrator/gatekeeper.rs`
- [x] T017 [US1] Update runnable skill behavior in `.agents/skills/canon-review/SKILL.md` and `defaults/embedded-skills/canon-review/skill-source.md`
- [x] T018 [US1] Capture review-mode validation evidence in `specs/007-review-mode-completion/validation-report.md`

---

## Phase 4: User Story 2 - Challenge Claims And Invariants (Priority: P2)

**Goal**: Deliver a runnable `verification` mode with adversarial outputs and explicit unresolved findings.

**Independent Test**: Run `canon run --mode verification ... --input <authored-verification-input>` and verify a real run, verification packet, and blocked readiness when findings stay unresolved.

### Validation for User Story 2

- [x] T019 [P] [US2] Add failing verification-mode tests in `tests/contract/cli_contract.rs`, `tests/integration/verification_run.rs`, and engine unit coverage where needed
- [x] T020 [US2] Record verification-mode decisions in `specs/007-review-mode-completion/decision-log.md`

### Implementation for User Story 2

- [x] T021 [US2] Implement `verification` run flow in `crates/canon-engine/src/orchestrator/service.rs`
- [x] T022 [US2] Implement verification artifact rendering and readiness blocking in `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/orchestrator/gatekeeper.rs`
- [x] T023 [US2] Update runnable skill behavior in `.agents/skills/canon-verification/SKILL.md` and `defaults/embedded-skills/canon-verification/skill-source.md`
- [x] T024 [US2] Capture verification-mode validation evidence in `specs/007-review-mode-completion/validation-report.md`

---

## Phase 5: User Story 3 - Inspect And Continue Through Existing Surfaces (Priority: P3)

**Goal**: Keep `status`, `inspect`, `approve`, and `resume` coherent for the new modes and align user-facing docs with shipped truth.

**Independent Test**: After `review` and `verification` runs complete or gate, the existing inspection and follow-up flows work without mode-specific hacks.

### Validation for User Story 3

- [x] T025 [P] [US3] Add cross-surface tests for `status`, `inspect artifacts`, `inspect evidence`, and approval targets in `tests/contract/*` and `tests/integration/*`
- [x] T026 [US3] Record cross-surface decisions in `specs/007-review-mode-completion/decision-log.md`

### Implementation for User Story 3

- [x] T027 [US3] Update `README.md`, `MODE_GUIDE.md`, and `NEXT_FEATURES.md` for 0.7.0 runnable support
- [x] T028 [US3] Update shared skill references, validators, and support-state metadata under `.agents/skills/**`, `defaults/embedded-skills/**`, and `scripts/validate-canon-skills.*`
- [x] T029 [US3] Capture integration and documentation validation evidence in `specs/007-review-mode-completion/validation-report.md`

---

## Final Phase: Verification & Compliance

- [x] T030 [P] Run structural validation with `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and the skill validators
- [x] T031 [P] Run logical validation with targeted tests and `cargo nextest run`
- [x] T032 Perform independent review or adversarial validation and record it in `specs/007-review-mode-completion/validation-report.md`
- [x] T033 Confirm invariants still hold and close the validation report