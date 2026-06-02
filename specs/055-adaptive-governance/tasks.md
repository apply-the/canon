# Tasks: Adaptive Governance Semantics

**Input**: Design documents from `/specs/055-adaptive-governance/`  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Validation**: Layered validation is mandatory. This feature defines a systemic-impact cross-repo semantic contract, so every story includes executable checks plus captured review evidence.

**Organization**: Tasks are grouped by user story so each increment can be implemented, validated, and audited independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this belongs to (e.g. `US1`, `US2`, `US3`)
- Include exact file paths in every task description

## Phase 0: Governance & Artifacts

**Purpose**: Establish the durable review and evidence artifacts that authorize implementation.

- [x] T001 Create durable evidence artifacts in `specs/055-adaptive-governance/decision-log.md` and `specs/055-adaptive-governance/validation-report.md`
- [x] T002 Record execution mode, risk, scope boundaries, invariants, and contract-line decisions in `specs/055-adaptive-governance/spec.md`, `specs/055-adaptive-governance/plan.md`, and `specs/055-adaptive-governance/decision-log.md`
- [x] T003 [P] Capture required reviewers, approval gates, and cross-repo review checkpoints in `specs/055-adaptive-governance/validation-report.md`
- [x] T004 [P] Align the feature-local contract docs and walkthrough wording in `specs/055-adaptive-governance/contracts/adaptive-governance-v1-contract.md`, `specs/055-adaptive-governance/contracts/adaptive-governance-adapter-projection.md`, and `specs/055-adaptive-governance/quickstart.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare the repo-visible documentation and targeted test scaffolds this semantic slice will use.

- [x] T005 Create the S4 semantic-documentation anchors in `tech-docs/governance-semantics-and-authority-zones.md`, `tech-docs/integration/governance-adapter.md`, and `README.md`
- [ ] T006 [P] Prepare targeted semantic test scaffolds in `tests/governance_adapter_surface.rs`, `tests/mode_profiles.rs`, and `tests/policy_and_traces.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core typed vocabulary and compatibility scaffolding that all user stories rely on.

**⚠️ CRITICAL**: Full story sign-off depends on this phase completing. Packet-
metadata vocabulary work may proceed once the typed companion model is in
place.

- [x] T007 Create shared S4 governance-state and rollout-profile vocabulary in `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T008 [P] Add typed `AdaptiveGovernanceV1Envelope` and governed semantic pairing models in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [ ] T009 [P] Add compatibility helpers for required-baseline versus optional-companion availability in `crates/canon-engine/src/domain/artifact.rs`, `crates/canon-engine/src/domain/publish_profile.rs`, and `tests/policy_and_traces.rs`
- [ ] T010 Implement publish and adapter scaffolding for companion semantics while preserving approval/readiness/project-memory/lineage/promotion-state projection in `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/orchestrator/service.rs`, and `tech-docs/integration/governance-adapter.md`
- [x] T011 Encode the shared semantic/runtime boundary scaffold in `tech-docs/governance-semantics-and-authority-zones.md` and `README.md`

**Checkpoint**: Foundational semantic vocabulary, compatibility models, and adapter scaffolds are ready.

---

## Phase 3: User Story 1 - Publish One Adaptive Governance Vocabulary (Priority: P1) 🎯 MVP

**Goal**: Publish one clear Canon-owned vocabulary for governance state and rollout profile that downstream maintainers can use without reading source code.

**Independent Test**: Review Canon docs plus representative governed metadata and verify that a downstream maintainer can determine the meaning of `advisory`, `catch`, `rule`, `hook`, `minimal`, `guided`, `governed`, and `strict` without reading Canon implementation code.

### Validation for User Story 1

- [ ] T012 [P] [US1] Add failing coverage for governance-state and rollout-profile vocabulary in `tests/mode_profiles.rs` and `tests/governance_adapter_surface.rs`
- [x] T013 [US1] Record vocabulary and contract-line decisions in `specs/055-adaptive-governance/decision-log.md`

### Implementation for User Story 1

- [x] T014 [P] [US1] Define the S4 governance-state and rollout-profile vocabulary in `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T015 [US1] Attach the optional companion vocabulary to governed packet metadata while preserving approval/readiness/lineage/project-memory/promotion-state semantics in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [x] T016 [US1] Document the published vocabulary in `tech-docs/governance-semantics-and-authority-zones.md` and `specs/055-adaptive-governance/contracts/adaptive-governance-v1-contract.md`
- [x] T017 [US1] Capture story validation evidence in `specs/055-adaptive-governance/validation-report.md`

