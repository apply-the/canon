# Feature Specification: Interactive Init Experience

**Feature Branch**: `[063-interactive-init-ui]`

**Created**: 2026-06-01

**Status**: Draft

**Input**: User description: "Nuova esperienza `canon init` a schermo intero e guidata da tastiera, con modalita interattiva di default e bypass esplicito tramite `--non-interactive` per script e CI."

## Governance Context

**Execution Mode**: change

**Risk Classification**: bounded-impact because this feature changes the
default user-facing behavior of `canon init`, adds CLI-only terminal handling,
and preserves the existing engine-side initialization semantics and `.canon/`
runtime layout.

**Scope Boundaries**:

- In scope: default guided full-screen init in `canon-cli`; explicit
	`--non-interactive` bypass; guided preselection from `--ai`; structured
	output rejection in the interactive path; runtime layout-fit preflight;
	terminal restoration after success, failure, and `Ctrl+C`; validation,
	release-note, docs, site, and roadmap updates required by this feature.
- Out of scope: changes to `canon-engine` init semantics or `InitSummary`
	shape; changes to `.canon/` scaffold contents; interactive UX for commands
	other than `canon init`; a fixed documented minimum terminal size; any
	in-band cancel path other than `Ctrl+C`.

**Invariants**:

- `EngineService::init()` remains the only source of Canon runtime
	initialization side effects.
- Interactive terminal presentation logic stays inside `canon-cli`.
- `canon init --non-interactive` preserves the current argument-driven
	behavior, including assistant selection and structured output.
- Guided startup is allowed only when the current layout fits at runtime.
- Guided interruption happens only through `Ctrl+C` and must stop before init
	side effects begin.
- The terminal is restored before control returns to the shell on every guided
	exit path.

**Decision Traceability Expectations**: Clarified command-contract decisions for
terminal-size gating, structured-output rejection, `Ctrl+C` interruption, and
guided default behavior must remain recorded in this spec and its linked
planning artifacts. Any later change to those decisions must update the durable
feature packet rather than relying on chat-only context.

## Clarifications

### Session 2026-06-01

- Q: What happens when the terminal window is too small to present the branded full-screen layout? → A: The command must not open the TUI; it must exit with a clear message asking the user to resize the terminal or rerun with `--non-interactive`.
- Q: How should `--output` behave when `canon init` runs in the default interactive mode? → A: Structured output formats are supported only with `--non-interactive`; interactive invocations requesting them must fail fast with a clear error telling the user to rerun with `--non-interactive`.
- Q: How can a user abort the interactive init flow? → A: The guided flow is interrupted only with `Ctrl+C`; `Esc` is not a cancellation path.
- Q: If the terminal starts too small, should the TUI still open and rely on the user to resize it? → A: No. Insufficient terminal size remains a pre-start validation failure; the TUI must not open or wait for a later resize.
- Q: Should the spec define a fixed minimum terminal size such as `80x24` or `100x30`? → A: No. The contract uses runtime layout fit instead of a fixed numeric threshold; the TUI opens only when the current layout actually fits the available terminal space.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Guided Interactive Init (Priority: P1)

As a developer running `canon init` in a normal terminal session, I want a full-screen guided setup that lets me choose my AI assistant with the keyboard so that repository initialization feels intentional, branded, and easy to complete without memorizing flags.

**Why this priority**: This becomes the default path for human users, so it must deliver immediate value and replace the current first-run experience without extra configuration.

**Independent Test**: Can be fully tested by running `canon init` in an interactive terminal, moving through the assistant choices with the keyboard, confirming a selection, and verifying that initialization completes successfully.

**Acceptance Scenarios**:

1. **Given** an interactive terminal session and no `--non-interactive` flag, **When** the user runs `canon init`, **Then** the command opens a full-screen setup experience before initialization begins.
2. **Given** the interactive setup is open, **When** the user moves through the available assistant choices with the keyboard and presses `Enter`, **Then** the chosen assistant is used for initialization and the command completes using the existing initialization behavior.
3. **Given** the interactive setup is open, **When** the user confirms an option to proceed without assistant-specific setup, **Then** initialization completes successfully without requiring assistant materialization.

---

### User Story 2 - Script-Friendly Non-Interactive Init (Priority: P2)

As a developer or automation maintainer using scripts, CI jobs, or machine-readable output, I want to bypass the full-screen setup explicitly so that existing flag-based workflows remain predictable and parseable.

**Why this priority**: Existing automation must remain stable; otherwise the new default experience would break scripted usage and CI jobs.

**Independent Test**: Can be fully tested by running `canon init --non-interactive` with and without assistant flags, then verifying that no full-screen UI appears and the command returns the expected structured output.

**Acceptance Scenarios**:

1. **Given** a scripted or CI invocation, **When** the user runs `canon init --non-interactive`, **Then** the command skips the interactive setup and follows the current argument-driven behavior.
2. **Given** a non-interactive invocation with an explicit assistant flag and structured output option, **When** the command runs, **Then** it preserves the existing initialization outcome and output contract without rendering the full-screen interface.
3. **Given** an interactive terminal session, **When** the user requests a structured output format without `--non-interactive`, **Then** the command fails before opening the TUI and tells the user to rerun with `--non-interactive`.

---

### User Story 3 - Reliable Terminal Recovery (Priority: P3)

As a terminal user, I want the command to leave my shell in a clean, usable state after success, interruption, or failure so that I never have to reset the terminal manually.

**Why this priority**: Terminal corruption or input-state leakage would make the interactive path feel unsafe, even if the main initialization logic succeeds.

