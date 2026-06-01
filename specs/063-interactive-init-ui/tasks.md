---

description: "Task list for implementing the interactive init experience"

---

# Tasks: Interactive Init Experience

**Input**: Design documents from `/specs/063-interactive-init-ui/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/canon-init-cli-contract.md, quickstart.md

**Tests**: Included because the specification requires independent validation for each user story and the plan calls for targeted unit, integration, and quickstart coverage.

**Organization**: Tasks are grouped by user story so each slice can be implemented and validated independently once the shared foundation is in place.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Add the dependencies, module entrypoints, and test harness files required by the feature.

- [X] T001 Add `ratatui` and `crossterm` workspace dependencies in Cargo.toml and crates/canon-cli/Cargo.toml
- [X] T002 [P] Create guided-init module scaffolding in crates/canon-cli/src/tui/mod.rs, crates/canon-cli/src/tui/terminal.rs, crates/canon-cli/src/tui/init.rs, and crates/canon-cli/src/tui/render.rs
- [X] T003 [P] Add integration harness entrypoints in tests/init_guided_contract.rs, tests/init_non_interactive_contract.rs, and tests/init_terminal_recovery.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish the shared command routing, typed session models, render entrypoints, and terminal preflight primitives used by every story.

**⚠️ CRITICAL**: No user story work should start before this phase is complete.

- [X] T004 Extend init CLI parsing for `--non-interactive` and guided preselection in crates/canon-cli/src/app.rs
- [X] T005 [P] Introduce typed init invocation and session models in crates/canon-cli/src/commands/init.rs and crates/canon-cli/src/tui/init.rs
- [X] T006 [P] Implement terminal capability and layout-fit preflight primitives in crates/canon-cli/src/tui/terminal.rs
- [X] T007 [P] Add branded render entrypoints and reusable layout helpers in crates/canon-cli/src/tui/render.rs and crates/canon-cli/src/tui/mod.rs
- [X] T008 Create shared guided-init error propagation and module wiring in crates/canon-cli/src/main.rs and crates/canon-cli/src/commands.rs

**Checkpoint**: Shared init routing and TUI infrastructure are ready; user story work can proceed.

---

## Phase 3: User Story 1 - Guided Interactive Init (Priority: P1) 🎯 MVP

**Goal**: Make `canon init` open a branded full-screen assistant chooser by default in supported interactive terminals.

**Independent Test**: Run `canon init` in an interactive terminal, move through the assistant choices with the keyboard, confirm a selection or the no-assistant path, and verify initialization completes successfully.

### Tests for User Story 1

- [X] T009 [P] [US1] Add session-state unit tests for keyboard navigation, ignored `Esc` input, assistant confirmation, and no-assistant selection in crates/canon-cli/src/tui/init.rs
- [X] T010 [P] [US1] Add guided-init integration coverage for default launch, `--ai` preselection, and the 10-keypress reachability budget in tests/init_guided_contract.rs and tests/integration/init_guided_contract.rs

### Implementation for User Story 1

- [X] T011 [P] [US1] Implement assistant selection state transitions, ignored `Esc` handling, and confirm flow in crates/canon-cli/src/tui/init.rs
- [X] T012 [P] [US1] Implement branded full-screen assistant chooser and keyboard instruction rendering in crates/canon-cli/src/tui/render.rs
- [X] T013 [US1] Route default `canon init` through the guided event loop in crates/canon-cli/src/commands/init.rs and crates/canon-cli/src/tui/mod.rs
- [X] T014 [US1] Preserve engine-backed initialization handoff for confirmed assistant and no-assistant choices in crates/canon-cli/src/commands/init.rs

**Checkpoint**: The default interactive init path is functional and testable on its own.

---

## Phase 4: User Story 2 - Script-Friendly Non-Interactive Init (Priority: P2)

**Goal**: Preserve script-safe behavior, assistant flags, and structured output behind explicit non-interactive routing.

**Independent Test**: Run `canon init --non-interactive` with and without `--ai`, plus a structured output format, and verify no full-screen UI appears and the expected summary is returned.

### Tests for User Story 2

- [X] T015 [P] [US2] Update non-interactive regression coverage in tests/init_creates_canon.rs and tests/integration/init_creates_canon.rs
- [X] T016 [P] [US2] Add non-interactive contract coverage for assistant flags, structured output, no-TTY fallback, and no-TTY structured-output rejection without `--non-interactive` in tests/init_non_interactive_contract.rs and tests/integration/init_non_interactive_contract.rs

### Implementation for User Story 2

- [X] T017 [P] [US2] Implement explicit `--non-interactive` routing and no-TTY fallback in crates/canon-cli/src/commands/init.rs while preserving the explicit structured-output gate
- [X] T018 [US2] Preserve existing summary serialization for the non-interactive path in crates/canon-cli/src/commands/init.rs and crates/canon-cli/src/output.rs
- [X] T019 [US2] Fail fast on structured output requests without `--non-interactive` in interactive and no-TTY fallback scenarios in crates/canon-cli/src/app.rs and crates/canon-cli/src/commands/init.rs

**Checkpoint**: Scripted init, CI usage, and structured output remain independently functional.

---

## Phase 5: User Story 3 - Reliable Terminal Recovery (Priority: P3)

**Goal**: Guarantee terminal restoration and clear failure behavior after success, interruption, and guided-path errors.

**Independent Test**: Start the interactive flow, complete it successfully, interrupt it with `Ctrl+C`, and force a guided-path failure while verifying the shell prompt, cursor, and keyboard behavior are restored each time.

### Tests for User Story 3

- [X] T020 [P] [US3] Add terminal lifecycle unit tests for restore-on-drop and preflight rejection paths in crates/canon-cli/src/tui/terminal.rs
- [X] T021 [P] [US3] Add terminal recovery integration coverage for `Ctrl+C`, init failure, and too-small layout rejection in tests/init_terminal_recovery.rs and tests/integration/init_terminal_recovery.rs

### Implementation for User Story 3

- [X] T022 [US3] Implement terminal restore guards for success, failure, and interruption in crates/canon-cli/src/tui/terminal.rs and crates/canon-cli/src/commands/init.rs
- [X] T023 [US3] Implement `Ctrl+C` interruption handling before init side effects in crates/canon-cli/src/tui/init.rs and crates/canon-cli/src/commands/init.rs
- [X] T024 [US3] Implement too-small-layout rejection and post-teardown error reporting in crates/canon-cli/src/tui/terminal.rs, crates/canon-cli/src/tui/render.rs, and crates/canon-cli/src/main.rs

**Checkpoint**: Guided init now leaves the terminal in a usable state across the success, failure, and interruption paths.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finalize release metadata, update documentation surfaces, capture validation evidence, and close out independent review for the shipped slice.

- [X] T025 [P] Bump the workspace release version and release notes in Cargo.toml and CHANGELOG.md
- [X] T026 [P] Update operator-facing docs, site, and roadmap content in README.md, docs/guides/getting-started.md, site/guide/getting-started.md, site/roadmap/index.md, and ROADMAP.md
- [X] T027 [P] Align feature quickstart and CLI contract wording with the shipped behavior in specs/063-interactive-init-ui/quickstart.md and specs/063-interactive-init-ui/contracts/canon-init-cli-contract.md
- [X] T028 Run `cargo fmt`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, targeted `cargo test`, and patch coverage validation via scripts/common/coverage/intersect_patch_coverage.py against lcov.info so every changed Rust source file, including crates/canon-cli/src/app.rs, crates/canon-cli/src/commands/init.rs, and crates/canon-cli/src/tui/terminal.rs, finishes above 95% coverage
- [X] T029 Capture validation evidence in specs/063-interactive-init-ui/validation-report.md, including 10 first-attempt usability runs from clean temporary workspaces, pass or fail counts against the 9-of-10 success target, whether any external documentation was consulted, quickstart walkthrough notes, terminal-recovery checks, non-interactive regression results, 10-keypress reachability results, repo quality-gate outcomes, and changed-Rust-file coverage evidence
- [X] T030 Perform independent review of the CLI contract, terminal cleanup, dependency additions from Cargo.toml and crates/canon-cli/Cargo.toml, release-surface docs, and recorded validation evidence in specs/063-interactive-init-ui/validation-report.md, including verification of the SC-001 10-run usability protocol, any discovered non-interactive contract deviations, and final outcome recording

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies; start immediately.
- **Phase 2: Foundational**: Depends on Phase 1 and blocks all story work.
- **Phase 3: User Story 1**: Depends on Phase 2; delivers the MVP guided path.
- **Phase 4: User Story 2**: Depends on Phase 2; can proceed in parallel with User Story 1 once the foundation is complete.
- **Phase 5: User Story 3**: Depends on Phase 2 and the guided-path scaffolding from User Story 1.
- **Phase 6: Polish**: Depends on the completion of all desired user stories.

### User Story Dependencies

- **User Story 1 (P1)**: No dependency on other user stories after Phase 2 completes.
- **User Story 2 (P2)**: No dependency on other user stories after Phase 2 completes.
- **User Story 3 (P3)**: Extends the guided execution lifecycle introduced by User Story 1, so it should start after User Story 1 has landed or stabilized.

### Within Each User Story

- Write the listed tests before finishing implementation.
- Establish state and model behavior before command routing.
- Finish command routing before documentation or cross-cutting cleanup.
- Validate the story independently before moving to the next priority.

---

## Parallel Opportunities

- **Setup**: T002 and T003 can run in parallel after T001.
- **Foundational**: T005, T006, and T007 can run in parallel after T004.
- **User Story 1**: T009 and T010 can run in parallel; T011 and T012 can run in parallel once the tests exist.
- **User Story 2**: T015 and T016 can run in parallel; T017 can proceed while T015/T016 are under review.
- **User Story 3**: T020 and T021 can run in parallel; T022 and T023 can proceed in tandem once terminal primitives are in place.
- **Polish**: T025, T026, and T027 can run in parallel before T028; T029 follows T028; T030 follows T029.

---

## Parallel Example: User Story 1

```bash
# Launch both User Story 1 test tasks together:
Task: "T009 Add session-state unit tests in crates/canon-cli/src/tui/init.rs"
Task: "T010 Add guided-init integration coverage in tests/init_guided_contract.rs and tests/integration/init_guided_contract.rs"

