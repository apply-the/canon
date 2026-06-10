# Feature Specification: Early Signal Pass (First-Pass Risk Discovery)

**Feature Branch**: `075-pr-review-early-signal-pass`

**Created**: 2026-06-09

**Status**: Draft

**Input**: User description: "Replace the 'quick wins' wording with 'early signal pass' or 'first-pass risk discovery'. The early signal pass is not a substitute for the full review workflow. It is the first step used to discover high-confidence problems quickly before continuing into deeper review layers. Canon must not stop after the early signal pass."

## Summary

The Canon PR review workflow introduces a structured **early signal pass** as the first of seven ordered review layers. This pass rapidly identifies obvious high-confidence problems — broken builds, stale manifests, schema drift, removed-but-still-referenced files, missing tests for changed behavior, naming drift, and validation failures. The early signal pass replaces the informal "quick wins" concept with a named, governed first layer that produces traceable findings. Critically, Canon must never treat this pass as complete review coverage; finalization is gated on evidence that all high-risk layers were either reviewed or explicitly deferred.

The seven-layer review flow is:

1. **Early signal pass** — high-confidence, low-effort findings (broken builds, stale manifests, schema drift, dangling references, missing tests, naming drift, validation failures). Executed by Canon inside `prepare`.
2. **Application-source review** — deep inspection of every changed file in its own context. Performed by the LLM agent.
3. **High-risk surfaces review** — focused review of files/patterns matching known risk heuristics. Performed by the LLM agent.
4. **Related-context review** — inspection of call sites, dependents, and neighboring modules. Performed by the LLM agent.
5. **Logical stress review** — edge-case, concurrency, and invariant stress-testing. Performed by the LLM agent.
6. **Tests review** — coverage gap analysis and test-quality assessment. Performed by the LLM agent.
7. **Coverage accounting and final recommendation** — honest enumeration of what was and was not covered, with explicit deferral reasons. Compiled by Canon at `finalize`.

**Responsibility split**: Canon runs deterministic preparation (layer 1, file classification, context indexes, ordered instructions, validation at `accept`, artifact rendering at `finalize`). The LLM agent performs semantic reasoning for layers 2–6 and writes structured output into `layers/<NN>-<name>/output.md`. Canon never performs semantic code review; it structures, validates, and records.

## Clarifications

### Session 2026-06-09

- Q: How does the reviewing agent invoke the early signal pass? → A: Default-on inside `canon pr-review prepare --base <ref> --head <ref>`, with explicit opt-out `--skip-early-signal` for advanced/debug scenarios. If skipped, the run records `early_signal_status = "skipped_with_reason"`, the skip reason, operator/agent source, and impact on review confidence. `finalize` must include the early signal status in the coverage section. The CLI surface stays: `prepare` → `accept` → `finalize`.
- Q: What observability signals should the early signal pass emit? → A: Dual-channel: structured JSON events on stdout for agent consumption (when `--output json`), plus persisted JSONL trace events under `.canon/runs/<run_id>/pr-review/traces/early-signal.jsonl` for audit. Persisted artifacts include `findings.tsv`, `findings.json`, `summary.md`, and `trace.jsonl`. Event types: `early_signal.started`, `early_signal.file_classified`, `early_signal.finding_detected`, `early_signal.completed`, `early_signal.skipped`, `early_signal.failed`. Finding IDs must be stable across stdout, trace, and artifacts.
- Q: How does the agent progress between review layers? → A: `canon pr-review prepare` generates the full ordered review plan and all layer packets in ONE invocation — Canon runs deterministic preparation (early signal pass, file classification, context indexes, ordered instructions), then sets state to `awaiting_reviewer_output`. The LLM agent performs semantic review layer by layer, writing output into `layers/<NN>-<name>/output.md`, then invokes `canon pr-review accept` to validate. Validation enforces that required layers are either completed or explicitly deferred with reasons. `finalize` must not infer layer completion from instruction presence alone.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Reviewer receives early signal findings before deep review (Priority: P1)

A PR reviewer (human or LLM agent) invokes Canon to review a pull request. Canon executes the early signal pass first, producing a structured report of obvious problems — for example a broken build command, a removed file still imported elsewhere, or a stale manifest entry. These findings are surfaced immediately, before deeper layers begin. The reviewer can act on critical early signals (e.g., fix a build break) before investing time in deeper review.

**Why this priority**: The early signal pass is the entry point of every review. It runs automatically inside `canon pr-review prepare` — the agent invokes a single command and gets early signal findings plus the review plan. If it fails silently or is skipped, subsequent layers may waste time on changes that cannot even build or contain trivially broken references.

