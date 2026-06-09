# Feature Specification: Agent-Governed Onion-Layer PR Review

**Feature Branch**: `074-pr-review-onion-orchestration`

**Created**: 2026-06-08

**Status**: Draft

**Input**: User description: "Refactor Canon pr-review into an agent-governed review orchestration workflow with onion-layer context expansion (diff → whole file → related files → logical stress → tests), distinct CLI phases (prepare/accept/finalize), structured reviewer output contract, and deterministic test strategy."

**Templates**: `specs/072-pr-review-mode/templates/`

## Summary

Canon is not intended to be a standalone AI reviewer. Canon is a deterministic CLI invoked and governed by an external LLM agent. The LLM performs semantic code review. Canon prepares context, structures the review workflow, validates structured reviewer output, renders artifacts, preserves traceability, and applies governance gates.

The review must be performed "onion-style": start from the diff, expand to the whole modified file, expand to related files and call sites, then stress-test edge cases and test coverage. The goal is to avoid shallow diff-only review and force the LLM to understand the effect of a change in its local, module, and dependency context.

## Product Principle

```text
Canon structures and validates the review.
The LLM performs semantic reasoning.
Canon must not pretend that deterministic path heuristics are an actionable code review.
```

## Clarifications

### Session 2026-06-08

- Q: What is the normative operator workflow between `prepare`, `accept`, and `finalize`? → A: Phased onion workflow. Canon orchestrates five distinct LLM review layers (diff → whole-file → related-context → logical-stress → tests), each with its own run state, context packet, and instructions. The LLM performs semantic reasoning inside each layer. Canon records which layers were executed and which files were inspected. A review is not complete just because the diff was inspected; all layers must be executed or explicitly skipped with reasons recorded.
- Q: How does the LLM agent receive the review context from Canon? → A: File-based handoff is the normative workflow (Canon writes context packets under `.canon/runs/<run-id>/pr-review/`, LLM reads from disk and writes outputs back). Stdio is an optional secondary transport for automated agent pipelines, but must not bypass file-based traceability — all inputs and outputs must be persisted on disk before validation/finalization.
- Q: What happens when `finalize` is called without all layers completed? → A: `finalize` must block. Every required onion layer must end in a terminal state: `completed`, `skipped_with_reason`, or `failed`. Missing layers block finalize. Skipped layers require a skip record (layer name, reason, decision source, coverage impact, recommendation downgrade). Failed layers require a failure record. Incomplete reviews must not be silently presented as complete.
- Q: How should Canon optimize the LLM context handoff for token efficiency? → A: Progressive context discovery. Canon provides compact context indexes (TSV for large flat lists, Markdown for instructions, JSON for validated schemas and canonical outputs). The LLM starts from a small map of the review context and progressively expands only the files, hunks, ranges, callers, tests, docs, or contracts needed for the current review layer. Less context upfront, more precise context on demand, traceable evidence for every finding. Optional read-only helper commands (`canon pr-review context --show <id>`, `--related <id>`, `--tests <id>`) support incremental inspection.

## User Scenarios & Testing

### User Story 1 - Onion-Layer Review Context Preparation (Priority: P1)

As an LLM agent performing a local PR review, I want Canon to prepare a layered review context so that I can inspect not only the diff but also the full file, related files, call sites, contracts, and tests affected by the change.

**Why this priority**: Without layered context, the LLM can only perform shallow diff-only review, which is the core problem being solved.

**Independent Test**: Run `canon pr-review prepare --base main --head HEAD` on a fixture repository and verify that Canon emits a review context packet containing diff-level, file-level, relation-level, edge-case, and test-context sections.

**Acceptance Scenarios**:

1. **Given** a changed source file, **When** `prepare` runs, **Then** Canon includes the diff hunk, the full modified file path, and sets the run state to `awaiting_diff_review`.
2. **Given** the run state is `awaiting_diff_review`, **When** the LLM completes diff review, **Then** Canon records the layer as `diff_review_recorded` and advances to `awaiting_whole_file_review`.
3. **Given** a changed public API or contract file, **When** `prepare` runs, **Then** Canon marks it as high-risk and requests caller/consumer inspection in the related-context layer instructions.
4. **Given** the run state is `awaiting_whole_file_review`, **When** the LLM requests full file content, **Then** Canon provides or exposes the full modified files.
5. **Given** a large diff (>20 files or >500 lines), **When** `prepare` runs, **Then** Canon identifies high-risk files first and records skipped or sampled areas in coverage metadata.

