# Tasks: Logical Packet Ordering

**Input**: Design documents from `specs/049-logical-packet-ordering/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/packet-ordering-metadata.md`, `quickstart.md`

**Validation**: Layered validation is mandatory. Each user story starts with executable failing checks where practical and closes with recorded evidence in `specs/049-logical-packet-ordering/validation-report.md`.

**Organization**: Tasks are grouped by user story so packet directory ordering, metadata ordering, publish and summary preservation, and documentation clarity can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no direct dependency)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`, `US4`)
- Every task includes exact file paths

## Phase 0: Governance, Versioning, and Authorization

**Purpose**: Establish the release baseline and the durable artifacts that authorize implementation.

- [x] T001 Bump Canon version to `0.49.0` in `Cargo.toml`, `README.md`, `assistant/plugin-metadata.json`, `.codex-plugin/plugin.json`, `.claude-plugin/manifest.json`, `.cursor-plugin/manifest.json`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, and `tests/integration/skills_bootstrap.rs`.
- [x] T002 Update `specs/049-logical-packet-ordering/decision-log.md` and `specs/049-logical-packet-ordering/validation-report.md` with the 049-over-046 supersession note, approval expectations, and closeout evidence sections.
- [x] T003 [P] Freeze `primary_artifact` and `artifact_order` as required ordering fields, and document conditional emission rules for `publish_order` and `legacy_aliases` in `specs/049-logical-packet-ordering/data-model.md` and `specs/049-logical-packet-ordering/contracts/packet-ordering-metadata.md` before implementation starts.

---

## Phase 1: Foundational Ordering Infrastructure

**Purpose**: Build the shared ordering primitives and metadata plumbing that all stories depend on.

**⚠️ CRITICAL**: No user story implementation should start until this phase is complete.

- [x] T004 Add a Canon-owned per-mode packet ordering registry, shared ordered-filename helpers, and reader-facing versus sidecar classification in `crates/canon-engine/src/artifacts/contract.rs`, `crates/canon-engine/src/domain/artifact.rs`, and the relevant mode emitters under `crates/canon-engine/src/orchestrator/service/`.
- [x] T005 [P] Extend packet result and metadata plumbing to carry `primary_artifact`, `artifact_order`, and any compatibility aliases in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`.
- [x] T006 [P] Update stored-artifact and summary-facing readers to consume ordered packet metadata in `crates/canon-engine/src/persistence/store.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, and `crates/canon-engine/src/orchestrator/service/summarizers.rs`.
- [x] T007 Encode any shared invariant guards or normalization helpers needed for contiguous numbering and legacy alias handling in `crates/canon-engine/src/orchestrator/service/mode_shaping.rs` and `crates/canon-engine/src/orchestrator/service/inspect.rs`.

**Checkpoint**: Ordered packet naming and metadata primitives exist and governance artifacts remain current.

---

## Phase 2: User Story 1 - Read Packets In Intended Order (Priority: P1) 🎯 MVP

**Goal**: New packet directories expose the intended reading order directly through contiguous numeric prefixes and a stable `01-*` primary artifact.

**Independent Test**: Run representative requirements and architecture flows and verify the emitted packet directories show contiguous reader-facing ordering with the documented primary artifact first.

### Validation for User Story 1 (MANDATORY)

- [x] T008 [P] [US1] Add failing ordering, contiguous-numbering, and registry-completeness checks in `tests/requirements_contract.rs`, `tests/architecture_contract.rs`, `tests/runtime_filesystem.rs`, and `tests/mode_profiles.rs`.
- [x] T009 [US1] Record prefix-width, contiguous-numbering, and sidecar boundary decisions in `specs/049-logical-packet-ordering/decision-log.md` if implementation reveals any deviations from the current defaults.

### Implementation for User Story 1

- [x] T010 [P] [US1] Update per-mode emitted artifact names to use the packet ordering registry and ordered reader-facing filenames in `crates/canon-engine/src/artifacts/contract.rs` and the relevant mode emitters under `crates/canon-engine/src/orchestrator/service/`.
- [x] T011 [US1] Update artifact rendering and lookup logic to match ordered filenames in `crates/canon-engine/src/artifacts/markdown.rs`.
- [x] T012 [US1] Update architecture packet shaping so reader-facing view artifacts preserve contiguous numbering while manifests and sidecars reference the ordered packet body without joining it in `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`.
- [x] T013 [P] [US1] Add or update runtime coverage for ordered packet directories in `tests/requirements_run.rs`, `tests/architecture_run.rs`, and `tests/architecture_c4_run.rs`.
- [x] T014 [US1] Capture packet-directory validation evidence in `specs/049-logical-packet-ordering/validation-report.md`.

**Checkpoint**: User Story 1 is independently functional and validates ordered packet directories end to end.

---

## Phase 3: User Story 2 - Surface Primary Artifact And Order In Metadata (Priority: P1)

**Goal**: Packet metadata exposes `primary_artifact` and `artifact_order`, and Canon can still resolve historical packets through compatibility behavior.

**Independent Test**: Inspect packet metadata for new packets and verify ordered fields are present and aligned with emitted files, then confirm a historical packet still resolves without rewrite.

### Validation for User Story 2 (MANDATORY)

- [x] T015 [P] [US2] Add failing metadata contract checks in `tests/contract/requirements_contract.rs`, `tests/contract/architecture_contract.rs`, and `tests/run_lookup.rs`.
- [x] T016 [US2] Record the final metadata-field names and compatibility expectations in `specs/049-logical-packet-ordering/decision-log.md`.

### Implementation for User Story 2

- [x] T017 [P] [US2] Extend general packet metadata emission with `primary_artifact` and `artifact_order` in `crates/canon-engine/src/orchestrator/service.rs` and `crates/canon-engine/src/orchestrator/service/mode_shaping.rs`.
- [x] T018 [US2] Implement legacy filename alias or compatibility resolution in `crates/canon-engine/src/persistence/store.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, and `crates/canon-engine/src/orchestrator/service.rs`.
- [x] T019 [P] [US2] Add focused metadata and compatibility regressions in `tests/inspect_modes.rs`, `tests/run_lookup.rs`, and `tests/contract/runtime_filesystem.rs`.
- [x] T020 [US2] Capture metadata-order validation evidence in `specs/049-logical-packet-ordering/validation-report.md`.

