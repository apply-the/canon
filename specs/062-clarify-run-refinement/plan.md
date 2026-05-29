# Implementation Plan: Mode Clarification And Run Refinement

**Branch**: `062-clarify-run-refinement` | **Date**: 2026-05-29 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/062-clarify-run-refinement/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Introduce a durable run-refinement lifecycle that keeps the same Canon work
identity across clarification for `requirements`, `discovery`,
`system-shaping`, `architecture`, and `change`, while extending all modes with
explicit continue or refine identity continuity. The implementation reuses the
existing run identity and `Draft` state, adds typed clarification-refinement
state to persisted run context, adds successor lineage to run manifests for
post-start mode changes, materializes working-brief artifacts under
`.canon/runs/<RUN_ID>/artifacts/`, and exposes refinement state through
`status`, `inspect`, templates, and skill guidance without mutating
`canon-input/`.

## Governance Context

**Execution Mode**: change
**Risk Classification**: systemic-impact; this slice changes governed run
identity continuity, persisted runtime state, status or inspect surfaces, and
mode-specific clarification behavior across multiple planning modes.
**Scope In**: Existing-run refinement identity continuity for all modes;
working-brief clarification lifecycle for `requirements`, `discovery`,
`system-shaping`, `architecture`, and `change`; typed clarification and
readiness persistence in run context; successor lineage for post-start mode
changes; additive `status` or `inspect` output for refinement state; targeted
template, method, and skill-source updates that describe the new lifecycle;
representative non-targeted continuity validation across `review`,
`verification`, `implementation`, `refactor`, `incident`, and `migration`,
plus shared `resume` and `status` operator surfaces.
**Scope Out**: Mutating `canon-input/` or starter templates in place;
redesigning approval semantics or publish destinations; first-class
working-brief clarification for every non-targeted mode; automatic continuation
without explicit intent; any new persistence family outside `.canon/runs/`.

**Invariants**:

- Authored inputs under `canon-input/` remain immutable evidence; refinement
  state lives only in `.canon/runs/<RUN_ID>/context.toml`,
  `.canon/runs/<RUN_ID>/manifest.toml`, and additive run artifacts.
- The durable draft work item reuses the existing Canon run identity in
  `RunState::Draft`; Canon does not introduce a second identity family for
  pre-run clarification in this slice.
- Candidate detection is advisory only; Canon mutates an existing draft or run
  only after explicit continuation intent.
- Post-start mode changes create successor lineage (`carried_from`,
  `supersedes`) instead of mutating the started run in place.
- New persisted or serialized refinement shapes use typed Rust structs or enums
  with serde derives; no ad hoc map assembly or raw stable-key scattering.

