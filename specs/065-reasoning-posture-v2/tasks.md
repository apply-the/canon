# Tasks: Governed Reasoning Posture v2

**Input**: Design documents from `/specs/065-reasoning-posture-v2/`

**Prerequisites**: `plan.md` (required), `spec.md` (required for user stories),
`research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Required for this feature because the specification explicitly
requires executable contract validation, machine-checkable examples, release
alignment checks, coverage validation, formatting validation, lint validation,
independent review, and human approval evidence for a systemic-impact slice.

**Organization**: Tasks are grouped by user story so each story can be
implemented and validated independently after the shared foundation is in
place, then closed with independent review and an approval gate before the
feature is marked complete.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the feature-local scaffolding for executable fixtures,
traceability, review evidence, and approval evidence.

- [X] T001 Create the v2 fixture catalog scaffold in tests/fixtures/governed_reasoning_posture_v2/README.md
- [X] T002 [P] Create the feature validation report scaffold in specs/065-reasoning-posture-v2/validation-report.md
- [X] T003 [P] Create the feature decision log scaffold in specs/065-reasoning-posture-v2/decision-log.md
- [X] T004 Create and maintain finalized decisions for v1/v2 migration, active-versus-legacy contract lines, profile selector shape, confidence handoff shape, provenance shape, independence shape, release metadata compatibility, and rejection behavior in specs/065-reasoning-posture-v2/decision-log.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared fixture-loading, contract-line typing,
publication-state primitives, and canonical example IDs that every user story
depends on.

**CRITICAL**: No user story work should begin until this phase is complete.

- [X] T005 Add shared v2 fixture-loading and assertion helpers in tests/contract/governed_reasoning_posture_contract.rs
- [X] T006 [P] Add canonical v2 example IDs and fixture references in specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2-examples.md
- [X] T007 [P] Add typed reasoning-posture contract-line constants and helper types in crates/canon-engine/src/domain/publish_profile/semantic.rs
- [X] T008 [P] Add active-versus-legacy publication-state helpers in crates/canon-engine/src/domain/publish_profile/publication.rs
- [X] T009 [P] Add reasoning-posture authority boundary helpers in crates/canon-engine/src/domain/publish_profile/authority.rs

**Checkpoint**: Shared fixture plumbing, typed contract primitives,
publication semantics, and canonical example IDs are ready for story-by-story
implementation.

---

## Phase 3: User Story 1 - Publish A Typed v2 Contract (Priority: P1)

**Goal**: Publish a typed `governed_reasoning_posture_v2` contract that a
Boundline maintainer can consume from repository artifacts alone.

**Independent Test**: Review the stable contract, the feature-local contract
brief, and the example fixtures, then run the governed reasoning posture
contract test to confirm that the required blocks, selector rules,
independence contract, confidence handoff, provenance, and fail-closed
behavior are understandable without reading Canon implementation code.

### Tests For User Story 1

- [X] T010 [P] [US1] Add v2 required-block and selector assertions in tests/contract/governed_reasoning_posture_contract.rs
- [X] T011 [P] [US1] Create the valid v2 payload fixture in tests/fixtures/governed_reasoning_posture_v2/valid-v2-posture.toml
- [X] T012 [P] [US1] Create the dual-selector rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-selector-both-present.toml
- [X] T013 [P] [US1] Create the missing-selector rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-selector-neither-present.toml

### Implementation For User Story 1

- [X] T014 [US1] Publish the canonical v2 producer shape in tech-docs/integration/governed-reasoning-posture-contract.md
- [X] T015 [US1] Align the feature-local v2 contract mirror in specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2.md
- [X] T016 [US1] Encode typed selector, independence, confidence-handoff, provenance, and compatibility helpers in crates/canon-engine/src/domain/publish_profile/semantic.rs
- [X] T017 [US1] Wire valid-v2 and selector-conflict fixture coverage into tests/contract/governed_reasoning_posture_contract.rs
- [X] T018 [US1] Update quickstart artifact references to the concrete v2 fixture set in specs/065-reasoning-posture-v2/quickstart.md
- [X] T019 [US1] Run `cargo test --test governed_reasoning_posture_contract` for the valid-v2 and selector fixtures anchored by tests/contract/governed_reasoning_posture_contract.rs

**Checkpoint**: `governed_reasoning_posture_v2` is documented, mirrored,
validated as a typed Canon-owned contract line, and independently testable from
repository artifacts alone.

---

## Phase 4: User Story 2 - Fail Closed On Drift And Malformed Data (Priority: P2)

**Goal**: Reject malformed posture payloads, stale release metadata, and
incompatible compatibility claims before runtime execution begins.

**Independent Test**: Run the contract validation against the invalid fixture
set plus release-facing metadata, then confirm that each malformed case fails
with the expected rejection reason and that stale or contradictory release
surfaces are treated as contract drift.

### Tests For User Story 2

- [X] T020 [P] [US2] Create the omitted-minimum-independence rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-independence-missing-block.toml
- [X] T021 [P] [US2] Create the independence-guidance-override rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-independence-guidance-override.toml
- [X] T022 [P] [US2] Create the contradictory-independence rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-independence-contradictory.toml
- [X] T023 [P] [US2] Create the impossible-independence-minima rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-independence-impossible-minima.toml
- [X] T024 [P] [US2] Create the omitted-confidence-handoff rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-confidence-missing-block.toml
- [X] T025 [P] [US2] Create the contradictory-confidence-none-state rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-confidence-none-contradictory.toml
- [X] T026 [P] [US2] Create the incomplete-confidence-handoff rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-confidence-required-missing-fields.toml
- [X] T027 [P] [US2] Create the omitted-provenance rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-provenance-missing-block.toml
- [X] T028 [P] [US2] Create the invalid-provenance-reference-kind rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-provenance-missing-reference-kind.toml
- [X] T029 [P] [US2] Create the provenance-incompatible-handoff rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-provenance-incompatible-handoff.toml
- [X] T030 [P] [US2] Create the stale-release-metadata rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-release-metadata-stale.json
- [X] T031 [P] [US2] Create the contradictory-release-metadata rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-release-metadata-contradictory.json
- [X] T070 [P] [US2] Create the invalid-compatibility-window rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-compatibility-window.toml
- [X] T072 [P] [US2] Create the unsupported-vocabulary rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-unsupported-vocabulary.toml
- [X] T073 [P] [US2] Create the stale-provenance rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-provenance-stale.toml
- [X] T074 [P] [US2] Create the contradictory-provenance rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-provenance-contradictory.toml
- [X] T032 [US2] Add omitted-block, guidance-override, contradictory, and impossible-independence assertions in tests/contract/governed_reasoning_posture_contract.rs
- [X] T033 [US2] Add omitted-block, `state = none` contradiction, and required-without-fields confidence-handoff assertions in tests/contract/governed_reasoning_posture_contract.rs
- [X] T034 [US2] Add omitted-block, invalid-reference-kind, stale-provenance, contradictory-provenance, incompatible-handoff, unsupported-vocabulary, invalid-compatibility-window, and stale-or-contradictory release-metadata assertions in tests/contract/governed_reasoning_posture_contract.rs

### Implementation For User Story 2

- [X] T035 [US2] Expand fail-closed rejection rules and malformed-case examples in tech-docs/integration/governed-reasoning-posture-contract.md
- [X] T036 [US2] Align malformed-case coverage and expected reasons in specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2-examples.md
- [X] T037 [US2] Implement provenance, confidence-handoff, and compatibility validation helpers in crates/canon-engine/src/domain/publish_profile/semantic.rs
- [X] T038 [US2] Align supported release-window claims in defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml
- [X] T039 [US2] Align assistant release metadata with v2 validation surfaces in assistant/plugin-metadata.json
- [X] T040 [US2] Run `cargo test --test governed_reasoning_posture_contract` for malformed payload, unsupported-vocabulary, invalid-compatibility-window, stale-or-contradictory provenance, and release-drift fixtures anchored by tests/contract/governed_reasoning_posture_contract.rs

**Checkpoint**: Invalid payloads, stale metadata, contradictory metadata, and
incompatible version claims fail closed with explicit reasons.

---

## Phase 5: User Story 3 - Governed Migration Between v1 And v2 (Priority: P3)

**Goal**: Make dual-line coexistence, migration, and incompatibility rules
between `v1` and `v2` explicit and machine-checkable.

**Independent Test**: Review the migration contract and run coexistence and
migration-rejection fixtures to confirm that only one active line is allowed,
legacy publication is explicit, and unsupported mixed-line states are rejected.

### Tests For User Story 3

- [X] T041 [P] [US3] Create the valid dual-line coexistence fixture in tests/fixtures/governed_reasoning_posture_v2/dual-line-coexistence-valid.toml
- [X] T042 [P] [US3] Create the ambiguous dual-line rejection fixture in tests/fixtures/governed_reasoning_posture_v2/dual-line-coexistence-ambiguous.toml
- [X] T043 [P] [US3] Create the v2-to-v1-consumer rejection fixture in tests/fixtures/governed_reasoning_posture_v2/migration-rejection-v2-to-v1-consumer.toml
- [X] T044 [P] [US3] Create the v1-to-v2-required rejection fixture in tests/fixtures/governed_reasoning_posture_v2/migration-rejection-v1-to-v2-required.toml
- [X] T045 [US3] Add coexistence and migration-rejection assertions in tests/contract/governed_reasoning_posture_contract.rs

### Implementation For User Story 3

- [X] T046 [US3] Update executable migration rules in specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2-migration.md
- [X] T047 [US3] Publish active-versus-legacy coexistence rules in tech-docs/integration/governed-reasoning-posture-contract.md
- [X] T048 [US3] Implement active/legacy publication semantics in crates/canon-engine/src/domain/publish_profile/publication.rs
- [X] T049 [US3] Enforce reasoning-posture authority boundaries for migration states in crates/canon-engine/src/domain/publish_profile/authority.rs
- [X] T050 [US3] Wire coexistence and migration fixture IDs and expected rejection reasons into specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2-examples.md
- [X] T051 [US3] Run `cargo test --test governed_reasoning_posture_contract` for coexistence and migration fixtures anchored by tests/contract/governed_reasoning_posture_contract.rs

**Checkpoint**: Mixed `v1`/`v2` ecosystems have one explicit active line,
deterministic rejection rules, and executable migration evidence.

---

## Phase 6: User Story 4 - Publish The New Contract Truthfully (Priority: P4)

**Goal**: Align Canon `0.64.0` release surfaces and user-facing documentation
with the new contract line and its unchanged ownership boundary.

**Independent Test**: Review release metadata, README, CHANGELOG, CLI docs, and
the stable contract, then confirm that a reviewer can identify the semantic
delta from `v1`, the explicit migration boundary, and the unchanged
Canon-versus-Boundline ownership split in under 10 minutes.

### Tests For User Story 4

- [X] T052 [US4] Add Canon `0.64.0` release-surface alignment assertions in tests/contract/governed_reasoning_posture_contract.rs

### Implementation For User Story 4

- [X] T053 [US4] Update Canon release version alignment in Cargo.toml
- [X] T054 [P] [US4] Update release notes for `governed_reasoning_posture_v2` in CHANGELOG.md
- [X] T055 [P] [US4] Update the user-facing contract overview in README.md
- [X] T056 [P] [US4] Update CLI and reference release messaging in docs/reference/cli.md
- [X] T057 [P] [US4] Update roadmap framing for the `v2` contract line in ROADMAP.md
- [X] T058 [US4] Align the reviewer validation flow with implemented surfaces in specs/065-reasoning-posture-v2/quickstart.md
- [X] T059 [US4] Record release-surface review evidence in specs/065-reasoning-posture-v2/validation-report.md

**Checkpoint**: Release-facing truth surfaces consistently describe `v2`, the
`0.64.0` alignment, and the preserved runtime ownership boundary.

---

## Phase 7: Cross-Cutting Validation, Review, And Approval

**Purpose**: Final synchronization, explicit command execution, independent
review, approval gating, and recorded completion evidence across all stories.

- [X] T060 Refresh the feature-local contract mirror after implementation in specs/065-reasoning-posture-v2/contracts/governed-reasoning-posture-v2.md
- [X] T061 Run `cargo test --test governed_reasoning_posture_contract` against the completed v2 fixture harness anchored by tests/contract/governed_reasoning_posture_contract.rs
- [X] T062 Run `cargo test --no-run --all-targets` for workspace-wide compile validation anchored by Cargo.toml
- [X] T063 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` for touched code anchored by Cargo.toml
- [X] T064 Run `cargo fmt --check` for workspace formatting validation anchored by Cargo.toml
- [X] T065 Run `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info` and confirm 95% modified-file coverage in lcov.info
- [X] T066 Execute the quickstart verification path documented in specs/065-reasoning-posture-v2/quickstart.md
- [X] T067 Run an independent adversarial review of the implemented v2 contract, fixtures, migration rules, and release surfaces and record findings in specs/065-reasoning-posture-v2/validation-report.md
- [X] T068 Obtain explicit human approval for the v1/v2 semantic delta, active-versus-legacy rule, and release-surface claims and record the decision in specs/065-reasoning-posture-v2/validation-report.md
- [X] T069 Record contract-test, compile, Clippy, formatting, coverage, quickstart, independent-review, and approval evidence in specs/065-reasoning-posture-v2/validation-report.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup** has no dependencies and can start immediately.
- **Phase 2: Foundational** depends on Phase 1 and blocks all user stories.
- **Phase 3: US1** depends on Phase 2 and delivers the MVP contract line.
- **Phase 4: US2** depends on Phase 3 because malformed-data rejection extends
  the typed `v2` contract and fixture harness.
