# Tasks: High-Risk Operational Programs

**Input**: Design documents from `/specs/014-high-risk-ops/`  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/incident-artifact-contract.md`, `contracts/migration-artifact-contract.md`, `quickstart.md`

**Validation**: Layered validation is mandatory. Contract, runtime, publish/readability, and independent review evidence must be recorded in `specs/014-high-risk-ops/validation-report.md`.

**Organization**: Tasks are grouped by user story so `incident`, `migration`, and packet-readability work can be implemented, validated, and audited as independent increments.

## Format: `[ID] [P?] [Story] Description`

## Constitution Alignment

- Governance, risk, scope, invariants, and validation ownership stay current before implementation begins.
- No operational-mode code lands before the artifact contracts and decision log entries that authorize it.
- Every user story includes failing validation work and evidence capture.
- Systemic-impact work closes only after an independent operational packet review separate from generation.

## Phase 0: Governance & Artifacts

- [X] T001 Reconfirm execution mode, systemic-impact risk, scope boundaries, invariants, and validation ownership in `specs/014-high-risk-ops/spec.md` and `specs/014-high-risk-ops/plan.md`
- [X] T002 Refresh shared operational sequencing, artifact-family, and readiness-posture decisions in `specs/014-high-risk-ops/decision-log.md`
- [X] T003 Expand the validation matrix, evidence paths, and independent review checkpoints in `specs/014-high-risk-ops/validation-report.md`

---

## Phase 1: Setup (Shared Infrastructure)

- [X] T004 Create top-level test entrypoints for the new operational suites in `tests/incident_contract.rs`, `tests/incident_run.rs`, `tests/migration_contract.rs`, and `tests/migration_run.rs`
- [X] T005 [P] Create publish/readability integration suite entrypoints in `tests/integration/incident_publish.rs` and `tests/integration/migration_publish.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Complete the shared runtime plumbing that both operational modes depend on before story-level implementation begins.

**⚠️ CRITICAL**: No user story work starts before these tasks are complete.

- [X] T006 [P] Add failing full-depth mode-discovery and profile assertions for `incident` and `migration` in `tests/contract/inspect_modes.rs` and `tests/integration/mode_profiles.rs`
- [X] T007 [P] Add failing shared runtime and summary coverage for unsupported operational-mode paths in `tests/direct_runtime_coverage.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, and `crates/canon-engine/src/orchestrator/service/tests.rs`
- [X] T008 Promote shared mode metadata and canonical input binding in `crates/canon-engine/src/domain/mode.rs`, `defaults/methods/incident.toml`, `defaults/methods/migration.toml`, and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T009 Implement shared incident/migration gate contexts and refresh-state dispatch in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/service.rs`
- [X] T010 Establish shared operational mode-result and action-chip scaffolding in `crates/canon-engine/src/orchestrator/service/summarizers.rs` and `crates/canon-cli/src/output.rs`

**Checkpoint**: `incident` and `migration` are recognized as full-depth runtime paths with shared dispatch, gate, and summary plumbing in place.

---

## Phase 3: User Story 1 - Incident lead gets a governed containment packet (Priority: P1) 🎯 MVP

**Goal**: Deliver a runnable `incident` mode that emits a governed containment packet with explicit blast radius, sequencing, fallback, and gate posture.

**Independent Test**: Run `canon run --mode incident --risk systemic-impact --zone red --owner incident-commander --system-context existing --input incident.md` and verify the packet contains all six incident artifacts with explicit containment and evidence posture.

### Validation for User Story 1

- [X] T011 [P] [US1] Add failing incident artifact-contract coverage in `tests/contract/incident_contract.rs` and `tests/incident_contract.rs`
- [X] T012 [P] [US1] Add failing incident runtime assertions in `tests/integration/incident_run.rs`, `tests/incident_run.rs`, and `tests/direct_runtime_coverage.rs`
- [X] T013 [US1] Record incident packet and containment-readiness decisions in `specs/014-high-risk-ops/decision-log.md`

### Implementation for User Story 1

