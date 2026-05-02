# Implementation Plan: Guided Run Operations And Review Experience

**Branch**: `038-guided-run-operations` | **Date**: 2026-05-02 | **Spec**: `specs/038-guided-run-operations/spec.md`
**Input**: Feature specification from `/specs/038-guided-run-operations/spec.md`

## Summary

Deliver `0.37.0` as a bounded operator-surface slice by aligning the existing
run or status summary pipeline around one coherent review-first guidance
contract. Reuse the current `RunSummary`, `StatusSummary`, `ModeResultSummary`,
`RecommendedActionSummary`, and `ActionChip` surfaces, then add one shared
operator-guidance derivation that keeps recommended next steps, ordered possible
actions, progressive-enhancement chips, CLI markdown rendering, and shared
`render-next-steps` scripts semantically aligned for completed, blocked,
approval-gated, resumed, and partially publishable runs.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact; this work changes the operator-facing
summary and guidance contract across runtime, CLI, shared skill-facing helper
scripts, docs, and tests, but it does not introduce a new governed mode, new
run-state family, new persistence layout, or new approval semantics.  
**Scope In**: run or status runtime details assembly; recommended next action
derivation; ordered possible actions; progressive-enhancement action chips;
CLI markdown rendering for run or status; shared next-step helper scripts and
their output contract; focused unit, contract, and integration tests; docs,
roadmap, changelog, and explicit `0.37.0` release alignment.  
**Scope Out**: new Canon modes; new `.canon/` storage families; hidden planner
behavior; changes to approval policy meaning; broad artifact-family redesign;
automatic run follow-up execution; unrelated authoring-lifecycle work reserved
for feature `039`.

**Invariants**:

- Approval, evidence, blocked-gate, and recommendation-only semantics remain
  explicit and must not be collapsed into generic “continue” guidance.
- `Possible Actions:` text remains mandatory, and chips remain progressive
  enhancement rather than a second execution surface.
- The CLI and governance adapter remain the canonical control surface for
  steering a run.
- Existing artifact paths, approval targets, and readable packet summaries stay
  lossless across run and status output.

**Decision Log**: `specs/038-guided-run-operations/decision-log.md`  
**Validation Ownership**: Generation updates runtime summary derivation, CLI
rendering, shared helper scripts, docs, roadmap, changelog, and test fixtures;
validation is performed through focused Rust tests, shared-script regression
checks, release-alignment checks, coverage review for touched Rust files,
`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
and an independent review of operator-guidance honesty.  
**Approval Gates**: No new human approval gate is introduced for this
bounded-impact slice; independent validation evidence remains mandatory.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Bash, PowerShell, and Markdown documentation artifacts  
**Primary Dependencies**: workspace crates `canon-cli`, `canon-engine`, and `canon-adapters`; existing `serde`/`serde_json` output contracts; shared skill helper scripts under `.agents/skills` and `defaults/embedded-skills`  
**Storage**: repository files plus the existing `.canon/` runtime artifact layout only; no new persistence family  
**Testing**: focused `cargo test` targets for engine or CLI summaries, `tests/render_next_steps.rs`, targeted integration or contract tests for gated and resumed runs, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo nextest run`  
**Target Platform**: local CLI-driven Canon workflows and compatible chip-capable skill hosts  
**Project Type**: Rust CLI workspace with shared script helpers and Markdown skill references  
**Existing System Touchpoints**: `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/next_action.rs`, `crates/canon-engine/src/orchestrator/service/summarizers.rs`, `crates/canon-engine/src/orchestrator/service/tests.rs`, `crates/canon-cli/src/output.rs`, `tests/render_next_steps.rs`, focused integration tests such as `tests/integration/implementation_run.rs`, shared helper scripts under `.agents/skills/canon-shared/scripts/` and `defaults/embedded-skills/canon-shared/scripts/`, skill output references under `.agents/skills/canon-shared/references/output-shapes.md`, and release or docs surfaces including `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `Cargo.toml`, and `Cargo.lock`  
**Performance Goals**: no hidden inference, one deterministic recommended action at most, ordered possible actions that stay valid for the active run state, and no regression to existing run/state or artifact contracts  
**Constraints**: keep chip behavior as progressive enhancement; preserve current `Completed`, `Blocked`, and `AwaitingApproval` semantics; keep version alignment at `0.37.0`; maintain docs and roadmap honesty; cover touched Rust files with focused automated validation before full-suite closeout  
**Scale/Scope**: one shared operator-guidance derivation update, one CLI rendering update, one shared-script contract update, bounded docs or roadmap cleanup, and one release-alignment pass

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Declared-risk approval checkpoints are named where required by the risk classification
- [x] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/038-guided-run-operations/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── operator-guidance.md
│   └── render-next-steps.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-cli/
│   └── src/
│       └── output.rs
└── canon-engine/
    └── src/
        └── orchestrator/
            └── service/
                ├── next_action.rs
                ├── summarizers.rs
                ├── tests.rs
                └── service.rs

tests/
├── integration/
│   └── implementation_run.rs
└── render_next_steps.rs

.agents/skills/canon-shared/
├── references/output-shapes.md
└── scripts/
    ├── render-next-steps.ps1
    └── render-next-steps.sh

defaults/embedded-skills/canon-shared/
├── references/output-shapes.md
└── scripts/
    ├── render-next-steps.ps1
    └── render-next-steps.sh

README.md
ROADMAP.md
CHANGELOG.md
Cargo.toml
Cargo.lock
```

**Structure Decision**: Keep feature `038` localized to the existing operator
summary and helper-script surfaces instead of adding a new runtime subsystem.
The shared engine service remains the single place where run-state facts are
turned into operator guidance, while CLI and skill-facing outputs become thin
renderers over that same contract.

## Complexity Tracking

No constitution deviations are currently identified.
