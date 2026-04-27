# Tasks: Mode Authoring Specialization Completion

**Input**: Design documents from `/specs/020-authoring-specialization-completion/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`, `decision-log.md`, `validation-report.md`

**Validation**: Layered validation is mandatory. Add executable test tasks wherever artifact contracts, renderer behavior, docs synchronization, or release/version surfaces can regress.

**Organization**: Tasks are grouped by user story so each story can be implemented, validated, and reviewed independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no incomplete dependencies)
- **[Story]**: Which user story this belongs to (`US1`, `US2`, `US3`)
- All paths are repo-root-relative.

## Constitution Alignment

- Every feature starts with mode, risk, scope, invariant, and artifact tasks.
- No implementation task appears before the artifacts that authorize it.
- Every user story includes validation and evidence-capture work.
- Independent review remains separate from generation work.

## Phase 0: Governance & Artifacts

**Purpose**: Lock the controls that authorize implementation.

- [ ] T001 Confirm execution mode `change`, risk `bounded-impact`, scope boundaries, and invariants in `specs/020-authoring-specialization-completion/spec.md` and `specs/020-authoring-specialization-completion/plan.md`
- [ ] T002 Confirm the targeted authored-body contract and artifact-to-heading mapping in `specs/020-authoring-specialization-completion/contracts/mode-authored-body-contracts.md`
- [ ] T003 Confirm decision logging and validation scaffolding in `specs/020-authoring-specialization-completion/decision-log.md` and `specs/020-authoring-specialization-completion/validation-report.md`
- [ ] T004 Record the hard non-regression boundaries for critique posture, recommendation-only posture, missing-body honesty, and `0.20.0` release/docs sync in `specs/020-authoring-specialization-completion/decision-log.md`

---

## Phase 1: Setup (Shared Baseline)

**Purpose**: Capture the current targeted surfaces before feature work begins.

- [ ] T005 Capture the focused baseline with `cargo test --test review_run --test verification_run --test incident_run --test migration_run --test review_contract --test verification_contract --test incident_contract --test migration_contract` and record results in `specs/020-authoring-specialization-completion/validation-report.md`
- [ ] T006 [P] Confirm the current authored guidance surfaces in `defaults/embedded-skills/canon-review/skill-source.md`, `defaults/embedded-skills/canon-verification/skill-source.md`, `defaults/embedded-skills/canon-incident/skill-source.md`, `defaults/embedded-skills/canon-migration/skill-source.md`, `.agents/skills/canon-review/SKILL.md`, `.agents/skills/canon-verification/SKILL.md`, `.agents/skills/canon-incident/SKILL.md`, and `.agents/skills/canon-migration/SKILL.md`
- [ ] T007 [P] Confirm the current runtime and release surfaces in `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/service/mode_review.rs`, `crates/canon-engine/src/orchestrator/service/mode_incident.rs`, `crates/canon-engine/src/orchestrator/service/mode_migration.rs`, `Cargo.toml`, `CHANGELOG.md`, `ROADMAP.md`, `docs/guides/modes.md`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, and `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared contract and runtime scaffolding that both story streams depend on.

**⚠️ CRITICAL**: No user-story implementation begins until this phase is complete.

- [ ] T008 Align `crates/canon-engine/src/artifacts/contract.rs` with the authored-body contract for `review`, `verification`, `incident`, and `migration` captured in `specs/020-authoring-specialization-completion/contracts/mode-authored-body-contracts.md`
- [ ] T009 [P] Introduce the shared authored-section mapping, missing-body preservation scaffolding, and any review/verification compatibility aliases in `crates/canon-engine/src/artifacts/markdown.rs`
- [ ] T010 [P] Restore or preserve authored-source handoff for the targeted modes in `crates/canon-engine/src/orchestrator/service/mode_review.rs`, `crates/canon-engine/src/orchestrator/service/mode_incident.rs`, and `crates/canon-engine/src/orchestrator/service/mode_migration.rs`

**Checkpoint**: Shared contract, renderer scaffolding, and authored-source handoff are ready for story work.

---

## Phase 3: User Story 1 - Reviewers And Validators Author Real Bodies (Priority: P1) 🎯 MVP

**Goal**: Make `review` and `verification` discoverable and runtime-honest under the canonical authored-body contract.

**Independent Test**: Read the updated skill/template/example for `review` or `verification`, then run one complete and one incomplete packet and confirm authored sections are preserved verbatim while missing sections emit `## Missing Authored Body` without changing critique posture.

### Validation for User Story 1 (MANDATORY)

- [ ] T011 [P] [US1] Add failing docs-sync coverage for `review` and `verification` authored guidance in `tests/review_authoring_docs.rs` and `tests/verification_authoring_docs.rs`
- [ ] T012 [P] [US1] Extend the existing contract coverage in `tests/contract/review_contract.rs` and `tests/contract/verification_contract.rs`, and add failing renderer coverage in new files `tests/review_authoring_renderer.rs` and `tests/verification_authoring_renderer.rs`
- [ ] T013 [P] [US1] Add failing run coverage for complete and incomplete authored packets in `tests/integration/review_run.rs` and `tests/integration/verification_run.rs`