**Independent Test**: Run `canon pr-review prepare --base main --head feature-branch` on a fixture PR with a deliberate broken build command and a stale manifest entry. Verify the output includes both early signal findings with clear location references and severity classification, before the review plan sections. Verify no separate subcommand is needed.

**Acceptance Scenarios**:

1. **Given** a PR where `Cargo.toml` references a dependency version that does not exist, **When** the agent runs `canon pr-review prepare --base main --head feature-branch`, **Then** a finding is emitted with the file path, line reference, and severity "blocking", before the review plan output.
2. **Given** a PR that deletes `src/validator.rs` but leaves an import in `src/main.rs`, **When** the agent runs `canon pr-review prepare --base main --head feature-branch`, **Then** a finding identifies the dangling reference with both file locations.
3. **Given** a PR where all early signal checks pass cleanly, **When** the agent runs `canon pr-review prepare`, **Then** the pass reports zero findings and the workflow proceeds to generate the remaining review plan and context indexes.
4. **Given** an advanced/debug scenario, **When** the agent runs `canon pr-review prepare --skip-early-signal`, **Then** the run metadata records `early_signal_status = "skipped_with_reason"` with the skip reason and impact on review confidence, and no early signal findings are emitted.

---

### User Story 2 — Canon never stops after early signal pass alone (Priority: P1)

After the early signal pass completes (with or without findings), Canon must automatically continue to the next review layer. There is no workflow exit point that allows finalization after layer 1 only. The agent or reviewer cannot "approve" or "finalize" the review based solely on the early signal pass.

**Why this priority**: This is the core safety invariant. If Canon stops after the early signal pass, the review is incomplete and misleading — high-confidence fast checks are not a substitute for deep review.

**Independent Test**: Run a review on a real PR. After the early signal pass layer completes, attempt to finalize the review. Verify Canon rejects finalization and requires continuation through subsequent layers.

**Acceptance Scenarios**:

1. **Given** a review that has only completed the early signal pass (layer 1), **When** an agent attempts to finalize the review, **Then** Canon rejects finalization with a message indicating which layers are still pending.
2. **Given** a review that has completed all seven layers or explicitly deferred remaining high-risk layers, **When** an agent finalizes the review, **Then** Canon accepts finalization and records the coverage accounting.
3. **Given** a review where layer 1 found blocking findings, **When** the agent continues to layer 2, **Then** Canon allows continuation and carries forward the layer 1 findings as context.

---

### User Story 3 — Coverage accounting forces honest enumeration (Priority: P2)

When a review reaches finalization, Canon must produce a coverage accounting that honestly lists which high-risk areas were reviewed and which were deferred. Deferrals must carry explicit reasons. The final recommendation is only valid when this accounting is present.

**Why this priority**: Without honest coverage accounting, reviewers can skip difficult areas without traceability, creating a false sense of completeness.

**Independent Test**: Run a review on a complex PR. Explicitly defer review of one high-risk file. Finalize. Verify the coverage accounting lists the deferred file with the stated reason.

**Acceptance Scenarios**:

1. **Given** a review where the logical stress layer was skipped, **When** the agent finalizes, **Then** the coverage accounting records "logical stress review: deferred — reason: [explicit reason]" and the final recommendation notes the gap.
2. **Given** a review where all seven layers were completed, **When** the agent finalizes, **Then** the coverage accounting shows all layers as reviewed with artifact references.
3. **Given** a review where a high-risk file was deferred without a reason, **When** the agent attempts to finalize, **Then** Canon rejects finalization and requires an explicit deferral reason for that file.

---

### Edge Cases