---

### User Story 2 - LLM-Governed Semantic Review (Priority: P1)

As an external LLM agent, I want clear instructions for onion-layer review so that I can produce actionable findings based on diff, whole-file, related-file, edge-case, and test evidence.

**Why this priority**: The reviewer instructions are the contract between Canon and the LLM; without them, the LLM cannot produce structured, useful output.

**Independent Test**: Use a deterministic stub reviewer output following the generated reviewer schema and verify that Canon can accept and finalize it.

**Acceptance Scenarios**:

1. **Given** a prepared review packet, **When** the LLM reads `reviewer-instructions.md`, **Then** it receives step-by-step onion-layer review instructions.
2. **Given** a change in one function, **When** the LLM reviews it, **Then** the expected reviewer output can include findings about caller impact, whole-file state, error handling, performance, and tests.
3. **Given** no accepted reviewer output, **When** finalize is requested, **Then** Canon must not claim that actionable review was executed.

---

### User Story 3 - Reviewer Output Acceptance And Validation (Priority: P1)

As a reviewer, I want Canon to validate structured reviewer output before rendering artifacts so that invalid paths, fabricated lines, malformed JSON, and inconsistent severities cannot become GitHub-ready comments.

**Why this priority**: Validation is the gate between untrusted LLM output and trusted review artifacts.

**Independent Test**: Provide valid and invalid `reviewer-output.json` fixtures to `canon pr-review accept` and verify accepted and rejected states.

**Acceptance Scenarios**:

1. **Given** valid reviewer output, **When** `accept` runs, **Then** Canon validates schema, comment IDs, severities, and target paths.
2. **Given** invalid JSON, **When** `accept` runs, **Then** Canon records `actionable_review_failed`.
3. **Given** a line target that does not apply to the diff, **When** `accept` runs, **Then** Canon downgrades it to hunk-level or global finding, or rejects it according to policy.
4. **Given** duplicate comment IDs, **When** `accept` runs, **Then** Canon rejects the reviewer output.

---

### User Story 4 - Reviewer-Facing Artifact Rendering (Priority: P1)

As a PR reviewer or author, I want generated review artifacts to be useful and copy-ready so that I can apply comments manually or through automation.

**Why this priority**: Artifacts are the primary deliverable of pr-review.

**Independent Test**: Given an accepted reviewer output fixture, run `finalize` and verify that Markdown and JSON artifacts are generated with matching IDs and coherent recommendation.

**Acceptance Scenarios**:

1. `01-review-summary.md` gives recommendation, rationale, severity summary, review status, coverage, and governance notes.
2. `02-conventional-comments.md` groups comments by severity, file path, and global scope.
3. `03-github-comments.json` contains the same actionable comments as the Markdown file.
4. `06-review-report.md` summarizes issues by severity and provides `Approve`, `Comment`, or `Request changes`.
5. Governance-only observations remain secondary and do not pollute actionable comments.

## Architecture

### LLM Context Format Strategy

Canon optimizes for token-efficient progressive discovery rather than large duplicated context payloads.

| Format | Purpose |
|---|---|
| **Markdown** | Instructions, review briefs, review plans, human-readable guidance, LLM-authored layer outputs |
| **TSV** | Compact indexes and large flat lists intended for LLM scanning — avoids repeated field names |
| **JSON** | State, schemas, canonical machine outputs, final automation artifacts |
| **Patch files** | Raw `git diff` |
| **Repository files** | Referenced by path and line range, inspected on demand |

The LLM should start from a small map of the review context and progressively expand only the files, hunks, ranges, callers, tests, docs, or contracts needed for the current review layer.

Design principle:

```text
less context upfront
more precise context on demand
traceable evidence for every finding
```

### File-Based Handoff (Normative)

Canon and the LLM agent communicate through the filesystem. All context packets,
instructions, layer outputs, and final review output are persisted
under `.canon/runs/<run-id>/pr-review/`.

