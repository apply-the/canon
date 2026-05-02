# Feature Specification: Guided Run Operations And Review Experience

**Feature Branch**: `038-guided-run-operations`  
**Created**: 2026-05-02  
**Status**: Draft  
**Input**: User description: "Deliver feature 038 as a full guided run operations and review experience slice: unify run/status/result/blocker/approval/publish guidance across CLI and skill-facing outputs, strengthen operator ergonomics for approval-gated blocked resumable and partially publishable runs, preserve the governance adapter and existing approval/evidence semantics, include explicit 0.37.0 version alignment task, docs/changelog task, roadmap cleanup, coverage for touched Rust files, clean clippy and cargo fmt closeout, and implement end-to-end without splitting into slices."

## Governance Context *(mandatory)*

**Mode**: change  
**Risk Classification**: bounded-impact. This work changes operator-facing run,
status, and review guidance across the existing CLI and skill-oriented summary
surfaces, but it does not introduce a new governed mode, new persistence
family, or new approval semantics.  
**Scope In**: run and status summary payloads; result-first operator guidance;
blocked, awaiting-approval, resumable, and partially publishable flows;
action-chip and text fallback ergonomics; CLI markdown output for run or status;
focused tests for summary, next-step, and operator review surfaces; docs,
changelog, roadmap, and version alignment required for the `0.37.0` delivery.  
**Scope Out**: new run lifecycle states; changes to approval policy meaning;
new `.canon/` persistence layout; broad redesign of artifact contracts;
governance-adapter version changes; unrelated mode logic beyond the summary and
operator control surfaces this feature needs.

**Invariants**:

- Canon MUST preserve explicit approval, blocked-gate, evidence, and
  recommendation-only semantics instead of collapsing them into generic success
  or “continue” language.
- Canon MUST keep the CLI and governance adapter as the canonical control
  surfaces; action chips and text next steps remain guidance, not a second
  execution model.
- Canon MUST keep emitted artifact paths, approval targets, and readable packet
  results lossless across run and status summaries.
- Canon MUST NOT introduce a new mode family, hidden planner loop, or implicit
  execution from summary surfaces.