- What happens when the early signal pass itself fails to execute (e.g., build command hangs)? Canon must emit `early_signal.failed` with the error and partial findings count, persist the failure to trace, and still generate the remaining review plan and layer instructions. The review plan must note the incomplete early signal status and recommend deferring affected layers to a follow-up review or manual inspection.
- What happens when a finding overlaps between the early signal pass and a deeper layer? Canon must deduplicate findings by canonical location reference, keeping the earliest severity.
- How does Canon handle PRs with zero changed files (e.g., only metadata changes)? The early signal pass may produce zero findings; the review still proceeds through all layers but layers 4-6 may report "nothing to review" rather than being skipped.
- What if the diff is too large for all seven layers to complete within a reasonable time budget? Canon should allow explicit layer deferral with reasons, not implicit skipping.
- What happens when `--skip-early-signal` is used without a reason? Canon must reject the invocation with a validation error requiring a non-empty skip reason.
- What happens when `--skip-early-signal` is used but the review later reports "no issues found"? Canon must still record the skipped status and reduced confidence; the final report must not imply early-risk coverage was achieved.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST define a named early signal pass as the first of seven ordered review layers in the PR review workflow.
- **FR-002**: System MUST replace all existing "quick wins" terminology with "early signal pass" or equivalent "first-pass risk discovery" language in code, docs, and user-facing output.
- **FR-003**: System MUST automatically execute the early signal pass as the first step of every `pr-review` workflow invocation.
- **FR-004**: System MUST produce structured, traceable findings from the early signal pass, each with at minimum: file location, line reference, description, and severity classification.
- **FR-005**: The early signal pass MUST check for exactly the following seven categories: broken build commands, stale manifests, schema drift, removed files still referenced, missing tests for changed behavior, naming drift, and validation failures. Additional check rules are deferred to a future extensibility mechanism and are out of scope for this feature.
- **FR-006**: System MUST NOT allow review finalization after completing only the early signal pass (layer 1). Finalization requires either all seven layers completed or explicit deferral with reasons for any skipped high-risk layers.
- **FR-007**: System MUST enforce the ordered layer progression: early signal pass → application-source review → high-risk surface review → related-context review → logical stress review → test coverage review → coverage accounting and final recommendation.
- **FR-008**: System MUST produce a coverage accounting artifact at finalization that honestly enumerates which layers were reviewed, which were deferred, and the explicit reasons for any deferrals.
- **FR-009**: System MUST reject finalization when a high-risk area is deferred without an explicit reason.
- **FR-010**: System MUST deduplicate findings within the early signal pass by canonical location reference (file path + line range), preserving the earliest-assigned (highest) severity when multiple rules flag the same location. Cross-layer deduplication (e.g., early signal finding confirmed by application-source review) is deferred to the reviewer output validation in `accept`.
- **FR-011**: System MUST execute the early signal pass as a default-on, implicit first step of `canon pr-review prepare`. The agent SHALL NOT need to invoke a separate subcommand for the early signal pass.
- **FR-012**: System MUST provide an explicit opt-out flag `--skip-early-signal` on `canon pr-review prepare` for advanced or debugging scenarios.
- **FR-013**: System MUST record, in persistent run metadata, the early signal status as `"completed"` or `"skipped_with_reason"`. When skipped, the metadata MUST additionally capture: the skip reason, the operator or agent source, and the impact on review confidence.
- **FR-014**: System MUST include the early signal status (completed or skipped) in the coverage section of the `finalize` output. A skipped early signal MUST reduce the review confidence assessment and MUST NOT allow the final report to imply full early-risk coverage.
- **FR-015**: System MUST validate that `--skip-early-signal` includes a non-empty skip reason when used interactively or via governance adapter; a missing reason SHALL cause the command to fail with a validation error.
- **FR-016**: System MUST emit agent-consumable structured JSON events on stdout when `--output json` is selected during `canon pr-review prepare`. Events SHALL include lifecycle transitions (`early_signal.started`, `early_signal.completed`, `early_signal.skipped`, `early_signal.failed`) and per-finding events (`early_signal.finding_detected`).
- **FR-017**: System MUST persist early signal trace events as JSONL under `.canon/runs/<run_id>/pr-review/traces/early-signal.jsonl`. Trace persistence MUST NOT require `--output json` and MUST include lifecycle events, rule execution events, skipped-rule reasons, detected findings, classification counts, timing information, and error/failure information.
- **FR-018**: Each `early_signal.finding_detected` event (both stdout and trace) MUST include: run_id, rule_id, finding_id, severity, category, path, optional line range, evidence context IDs, short summary, suggested next review layer, and whether the finding is an actionable review comment candidate.
- **FR-019**: System MUST emit a final early signal summary event containing: total files classified, total findings, findings by severity, findings by bucket, high-risk files identified, suggested next layers, and early signal status.
- **FR-020**: Finding IDs MUST be stable and consistent across all outputs: stdout JSON, persisted trace, `findings.json`, `findings.tsv`, `summary.md`, and final review artifacts.
- **FR-021**: System SHOULD persist the early signal result as artifacts under `.canon/runs/<run_id>/pr-review/early-signal/`: `findings.tsv` (LLM-scannable), `findings.json` (Canon-validatable), `summary.md` (human-readable), and `trace.jsonl` (audit/debug).
- **FR-022**: System MUST include rule IDs for detected findings to support debugging and test assertions.
- **FR-023**: System MUST emit a structured failure event before exiting when the early signal pass cannot complete, and MUST record the failure in the persisted trace.
- **FR-024**: `canon pr-review prepare` MUST generate the full ordered review plan and all layer directories in a single invocation. The command SHALL run deterministic preparation (early signal pass, file classification, context indexes, ordered layer instructions), then set the run state to `awaiting_reviewer_output` or equivalent.
- **FR-025**: `prepare` MUST generate the following per-layer artifacts under `layers/<NN>-<name>/`: `instructions.md` (what to review), `required-context.tsv` (files/context to load), and an empty `output.md` placeholder for the reviewer to fill. The layer order MUST be recorded in `review-plan.md`.
- **FR-026**: `prepare` MUST mark only the early signal pass (layer 1) as executed by Canon. Semantic layers (2–7) MUST remain in `pending` status and SHALL only be marked completed when `accept` validates the corresponding `output.md`.
- **FR-027**: `canon pr-review accept` MUST validate that every required layer has either a valid `output.md` with coverage record, or an explicit deferral with reason recorded. Layers with only `instructions.md` but no reviewer output MUST NOT be accepted as completed.
- **FR-028**: `canon pr-review finalize` MUST NOT infer layer completion from the presence of `instructions.md`. A layer is treated as completed only when the accepted reviewer output contains a valid output and coverage record.
- **FR-029**: Canon SHALL NOT require one CLI invocation per review layer. The CLI surface remains `prepare` → `accept` → `finalize`. Canon is responsible for deterministic preparation and validation; the LLM agent is responsible for performing the semantic review and writing layer outputs.