**Checkpoint**: User Stories 1 and 2 both work independently, and machine-readable packet ordering is stable.

---

## Phase 4: User Story 3 - Preserve Order During Publish And Summaries (Priority: P2)

**Goal**: Publish, status, and inspect surfaces preserve logical packet order and surface the primary artifact first.

**Independent Test**: Publish representative runs and inspect status or inspect output to confirm ordered artifacts and primary-artifact-first summaries remain stable.

### Validation for User Story 3 (MANDATORY)

- [x] T021 [P] [US3] Add failing ordered-publish, ordered-status, and ordered-summary checks in `tests/publish_runtime.rs`, `tests/governance_cli.rs`, `tests/inspect_clarity.rs`, `tests/contract/inspect_clarity.rs`, and the relevant `tests/integration/*publish*.rs` files.
- [x] T022 [US3] Record any publish-index ordering or summary-surface decisions in `specs/049-logical-packet-ordering/decision-log.md`.

### Implementation for User Story 3

- [x] T023 [P] [US3] Preserve packet ordering and numeric prefixes in publish outputs and indexes in `crates/canon-engine/src/orchestrator/publish.rs`.
- [x] T024 [P] [US3] Surface the primary artifact first in engine summaries and inspect results in `crates/canon-engine/src/orchestrator/service/summarizers.rs` and `crates/canon-engine/src/orchestrator/service/inspect.rs`.
- [x] T025 [US3] Align CLI-facing ordered status and inspect rendering in `crates/canon-cli/src/output.rs` and adjust `crates/canon-cli/src/commands/status.rs` or `crates/canon-cli/src/commands/inspect.rs` only where wiring changes are required.
- [x] T026 [P] [US3] Add publish, status, and summary regressions in `tests/publish_runtime.rs`, `tests/governance_cli.rs`, `tests/inspect_modes.rs`, `tests/inspect_clarity.rs`, `tests/pr_review_publish.rs`, and `tests/migration_publish.rs`.
- [x] T027 [US3] Capture publish and summary validation evidence in `specs/049-logical-packet-ordering/validation-report.md`.

**Checkpoint**: Ordered packet semantics survive publish and summary flows.

---

## Phase 5: User Story 4 - Clarify Domain Language Versus Domain Model (Priority: P2)

**Goal**: Mode docs clearly distinguish `domain-language` from `domain-model` and publish their ordered artifact sequences.

