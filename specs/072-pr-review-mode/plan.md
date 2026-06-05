# Implementation Plan: Refactor Canon pr-review Into An Actionable Code Review Mode

**Branch**: `072-pr-review-mode` | **Date**: 2026-06-05 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/072-pr-review-mode/spec.md`

## Summary

Refactoring the Canon `pr-review` mode to focus heavily on actionable developer feedback. This includes generating new structured JSON and Markdown artifacts (`review-summary.md`, `github-comments.json`, `conventional-comments.md`, `missing-tests.md`, `review-findings.json`) that can pinpoint exact line or hunk references. Legacy governance features remain as secondary audits.

## Governance Context

- **Execution Mode**: `pr-review`
- **Risk Classification**: Bounded-Impact
- **Scope Boundaries**: In-scope: outputting actionable code reviews, JSON tracking, diff extraction logic. Out-of-scope: auto-submitting comments to GitHub API via Canon.
- **Invariants**: Governance tracking must persist; `Approve` decision blocked by blocking findings.
- **Validation Ownership**: An independent layer must map the generated LLM responses to deterministic findings before merging.

## Technical Context

**Language/Version**: Rust 1.96.0

**Primary Dependencies**: `serde`, `serde_json`, `clap`, `canon-engine`, `canon-cli`

**Storage**: Local `.canon/` and `specs/` directory files

**Testing**: `cargo test`, `cargo nextest run`

**Target Platform**: Linux/macOS CLI

**Project Type**: CLI Application

**Performance Goals**: N/A

**Constraints**: > 20 changed files or > 500 lines of code triggers explicit sampling behavior for the LLM.

**Scale/Scope**: Repository-level PR analysis

## Constitution Check

*GATE: Passed. Mode, risk, and boundaries are explicit. Serialization shapes will be strictly typed in Rust via `serde`. No panic-prone control flow permitted in the core logic.*

## Project Structure

### Documentation (this feature)

```text
specs/072-pr-review-mode/
├── plan.md              # This file
├── research.md          
├── data-model.md        
├── quickstart.md        
├── contracts/           
│   └── cli.md           # CLI inputs/outputs
└── validation-report.md 
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   ├── src/
│   │   ├── review/
│   │   │   ├── evaluator.rs    # Core parsing logic for review schemas
│   │   │   ├── generators.rs   # Markdown emitters for summary, comments, tests
│   │   │   ├── findings.rs     # Structs for Review Finding and mapping
│   │   │   └── diff.rs         # Diff parser / line mapper
│   └── tests/
└── canon-cli/
    ├── src/
    │   └── commands/
    │       └── pr_review.rs    # Modified CLI handler orchestrating the engine
    └── tests/
```

**Structure Decision**: Using the single-project Rust structure with `canon-engine` housing the domain logic for PR parsing/extraction and `canon-cli` managing the execution orchestrator.