```
.canon/runs/<run-id>/pr-review/
├── run-state.json
├── review-brief.md
├── review-plan.md
├── context-index.tsv
├── context-index.json
├── changed-files.tsv
├── high-risk-files.tsv
├── relation-hints.tsv
├── diff.patch
├── reviewer-output.schema.json
├── layers/
│   ├── 01-diff/
│   │   ├── instructions.md
│   │   ├── required-context.tsv
│   │   └── output.md
│   ├── 02-whole-file/
│   │   ├── instructions.md
│   │   ├── required-context.tsv
│   │   └── output.md
│   ├── 03-related-context/
│   │   ├── instructions.md
│   │   ├── required-context.tsv
│   │   └── output.md
│   ├── 04-logical-stress/
│   │   ├── instructions.md
│   │   ├── required-context.tsv
│   │   └── output.md
│   └── 05-tests/
│       ├── instructions.md
│       ├── required-context.tsv
│       └── output.md
├── reviewer-output.md
└── canonical-review-output.json
```

### File Responsibilities

| File | Responsibility |
|---|---|
| `review-brief.md` | Short human/LLM-readable summary: base/head, review mode, expected outcome |
| `review-plan.md` | Onion-layer sequence, required layers, progressive context expansion guide, evidence recording rules |
| `context-index.tsv` | Primary LLM-facing context map — compact table of IDs, types, paths, line ranges, reasons, risk, and review layer |
| `context-index.json` | Machine-readable equivalent of the context index, used by Canon for validation |
| `changed-files.tsv` | Compact changed-file inventory |
| `high-risk-files.tsv` | Changed files Canon considers high-risk, with risk reason and suggested review layer |
| `relation-hints.tsv` | Caller, test, example, doc, contract, and export hints for related-context review |
| `diff.patch` | Raw `git diff` |
| `layers/<NN>-<layer>/instructions.md` | Layer-specific instructions for the LLM |
| `layers/<NN>-<layer>/required-context.tsv` | Compact list of context IDs recommended for that layer |
| `layers/<NN>-<layer>/output.md` | LLM-authored output for that layer (Markdown with structured sections) |
| `reviewer-output.md` | Final LLM-authored aggregate review output |
| `canonical-review-output.json` | Canon-generated validated machine representation |

### Context Index Format

The context index is the primary LLM-facing map of the review. Example:

```tsv
id	type	path	start_line	end_line	reason	risk	layer
C001	diff	src/transport/http.rs	42	96	timeout behavior changed	high	diff
C002	file	src/transport/http.rs			full file needed for timeout semantics	high	whole_file
C003	related	src/client.rs	120	210	caller handles transport result	medium	related_context
C004	test	tests/transport_timeout.rs			related timeout tests	high	tests
C005	doc	docs/timeout-policy.md			documented timeout behavior may be stale	medium	related_context
```

The LLM should use these context IDs when describing what it inspected and when citing evidence for findings.

### Progressive Retrieval Commands

Optional read-only helper commands for incremental context inspection:

```bash
canon pr-review context --run <id> --list
canon pr-review context --run <id> --show <context-id>
canon pr-review context --run <id> --show <context-id> --range <start>..<end>
canon pr-review context --run <id> --related <context-id>
canon pr-review context --run <id> --tests <context-id>
canon pr-review context --run <id> --explain <context-id>
```

Rules:
- Context IDs are stable within a run.
- Helper commands are read-only.
- Helper commands read local repository files and Canon run artifacts.
- Output is human-readable by default; `--json` may be supported for automation.
- Any context used as evidence must be recorded in review coverage.

### LLM Output vs Canonical Output

| Layer | LLM-authored | Canon-generated |
|---|---|---|
| Per-layer findings | `layers/<NN>-<layer>/output.md` | — |
| Aggregate review | `reviewer-output.md` | `canonical-review-output.json` |
| Comments | — | `github-comments.json` |
| Findings | — | `review-findings.json` |
| Artifacts | — | `01-review-summary.md`, `02-conventional-comments.md`, `06-review-report.md` |

Canon parses and validates the structured parts of LLM-authored Markdown files, then compiles them into validated JSON and final Markdown artifacts. This gives the LLM a natural authoring format while keeping Canon responsible for validated machine outputs.

## Onion-Layer Review Model

### Orchestration Workflow

The normative workflow is NOT a single `prepare → LLM review → accept → finalize` pipeline. Canon must orchestrate five distinct LLM review steps, each producing its own findings and advancing the run state.