- **Phase 5: US3** depends on Phase 3 because migration rules require the `v2`
  contract line and its fixture inventory to exist, but it may proceed in
  parallel with Phase 4 once Phase 3 is stable.
- **Phase 6: US4** depends on Phases 3, 4, and 5 so release surfaces reflect
  the final contract, rejection behavior, and migration policy.
- **Phase 7: Cross-Cutting Validation, Review, And Approval** depends on all
  desired user stories being complete and MUST finish before the feature can be
  marked complete.

### User Story Dependencies

- **US1 (P1)** starts immediately after the foundational phase and has no
  dependency on later stories.
- **US2 (P2)** builds on US1's typed payload and fixture harness.
- **US3 (P3)** builds on US1's published `v2` line but is otherwise
  independently testable from US2.
- **US4 (P4)** summarizes and publishes the outcome of the earlier stories, so
  it should run after US1 through US3 stabilize.

### Parallel Opportunities

- Setup tasks `T002` and `T003` can run in parallel.
- Foundational tasks `T006` through `T009` can run in parallel after `T005`.
- In **US1**, `T010` through `T013` can run in parallel.
- In **US2**, fixture tasks `T020` through `T031` plus `T070` and `T072` through `T074` can run in parallel.
- In **US3**, `T041` through `T044` can run in parallel.
- In **US4**, `T054` through `T057` can run in parallel after `T053` anchors
  the release line.