### Key Entities

- **Review Layer**: One of the seven ordered phases in the review workflow. Each layer has a name, an ordinal position, a completion status (pending/in-progress/completed/deferred), and zero or more Findings.
- **Early Signal Finding**: A high-confidence problem discovered during layer 1. Key attributes: file location, line reference, description, severity (blocking/high/medium/low/info), category, rule_id, detection rule reference, and an actionable comment candidate flag.
- **Early Signal Event**: A structured JSON record emitted during early signal execution. Event types: `started`, `file_classified`, `finding_detected`, `completed`, `skipped`, `failed`. Each event carries a run_id and, where applicable, finding IDs that match across stdout, trace, and persisted artifacts.
- **Coverage Accounting**: The final artifact listing each review layer with its status (reviewed/deferred) and, for deferrals, an explicit reason string. Must include the early signal status — either `"completed"` or `"skipped_with_reason"` with the corresponding skip metadata. Produced at finalization. Contains an `overall_confidence` field with one of: `high` (all layers reviewed, no skip), `medium` (one layer deferred or early signal skipped), `low` (multiple layers deferred), `insufficient` (no semantic layers reviewed). A skipped early signal pass automatically caps confidence at `medium` or lower.
- **Review Finalization**: The terminal state of a review workflow. Only reachable when coverage accounting is present, all layers are either reviewed or deferred with reasons, and no blocking early signal findings remain unresolved.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The early signal pass completes and emits findings within 30 seconds for a typical PR (up to 50 changed files).
- **SC-002**: The default (non-skip) execution path always runs the early signal pass as the first layer. Zero invocations bypass it silently — the only way to skip is the explicit `--skip-early-signal` flag with a non-empty reason.
- **SC-003**: In all attempted finalization scenarios, Canon rejects finalization when fewer than all seven layers have been reviewed or explicitly deferred with recorded reasons.
- **SC-004**: Every finalized review produces a coverage accounting artifact containing status entries for all seven layers.
- **SC-005**: No user-facing output or documentation contains the deprecated term "quick wins" after this feature is implemented.
- **SC-006**: When `--output json` is used, stdout contains exactly one `early_signal.completed` or `early_signal.skipped` event per invocation, and finding IDs in stdout match those in the persisted `findings.json`.
- **SC-007**: A `trace.jsonl` file exists under the run's `pr-review/traces/` directory after every early signal pass execution (including skipped and failed passes), and each line is valid JSON.
- **SC-008**: `prepare` generates all seven layer directories with `instructions.md`, `required-context.tsv`, and an empty `output.md` placeholder in a single CLI invocation; no per-layer subcommand is required.

## Assumptions

- The existing `canon pr-review prepare` command (from `074-pr-review-onion-orchestration`) accepts `--base` and `--head` refs; the early signal pass runs as an implicit first step within that command without changing the CLI phase model (`prepare` → `accept` → `finalize`).
- "Quick wins" terminology exists in the current codebase as informal references to preliminary checks; these will be renamed but the underlying check logic may be reused.
- The early signal pass checks are deterministic file-system and manifest inspections; they do not require LLM invocation.
- Layer deferral reasons are free-text strings provided by the reviewing agent; Canon validates presence, not semantic quality.
- The seven-layer order is fixed; reordering or parallelism between layers is out of scope for this feature.
- The `--skip-early-signal` flag is intended for advanced users and debugging; it is not the normal workflow path. When used, Canon must still refuse to imply early-risk coverage in the final report.
