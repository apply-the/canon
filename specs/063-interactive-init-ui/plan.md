# Implementation Plan: Interactive Init Experience

**Branch**: `063-interactive-init-ui` | **Date**: 2026-06-01 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/063-interactive-init-ui/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Introduce a default guided `canon init` experience in `canon-cli` using a
full-screen `ratatui` interface backed by `crossterm`, while preserving the
existing argument-driven behavior behind `--non-interactive`. The design keeps
`EngineService::init()` as the only source of initialization side effects,
adds CLI-only preflight gates for interactive terminal support, runtime
layout-fit, and structured-output rejection, and validates the new flow with
targeted unit, integration, and command-contract coverage.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact because this slice changes the default
user-facing behavior of one existing CLI command, adds terminal-state handling
and new dependencies in `canon-cli`, and extends command-contract validation
without changing engine persistence, approval semantics, publish behavior, or
the `.canon/` runtime layout.
**Scope In**: default guided init flow for interactive terminals; explicit
`--non-interactive` contract; assistant preselection from `--ai`; runtime
layout-fit preflight; structured-output rejection in interactive mode;
terminal restoration on success, `Ctrl+C`, and failure; additive `ratatui` and
`crossterm` dependencies; CLI-focused tests and feature documentation.
**Scope Out**: changes to `canon-engine` init semantics or `InitSummary`
fields; changes to `.canon/` scaffold contents; new assistant kinds beyond the
current `AiTool` catalog; interactive UX for any command other than `canon
init`; publish, governance, or run-execution behavior outside init.

**Invariants**:

- `EngineService::init()` remains the source of truth for Canon runtime-state
  creation and summary generation.
- All interactive presentation logic stays inside `canon-cli`; no TUI state,
  layout, or terminal-control code moves into `canon-engine`.
- `canon init --non-interactive` preserves the current argument-based flow,
  including assistant selection and output formatting.
- The guided path opens only when the terminal is interactive, the requested
  output remains text-compatible, and the current branded layout fits the
  available terminal space at runtime.
- The terminal is restored on success, interruption via `Ctrl+C`, and failure
  before control returns to the shell.
- Preflight rejections or interruptions before `service.init()` begins do not
  create `.canon/` side effects in the target workspace.

**Decision Log**: `specs/063-interactive-init-ui/spec.md#clarifications`
**Validation Ownership**: Generation is performed through this plan and the
derived design artifacts; validation is performed through targeted CLI unit and
integration tests, contract checks, a 10-run first-attempt interactive
usability walkthrough from clean temporary workspaces without external
documentation, `cargo fmt --check`, and `cargo clippy --workspace
--all-targets --all-features -- -D warnings`, with evidence and independent
review findings recorded in `specs/063-interactive-init-ui/validation-report.md`.
**Approval Gates**: Independent review of the CLI contract, terminal cleanup,
and dependency addition before merge; explicit review of any deviation from the
non-interactive init contract discovered during implementation.
**Release Closeout Scope**: Implementation closeout must cover the guided init
path, non-interactive regression behavior, terminal cleanup, and temp-workspace
operator verification; documentation closeout must update interactive-init
guidance and operator expectations where `canon init` is described; validation
evidence and independent review findings must close out in
`specs/063-interactive-init-ui/validation-report.md`.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024; Markdown planning artifacts
**Primary Dependencies**: workspace crates `canon-cli` and `canon-engine`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, and `tracing`; planned `ratatui` plus `crossterm` for the full-screen init UI; existing `assert_cmd` and `tempfile` for CLI validation
**Storage**: `.canon/` runtime scaffolding in the target workspace; planning and contract artifacts under `specs/063-interactive-init-ui/`
**Testing**: `cargo test`; targeted unit tests in `crates/canon-cli`; integration coverage in `tests/integration/init_creates_canon.rs` plus new guided-init contract tests; manual temp-workspace walkthroughs for interactive behavior; `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`
**Target Platform**: Canon CLI on macOS, Linux, and Windows terminals; non-interactive CI shells that must keep using the arg-driven path
**Project Type**: Rust CLI workspace with local filesystem runtime scaffolding and markdown governance artifacts
**Existing System Touchpoints**:
  - `crates/canon-cli/src/app.rs` for `Command::Init`, `AiTarget`, and clap flag parsing.
  - `crates/canon-cli/src/commands/init.rs` for the init command boundary and summary printing.
  - `crates/canon-cli/src/output.rs` and `crates/canon-cli/src/main.rs` for final output formatting and stderr reporting after TUI teardown.
  - `crates/canon-engine/src/orchestrator/service.rs` for the unchanged `EngineService::init()` backend and `AiTool` contract.
  - `Cargo.toml` and `crates/canon-cli/Cargo.toml` for additive TUI dependencies.
  - `tests/integration/init_creates_canon.rs` and adjacent CLI tests for regression coverage around `.canon/` creation and idempotence.