```text
1. canon pr-review prepare --base <base> --head <head>
   Canon creates the run, collects diff, classifies surfaces,
   produces initial review context, sets state to awaiting_diff_review.

2. LLM Step 1: Diff review (Layer: diff)
   Canon provides diff hunks, changed files, high-risk classification.
   LLM summarizes intent, identifies obvious local issues.
   Canon records this layer as diff_review_recorded.

3. LLM Step 2: Whole-file review (Layer: whole_file)
   Canon provides/exposes full modified files.
   LLM checks local invariants, state, error handling, concurrency, performance.
   Canon records this layer as whole_file_review_recorded.

4. LLM Step 3: Related-context review (Layer: related_context)
   Canon provides related context hints: callers, usages, tests, examples, docs,
   contracts, public API exports.
   LLM reviews impact outside the changed file.
   Canon records this layer as related_context_review_recorded.

5. LLM Step 4: Logical stress review (Layer: logical_stress)
   LLM acts as QA/security reviewer: edge cases, malformed inputs, async behavior,
   retries, timeouts, null/empty states, invalid transitions.
   Canon records this layer as stress_review_recorded.

6. LLM Step 5: Test review (Layer: tests)
   Canon provides changed tests and related tests.
   LLM checks whether tests cover the actual behavioral change, identifies missing tests.
   Canon records this layer as test_review_recorded.

7. canon pr-review accept --run <run-id> --reviewer-output <path>
   Canon validates the final structured reviewer output: schema, paths, line targets,
   hunks, severity values, comment IDs, and coverage. Rejects malformed output.

8. canon pr-review finalize --run <run-id>
   Canon renders all artifacts and sets state to finalized.
```

### Run States

```text
prepared
awaiting_diff_review
diff_review_recorded
awaiting_whole_file_review
whole_file_review_recorded
awaiting_related_context_review
related_context_review_recorded
awaiting_stress_review
stress_review_recorded
awaiting_test_review
test_review_recorded
reviewer_output_accepted
reviewer_output_rejected
finalized
```

### Canonical Layer Names

```text
diff
whole_file
related_context
logical_stress
tests
global
```

Every reviewer finding MUST include the `layer` field identifying which layer produced it.

### Completion Rule

A review is not complete just because the diff was inspected.

Each layer must end in one of these terminal states:

| State | Meaning |
|---|---|
| `completed` | The layer was executed successfully |
| `skipped_with_reason` | The layer was intentionally skipped with a recorded reason |
| `failed` | The layer execution failed |

`finalize` is allowed only when every required layer has a terminal state.

**Skip record** must include: layer name, reason, operator/agent decision source, impact on review coverage, whether the skip downgrades the recommendation, and timestamp.

**Failure record** must include: layer name, failure reason, whether partial output exists, whether the review can continue, and impact on final recommendation.

If important layers are skipped, the recommendation must usually be downgraded to `Comment`, unless governance policy requires `Request changes`.

`Approve` is allowed only when required layers are completed or skipped for reasons that do not materially reduce review confidence.

Example — allowed: `diff` and `whole_file` completed, `related_context` skipped because no related files were found, `logical_stress` completed, `tests` skipped because no tests exist in the repository → finalize allowed with coverage limitations reported.

Example — blocked: `diff` and `whole_file` completed, but `logical_stress` and `tests` missing → block finalize.

This keeps the review honest: incomplete is acceptable only when explicitly declared, not silently hidden.

### Layer 1: Diff Analysis

Purpose: understand what changed and infer the likely intent.

Canon provides: changed files, diff hunks, added/removed lines, file status, high-risk changed surfaces, touched tests and docs.

LLM checks: obvious bugs, typo/debug leftovers, incompatible changed logic, changed behavior not reflected in tests, changed public API or contract semantics, whether the change intent is clear.

### Layer 2: Whole-File Analysis

Purpose: inspect the modified code inside its immediate file context.

Canon provides: full file paths for modified files, instructions for LLM to read full files when needed, file-level risk hints, symbols/functions/classes touched when available.

LLM checks: local invariants, state consistency, error handling, lifetime/resource management, concurrency/thread-safety, duplicated logic, performance inside the file, whether the diff conflicts with the surrounding design.

### Layer 3: Related Files And Call Graph Context

Purpose: inspect callers, consumers, tests, contracts, examples, and dependent modules.

Canon provides candidate relation hints using deterministic local mechanisms: changed symbol names extracted from diff, references found through text search, related tests by naming convention, examples importing changed APIs, contract files touching the changed module, docs mentioning changed APIs, public API exports, module declarations.