# Implement the guided experience in parallel after tests exist:
Task: "T011 Implement assistant selection state transitions in crates/canon-cli/src/tui/init.rs"
Task: "T012 Implement branded full-screen rendering in crates/canon-cli/src/tui/render.rs"
```

## Parallel Example: User Story 2

```bash
# Expand script-safe validation in parallel:
Task: "T015 Update regression coverage in tests/init_creates_canon.rs and tests/integration/init_creates_canon.rs"
Task: "T016 Add non-interactive contract coverage in tests/init_non_interactive_contract.rs and tests/integration/init_non_interactive_contract.rs"
```

## Parallel Example: User Story 3

```bash
# Validate terminal recovery behavior from both angles:
Task: "T020 Add terminal lifecycle unit tests in crates/canon-cli/src/tui/terminal.rs"
Task: "T021 Add terminal recovery integration coverage in tests/init_terminal_recovery.rs and tests/integration/init_terminal_recovery.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: Foundational.
3. Complete Phase 3: User Story 1.
4. Validate the guided path independently before widening scope.

### Incremental Delivery

1. Finish Setup and Foundational work to stabilize the shared CLI/TUI surface.
2. Deliver User Story 1 as the MVP guided init path.
3. Add User Story 2 to preserve script-safe and machine-readable behavior.
4. Add User Story 3 to harden interruption, layout-fit rejection, and terminal restoration.
5. Finish with documentation, repo quality gates, validation evidence, and independent review.

### Parallel Team Strategy

1. One developer handles dependencies and module scaffolding in Phase 1 while another prepares the new integration harness files.
2. After Phase 2, one developer can own the guided path (User Story 1) while another preserves non-interactive behavior (User Story 2).
3. Once guided lifecycle primitives are stable, terminal recovery hardening (User Story 3) can proceed without blocking documentation updates.

---

## Notes

- `[P]` marks tasks that touch different files and can proceed without waiting on another in-progress task.
- `[US1]`, `[US2]`, and `[US3]` map tasks back to the prioritized user stories in spec.md.
- Keep `canon-engine` behavior unchanged unless implementation evidence proves a CLI-only solution is impossible.
- Run manual walkthrough steps from specs/063-interactive-init-ui/quickstart.md before closing the feature, and record closeout evidence in specs/063-interactive-init-ui/validation-report.md.