**Performance Goals**: Guided init should remain responsive for manual keyboard use, avoid layout corruption during redraw, and restore the terminal reliably; non-interactive init should preserve current scripting latency and output behavior. The measurable closeout gates remain the spec-defined success criteria rather than a separate time-budget commitment.
**Constraints**: TUI logic must stay in `canon-cli`; interactive mode must reject `json`, `yaml`, and `markdown` output unless `--non-interactive` is used; runtime layout-fit is evaluated dynamically rather than through a fixed published terminal size; too-small layouts fail before the TUI opens; `Ctrl+C` is the only user-driven interruption path; Rust production code outside allowed locations must avoid panic-prone control flow and magic literals.
**Scale/Scope**: One existing command surface, one new TUI module subtree under `canon-cli`, one CLI contract document, one quickstart walkthrough, focused unit/integration coverage, and no engine or persistence-schema changes.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Pre-Phase 0 Gate

- [x] Execution mode is declared and matches an existing-system CLI change.
- [x] Risk classification is explicit and bounded to one command surface.
- [x] Scope boundaries and exclusions are recorded.
- [x] Invariants are explicit before implementation.
- [x] Required artifacts and owners are identified.
- [x] Decision traceability points to the clarified feature spec.
- [x] Validation planning separates generation from validation.
- [x] No constitution deviation requires a justified exception.

### Post-Phase 1 Re-Check

- [x] Design artifacts keep terminal UI, event handling, and cleanup in
  `canon-cli`, preserving the engine boundary.
- [x] The CLI contract artifact captures the guided-path gates, rejection
  cases, and non-interactive compatibility expectations.
- [x] The data model and quickstart preserve runtime-side-effect invariants and
  temp-workspace isolation.
- [x] Validation remains layered across unit tests, integration tests,
  contract checks, manual walkthroughs, and repo quality gates.

## Project Structure

### Documentation (this feature)

```text
specs/063-interactive-init-ui/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── canon-init-cli-contract.md
├── validation-report.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/canon-cli/
├── Cargo.toml
└── src/
    ├── app.rs
    ├── main.rs
    ├── output.rs
    ├── commands/
    │   └── init.rs
    └── tui/
        ├── mod.rs
        ├── terminal.rs
        ├── init.rs
        └── render.rs

crates/canon-engine/src/
└── orchestrator/service.rs

tests/
├── init_creates_canon.rs
├── init_guided_contract.rs
├── init_non_interactive_contract.rs
├── init_terminal_recovery.rs
└── integration/
  ├── init_creates_canon.rs
  ├── init_guided_contract.rs
  ├── init_non_interactive_contract.rs
  └── init_terminal_recovery.rs

.github/
└── copilot-instructions.md
```

**Structure Decision**: Keep the feature inside the existing Rust workspace,
add a dedicated `crates/canon-cli/src/tui/` subtree for terminal lifecycle,
event-loop state, and rendering, preserve `crates/canon-cli/src/commands/init.rs`
as the orchestration boundary, and leave `canon-engine` unchanged except as the
existing init backend.

## Complexity Tracking

> No constitution violations. All gates pass.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | N/A | N/A |
