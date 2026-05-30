# Implementation Plan: pr-review Optional Inline Anchors

**Branch**: `060-pr-review-anchors` | **Date**: 2026-05-19 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/060-pr-review-anchors/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See
`.specify/templates/plan-template.md` for the execution workflow.

## Summary

Extend the existing `pr-review` Conventional Comments flow with an optional,
typed inline anchor that points to one changed line or one contiguous changed
span when the persisted diff evidence supports that precision. The plan keeps
the delivered explicit `pr`/`file`/`surface` scope model as the mandatory
baseline, derives anchors only from the existing diff-backed review pipeline,
renders them in host-agnostic Markdown, degrades honestly to scope-only
comments whenever the evidence is missing, stale, ambiguous, or cross-surface,
and aligns release-facing version, documentation, and companion wiki surfaces
with the delivered behavior.

## Governance Context

**Execution Mode**: change
**Risk Classification**: bounded-impact because this slice extends the `pr-review`
domain model and published Conventional Comments artifact without changing Canon
run orchestration, approval semantics, or non-review modes.
**Scope In**:

- feature-local planning artifacts under `specs/060-pr-review-anchors/`
- `pr-review` domain types in `crates/canon-engine/src/review/findings.rs`
- Conventional Comments rendering in `crates/canon-engine/src/artifacts/markdown/governance.rs`
- validation and fixture coverage for anchored and degraded findings in engine
  and integration tests
- reviewer guidance mirrors for `canon-pr-review`
- repository docs in `README.md` and `docs/guides/modes.md`, plus companion wiki
  guidance under `../canon.wiki/`
- release-facing version, lockfile, and changelog alignment in `Cargo.toml`,
  `Cargo.lock`, and `CHANGELOG.md`

**Scope Out**:

- reworking the already-delivered explicit `pr`/`file`/`surface` scope model
- host-specific review-thread exports or deep links
- `review-summary.md` status semantics, approval posture, or gate outcomes
- non-`pr-review` modes and live-repo coordinate recomputation after evidence capture

**Invariants**:

- Every Conventional Comment entry must keep exactly one explicit scope annotation,
  regardless of anchor presence.
- Inline anchors must only be emitted when one finding can be tied to one
  concrete changed surface and one durable contiguous coordinate interval.
- `review-summary.md` remains the primary artifact and canonical readiness surface.
- Missing, stale, ambiguous, or cross-surface evidence must degrade to scope-only
  output rather than fabricate precision.

**Decision Log**: `specs/060-pr-review-anchors/decision-log.md`  
**Validation Ownership**: Generation is owned by the feature author through the
plan, research, contract, and implementation artifacts; validation is owned by
independent automated checks plus a maintainer review of rendered packet samples
and contract alignment recorded in `validation-report.md`.  
**Approval Gates**: No additional pre-implementation approval gate is required
for bounded-impact work; merge readiness depends on independent review of the
validation evidence and packet-sample behavior.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024, plus Markdown contract and planning artifacts  
**Primary Dependencies**: existing workspace crates `canon-engine`, `canon-cli`, and `canon-adapters`; `serde`, `strum_macros`, `thiserror`, `toml`, `tracing`, `uuid`, and `time` already used by the workspace  
**Storage**: repository planning artifacts under `specs/060-pr-review-anchors/` and existing `.canon/` runtime evidence/artifact files for `pr-review` runs  
**Testing**: `cargo test`, `cargo nextest run`, focused unit and integration coverage for `pr-review`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, plus packet-sample review recorded in `validation-report.md`  
**Target Platform**: macOS/Linux CLI workflows using local git diffs and published Markdown artifacts  
**Project Type**: Rust workspace CLI plus engine/adapters crates with published governance artifacts  
**Existing System Touchpoints**: `crates/canon-adapters/src/shell.rs`, `crates/canon-engine/src/review/findings.rs`, `crates/canon-engine/src/artifacts/markdown/governance.rs`, `crates/canon-engine/src/review/summary.rs`, `tests/contract/pr_review_anchor_contract.rs`, `tests/fixtures/`, `tests/integration/pr_review_run.rs`, `.agents/skills/canon-pr-review/SKILL.md`, `defaults/embedded-skills/canon-pr-review/skill-source.md`, `README.md`, `docs/guides/modes.md`, `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, `../canon.wiki/Canon-Modes.md`, `../canon.wiki/Example-Flow-Code-Review.md`, and `../canon.wiki/Reference.md`  
**Performance Goals**: preserve current `pr-review` runtime posture with no observable change beyond lightweight diff-hunk parsing during packet construction  
**Constraints**: keep anchors host-agnostic, preserve explicit scope as the fallback contract, avoid panic-prone control flow, avoid magic literals in owned Rust logic, and use typed serde models for any stable anchor shape  
**Scale/Scope**: one review domain flow, one published artifact family, two to three Rust source files plus contract/integration fixtures, reviewer guidance mirrors, and release/documentation alignment surfaces

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

Post-design recheck: Phase 0 research and Phase 1 design artifacts preserve the
same bounded-impact scope, invariants, and validation split; no additional
constitution deviations were introduced.

## Project Structure

### Documentation (this feature)

```text
specs/060-pr-review-anchors/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── conventional-comment-anchor-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
Cargo.toml
Cargo.lock
CHANGELOG.md
README.md

crates/
├── canon-adapters/
│   └── src/
│       └── shell.rs
├── canon-engine/
│   └── src/
│       ├── artifacts/
│       │   └── markdown/
│       │       └── governance.rs
│       ├── review/
│       │   ├── findings.rs
│       │   └── summary.rs
│       └── orchestrator/
│           └── service/
│               └── mode_pr_review.rs

defaults/
└── embedded-skills/
    └── canon-pr-review/
        └── skill-source.md

.agents/
└── skills/
    └── canon-pr-review/
        └── SKILL.md

docs/
└── guides/
  └── modes.md

tests/
├── contract/
│   └── pr_review_anchor_contract.rs
├── fixtures/
│   ├── pr_review_anchor_line.diff
│   ├── pr_review_anchor_span.diff
│   ├── pr_review_anchor_cross_surface.diff
│   └── pr_review_anchor_stale.diff
└── integration/
  └── pr_review_run.rs

../canon.wiki/
├── Canon-Modes.md
├── Example-Flow-Code-Review.md
└── Reference.md
```

**Structure Decision**: Keep the change inside the existing diff-backed
`pr-review` pipeline. The shell adapter remains the authoritative source of
changed surfaces and zero-context diff hunks, the review domain owns anchor
eligibility and typed shape decisions, the Markdown governance renderer owns
human-readable output, and the reviewer guidance, release surfaces, and
companion wiki stay aligned with the published artifact behavior.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | Constitution gates pass without deviation for this planning slice. |