---

## Parallel Example: User Story 1

```bash
Task: "Add v2 required-block and selector assertions in tests/contract/governed_reasoning_posture_contract.rs"
Task: "Create the valid v2 payload fixture in tests/fixtures/governed_reasoning_posture_v2/valid-v2-posture.toml"
Task: "Create the dual-selector rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-selector-both-present.toml"
Task: "Create the missing-selector rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-selector-neither-present.toml"
```

## Parallel Example: User Story 2

```bash
Task: "Create the omitted-minimum-independence rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-independence-missing-block.toml"
Task: "Create the independence-guidance-override rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-independence-guidance-override.toml"
Task: "Create the omitted-confidence-handoff rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-confidence-missing-block.toml"
Task: "Create the contradictory-confidence-none-state rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-confidence-none-contradictory.toml"
Task: "Create the omitted-provenance rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-provenance-missing-block.toml"
Task: "Create the stale-provenance rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-provenance-stale.toml"
Task: "Create the contradictory-provenance rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-provenance-contradictory.toml"
Task: "Create the unsupported-vocabulary rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-unsupported-vocabulary.toml"
Task: "Create the invalid-compatibility-window rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-compatibility-window.toml"
Task: "Create the stale-release-metadata rejection fixture in tests/fixtures/governed_reasoning_posture_v2/invalid-release-metadata-stale.json"
```