LLM checks: caller compatibility, changed return values or errors not handled by callers, idempotency and side-effect changes, serialization/deserialization compatibility, public contract drift, examples/docs that no longer match behavior, tests that no longer cover the right behavior.

### Layer 4: Logical Stress Test

Purpose: use the LLM as a devil's advocate.

LLM checks: null/none/empty inputs, malformed inputs, oversized inputs, timeout and retry behavior, race conditions, async cancellation, partial failure, error propagation, security and secret exposure, resource exhaustion, boundary conditions, non-idempotent retries, invalid state transitions.

### Layer 5: Test And Verification Review

Purpose: make the change verifiable.

Canon provides: changed tests, related test files, test command hints, coverage hints when available, missing-test output contract.

LLM checks: new branches not tested, changed errors not tested, changed public API not tested, failure paths not tested, compatibility not tested, performance-sensitive paths not covered, tests that merely mirror implementation, weak tests that do not assert behavior.

## Requirements

### Functional Requirements

- **FR-001**: Canon MUST implement `pr-review prepare`, `pr-review accept`, and `pr-review finalize` as distinct CLI phases.
- **FR-002**: `prepare` MUST emit diff, changed files, review context, high-risk files, related-context hints, reviewer schema, and reviewer instructions.
- **FR-003**: Reviewer instructions MUST require onion-layer review: diff, whole file, related files, logical stress, and tests.
- **FR-004**: Canon MUST NOT claim actionable review executed unless reviewer output was accepted successfully.
- **FR-005**: Canon MUST NOT convert invalid reviewer output into empty successful review.
- **FR-006**: Canon MUST validate reviewer output schema before rendering actionable artifacts.
- **FR-007**: Canon MUST validate paths, line targets, hunk targets, comment IDs, severity values, and recommendation values.
- **FR-008**: Canon MUST render `conventional-comments.md` from the canonical actionable comment set.
- **FR-009**: Canon MUST render `github-comments.json` from the same canonical actionable comment set.
- **FR-010**: Canon MUST ensure Markdown and JSON comment IDs match.
- **FR-011**: Canon MUST render global comments at the end of `conventional-comments.md`.
- **FR-012**: Canon MUST sort file comments lexicographically by path and then by severity and target location.
- **FR-013**: Canon MUST render a review report summarizing findings by severity and giving a final recommendation.
- **FR-014**: Canon MUST keep governance-only findings out of actionable comments unless converted into concrete review findings.
- **FR-015**: Canon MUST render governance notes as secondary artifacts.
- **FR-016**: Canon MUST record review coverage, inspected files, skipped files, and limitations.
- **FR-017**: Canon MUST reject internally inconsistent review summaries.
- **FR-018**: Canon MUST support deterministic reviewer output fixtures for tests.
- **FR-019**: Canon MUST NOT require live LLM calls in CI.

- **FR-020**: Canon MUST generate layer-specific instructions for each of the five review layers.
- **FR-021**: Canon MUST expose or produce layer-specific context packets for each review layer.
- **FR-022**: Canon MUST record which review layers were executed.
- **FR-023**: Canon MUST record which files were inspected deeply at each layer.
- **FR-024**: Canon MUST record which related files were skipped and why.
- **FR-025**: Canon MUST reject or downgrade final recommendations when required layers were skipped without explanation.
- **FR-026**: Every reviewer finding MUST include the `layer` field identifying which onion layer produced it.
- **FR-027**: Canon MUST advance the run state through each layer's state transition upon layer completion.
- **FR-028**: Canon MUST block `finalize` when any required onion layer has no terminal state (not `completed`, `skipped_with_reason`, or `failed`). Incomplete reviews must not be presented as complete; all skipped or failed layers must be reflected in the coverage metadata and final recommendation.
- **FR-029**: Canon MUST require a skip record (layer name, reason, decision source, coverage impact, recommendation downgrade, timestamp) for each skipped layer.
- **FR-030**: Canon MUST require a failure record (layer name, failure reason, partial output existence, continuability, recommendation impact) for each failed layer.
- **FR-032**: Canon MUST optimize LLM context handoff for progressive disclosure rather than large duplicated context payloads.
- **FR-033**: Canon MUST provide compact context indexes (TSV) with file paths, optional line ranges, risk hints, relation hints, and layer relevance.
- **FR-034**: Canon SHOULD use TSV for large flat LLM-facing indexes.
- **FR-035**: Canon MUST use Markdown for LLM-facing instructions, review briefs, and review plans.
- **FR-036**: Canon MUST use JSON for validated state, schemas, canonical machine outputs, and automation artifacts.
- **FR-037**: Canon MUST distinguish LLM-authored outputs (Markdown) from Canon-generated canonical outputs (JSON, final Markdown artifacts).
- **FR-038**: Canon MUST provide enough context references for the LLM to inspect whole files and related files on demand. At minimum, the context index must include all changed files as `diff` or `file` entries, plus any relation hints found (callers, tests, docs, contracts, examples) based on deterministic text search and naming conventions.
- **FR-039**: Canon MUST record review coverage using context IDs, inspected files, skipped files, and limitations.
- **FR-040**: Canon MUST avoid duplicating full file content across every layer when a path, range, and context ID are sufficient.
- **FR-041**: Canon SHOULD provide read-only helper commands for progressive context retrieval (`context --list`, `--show`, `--related`, `--tests`, `--explain`).