**Checkpoint**: User Story 1 provides a bounded MVP vocabulary that downstream consumers can understand from Canon-owned surfaces alone.

---

## Phase 4: User Story 2 - Preserve The Semantic And Runtime Boundary (Priority: P2)

**Goal**: Keep Canon adaptive-governance semantics advisory and semantic only, while downstream runtimes remain authoritative for operational behavior.

**Independent Test**: Compare the delivered Canon contract with the paired Boundline S4 spec and verify that Canon defines meaning and compatibility only, while Boundline owns runtime confidence, councils, degradation, escalation, and stop behavior.

### Validation for User Story 2

- [ ] T018 [P] [US2] Add failing boundary tests for advisory-only companion semantics and forbidden runtime directives in `tests/governance_adapter_surface.rs` and `tests/policy_and_traces.rs`
- [x] T019 [US2] Record semantic/runtime boundary decisions in `specs/055-adaptive-governance/decision-log.md`

### Implementation for User Story 2

- [ ] T020 [P] [US2] Keep companion semantics advisory-only in `crates/canon-engine/src/domain/publish_profile.rs` and `tech-docs/governance-semantics-and-authority-zones.md`
- [ ] T021 [US2] Prevent runtime-directive leakage through governed publication and adapter projection while keeping approval/readiness/lineage/project-memory/promotion-state semantics stable in `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/orchestrator/service.rs`, and `tech-docs/integration/governance-adapter.md`
- [x] T022 [US2] Align repo-facing documentation with the semantic/runtime boundary in `README.md` and `tech-docs/governance-semantics-and-authority-zones.md`
- [x] T023 [US2] Capture independent review findings for the boundary in `specs/055-adaptive-governance/validation-report.md`

**Checkpoint**: User Stories 1 and 2 both work independently and Canon remains the semantic authority rather than the runtime controller.

---

## Phase 5: User Story 3 - Evolve The Contract Safely (Priority: P3)

**Goal**: Make additive companion-contract evolution safe for older consumers while preserving fail-closed behavior on incompatible semantics.

**Independent Test**: Present one required-baseline-only packet, one packet with compatible optional companion semantics, and one packet with an unsupported companion contract and verify that downstream consumers can classify the baseline, use or ignore the companion safely, and fail closed only on incompatible required conditions.

### Validation for User Story 3

- [ ] T024 [P] [US3] Add failing compatibility tests for missing required baseline, missing companion, unsupported companion, and additive fields in `tests/governance_adapter_surface.rs` and `tests/policy_and_traces.rs`
- [x] T025 [US3] Record compatibility and versioning decisions in `specs/055-adaptive-governance/decision-log.md`

### Implementation for User Story 3

- [ ] T026 [P] [US3] Implement optional-companion compatibility states in `crates/canon-engine/src/domain/artifact.rs` and `crates/canon-engine/src/domain/publish_profile.rs`
- [ ] T027 [US3] Expose baseline-versus-companion availability together with approval/readiness/promotion-state semantics through adapter projection docs and service output in `tech-docs/integration/governance-adapter.md` and `crates/canon-engine/src/orchestrator/service.rs`
- [x] T028 [US3] Document additive evolution and fallback rules in `specs/055-adaptive-governance/contracts/adaptive-governance-adapter-projection.md` and `specs/055-adaptive-governance/quickstart.md`
- [ ] T029 [US3] Capture story validation and independent contract-review evidence in `specs/055-adaptive-governance/validation-report.md`

**Checkpoint**: All three user stories are independently functional and the additive contract rules are explicit.

---

## Final Phase: Verification & Compliance

**Purpose**: Close validation, release-facing documentation, and cross-repo review evidence.