## Parallel Example: User Story 3

```bash
Task: "Create the valid dual-line coexistence fixture in tests/fixtures/governed_reasoning_posture_v2/dual-line-coexistence-valid.toml"
Task: "Create the ambiguous dual-line rejection fixture in tests/fixtures/governed_reasoning_posture_v2/dual-line-coexistence-ambiguous.toml"
Task: "Create the v2-to-v1-consumer rejection fixture in tests/fixtures/governed_reasoning_posture_v2/migration-rejection-v2-to-v1-consumer.toml"
Task: "Create the v1-to-v2-required rejection fixture in tests/fixtures/governed_reasoning_posture_v2/migration-rejection-v1-to-v2-required.toml"
```

## Parallel Example: User Story 4

```bash
Task: "Update release notes for governed_reasoning_posture_v2 in CHANGELOG.md"
Task: "Update the user-facing contract overview in README.md"
Task: "Update CLI and reference release messaging in docs/reference/cli.md"
Task: "Update roadmap framing for the v2 contract line in ROADMAP.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational.
3. Complete Phase 3: User Story 1.
4. Run the US1 contract test independently before widening scope.

### Incremental Delivery

1. Deliver US1 to publish the typed `v2` line.
2. Add US2 to enforce fail-closed malformed-data and release-drift rejection.
3. Add US3 to define dual-line coexistence and migration rejection.
4. Add US4 to align release surfaces and reviewer-facing documentation.
5. Finish with Phase 7 command execution, independent review, approval, and
   evidence capture.

### Completion Gate

Do not mark the feature complete until Phase 7 has finished the workspace
validation commands, quickstart verification, independent adversarial review,
human approval checkpoint, and final evidence recording.

### Suggested MVP Scope

Implement through **Phase 3 / US1** first. That slice establishes the new
contract line, the typed payload shape, the first valid and invalid fixtures,
and the minimum executable validation needed to unblock the later stories.

---

## Notes

- Every task follows the required checklist format: checkbox, task ID,
  optional `[P]`, required `[US#]` for story work, and an exact file path.
- Tests and validation execution tasks are intentionally included because the
  spec requires executable validation and machine-checkable examples.
- Independent review and human approval are explicit tasks, not implied by
  ordinary evidence-recording work.
- Avoid cross-story scope creep: keep story-specific fixtures and assertions
  inside the story that needs them, then synchronize in Phase 7.