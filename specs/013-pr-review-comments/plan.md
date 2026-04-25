# Implementation Plan: PR Review Conventional Comments

**Branch**: `013-pr-review-comments` | **Date**: 2026-04-24 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/013-pr-review-comments/spec.md`

## Summary

Deliver Conventional Comments as an additive `pr-review` artifact without
changing the current review-disposition and publish flow. The implementation
adds a deterministic mapping from persisted `ReviewFinding` records into
reviewer-facing comment kinds, preserves `review-summary.md` as the primary
artifact, and extends tests, docs, and skills so the new output shape is
governed and publishable.

## Governance Context

**Execution Mode**: pr-review  
**Risk Classification**: bounded-impact. The work changes shipped artifact
contracts and reviewer-facing output for an existing mode, but it does not
change the core execution controls, policy semantics, or approval model.  
**Scope In**: `pr-review` artifact contract, markdown rendering, mode-result
surfaces, publish/readability behavior, tests, docs, and skill guidance for
Conventional Comments output.  
**Scope Out**: line-anchored host integrations, non-PR `review` mode,
architecture C4 output, and approval/gate redesign.

**Invariants**:

- Review-disposition gating for must-fix findings remains unchanged.
- `review-summary.md` remains the canonical status/next-step artifact.
- No Conventional Comments artifact may invent line-level anchors or host data
  absent from the persisted review packet.
- Every Conventional Comments entry remains traceable to persisted findings and
  changed surfaces.

**Decision Log**: `specs/013-pr-review-comments/decision-log.md`  
**Validation Ownership**: Generation work updates contract, renderers, tests,
and docs; validation is closed separately through contract/integration tests,
publish/readability checks, and an independent packet review recorded in
`validation-report.md`.  
**Approval Gates**: No new human gate is introduced; existing `pr-review`
review-disposition approval remains authoritative for must-fix findings.

## Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024  
**Primary Dependencies**: `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`,
`thiserror`, `tracing`, `uuid`, `time`  
**Storage**: Local filesystem under `.canon/` for runtime artifacts and
evidence, plus published markdown under `docs/reviews/prs/`  
**Testing**: `cargo test`, `cargo nextest run`, targeted contract and
integration suites under `tests/`  
**Target Platform**: Cross-platform local CLI workflows on macOS, Linux, and
Windows  
**Project Type**: Rust CLI + engine workspace  
**Existing System Touchpoints**: `crates/canon-engine/src/artifacts/contract.rs`,
`crates/canon-engine/src/artifacts/markdown.rs`,
`crates/canon-engine/src/review/findings.rs`,
`crates/canon-engine/src/orchestrator/service/summarizers.rs`,
`crates/canon-engine/src/orchestrator/service/mode_pr_review.rs`,
`crates/canon-engine/src/orchestrator/publish.rs`,
`crates/canon-cli/src/output.rs`,
`tests/contract/pr_review_contract.rs`, `tests/pr_review_run.rs`,
`tests/integration/pr_review_run.rs`, `README.md`, `NEXT_FEATURES.md`, and the
`canon-pr-review` skill sources.  
**Performance Goals**: Keep `pr-review` artifact generation effectively linear
in the number of persisted findings and changed surfaces; no material slowdown
in packet emission or publish readability.  
**Constraints**: Preserve backward-compatible status and next-step flows,
avoid host-specific export semantics, and keep the new artifact readable as
standalone markdown.  
**Scale/Scope**: One shipped mode, one additive artifact, focused contract and
integration coverage, plus bounded docs/skill updates.

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
specs/013-pr-review-comments/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── conventional-comments-artifact.md
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
        ├── artifacts/
        │   ├── contract.rs
        │   └── markdown.rs
        ├── orchestrator/
        │   └── service/
        │       ├── mode_pr_review.rs
        │       └── summarizers.rs
        └── review/
            └── findings.rs

defaults/
└── embedded-skills/
    └── canon-pr-review/

.agents/
└── skills/
    └── canon-pr-review/

tests/
├── contract/
│   └── pr_review_contract.rs
├── integration/
│   └── pr_review_run.rs
├── pr_review_contract.rs
└── pr_review_run.rs

README.md
NEXT_FEATURES.md
```

**Structure Decision**: Keep the work inside the existing `pr-review` runtime
surface. The engine gains one additive artifact and a deterministic mapping
layer; the rest of the implementation is bounded to tests, summaries, and docs.

## Workstreams

1. Define the additive Conventional Comments artifact contract and mapping
   rules.
2. Render reviewer-facing comment entries from persisted `ReviewFinding`
   records without weakening approval semantics.
3. Extend summaries, publish behavior, CLI output, tests, docs, and skills to
  advertise the delivered format.

## Phase Outcomes

### Phase 0: Research

- Decide whether the feature is additive or replacement-oriented.
- Decide the first-slice mapping from review findings into Conventional
  Comments kinds.
- Decide how to keep host-agnostic comments readable without fake line anchors.

### Phase 1: Design

- Define the additive artifact contract and data model.
- Record publish/readability behavior and validation evidence expectations.
- Confirm docs and skill surfaces that must change when the feature lands.

### Phase 2: Implementation Preparation

- Leave a task-ready design that keeps `review-summary.md` stable while adding
  the new artifact and test coverage.

## Complexity Tracking

No constitution deviations are planned.