**Independent Test**: Read the mode guide and supporting templates or examples and verify both domain-oriented modes describe distinct responsibilities and ordered packet artifacts.

### Validation for User Story 4 (MANDATORY)

- [x] T028 [P] [US4] Add failing doc or contract checks for ordered artifact listings, domain-mode distinctions, and mode-catalog completeness in `tests/system_shaping_domain_modeling_docs.rs`, `tests/architecture_domain_modeling_docs.rs`, `tests/governance_runtime_framing_docs.rs`, and `tests/mode_profiles.rs`.
- [x] T029 [US4] Record any vocabulary or packet-listing clarifications in `specs/049-logical-packet-ordering/decision-log.md`.

### Implementation for User Story 4

- [x] T030 [P] [US4] Update ordered artifact listings and domain-mode descriptions in `tech-docs/guides/modes.md`.
- [x] T031 [P] [US4] Update domain-language and domain-model templates or examples in `defaults/templates/canon-input/domain-language.md`, `defaults/templates/canon-input/domain-model.md`, `tech-docs/examples/canon-input/domain-language-order-fulfillment.md`, and `tech-docs/examples/canon-input/domain-model-order-fulfillment.md`.
- [x] T032 [US4] Update release-facing docs in `CHANGELOG.md` and `ROADMAP.md`.
- [x] T033 [US4] Capture documentation validation evidence in `specs/049-logical-packet-ordering/validation-report.md`.

**Checkpoint**: Documentation reflects the logical packet-ordering contract and clarifies the domain-mode split.

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, independent review, and quality closeout.

- [x] T034 Perform a catalog-wide audit of the current packet-emitting modes and docs, including an independent review of at least one new ordered packet and one historical packet, then record SC-001, SC-002, SC-003, and SC-004 evidence in `specs/049-logical-packet-ordering/validation-report.md`.
- [x] T035 Run `cargo fmt`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, the targeted packet-ordering test set, `cargo nextest run`, and `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`; confirm touched-file coverage is at least 95% and record the evidence in `specs/049-logical-packet-ordering/validation-report.md`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 0**: No dependencies. Must complete first.
- **Phase 1**: Depends on Phase 0. Blocks all user stories.
- **Phase 2 (US1)**: Depends on Phase 1.
- **Phase 3 (US2)**: Depends on Phase 1 and should build on the ordered filenames from US1.
- **Phase 4 (US3)**: Depends on Phases 1 through 3 because publish and summaries consume ordered metadata.
- **Phase 5 (US4)**: Depends on the stable packet-ordering contract from Phases 1 through 4.
- **Final Phase**: Depends on all desired user stories being complete.

### User Story Dependencies

- **US1**: The MVP. Establishes visible packet order on disk.
- **US2**: Depends on ordered packet naming and shared metadata plumbing from US1 and Phase 1.
- **US3**: Depends on ordered packet naming and metadata from US1 and US2.
- **US4**: Depends on the final packet-ordering contract so docs reflect shipped behavior.

### Within Each User Story

- Validation tasks and failing checks happen before implementation changes where executable checks are practical.
- Decision-log or contract updates happen before the implementation that relies on them.
- Evidence capture happens before a story is declared complete.

## Parallel Opportunities

- T003 can run in parallel with T002 after T001.
- T005 and T006 can run in parallel after T004.
- Within US1, T010 and T013 can run in parallel once the initial failing checks exist.
- Within US2, T017 and T019 can run in parallel after T016.
- Within US3, T023 and T024 can run in parallel after T021.
- Within US4, T030 and T031 can run in parallel after T028.

## Implementation Strategy

### MVP First

1. Complete Phase 0.
2. Complete Phase 1.
3. Complete Phase 2 (US1).
4. Validate packet directory ordering independently before moving on.

### Incremental Delivery

1. Land shared ordering infrastructure.
2. Ship ordered packet directories and validate them.
3. Add metadata and legacy compatibility.
4. Preserve order across publish, status, and inspect.
5. Finish with documentation and compliance closeout.

## Notes

- Keep `specs/049-logical-packet-ordering/decision-log.md` and `specs/049-logical-packet-ordering/validation-report.md` current as implementation decisions land.
- Do not rewrite historical governed runs; compatibility behavior must absorb legacy packet names.
- The last task is intentionally reserved for `cargo fmt`, `cargo fmt --check`, clippy, tests, and touched-file coverage >=95%.