**Decision Log**: `specs/062-clarify-run-refinement/decision-log.md`
**Validation Ownership**: Generation is performed by the implementation author
and AI-assisted design artifacts; validation is performed by targeted Rust and
CLI tests, contract checks, and an independent review pass over persistence,
lineage, and continuation semantics.
**Approval Gates**: Explicit human owner review before runtime persistence or
CLI contract changes for this systemic-impact slice are accepted; independent
review of manifest or context schema changes is required before implementation
closeout.
**Release Closeout Scope**: Runtime closeout must cover engine persistence,
identity, lifecycle, status, inspect, and working-brief rendering surfaces;
documentation closeout must update release notes and operator guidance in repo
docs and templates; wiki closeout must update the aligned public workflow and
lineage guidance pages after validation evidence is complete.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024; Markdown documentation and templates; existing Spec Kit shell helpers  
**Primary Dependencies**: workspace crates `canon-engine`, `canon-cli`, `canon-adapters`; existing `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`, `thiserror`, `tracing`, `uuid`, and `time`; repo-local methods, templates, and skill-source documents  
**Storage**: existing `.canon/runs/<RUN_ID>/manifest.toml`, `context.toml`, `artifacts/`, and `inputs/`; published docs and templates under `docs/`, `defaults/`, and `.agents/skills/`  
**Testing**: `cargo test`, targeted Rust unit or integration tests for run context, manifests, status, inspect, markdown rendering, and question-selection behavior; recorded operator walkthrough evidence for `status` and `inspect refinement`; explicit regression review for publish destinations, artifact families, and source-input honesty markers; `cargo fmt --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; doc or template validation where touched  
**Target Platform**: local filesystem Canon CLI on macOS, Linux, and Windows; assistant hosts that consume Canon status, inspect, and skill guidance  
**Project Type**: Rust CLI and engine workspace with markdown artifact rendering and repo-local governance templates  
**Existing System Touchpoints**:
  - `crates/canon-engine/src/domain/run.rs` for `RunState`, `RunContext`, and new typed refinement context.
  - `crates/canon-engine/src/persistence/manifests.rs` and `crates/canon-engine/src/persistence/store*.rs` for persisted lineage and context round-tripping.
  - `crates/canon-engine/src/orchestrator/service/identity.rs`, `clarity.rs`, and `input_handling.rs` for draft creation, candidate detection, authority rules, decision-changing question selection, and readiness delta computation.
  - `crates/canon-engine/src/orchestrator/service/run_summary.rs` and `inspect.rs` for run-scoped refinement visibility.
  - `crates/canon-engine/src/artifacts/markdown.rs` and mode-specific markdown renderers for working-brief materialization.
  - `crates/canon-cli/src/commands/status.rs`, `resume.rs`, `inspect.rs`, and `crates/canon-cli/src/output/inspect.rs` for explicit continuation and operator-facing output.
  - `defaults/methods/*.toml`, `defaults/embedded-skills/canon-*/skill-source.md`, and `docs/templates/canon-input/*.md` for mode-specific clarification guidance and authoritative brief expectations.
**Performance Goals**: Preserve current local file-backed status and inspect responsiveness while surfacing refinement state; no extra persistence family scans; recorded walkthrough evidence shows a reviewer can identify the authoritative brief, clarification history, and readiness delta in under 2 minutes.  
**Constraints**: Reuse the existing run identity and `Draft` lifecycle; keep `canon-input/` immutable; preserve approval and recommendation-only semantics; keep `inspect clarity` focused on authored-input analysis; bound clarification questions to decision-changing gaps only; use typed serde models for any new stable persisted shapes; explicitly prove that publish destinations, artifact families, and source-input honesty markers stay unchanged unless an additive refinement surface requires otherwise.  
**Scale/Scope**: 5 targeted modes with first-class working-brief refinement, a representative non-targeted validation matrix spanning `review`, `verification`, `implementation`, `refactor`, `incident`, and `migration`, all modes with explicit continuation identity continuity, 1 additive refinement context model, 1 lineage extension on run manifests, additive CLI/runtime output changes, and mirrored guidance updates across templates and skill sources.

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
specs/062-clarify-run-refinement/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── runtime-refinement-state-contract.md
│   ├── status-and-inspect-refinement-contract.md
│   └── working-brief-artifact-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/canon-engine/src/
├── domain/run.rs
├── persistence/
│   ├── manifests.rs
│   ├── store.rs
│   └── store/runtime.rs
├── orchestrator/service/
│   ├── identity.rs
│   ├── clarity.rs
│   ├── input_handling.rs
│   ├── inspect.rs
│   └── run_summary.rs
└── artifacts/
    ├── markdown.rs
    └── markdown/

crates/canon-cli/src/
├── app.rs
├── commands/
│   ├── inspect.rs
│   ├── resume.rs
│   └── status.rs
└── output/
    └── inspect.rs

defaults/
├── methods/
└── embedded-skills/

docs/
├── templates/canon-input/
└── guides/

tests/
├── contract/
├── integration/
└── unit/
```

**Structure Decision**: Keep the existing Rust workspace and `.canon/runs/`
runtime layout. Persist new refinement state by extending current run context
and manifest models, render additive working-brief artifacts under the existing
artifact tree, and update only the targeted mode guidance, templates, and CLI
surfaces that already own authoring and inspection behavior.

## Complexity Tracking

> No constitution violations. All gates pass.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| — | — | — |
