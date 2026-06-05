# Feature Specification: Refactor Canon pr-review Into An Actionable Code Review Mode

**Feature Branch**: `072-pr-review-mode`

**Created**: 2026-06-05

**Status**: Draft

**Input**: User description: "Refactor Canon `pr-review` Into An Actionable Code Review Mode..."

## Governance Context

- **Mode**: `pr-review` (modifying the existing mode logic and artifact emissions).
- **Risk Profile**: Bounded-Impact. Modifies output formatting and AI prompt interactions for code reviews, but does not execute unsandboxed commands or introduce systemic architecture shifts.
- **Scope In**:
  - Refactoring `pr-review` prompt, logic, and extraction tools to produce actionable code reviews instead of purely governance-focused output.
  - Adding extraction rules for exact line-level, hunk-level, and file-level GitHub comments.
  - Producing new mandatory artifacts: `review-summary.md`, `github-comments.json`, `conventional-comments.md`, `missing-tests.md`, `review-findings.json`.
  - Modifying review decision states to distinguish between `Approve`, `Comment`, and `Request changes`.
  - Preserving backward compatibility for downstream processors that need `run.toml`, `evidence.toml`, etc.
- **Scope Out**:
  - Auto-submitting the generated GitHub comments via GitHub API directly in this workflow (only producing the JSON artifacts for later).
  - Enforcing 100% comment coverage on gigabyte-sized diffs (sampling large diffs is acceptable).
- **Invariants**:
  - Governance tracking and gate evaluation must remain intact but be rendered as secondary outputs.
  - `pr-review` must never output `Approve` if there are blocking findings.

## Clarifications

### Session 2026-06-05

- Q: If the LLM generates a line number that does not actually exist in the provided diff, how should the system handle the finding? → A: Attempt to validate the line deterministically; if invalid, downgrade to hunk-level or general PR finding to preserve feedback.
- Q: At what threshold should a diff be considered "large" to trigger explicit sampling instructions? → A: Hard threshold: > 20 changed files or > 500 lines of code.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Actionable PR Review Generation (Priority: P1)

As a code reviewer or PR author, I want `pr-review` to generate precise inline and hunk-level feedback so that I can directly understand what needs fixing in the code.

**Why this priority**: Precise, actionable code-review findings are the core goal of this refactor.

**Independent Test**: Can be tested by running `canon` in `pr-review` mode against a test fixture containing deliberate security and logic flaws, and verifying `github-comments.json` includes exact `line` and `hunk_header` references for those flaws.

**Acceptance Scenarios**:

1. **Given** a pull request with an evidence validation flaw on line 123, **When** `pr-review` processes it, **Then** `github-comments.json` contains a blocking issue aimed at line 123.
2. **Given** a large, multi-file pull request, **When** `pr-review` processes it, **Then** `review-summary.md` explains the decision and lists blocking findings explicitly.

### User Story 2 - Governance Preservation (Priority: P2)

As a security auditor, I need `pr-review` to continue emitting governance state and blocking readiness if evidence requirements are not met, so that compliance is not degraded.

**Why this priority**: Compliance with Constitution constraints is mandatory.

**Independent Test**: Can be tested by providing a diff that fails basic gating and confirming that `state.toml` and release readiness still correctly capture the blocked state, alongside the new primary review output.

**Acceptance Scenarios**:

1. **Given** a PR that introduces missing evidence scenarios, **When** `pr-review` runs, **Then** release readiness is marked as blocked with a concrete reason tied to a missing test or code validation gap.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST output `review-summary.md` containing `Decision`, `Executive Summary`, `Must Fix`, `Should Fix`, `Missing Tests`, `GitHub-Ready Comments`, `General Findings`, and `Governance Notes`.
- **FR-002**: System MUST output `github-comments.json` structured with `id`, `path`, `line`, `side`, `hunk_header`, `area`, `type`, `severity`, `blocking`, `category`, `body`, `why_it_matters`, and `suggested_remediation`.
- **FR-003**: System MUST output `conventional-comments.md` grouping comments by file using Conventional Comment vocabulary (`issue`, `suggestion`, `question`, etc.).
- **FR-004**: System MUST output `missing-tests.md` detailing any missing critical test scenarios and whether they are blocking.
- **FR-005**: System MUST output `review-findings.json` containing a normalized list of all extracted findings.
- **FR-006**: System MUST classify findings correctly into Inline, Hunk-level, or General PR levels.
- **FR-007**: System MUST provide a `review_coverage` JSON block using explicit risk-based sampling when the diff exceeds the hard threshold of > 20 changed files or > 500 lines of code.
- **FR-008**: System MUST emit decisions explicitly as `Approve`, `Comment`, or `Request changes`.

### Key Entities

- **Review Finding**: A structured code review comment containing severity, exact diff location, conventional classification, and remediation steps.
- **Review Decision**: The aggregate outcome (`Approve`, `Comment`, or `Request changes`) driven by the blocking status of Review Findings.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of generated review comments meant for specific lines or hunks contain valid `path` and `line` / `hunk_header` attributes. Invalid line targets MUST be deterministically validated against the diff and downgraded to hunk-level or general findings rather than emitted silently or discarded.
- **SC-002**: A `Request changes` decision is produced automatically whenever a `blocking` finding exists in the review.
- **SC-003**: 100% of missing test findings map clearly to an explicit changed behavior or risk within the diff.
- **SC-004**: Legacy consumers of `run.toml` and `state.toml` continue parsing the governance artifacts without breaking changes.

## Validation Plan

- **Structural Validation**: Ensure all 5 requested files (`review-summary.md`, `github-comments.json`, `conventional-comments.md`, `missing-tests.md`, `review-findings.json`) are produced by the updated evaluator.
- **Logical Validation**: Execute `pr-review` mode using fixtures containing intentional syntax and business logic errors. Assert the engine categorizes errors correctly, pinpoints the line numbers, and blocks release readiness with an explanation.

## Decision Log

- **DECISION 1**: The primary output layer will be surfaced in `review-summary.md` and related review files, while `state.toml` and similar are kept as secondary artifacts. Rationale: Reviews must be actionable for developers, while governance is an audit layer.
- **DECISION 2**: When a PR is too large, the LLM will sample and emit `review_coverage` instead of hallucinating line numbers. Rationale: Accuracy on a subset of high-risk files is better than inaccurate exhaustive coverage.

## Non-Goals

- Attempting to auto-submit the GitHub comments to the GitHub API within the `canon` CLI tool (this requires external orchestration logic or GitHub Actions).
- Removing the evidence tracking or governance records altogether.

## Assumptions

- We assume that `diff.patch` and `changed-files.txt` are already accessible or provided accurately to the LLM agent running the review mode.
- We assume downstream integrations (if any) are capable of parsing `github-comments.json` when mapping the output to a live PR.