- [X] T014 [US1] Implement `incident` runtime orchestration in `crates/canon-engine/src/orchestrator/service/mode_incident.rs` and wire it through `crates/canon-engine/src/orchestrator/service.rs`
- [X] T015 [US1] Implement the `incident` artifact contract and markdown rendering in `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [X] T016 [US1] Implement incident-specific containment/release gate semantics and mode-result summaries in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [X] T017 [US1] Capture incident validation evidence in `specs/014-high-risk-ops/validation-report.md`

**Checkpoint**: `incident` is runnable end to end and forms the MVP for high-risk operational mode completion.

---

## Phase 4: User Story 2 - Migration owner gets a compatibility-aware rollout packet (Priority: P2)

**Goal**: Deliver a runnable `migration` mode that emits a compatibility-aware packet with explicit sequencing, fallback, and migration-safety posture.

**Independent Test**: Run `canon run --mode migration --risk systemic-impact --zone yellow --owner migration-lead --system-context existing --input migration.md` and verify the packet contains all six migration artifacts with explicit compatibility and fallback posture.

### Validation for User Story 2

- [X] T018 [P] [US2] Add failing migration artifact-contract coverage in `tests/contract/migration_contract.rs` and `tests/migration_contract.rs`
- [X] T019 [P] [US2] Add failing migration runtime assertions in `tests/integration/migration_run.rs`, `tests/migration_run.rs`, and `tests/direct_runtime_coverage.rs`
- [X] T020 [US2] Record migration compatibility, sequencing, and fallback decisions in `specs/014-high-risk-ops/decision-log.md`

### Implementation for User Story 2

- [X] T021 [US2] Implement `migration` runtime orchestration in `crates/canon-engine/src/orchestrator/service/mode_migration.rs` and wire it through `crates/canon-engine/src/orchestrator/service.rs`
- [X] T022 [US2] Implement the `migration` artifact contract and markdown rendering in `crates/canon-engine/src/artifacts/contract.rs` and `crates/canon-engine/src/artifacts/markdown.rs`
- [X] T023 [US2] Implement migration-specific safety gates and mode-result summaries in `crates/canon-engine/src/orchestrator/gatekeeper.rs` and `crates/canon-engine/src/orchestrator/service/summarizers.rs`
- [X] T024 [US2] Capture migration validation evidence in `specs/014-high-risk-ops/validation-report.md`

**Checkpoint**: `migration` is runnable end to end with explicit compatibility and fallback governance.

---

## Phase 5: User Story 3 - Approver reviews high-risk readiness from the packet alone (Priority: P3)

**Goal**: Make incident and migration packets readable, publishable, and honestly discoverable outside the runtime, including status, inspect, docs, and skill surfaces.

**Independent Test**: Publish completed and blocked `incident` or `migration` runs and verify a reviewer can determine readiness, gaps, and fallback posture from the packet alone without consulting internal manifests.

### Validation for User Story 3

- [X] T025 [P] [US3] Add failing publish/readability coverage for operational packets in `tests/integration/incident_publish.rs`, `tests/integration/migration_publish.rs`, and `tests/contract/runtime_filesystem.rs`
- [X] T026 [P] [US3] Add failing inspect/status and evidence-surface coverage in `tests/contract/runtime_evidence_contract.rs`, `tests/policy_and_traces.rs`, and `crates/canon-engine/src/orchestrator/service/tests.rs`
- [X] T027 [US3] Record publish/readability and runnable-skill decisions in `specs/014-high-risk-ops/decision-log.md`

### Implementation for User Story 3

- [X] T028 [US3] Complete incident and migration publish/inspect surfaces in `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, and `crates/canon-engine/src/orchestrator/service/tests.rs`
- [X] T029 [US3] Replace modeled-only incident guidance with runnable authored-body guidance in `defaults/embedded-skills/canon-incident/skill-source.md` and `.agents/skills/canon-incident/SKILL.md`
- [X] T030 [US3] Replace modeled-only migration guidance with runnable authored-body guidance in `defaults/embedded-skills/canon-migration/skill-source.md`, `.agents/skills/canon-migration/SKILL.md`, and `defaults/embedded-skills/canon-shared/references/skill-index.md`
- [X] T031 [US3] Update operational-mode documentation, roadmap state, and capture story evidence in `README.md`, `MODE_GUIDE.md`, `NEXT_FEATURES.md`, and `specs/014-high-risk-ops/validation-report.md`