- [ ] T030 [P] Run targeted semantic validation in `tests/governance_adapter_surface.rs`, `tests/mode_profiles.rs`, and `tests/policy_and_traces.rs` for supported, missing-required-baseline, missing-companion, and unsupported-companion scenarios, then append the results to `specs/055-adaptive-governance/validation-report.md`
- [ ] T031 [P] Run workspace validation with `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --no-run --all-targets`, and `cargo nextest run --workspace --all-features`, then capture the results in `specs/055-adaptive-governance/validation-report.md`
- [ ] T032 Perform cross-repo closeout updates in `CHANGELOG.md`, `README.md`, and `specs/055-adaptive-governance/quickstart.md` after the final Boundline contract review is recorded in `specs/055-adaptive-governance/validation-report.md`
- [ ] T033 Increase the workspace version and release-facing references in `Cargo.toml`, `CHANGELOG.md`, package manifests under `crates/`, and any governed release metadata that surfaces the shipped semantic contract version
- [ ] T034 Apply an appropriate design pattern to break up oversized files or functions in `crates/canon-engine/src/orchestrator/publish.rs`, `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/domain/artifact.rs`, and any newly expanded adaptive-governance modules before final sign-off
- [ ] T035 Ensure modified-file and feature-level coverage remains at 95% or higher by refreshing the relevant coverage evidence, rerunning the targeted semantic suites, and recording the result in `specs/055-adaptive-governance/validation-report.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Governance & Artifacts (Phase 0)**: No dependencies. Must complete first.
- **Setup (Phase 1)**: Depends on Phase 0 completion.
- **Foundational (Phase 2)**: Depends on Setup completion and blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational completion.
- **User Story 2 (Phase 4)**: Depends on Foundational completion and reuses the vocabulary stabilized in User Story 1.
- **User Story 3 (Phase 5)**: Depends on Foundational completion and reuses the compatibility models stabilized in User Story 1.
- **Final Phase**: Depends on all desired user stories being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Independent MVP after Foundational.
- **User Story 2 (P2)**: Builds on the published vocabulary from US1 but remains independently testable through semantic/runtime boundary review.
- **User Story 3 (P3)**: Builds on the shared vocabulary and pairing models from Foundational and US1 but remains independently testable through compatibility scenarios.

### Within Each User Story

- Validation tasks and failing checks MUST happen before implementation.
- Decision or invariant updates MUST be recorded before the affected code is signed off.
- Typed models come before publish and adapter projection work.
- Human-facing and machine-facing contract docs must remain aligned before the story closes.

### Parallel Opportunities

- `T003` and `T004` can run in parallel after `T001` and `T002`.
- `T008` and `T009` can run in parallel after `T007`.
- Story validation tasks marked `[P]` can run in parallel.
- After Foundational completion, US2 and US3 prep can proceed in parallel while US1 vocabulary work stabilizes shared semantics.

---

## Parallel Example: User Story 1

```bash
# Launch the vocabulary checks together:
Task: "Add failing coverage for governance-state and rollout-profile vocabulary in tests/mode_profiles.rs and tests/governance_adapter_surface.rs"
Task: "Define the S4 governance-state and rollout-profile vocabulary in crates/canon-engine/src/domain/policy.rs and crates/canon-engine/src/domain/mode.rs"
```

## Parallel Example: User Story 2

```bash
# Launch the boundary checks together:
Task: "Add failing boundary tests for advisory-only companion semantics and forbidden runtime directives in tests/governance_adapter_surface.rs and tests/policy_and_traces.rs"
Task: "Keep companion semantics advisory-only in crates/canon-engine/src/domain/policy.rs and crates/canon-engine/src/domain/mode.rs"
```

## Parallel Example: User Story 3

```bash
# Launch the compatibility checks together:
Task: "Add failing compatibility tests for missing companion, unsupported companion, and additive fields in tests/governance_adapter_surface.rs and tests/policy_and_traces.rs"
Task: "Implement optional-companion compatibility states in crates/canon-engine/src/domain/artifact.rs and crates/canon-engine/src/domain/publish_profile.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 0: Governance & Artifacts.
2. Complete Phase 1: Setup.
3. Complete Phase 2: Foundational.
4. Complete Phase 3: User Story 1.
5. **STOP and VALIDATE**: Confirm the vocabulary and contract boundary are understandable from Canon-owned docs and metadata before expanding scope.

### Incremental Delivery

1. Complete Governance + Setup + Foundational.
2. Deliver User Story 1 and validate the published vocabulary.
3. Deliver User Story 2 and validate the semantic/runtime boundary.
4. Deliver User Story 3 and validate additive compatibility rules.
5. Finish with the Final Phase cross-repo and workspace validation.

### Parallel Team Strategy

With multiple developers:

1. Team completes Governance, Setup, and Foundational together.
2. Once Foundational is stable:
   - Developer A: User Story 1 vocabulary and packet metadata.
   - Developer B: User Story 2 boundary protection and adapter documentation.
   - Developer C: User Story 3 compatibility and versioning rules.
3. Merge only after the validation report captures story-level evidence and independent review findings.

---

## Notes

- [P] tasks touch different files and can proceed without waiting on another incomplete task in the same phase.
- `authority-governance-v1` remains the required baseline contract throughout this task list.
- `adaptive-governance-v1` stays additive, optional, and semantic-only unless a future contract line explicitly changes that boundary.
- Independent review evidence is mandatory for the semantic/runtime boundary because this slice is classified as systemic-impact.