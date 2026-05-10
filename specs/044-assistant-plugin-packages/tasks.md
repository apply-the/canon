# Tasks: Assistant Plugin Packages

**Input**: Design documents from `/specs/044-assistant-plugin-packages/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Validation**: Validation is mandatory. Rust tests and the shell wrapper must prove manifest JSON validity, required fields, version alignment, path references, required method surfaces, prohibited language rejection, docs links, and final touched-file coverage.

**Organization**: Tasks are grouped by user story to keep install/discovery, capability exposure, and drift prevention independently reviewable.

## Phase 0: Version, Governance & Artifacts

**Purpose**: Honor the requested first task, then ensure the governed feature artifacts remain current.

- [x] T001 Upgrade Canon version surfaces from `0.43.0` to `0.44.0` in `Cargo.toml`, `Cargo.lock`, `README.md`, runtime compatibility references, and related tests
- [x] T002 Record current governance decisions and validation plan in `specs/044-assistant-plugin-packages/decision-log.md` and `specs/044-assistant-plugin-packages/validation-report.md`
- [x] T003 Confirm specification, plan, research, data model, contract, and quickstart artifacts are coherent in `specs/044-assistant-plugin-packages/`

---

## Phase 1: Validation Harness

**Purpose**: Create the failing validation before package implementation.

- [x] T004 [P] Write failing assistant plugin package validation tests in `tests/assistant_plugin_packages.rs`
- [x] T005 Run `cargo test --test assistant_plugin_packages` and record the expected missing-file/version failure in `specs/044-assistant-plugin-packages/validation-report.md`
- [x] T006 Add the validation command wrapper in `scripts/validate-assistant-plugins.sh`

---

## Phase 2: Foundational Shared Package Material

**Purpose**: Add shared Canon-owned metadata, commands, prompts, and assets that host packages reference.

- [x] T007 Add shared plugin metadata in `assistant/plugin-metadata.json`
- [x] T008 Add shared governed method command definitions in `assistant/commands/governed-methods.json`
- [x] T009 [P] Add shared starter prompts and Copilot command/prompt pack in `assistant/prompts/starter-prompts.md` and `assistant/prompts/copilot-command-pack.md`
- [x] T010 [P] Add package icon and logo assets in `assistant/assets/canon-plugin-icon.svg` and `assistant/assets/canon-plugin-logo.svg`

---

## Phase 3: User Story 1 - Install Canon Support For A Host (Priority: P1) MVP

**Goal**: Users can identify supported host package folders and installation instructions.

**Independent Test**: `cargo test --test assistant_plugin_packages package_folders_and_docs_are_present`

### Validation for User Story 1

- [x] T011 [US1] Run `cargo test --test assistant_plugin_packages package_folders_and_docs_are_present` and confirm it fails before host folders/docs exist
- [x] T012 [US1] Record install-surface decision notes in `specs/044-assistant-plugin-packages/decision-log.md`

### Implementation for User Story 1

- [x] T013 [US1] Add Claude Code package manifest and command glue in `.claude-plugin/manifest.json` and `.claude-plugin/commands.json`
- [x] T014 [US1] Add Codex plugin manifest in `.codex-plugin/plugin.json`
- [x] T015 [US1] Add Cursor package manifest and command glue in `.cursor-plugin/manifest.json` and `.cursor-plugin/commands.json`
- [x] T016 [US1] Add assistant plugin installation guide in `docs/guides/assistant-plugin-packages.md`
- [x] T017 [US1] Add README assistant plugin package summary in `README.md`
- [x] T018 [US1] Run `cargo test --test assistant_plugin_packages package_folders_and_docs_are_present` and record passing evidence in `specs/044-assistant-plugin-packages/validation-report.md`

**Checkpoint**: Host package folders and installation docs are present and independently validated.

---

## Phase 4: User Story 2 - Discover Governed Canon Capabilities Natively (Priority: P2)

**Goal**: Host packages expose Canon's governed method surfaces through metadata, commands, prompts, and skill/method references.

**Independent Test**: `cargo test --test assistant_plugin_packages manifests_expose_required_governed_methods`

### Validation for User Story 2

- [x] T019 [US2] Run `cargo test --test assistant_plugin_packages manifests_expose_required_governed_methods` and confirm required capability coverage before declaring the story complete
- [x] T020 [US2] Record capability binding decisions in `specs/044-assistant-plugin-packages/decision-log.md`

### Implementation for User Story 2

- [x] T021 [US2] Ensure `.claude-plugin/manifest.json`, `.codex-plugin/plugin.json`, and `.cursor-plugin/manifest.json` declare shared metadata, positioning, capabilities, prompts, assets, and skill/method references
- [x] T022 [US2] Ensure `.claude-plugin/commands.json`, `.cursor-plugin/commands.json`, and `assistant/commands/governed-methods.json` expose clarify input, start governed packet, inspect status, inspect evidence, review packet, verify claims, and publish packet
- [x] T023 [US2] Ensure `assistant/prompts/starter-prompts.md` and `assistant/prompts/copilot-command-pack.md` preserve Canon runtime authority boundaries
- [x] T024 [US2] Run `cargo test --test assistant_plugin_packages manifests_expose_required_governed_methods` and record passing evidence in `specs/044-assistant-plugin-packages/validation-report.md`

**Checkpoint**: Supported package manifests and command glue expose the required governed method surfaces without divergent Canon behavior.

---

## Phase 5: User Story 3 - Prevent Plugin Metadata Drift (Priority: P3)

**Goal**: Automated validation catches drift in versions, paths, fields, JSON, required commands, and prohibited positioning.

**Independent Test**: `cargo test --test assistant_plugin_packages`

### Validation for User Story 3

- [x] T025 [US3] Confirm `tests/assistant_plugin_packages.rs` includes negative validation cases for version drift, missing paths, missing required fields, missing method surfaces, invalid JSON, and prohibited positioning
- [x] T026 [US3] Run `cargo test --test assistant_plugin_packages` and confirm validation catches all negative cases

### Implementation for User Story 3

- [x] T027 [US3] Wire `scripts/validate-assistant-plugins.sh` to run the focused Rust validation test
- [x] T028 [US3] Run `bash scripts/validate-assistant-plugins.sh` and record passing evidence in `specs/044-assistant-plugin-packages/validation-report.md`
- [x] T029 [US3] Update `specs/044-assistant-plugin-packages/validation-report.md` with manual readback notes comparing package manifests/docs against the contract

**Checkpoint**: Package validation prevents metadata drift and gives maintainers a repeatable command.

---

## Final Phase: Verification & Compliance

**Purpose**: Cross-cutting validation, documentation, and closeout.

- [x] T030 Update `specs/044-assistant-plugin-packages/validation-report.md` with final invariant confirmation and implementation evidence
- [x] T031 Run `rg -n "0\\.43\\.0|agent framework|orchestrator|coding agent|workspace mutation engine" .claude-plugin .codex-plugin .cursor-plugin assistant docs/guides/assistant-plugin-packages.md README.md specs/044-assistant-plugin-packages`, resolve invalid package-positioning findings, and record intentional negative-boundary mentions
- [x] T032 Confirm all task checkboxes are complete in `specs/044-assistant-plugin-packages/tasks.md`
- [x] T033 Ensure at least 95% line coverage on every new or modified Rust file, run `cargo clippy --workspace --all-targets --all-features -- -D warnings`, ensure tests are green with `cargo test`, and use `cargo fmt --check`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Version, Governance & Artifacts (Phase 0)**: No dependencies. T001 MUST be first.
- **Validation Harness (Phase 1)**: Depends on Phase 0 completion.
- **Foundational Shared Package Material (Phase 2)**: Depends on failing validation harness.
- **User Stories (Phase 3+)**: Depend on shared package material.
- **Verification & Compliance (Final Phase)**: Depends on all user stories.

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Phase 2 and creates the install/discovery MVP.
- **User Story 2 (P2)**: Depends on the host folders from US1 and enriches them with governed method capability coverage.
- **User Story 3 (P3)**: Depends on package files and validates long-term drift prevention.

### Parallel Opportunities

- T004 can be written while validation report notes are updated.
- T009 and T010 can run in parallel after shared metadata shape is known.
- Host package manifest tasks T013, T014, and T015 touch separate folders and can be reviewed independently.

## Implementation Strategy

1. Bump the version first.
2. Write the package validation test and observe failure before adding package files.
3. Add shared metadata and assets once, then host-specific manifests that reference shared paths.
4. Add docs and README guidance after package paths stabilize.
5. Run focused validation after each story and complete with full repository validation plus coverage.