**Decision Traceability**: Decisions and validation evidence for this feature
MUST be recorded in
`specs/038-guided-run-operations/decision-log.md` and
`specs/038-guided-run-operations/validation-report.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Understand Run State Immediately (Priority: P1)

As an operator reading a Canon run or status result, I want the summary to tell
me immediately what state the run is in, what packet is readable, what is
blocking progress, and what I should do next, so I do not have to reconstruct
state from multiple commands.

**Why this priority**: This is the core operator workflow value. If run and
status still require manual reconstruction, the broader review or approval
experience stays fragmented.

**Independent Test**: A reviewer can run Canon on completed, blocked, and
approval-gated cases and determine the primary artifact, blockers, and next
step directly from the run or status output without needing a second command to
understand the situation.

**Acceptance Scenarios**:

1. **Given** a completed run with a readable packet, **When** the operator
   reads `run` or `status`, **Then** Canon surfaces the primary artifact,
   result excerpt, packet summary, and next-step guidance as one coherent
   operator view.
2. **Given** a blocked run with readable artifacts, **When** the operator reads
   `run` or `status`, **Then** Canon distinguishes blocker-driven stoppage from
   pending approval and points the operator to the most useful review action.
3. **Given** an awaiting-approval run, **When** the operator reads `run` or
   `status`, **Then** Canon preserves the approval target, readable review path,
   and post-approval continuation guidance without implying the run is already
   complete.

---

### User Story 2 - Review And Remediate Governed Runs Coherently (Priority: P2)

As a maintainer handling approvals, blockers, resumes, and publish decisions, I
want Canon to guide the review and remediation flow coherently across action
chips and text fallbacks, so each control surface says the same thing about the
next governed action.

**Why this priority**: Once the state is readable, the next failure mode is
misguided operator action. This story closes the gap between readable state and
safe governed action.

**Independent Test**: For approval-gated, blocked, and resumable runs, a
reviewer can compare the structured action chips and the text fallback guidance
and find the same recommended action ordering and rationale.

**Acceptance Scenarios**:

1. **Given** a run awaiting approval with a readable packet, **When** Canon
   renders next actions, **Then** it recommends reviewing the packet or
   evidence before recording approval rather than jumping straight to resume.
2. **Given** a run that is blocked without pending approval, **When** Canon
   renders next actions, **Then** it recommends remediation-oriented inspection
   instead of approval-oriented actions.
3. **Given** a run whose approval is already recorded and that now needs
   continuation, **When** Canon renders next actions, **Then** it recommends
   resume explicitly and does not leave the operator on an approval path that no
   longer exists.

---

### User Story 3 - Keep Release, Docs, And Roadmap Aligned With 038 (Priority: P3)

As a Canon maintainer, I want the docs, changelog, roadmap, and version
surfaces to describe the shipped `0.37.0` guided run-operations contract, so
the operator story in the repository matches the runtime behavior.

**Why this priority**: This feature is product-surface work. If README, guides,
roadmap, and release checks do not reflect the new operator contract, the slice
is incomplete.

**Independent Test**: A reviewer can inspect the updated docs, roadmap, and
release-surface checks and find one coherent `0.37.0` story about guided run
operations and review experience.

**Acceptance Scenarios**:

1. **Given** the updated branch, **When** a reviewer checks the README and mode
   guidance, **Then** they see the new run/status/operator-review behavior
   described consistently.
2. **Given** the updated roadmap, **When** a maintainer checks remaining
   features, **Then** `038` is no longer listed as future work and the next
   remaining roadmap scope starts cleanly after the delivered slice.
3. **Given** the release-alignment tests and changelog, **When** they are
   reviewed, **Then** they reflect the operator-guidance contract actually
   shipped in `0.37.0`.

### Edge Cases

- A run is blocked and also has a readable packet; Canon must not send the
  operator down an approval path when the real next step is packet inspection.
- A run is awaiting approval but has no readable packet; Canon must guide the
  operator to evidence inspection or direct approval target review instead of an
  unavailable artifact.
- A completed run has a mode result and readable artifact packet; Canon must
  keep the happy path result-first without hiding drill-down surfaces.
- A resumed run no longer has approval targets; Canon must not keep stale
  approval actions visible.
- Text next steps and action chips must stay semantically aligned even when one
  host cannot render rich chips.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Canon MUST surface a coherent operator summary across `run` and
  `status` that includes the run state, primary readable artifact when present,
  packet result excerpt, blocked gates when present, and recommended next step.
- **FR-002**: Canon MUST distinguish approval-gated runs from artifact-blocked
  or remediation-blocked runs in both structured summaries and markdown/text
  renderers.
- **FR-003**: Canon MUST recommend review-first guidance when a readable packet
  exists for an approval-gated run.
- **FR-004**: Canon MUST recommend blocker-remediation inspection instead of
  approval actions when a run is blocked without pending approval targets.
- **FR-005**: Canon MUST recommend resume only when a governed continuation is
  actually the next valid action and there are no remaining approval targets.
- **FR-006**: Action chips and text fallback guidance MUST remain semantically
  aligned for the same run state.
- **FR-007**: Canon MUST preserve explicit approval targets, artifact paths,
  and evidence-review paths without requiring host surfaces to infer them.
- **FR-008**: Completed runs with a readable `mode_result` MUST continue to use
  the result-first summary as the happy path rather than forcing an inspect step
  to understand the outcome.
- **FR-009**: The operator summary contract MUST remain additive to existing run
  and status payloads and MUST NOT require a new run-state enum or new runtime
  persistence family.
- **FR-010**: Governance-adapter semantics, approval policy meaning, and
  recommendation-only posture MUST remain unchanged by this feature.
- **FR-011**: Focused automated tests MUST cover completed, blocked,
  approval-gated, and resumable operator flows for both structured summary data
  and CLI markdown/text rendering.
- **FR-012**: The implementation plan and tasks MUST include an explicit task
  for version alignment at `0.37.0` even if the version does not change from
  the current workspace baseline.
- **FR-013**: The implementation plan and tasks MUST include an explicit task
  for impacted docs and changelog updates.
- **FR-014**: The implementation plan and tasks MUST include explicit coverage,
  `cargo clippy`, and `cargo fmt` closeout tasks for touched Rust files.
- **FR-015**: The implementation MUST include roadmap cleanup so the delivered
  `038` slice is removed from future-work listings after completion.
- **FR-016**: Validation evidence for this slice MUST record the focused Rust
  checks, release-surface checks, docs review, coverage evidence, and
  independent operator-flow review.

### Key Entities *(include if feature involves data)*

- **Operator Summary Surface**: The combined run or status representation that
  exposes state, result packet readability, blockers, approval targets, and next
  actions coherently.
- **Recommended Next Action Summary**: The durable record of the next governed
  action Canon wants the operator to take, including action id, rationale, and
  optional target.
- **Action Chip Contract**: The host-facing guided action description that
  preserves skill name, required user inputs, recommended status, and text
  fallback.
- **Readable Artifact Result**: The result-first packet summary made from the
  primary artifact title, path, action, and excerpt.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In focused operator-flow validation, 100% of completed,
  approval-gated, blocked, and resumable test runs expose the correct next-step
  class without requiring downstream inference.
- **SC-002**: A reviewer can determine the state, primary readable packet, and
  next action of any focused validation run in under 30 seconds from the run or
  status output alone.
- **SC-003**: Structured action chips and markdown/text fallback guidance stay
  aligned across all focused operator-flow tests with zero mismatched
  recommendation classes.
- **SC-004**: Release/docs alignment checks confirm one coherent `0.37.0`
  operator-guidance story across runtime, README, roadmap, and changelog.

## Validation Plan *(mandatory)*

- **Structural validation**: focused summary-contract and CLI renderer tests,
  release-surface alignment checks, `cargo fmt --check`, and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- **Logical validation**: targeted run/status/operator-flow scenarios covering
  completed, blocked, approval-gated, and resumable cases plus next-step and
  action-chip behavior.
- **Independent validation**: a separate review of operator guidance honesty,
  approval/remediation separation, and roadmap/docs coherence after
  implementation.
- **Evidence artifacts**: `specs/038-guided-run-operations/validation-report.md`,
  focused Rust tests, `lcov.info`, and updated release/docs surfaces.

## Decision Log *(mandatory)*

- **D-001**: Extend the existing run/status/result/action guidance surfaces
  rather than introduce a new operator workflow subsystem, **Rationale**: the
  relevant control plane already exists in `RunSummary`, `StatusSummary`,
  `RecommendedActionSummary`, action chips, and CLI renderers, so the safest
  path is to make those summaries coherent instead of widening the runtime.

## Non-Goals

- Add a new governed mode, new approval class, or new run-state family.
- Replace the governance adapter or change its public contract in this slice.
- Introduce implicit execution from action chips or text summaries.
- Redesign artifact families unrelated to operator review and next-step
  guidance.

## Assumptions

- Existing run and status summaries already carry enough structured data to
  support a more coherent operator-review surface.
- The main operator friction is summary coherence and safe next-step guidance,
  not missing artifact generation or missing mode coverage.
- The workspace remains on `0.37.0` for this slice, but release-alignment tasks
  and validations still need to record that explicit baseline.
- CLI markdown/text rendering remains the fastest validation target for host-
  independent operator ergonomics before any richer frontend surface changes.