**Checkpoint**: High-risk operational packets are publishable, inspectable, and documented honestly outside the runtime.

---

## Final Phase: Verification & Compliance

- [X] T032 [P] Increase non-regression operational coverage in `tests/contract/inspect_modes.rs`, `tests/integration/mode_profiles.rs`, `tests/direct_runtime_coverage.rs`, `tests/policy_and_traces.rs`, and `tests/artifact_confinement.rs`
- [X] T033 [P] Run structural validation with `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `/bin/bash scripts/validate-canon-skills.sh`; record results in `specs/014-high-risk-ops/validation-report.md`
- [X] T034 [P] Run logical validation for `tests/incident_contract.rs`, `tests/incident_run.rs`, `tests/migration_contract.rs`, `tests/migration_run.rs`, `tests/direct_runtime_coverage.rs`, `tests/integration/mode_profiles.rs`, `tests/integration/incident_publish.rs`, and `tests/integration/migration_publish.rs`; record results in `specs/014-high-risk-ops/validation-report.md`
- [X] T035 Perform independent operational packet review and close `specs/014-high-risk-ops/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0: Governance & Artifacts**: no dependencies; must complete first.
- **Phase 1: Setup**: depends on Phase 0.
- **Phase 2: Foundational**: depends on Phase 1 and blocks all user stories.
- **Phase 3: US1**: depends on Phase 2.
- **Phase 4: US2**: depends on US1 because migration reuses the shared operational pipeline proven by the incident slice.
- **Phase 5: US3**: depends on US1 and US2 because publish/readability and skill guidance must reflect both runnable modes.
- **Final Phase**: depends on all desired user stories being complete.

### User Story Completion Order

- **US1 (P1)**: MVP. Deliver the `incident` packet.
- **US2 (P2)**: Deliver the `migration` packet on the same completed runtime path.
- **US3 (P3)**: Make both packets publishable, inspectable, and honestly documented.

### Within Each User Story

- Failing validation tasks land before implementation tasks.
- Decision-log updates happen before the affected behavior is declared complete.
- Runtime orchestration lands before packet evidence is captured.
- Validation evidence is required before the story checkpoint closes.

---

## Parallel Examples

### User Story 1

```bash
# Incident validation can start in parallel:
T011 Add failing incident artifact-contract coverage
T012 Add failing incident runtime assertions
```

### User Story 2

```bash
# Migration validation can start in parallel:
T018 Add failing migration artifact-contract coverage
T019 Add failing migration runtime assertions
```

### User Story 3

```bash
# Publish and inspect coverage can start in parallel:
T025 Add failing publish/readability coverage
T026 Add failing inspect/status and evidence-surface coverage
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 0, Phase 1, and Phase 2.
2. Deliver US1 and validate the `incident` packet end to end.
3. Stop and confirm the containment packet is honest and reviewable before moving to migration.

### Incremental Delivery

1. Complete shared governance, setup, and foundational runtime plumbing.
2. Add `incident` as the first runnable operational mode.
3. Add `migration` on the same completed runtime path.
4. Finish by making both packets publishable, inspectable, and honestly documented.
5. Close with structural, logical, and independent validation.

## Notes

- Total tasks: 35
- User-story task counts: US1 = 7, US2 = 7, US3 = 7
- Parallel opportunities identified: new failing validation suites in each story, plus publish/readability and inspect/status coverage in US3
- Independent test criteria: US1 = runnable incident packet, US2 = runnable migration packet, US3 = publish/readability without hidden state
- Validation evidence paths: `specs/014-high-risk-ops/validation-report.md` and the emitted `.canon/` run evidence bundles
- Independent review checkpoint: T035 reviews real incident and migration packets separately from generation
- Suggested MVP scope: through Phase 3 (US1) only