### Implementation for User Story 1

- [ ] T014 [US1] Update authored guidance in `defaults/embedded-skills/canon-review/skill-source.md`, `defaults/embedded-skills/canon-verification/skill-source.md`, `.agents/skills/canon-review/SKILL.md`, and `.agents/skills/canon-verification/SKILL.md`
- [ ] T015 [US1] Convert the starter inputs to the canonical H2 contract in `docs/templates/canon-input/review.md` and `docs/templates/canon-input/verification.md`
- [ ] T016 [US1] Rewrite the worked examples to exercise the full authored packet contract in `docs/examples/canon-input/review-db-migration.md` and `docs/examples/canon-input/verification-e2e-flakiness.md`
- [ ] T017 [US1] Extend `crates/canon-engine/src/artifacts/markdown.rs` and `crates/canon-engine/src/orchestrator/service/mode_review.rs` so `review` and `verification` preserve canonical authored bodies and emit honest missing-body markers without changing result semantics
- [ ] T018 [US1] Record review/verification contract, docs-sync, and runtime evidence in `specs/020-authoring-specialization-completion/decision-log.md` and `specs/020-authoring-specialization-completion/validation-report.md`

**Checkpoint**: A Canon user can author compliant `review` and `verification` packets without reading the Rust implementation, and the runtime preserves them honestly.

---

## Phase 4: User Story 2 - Operational Authors Get Honest Incident And Migration Packets (Priority: P2)

**Goal**: Make `incident` and `migration` preserve authored operational packet bodies verbatim and surface explicit gaps without changing recommendation-only posture.

**Independent Test**: Run complete and incomplete authored packets for `incident` and `migration`, then confirm complete packets preserve authored bodies and incomplete packets emit `## Missing Authored Body` while recommendation-only posture and publish behavior remain unchanged.

### Validation for User Story 2 (MANDATORY)

- [ ] T019 [P] [US2] Add failing docs-sync coverage for `incident` and `migration` authored guidance in `tests/incident_authoring_docs.rs` and `tests/migration_authoring_docs.rs`
- [ ] T020 [P] [US2] Extend the existing contract coverage in `tests/contract/incident_contract.rs` and `tests/contract/migration_contract.rs`, and add failing renderer coverage in new files `tests/incident_authoring_renderer.rs` and `tests/migration_authoring_renderer.rs`
- [ ] T021 [P] [US2] Add failing run coverage for complete and incomplete authored packets in `tests/integration/incident_run.rs` and `tests/integration/migration_run.rs`

### Implementation for User Story 2

- [ ] T022 [US2] Update authored guidance in `defaults/embedded-skills/canon-incident/skill-source.md`, `defaults/embedded-skills/canon-migration/skill-source.md`, `.agents/skills/canon-incident/SKILL.md`, and `.agents/skills/canon-migration/SKILL.md`
- [ ] T023 [US2] Convert the starter inputs to the canonical H2 contract in `docs/templates/canon-input/incident/brief.md` and `docs/templates/canon-input/migration/brief.md`
- [ ] T024 [US2] Rewrite the worked examples to exercise the full authored packet contract in `docs/examples/canon-input/incident/brief.md` and `docs/examples/canon-input/migration/brief.md`
- [ ] T025 [US2] Extend `crates/canon-engine/src/artifacts/markdown.rs`, `crates/canon-engine/src/orchestrator/service/mode_incident.rs`, and `crates/canon-engine/src/orchestrator/service/mode_migration.rs` so `incident` and `migration` preserve canonical authored bodies and emit honest missing-body markers without changing recommendation-only posture or publish roots
- [ ] T026 [US2] Record incident/migration contract, docs-sync, and runtime evidence in `specs/020-authoring-specialization-completion/decision-log.md` and `specs/020-authoring-specialization-completion/validation-report.md`

**Checkpoint**: Operators can author compliant `incident` and `migration` packets, and the runtime preserves them honestly while keeping existing operational posture.

---

## Phase 5: User Story 3 - Maintainers Can Ship 0.20.0 With Clear Documentation (Priority: P3)

**Goal**: Synchronize roadmap, guide, changelog, compatibility references, manifests, and closeout validation so the specialization rollout is visibly complete and released as `0.20.0`.

**Independent Test**: Read the updated roadmap, guide, changelog, compatibility references, and manifests, then run focused docs/version validation proving the repo describes rollout completion and reports `0.20.0` consistently.

### Validation for User Story 3 (MANDATORY)

- [ ] T027 [P] [US3] Add failing docs/version sync coverage in `tests/mode_authoring_completion_docs.rs`
- [ ] T028 [P] [US3] Add failing non-regression coverage for critique posture, recommendation-only posture, and release/version surfaces in `tests/policy_and_traces.rs` and `tests/direct_runtime_coverage.rs`

### Implementation for User Story 3