**Independent Test**: Can be fully tested by starting the interactive flow, then completing it, interrupting it with `Ctrl+C`, and forcing an initialization failure while confirming that the shell prompt, cursor, and keyboard behavior are restored each time.

**Acceptance Scenarios**:

1. **Given** the interactive setup is open, **When** the user interrupts the command with `Ctrl+C`, **Then** the command exits without starting initialization and the terminal is restored to a normal prompt.
2. **Given** the interactive setup has already collected a selection, **When** initialization later fails, **Then** the terminal is still restored before the failure is reported.
3. **Given** the terminal is interactive but too small for the current supported full-screen layout, **When** the user runs `canon init`, **Then** the command exits before opening the TUI or starting initialization, with a clear instruction to resize the terminal or rerun with `--non-interactive`.

### Edge Cases

- When `canon init` runs in an environment without interactive terminal capabilities and the user did not pass `--non-interactive`, the command falls back to the non-interactive path.
- When the user provides `--ai ...` without `--non-interactive`, the command keeps the guided experience and uses the flag value as the initial assisted selection.
- When the terminal window is too small for the current supported full-screen layout, the command exits without opening the TUI or waiting for a later resize and tells the user to resize the terminal or rerun with `--non-interactive`.
- When a user requests `json`, `yaml`, or `markdown` output without `--non-interactive`, the command fails before opening the TUI and tells the user to rerun with `--non-interactive`.
- When interactive terminal capabilities are unavailable and the user requests `json`, `yaml`, or `markdown` output without `--non-interactive`, the command still rejects the request and tells the user to rerun with `--non-interactive`.
- When the user interrupts with `Ctrl+C` before any repository initialization work starts, the command restores the terminal and exits without creating init side effects.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST launch `canon init` in a guided full-screen terminal experience when it runs in an interactive terminal and the user has not provided `--non-interactive`.
- **FR-002**: System MUST present the available assistant choices in a branded setup screen that includes a visible title or logo region, a highlighted current selection state, and on-screen keyboard instructions.
- **FR-003**: System MUST allow users to move the current selection with the up and down arrow keys and confirm the selection with `Enter`.
- **FR-004**: System MUST NOT treat `Esc` as a cancellation path for the guided setup.
- **FR-005**: System MUST allow users to continue initialization either with a selected assistant or without assistant-specific setup.
- **FR-006**: System MUST invoke the existing initialization behavior only after the guided setup has resolved the user choice.
- **FR-007**: System MUST bypass the full-screen setup whenever the user supplies `--non-interactive` and preserve the current argument-based behavior, including assistant flags and requested output format.
- **FR-008**: System MUST fall back to non-interactive behavior when interactive terminal capabilities are unavailable.
- **FR-009**: System MUST treat any assistant flag supplied without `--non-interactive` as an initial selection for the guided setup rather than as a reason to skip the guided flow.
- **FR-010**: System MUST restore the terminal to a usable state after successful completion, user interruption via `Ctrl+C`, and failure of the guided setup path.
- **FR-011**: System MUST keep repository initialization outcomes consistent between the guided path and the non-interactive path for the same resolved assistant choice.
- **FR-012**: When the terminal is interactive but too small to render the current guided layout, system MUST fail the guided path before opening the TUI or starting initialization, with a clear message instructing the user to resize the terminal or rerun with `--non-interactive`.
- **FR-013**: Structured output formats for `canon init` MUST be supported only with `--non-interactive`; when a user requests `json`, `yaml`, or `markdown` output without `--non-interactive`, including environments that would otherwise fall back because interactive terminal capabilities are unavailable, system MUST fail before initialization begins and instruct the user to rerun with `--non-interactive`.
- **FR-014**: System MUST handle `Ctrl+C` as the only user-driven interruption path for the guided setup and stop before initialization side effects begin.
- **FR-015**: System MUST determine whether interactive startup is allowed by checking whether the current guided layout fits the available terminal space at runtime, not by relying on a fixed documented minimum size.

### Key Entities *(include if feature involves data)*

- **Initialization Session**: A single `canon init` invocation, including whether it runs in guided or non-interactive mode, the resolved assistant choice, and the final outcome.
- **Assistant Choice**: A user-selectable initialization preference representing one supported assistant option or the decision to continue without assistant-specific setup.
- **Terminal Presentation State**: The temporary full-screen interaction state needed to render the setup interface, track the highlighted choice, and restore the shell environment afterward.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In a documented interactive usability walkthrough recorded in `specs/063-interactive-init-ui/validation-report.md`, at least 9 of 10 first-attempt operator runs from clean temporary workspaces complete `canon init` successfully using only the on-screen guidance and without consulting external documentation.
- **SC-002**: In validation across supported terminal environments, 100% of successful, interrupted, and failed guided runs return the shell to a usable prompt without requiring a manual reset.
- **SC-003**: In regression validation, all existing scripted `canon init --non-interactive` scenarios continue to complete without opening the guided interface.
- **SC-004**: In keyboard-only validation, users can reach any assistant choice and confirm it in no more than 10 keypresses from the default starting state.

## Assumptions

- The current repository initialization logic remains the source of truth for filesystem changes and assistant-specific materialization.
- Interactive mode targets human-operated terminal sessions, while automation and CI flows will explicitly opt into `--non-interactive` or naturally run without interactive terminal capabilities.
- When an assistant flag is provided in the default guided flow, the interface starts with that choice preselected instead of bypassing the setup experience.
- The guided flow continues to support repository initialization even when the user decides not to materialize assistant-specific setup.
- Users abort the guided flow only by interrupting the command with `Ctrl+C`, not through an in-band cancel key inside the TUI.
- Terminal-size readiness is derived from whether the active branded layout fits at runtime, rather than from a fixed published width-by-height threshold.