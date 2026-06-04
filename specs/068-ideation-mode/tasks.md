# Implementation Tasks: Brainstorming Ideation Mode

## Phase 0: Governance & Artifacts

- [X] T001 Define and record execution mode (`brainstorming`), risk (Green), and scope in `01-context.md` (or equivalent execution log).
- [X] T002 Ensure validation ownership is established for structural/logical validation before coding.
- [X] T003 Update `.specify/memory/constitution.md` (if required) or confirm adherence to existing rules.

## Phase 1: Setup

- [X] T004 Bump version to `0.65.0` in all relevant `Cargo.toml` files.
- [X] T005 [P] Update `README.md` (repo root) to mention the new `brainstorming` mode.
- [X] T006 [P] Update `CHANGELOG.md` (repo root) for version `0.65.0` with the ideation feature.
- [X] T007 [P] Remove `04-brainstorming-ideation.md` from `roadmap/features/` and update any roadmap indexes.
- [X] T008 [P] Update `tech-docs/` and `docs/` directories (repo root) to include a guide on how to use the `brainstorming` mode.

## Phase 2: Foundational

- [ ] T009 Create `crates/canon-engine/src/artifacts/option_map.rs` defining the models: `OptionMap`, `ConceptualApproach`, `TradeOffMatrix`, and `SpikeProposal` using typed serde models.
- [ ] T010 Add `crates/canon-engine/src/modes/brainstorming.rs` with the skeleton for the `brainstorming` mode logic.
- [ ] T011 Register the `brainstorming` mode in the engine's mode registry or dispatcher.

## Phase 3: User Story 1 - Explore Multiple Ideas (P1)

*Goal*: Users can run the CLI to explore ideas and get an option map with at least 3 distinct conceptual approaches and trade-off matrices.
*Independent Test*: Run `canon-cli brainstorm` with a mock prompt and verify `01-context.md`, `02-options.md`, and `03-tradeoffs.md` are correctly generated.

- [ ] T012 [US1] Implement CLI command `brainstorm` in `crates/canon-cli/src/commands/brainstorm.rs` using `clap`.
- [ ] T013 [US1] Link the CLI command to the engine's `brainstorming.rs` mode runner.
- [ ] T014 [US1] Implement divergence logic in `brainstorming.rs` to generate 3 distinct approaches.
- [ ] T015 [US1] Implement trade-off matrix generation (Pros, Cons, Unknowns) for each approach.
- [ ] T016 [US1] Output `01-context.md`, `02-options.md`, and `03-tradeoffs.md` securely without generating code.
- [ ] T016b [US1] Implement next mode recommendation logic (e.g., recommending `discovery` or `architecture` mode based on unknowns).
- [ ] T017 [US1] Write unit tests for `brainstorming.rs` mode execution (ensuring >95% coverage).

## Phase 4: User Story 2 - Propose Validation Spikes (P2)

*Goal*: When critical unknowns exist, the agent suggests bounded experiments.
*Independent Test*: Verify that if unknowns are present in trade-offs, a `05-spikes.md` and `04-open-questions.md` are correctly emitted.

- [ ] T018 [US2] Implement spike proposal logic in `brainstorming.rs` based on detected unknowns.
- [ ] T019 [US2] Output `04-open-questions.md` and `05-spikes.md`.
- [ ] T020 [US2] Write unit tests for the spike proposal generation logic (ensuring >95% coverage).
- [ ] T021 [US2] Write integration tests in `tests/integration/brainstorm_test.rs` using `assert_cmd` to validate the full workflow.
- [ ] T021b [US2] Write an integration test to ensure downstream `discovery` or `architecture` runs can correctly cite the output option map (SC-003).

## Phase 5: Verification & Compliance

- [ ] T022 Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and fix any issues.
- [ ] T023 Run `cargo fmt --check` to verify code style.
- [ ] T024 Run `cargo llvm-cov` (or equivalent) to ensure coverage on newly created/modified rust files is >95%.
- [ ] T025 Review AGENTS.md rules (no magic strings, extract helpers, typed serde) and ensure all code complies.
- [ ] T026 Final structural and logical validation of the produced packets.
