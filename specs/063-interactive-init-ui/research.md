# Research: Interactive Init Experience

## Decision 1: Keep the interactive UI entirely inside `canon-cli`

- Decision: Implement the guided `canon init` flow in `canon-cli` and keep
  `canon-engine` limited to the existing `EngineService::init()` backend.
- Rationale: The feature changes presentation, keyboard handling, terminal
  lifecycle, and CLI routing, not runtime materialization semantics.
  Preserving the current engine boundary keeps `.canon/` side effects,
  assistant-specific scaffolding, and `InitSummary` generation centralized in
  one place.
- Alternatives considered:
  - Move the TUI into `canon-engine`: rejected because terminal rendering and
    event handling are CLI concerns and would leak presentation into the core.
  - Duplicate init materialization logic in `canon-cli`: rejected because it
    would split the source of truth for runtime initialization.

## Decision 2: Use `ratatui` with the `crossterm` backend for the guided flow

- Decision: Build the full-screen init wizard with `ratatui` and `crossterm`
  rather than prompt-style crates.
- Rationale: The clarified feature requires branded ASCII presentation,
  bordered layout regions, colored text spans, arrow-key navigation,
  controlled redraw, and reliable terminal restoration. `ratatui` with
  `crossterm` matches those needs directly and stays compatible with the
  repository's Rust CLI stack.
- Alternatives considered:
  - `inquire` or `dialoguer`: rejected because they are optimized for prompt
    widgets, not a branded full-screen layout.
  - Raw `crossterm` only: rejected because it would require more custom layout
    and rendering code than the feature needs.

## Decision 3: Use explicit terminal preflight and restore guards

- Decision: Route guided init through a terminal preflight step and a dedicated
  terminal lifecycle guard that enters raw mode and the alternate screen only
  after validation passes, then restores the terminal on every exit path.
- Rationale: The user-facing contract requires reliable cleanup on success,
  failure, and `Ctrl+C`, plus rejection before any broken or truncated layout
  is shown. A dedicated guard keeps cleanup separate from business logic and
  avoids leaving the shell in raw mode when the command exits unexpectedly.
- Alternatives considered:
  - Inline terminal setup and teardown directly inside the command function:
    rejected because error and interruption handling become brittle.
  - Enter the TUI before checking layout fit: rejected because the spec says
    too-small terminals must block before the TUI starts.

## Decision 4: Treat guided and non-interactive init as two explicit command paths

- Decision: Make guided init the default path for supported interactive
  terminals, and preserve the current argument-driven behavior only when
  `--non-interactive` is set or when the environment cannot support the guided
  path.
- Rationale: This keeps the human-first default without breaking scripting and
  automation. It also gives the CLI one clear switch for structured output and
  one clear branch for existing machine-oriented workflows.
- Alternatives considered:
  - Keep the current non-interactive behavior as the default and add a new
    `--interactive` flag: rejected because it conflicts with the clarified spec.
  - Auto-detect intent from `--output` without a dedicated flag: rejected
    because it blurs the public contract and makes automation harder to reason
    about.

## Decision 5: Reject incompatible guided-mode requests instead of degrading them silently

- Decision: Reject guided init when a structured output format is requested,
  and reject too-small terminals before the TUI opens. Only missing interactive
  terminal capability may fall back to the existing non-interactive flow.
- Rationale: The clarified requirements explicitly allow fallback for missing
  interactive capabilities, but they require structured output to stay tied to
  `--non-interactive` and require undersized terminals to fail rather than
  render a broken UI. This keeps the command contract predictable.
- Alternatives considered:
  - Emit structured output after the guided flow completes: rejected because it
    mixes human and machine contracts in one execution mode.
  - Open the TUI in a degraded layout and wait for the terminal to resize:
    rejected because the spec requires a clean preflight failure.

## Decision 6: Separate rendering, session state, and event input for testability

- Decision: Model guided init as a small session state machine with separate
  rendering helpers and an abstracted event source.
- Rationale: The feature needs focused tests for assistant preselection,
  selection changes, confirm flows, preflight failures, and interruption-safe
  cleanup. A split between state, render, and input handling makes those checks
  possible without relying only on manual terminal runs.
- Alternatives considered:
  - Drive all logic from a blocking event loop with direct terminal reads:
    rejected because it would be difficult to unit-test and harder to keep
    deterministic.
  - Test only through end-to-end manual runs: rejected because regression risk
    is too high for a default CLI behavior change.