### Key Entities

- **Review Context Packet**: The output of `prepare`, containing diff, changed files, high-risk classification, related-context hints, reviewer schema, and reviewer instructions. Extended with layer-specific context packets emitted at each layer transition.
- **Reviewer Output**: The structured JSON produced by the LLM after all layers are complete, containing findings (each with a `layer` field), coverage, missing tests, and recommendation.
- **Review Finding**: A single actionable observation with severity, layer (`diff`/`whole_file`/`related_context`/`logical_stress`/`tests`/`global`), target path/line/hunk, and linked comment.
- **Canonical Comment Set**: The normalized set of actionable comments shared between `conventional-comments.md` and `github-comments.json`.
- **Review State**: The layer-by-layer run state machine: `prepared` → `awaiting_diff_review` → `diff_review_recorded` → `awaiting_whole_file_review` → `whole_file_review_recorded` → `awaiting_related_context_review` → `related_context_review_recorded` → `awaiting_stress_review` → `stress_review_recorded` → `awaiting_test_review` → `test_review_recorded` → `reviewer_output_accepted`/`reviewer_output_rejected` → `finalized`.
- **Actionable Review Status**: `actionable_review_executed`, `actionable_review_failed`, `actionable_review_not_provided`, `governance_only`.
- **Review Layer**: One of the five onion layers: `diff`, `whole_file`, `related_context`, `logical_stress`, `tests`, plus the global layer `global` for PR-level findings.

## Success Criteria

- **SC-001**: A reviewer can open `02-conventional-comments.md` and manually copy useful comments into a PR.
- **SC-002**: `03-github-comments.json` contains the same actionable comments as the Markdown file.
- **SC-003**: `06-review-report.md` shows severity counts and final recommendation.
- **SC-004**: Empty actionable comments are always explained by explicit review status.
- **SC-005**: A fixture reviewer output can produce non-empty comments end-to-end without a live LLM.
- **SC-006**: The LLM instructions force onion-layer review rather than diff-only review.
- **SC-007**: Governance findings remain secondary.
- **SC-008**: Canon no longer emits formally valid but substantively empty review packets without explanation.

## Assumptions

- The LLM agent has filesystem access to read full files referenced in the review context.
- The LLM agent can execute text search to find callers and references.
- The review is scoped to a single branch diff (base..head).
- The operator provides reviewer output as a file path to `accept`.

## Non-Goals

- Canon does not directly call a specific LLM provider from the core `pr-review` mode.
- Canon does not submit comments to GitHub through the GitHub API.
- Canon does not guarantee exhaustive review for very large diffs.
- Canon does not hardcode provider-specific review behavior.
- Tests do not assert that a live LLM finds a specific bug.

## Open Questions

- Should Canon provide helper commands such as `inspect-file`, `inspect-symbol`, or `inspect-related` in this feature, or should those be follow-up enhancements?
- Should related-context hints be based only on text search in V1, or should language-aware tools be added per ecosystem later?
- Should the reviewer output allow suggested code patches, or only comments and remediation text in V1?
- Should the context retrieval helper commands (`canon pr-review context --show`, `--related`, `--tests`) be implemented in this slice, or should V1 only generate compact context indexes and rely on the host IDE/agent to open referenced files?
