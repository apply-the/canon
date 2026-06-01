# Data Model: Interactive Init Experience

## Overview

This feature does not add a new persisted runtime schema. The relevant models
describe command inputs, interactive-session state, terminal readiness, and the
existing init backend handoff.

## Entity: InitInvocation

- Purpose: Represents one `canon init` command request before the CLI chooses
  the guided or non-interactive path.
- Fields:
  - `requested_mode`: `guided-default` or `non-interactive`
  - `requested_output`: `text`, `json`, `yaml`, or `markdown`
  - `requested_ai`: optional assistant target from the existing `AiTarget`
    catalog
  - `interactive_terminal_available`: boolean capability signal from the
    current terminal environment
  - `layout_fit`: `unknown`, `fits`, or `does-not-fit`
  - `effective_path`: `guided`, `non-interactive`, or `rejected`
  - `rejection_reason`: optional validation reason when the command cannot
    proceed
- Validation rules:
  - Structured output requires `requested_mode = non-interactive`.
  - Guided startup requires an interactive terminal.
  - Guided startup requires `layout_fit = fits` before the TUI begins.
  - `requested_ai`, when present, must map to an existing `AiTarget` value.

## Entity: AssistantSelection

- Purpose: Represents the assistant choice handed to `EngineService::init()`.
- Fields:
  - `catalog_value`: `codex`, `copilot`, `claude`, or `none`
  - `selection_source`: `default-none`, `cli-preselection`, or
    `guided-confirmation`
  - `confirmed`: boolean indicating whether the guided flow has confirmed the
    final choice
- Validation rules:
  - `catalog_value` must remain aligned with the existing engine-side `AiTool`
    enum.
  - `selection_source = cli-preselection` is allowed only when `requested_ai`
    was provided on the command line.
  - `confirmed = true` is required before the guided flow may start
    initialization.

## Entity: GuidedInitSession

- Purpose: Tracks ephemeral state for the full-screen wizard inside
  `canon-cli`.
- Fields:
  - `highlighted_choice`: current assistant row under the cursor
  - `pending_selection`: candidate `AssistantSelection`
  - `status`: `preflight`, `selecting`, `confirming`, `initializing`,
    `completed`, `interrupted`, or `failed`
  - `status_message`: optional informational or error message for the UI
  - `terminal_dimensions`: current width and height used for layout-fit
    evaluation
- Validation rules:
  - The session may enter `selecting` only after terminal capability and
    layout-fit checks pass.
  - The session may enter `initializing` only after a confirmed assistant
    selection exists.
  - `interrupted` is reachable only through `Ctrl+C`.
  - `completed` requires a successful `EngineService::init()` result.

## Entity: TerminalReadiness

- Purpose: Captures the checks that determine whether guided init may start.
- Fields:
  - `stdin_is_tty`: boolean
  - `stdout_is_tty`: boolean
  - `alternate_screen_supported`: boolean or best-effort capability result
  - `raw_mode_supported`: boolean or best-effort capability result
  - `layout_fit`: `fits` or `does-not-fit`
  - `blocking_reason`: optional enum-like reason such as `non-interactive`,
    `structured-output-requested`, or `layout-too-small`
- Validation rules:
  - Guided mode requires all terminal capability checks to pass.
  - `blocking_reason = layout-too-small` must stop execution before entering
    the alternate screen.
  - `blocking_reason = structured-output-requested` must not silently fall back
    to the guided text flow.

## Entity: InitExecutionOutcome

- Purpose: Records how the CLI exits after routing and, if applicable, after
  calling the existing init backend.
- Fields:
  - `result_kind`: `completed`, `rejected`, `interrupted`, or `failed`
  - `summary_present`: boolean indicating whether an `InitSummary` was
    produced
  - `terminal_restored`: boolean
  - `side_effects_started`: boolean indicating whether `EngineService::init()`
    was invoked
- Validation rules:
  - `summary_present = true` only when `result_kind = completed`.
  - `side_effects_started = false` is required for preflight rejection and
    guided-path interruption before initialization.
  - `terminal_restored = true` is required for every guided-path outcome.

## Relationships

- One `InitInvocation` may produce one `GuidedInitSession` when the effective
  path is guided.
- One `InitInvocation` resolves exactly one `AssistantSelection`, either from
  CLI input or from guided confirmation.
- One `InitInvocation` produces one `InitExecutionOutcome`.
- One successful `InitExecutionOutcome` wraps the existing engine-generated
  `InitSummary` without altering its structure.

## State Transitions

```text
InitInvocation
  -> rejected                     when structured output is requested without --non-interactive
  -> non-interactive              when --non-interactive is set
  -> non-interactive              when interactive terminal capability is unavailable
  -> guided preflight             when default path remains eligible

GuidedInitSession
  preflight -> selecting          when terminal readiness passes
  preflight -> failed             when layout fit fails
  selecting -> confirming         when the user chooses Enter on a valid option
  selecting -> interrupted        when Ctrl+C is received
  confirming -> selecting         when the user goes back to change the choice
  confirming -> initializing      when the user confirms the selection
  confirming -> interrupted       when Ctrl+C is received
  initializing -> completed       when EngineService::init() succeeds
  initializing -> failed          when EngineService::init() returns an error
```