- [ ] T029 [US3] Update rollout guidance in `docs/guides/modes.md`, `ROADMAP.md`, and `CHANGELOG.md`
- [ ] T030 [US3] Bump release/version surfaces in `Cargo.toml`, regenerate `Cargo.lock` through the normal Cargo workflow, and update `.agents/skills/canon-shared/references/runtime-compatibility.toml` plus `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- [ ] T031 [US3] Update shared docs/version validation fixtures in `tests/mode_authoring_completion_docs.rs`, `tests/policy_and_traces.rs`, and `tests/direct_runtime_coverage.rs`
- [ ] T032 [US3] Record rollout, release, and non-regression evidence in `specs/020-authoring-specialization-completion/decision-log.md` and `specs/020-authoring-specialization-completion/validation-report.md`

**Checkpoint**: Maintainers have synchronized guidance, versioning, and non-regression evidence for the completed rollout.

---

## Final Phase: Verification & Compliance

**Purpose**: Complete cross-cutting validation, independent review, and closeout.

- [ ] T033 [P] Run structural validation: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`; record results in `specs/020-authoring-specialization-completion/validation-report.md`
- [ ] T034 [P] Run logical validation: the focused `review`, `verification`, `incident`, and `migration` contract, renderer, run, docs-sync, and non-regression suites; record results in `specs/020-authoring-specialization-completion/validation-report.md`
- [ ] T035 Perform independent review of `specs/020-authoring-specialization-completion/spec.md`, `specs/020-authoring-specialization-completion/plan.md`, `specs/020-authoring-specialization-completion/tasks.md`, and `specs/020-authoring-specialization-completion/quickstart.md`, then record findings in `specs/020-authoring-specialization-completion/validation-report.md`
- [ ] T036 Confirm invariants, unchanged governance posture, consistent `0.20.0` release/docs surfaces, and close `specs/020-authoring-specialization-completion/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 0 must complete first.
- Phase 1 depends on Phase 0.
- Phase 2 depends on Phase 1 and blocks all story implementation.
- Phase 3 depends on Phase 2.
- Phase 4 depends on Phase 2 and should land before Phase 5 if the same contributor owns `markdown.rs` and the operational orchestrator files.
- Phase 5 depends on Phases 3 and 4 because it closes shared docs/version/non-regression surfaces.
- Final Phase depends on all desired user stories being complete.

### User Story Dependencies

- **US1** can ship independently once `review` and `verification` skills, templates, examples, renderer behavior, and run tests agree on the same authored H2 contract.
- **US2** can ship independently once `incident` and `migration` skills, templates, examples, renderer behavior, and run tests agree on the same authored H2 contract.
- **US3** depends on the delivered runtime/docs contract from US1 and US2 because it closes the release/docs and version surfaces for rollout completion.

### Within Each User Story

- Validation tasks happen before the corresponding implementation tasks.
- Contract and guidance changes land before a story is declared complete.
- Evidence capture happens before a story is marked done.

## Parallel Execution Examples

- T006 and T007 can run in parallel.
- T009 and T010 can run in parallel after T008.
- T011, T012, and T013 can run in parallel.
- T019, T020, and T021 can run in parallel.
- T027 and T028 can run in parallel.
- T033 and T034 can run in parallel.

### Parallel Example: User Story 1

```bash
Task: "Add failing docs-sync coverage in tests/review_authoring_docs.rs and tests/verification_authoring_docs.rs"
Task: "Add failing renderer coverage in tests/review_authoring_renderer.rs and tests/verification_authoring_renderer.rs"
Task: "Add failing run coverage in tests/integration/review_run.rs and tests/integration/verification_run.rs"
```

### Parallel Example: User Story 2

```bash
Task: "Add failing docs-sync coverage in tests/incident_authoring_docs.rs and tests/migration_authoring_docs.rs"
Task: "Add failing renderer coverage in tests/incident_authoring_renderer.rs and tests/migration_authoring_renderer.rs"
Task: "Add failing run coverage in tests/integration/incident_run.rs and tests/integration/migration_run.rs"
```

### Parallel Example: User Story 3

```bash
Task: "Add failing docs/version sync coverage in tests/mode_authoring_completion_docs.rs"
Task: "Add failing non-regression coverage in tests/policy_and_traces.rs and tests/direct_runtime_coverage.rs"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Governance, Setup, and Foundational phases.
2. Complete User Story 1.
3. Stop and validate that authors can discover and run the `review` and `verification` contracts without reading source code.

### Incremental Delivery

1. Deliver `review` and `verification` authored-body specialization (US1).
2. Deliver `incident` and `migration` authored-body specialization (US2).
3. Deliver release/docs/version closeout for `0.20.0` (US3).
4. Finish with structural validation, logical validation, independent review, and closeout.

### Parallel Team Strategy

1. One stream can prepare failing tests while another updates the next story's docs surfaces.
2. Shared runtime files in `crates/canon-engine/src/artifacts/markdown.rs` and the orchestrator service files remain single-owner at a time.
3. Final validation and independent review stay separate from the implementation stream.