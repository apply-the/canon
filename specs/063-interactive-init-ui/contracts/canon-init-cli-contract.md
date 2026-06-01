# Contract: `canon init`

## Command Surface

```text
canon init [--ai <codex|copilot|claude>] [--non-interactive] [--output <text|json|yaml|markdown>]
```

## Inputs

- `--ai <codex|copilot|claude>`
  - Optional assistant target.
  - In guided mode, this value preselects the matching assistant in the UI.
  - In non-interactive mode, this value is passed directly to the existing init
    backend.
- `--non-interactive`
  - Forces the existing argument-driven init path.
  - Required when the caller needs machine-readable output.
- `--output <text|json|yaml|markdown>`
  - Defaults to `text`.
  - `json`, `yaml`, and `markdown` are valid only with `--non-interactive`.

## Behavioral Matrix

| Inputs and environment | Expected behavior |
|------------------------|-------------------|
| Default invocation in a supported interactive terminal with fitting layout | Launch the guided full-screen init UI |
| Default invocation with `--ai <value>` in a supported interactive terminal | Launch the guided UI with that assistant preselected |
| `--non-interactive` with no `--ai` | Run the existing init backend with no assistant selection |
| `--non-interactive --ai <value>` | Run the existing init backend with that assistant |
| Default invocation in a shell without required interactive terminal capability | Fall back to the existing non-interactive init flow |
| Default invocation with `--output json`, `yaml`, or `markdown` | Reject before guided init starts and instruct the caller to use `--non-interactive` |
| Default invocation in an interactive terminal whose current layout does not fit the branded UI | Reject before guided init starts; do not open the TUI |

## Guided Interaction Rules

- The guided UI is the default experience only when the terminal supports the
  required interactive capabilities and the current layout fits the branded
  screen.
- Arrow keys move the active assistant selection.
- The first `Enter` confirms the current selection and advances to the
  confirmation step.
- `Enter` on the confirmation step starts initialization.
- Arrow keys on the confirmation step return the flow to selection mode with
  the new highlighted assistant.
- `Ctrl+C` is the only user-driven interruption path.
- `Esc` does not cancel the flow.
- The guided path must restore the terminal before returning control to the
  shell.

## Rejection Rules

- Structured output without `--non-interactive` is rejected.
- Too-small terminal layouts are rejected before entering the alternate screen.
- Guided mode must not silently degrade to a broken or truncated layout.
- Rejection before backend initialization must not create `.canon/` side
  effects in the target workspace.

## Output Contract

- Successful non-interactive execution returns the existing `InitSummary`
  formatted through the selected output serializer.
- Successful guided execution returns the existing init success result after the
  TUI exits and the terminal is restored.
- Guided-mode validation failures and terminal-size rejections produce a
  human-readable CLI error instead of structured output.
- Guided-mode interruption via `Ctrl+C` exits only after terminal restoration.

## Backend Boundary

- `EngineService::init()` remains the only backend operation that performs
  Canon runtime initialization.
- The guided UI may collect or confirm an assistant selection, but it must not
  duplicate runtime materialization behavior.
- The CLI contract does not change the shape of `InitSummary`.