# Canon Mode Guide

This guide explains what each implemented Canon mode is for, what input it
expects, which questions it helps answer, and which questions belong in a
different mode.

Use this guide when you are deciding which mode to run or when you need a
starting template for the `--input` material that Canon consumes.

## Supported Today

- [`discovery`](#mode-discovery)
- [`requirements`](#mode-requirements)
- [`system-shaping`](#mode-system-shaping)
- [`architecture`](#mode-architecture)
- [`brownfield-change`](#mode-brownfield-change)
- [`review`](#mode-review)
- [`verification`](#mode-verification)
- [`pr-review`](#mode-pr-review)

Modes that are still modeled but not implemented end to end remain outside the
scope of this guide.

## Input Binding Rules

For file-backed modes, Canon now has a single canonical authored-input
convention:

- `requirements`: `canon-input/requirements.md` or `canon-input/requirements/`
- `discovery`: `canon-input/discovery.md` or `canon-input/discovery/`
- `system-shaping`: `canon-input/system-shaping.md` or `canon-input/system-shaping/`
- `architecture`: `canon-input/architecture.md` or `canon-input/architecture/`
- `brownfield-change`: `canon-input/brownfield-change.md` or `canon-input/brownfield-change/`
- `review`: `canon-input/review.md` or `canon-input/review/`
- `verification`: `canon-input/verification.md` or `canon-input/verification/`

Repo-local skills may auto-bind only from those canonical mode-specific
locations. They must not treat the active editor file, open tabs, generated
artifacts under `.canon/`, or any other incidental file as the current run
input.

`canon run` and `canon inspect risk-zone` also accept explicit inline authored
input through `--input-text`. Inline input is an explicit alternative, not a
new canonical location, and real runs snapshot it only under
`.canon/runs/<RUN_ID>/inputs/`.

If you prefer to keep a mode brief as a folder, Canon expands the files in that
folder into the run context and fingerprints each authored file separately.

Every mode that expects authored input now fails before execution if the input
is missing, empty, whitespace-only, or structurally insufficient. That includes
empty files, empty directories, and directory expansions that produce no usable
authored content.

`review` is stricter in the current runtime slice: it expects exactly one
authored review packet at `canon-input/review.md` or `canon-input/review/`.
Do not point it at arbitrary code folders such as `src/` or the repo root.
Use `pr-review` when the real target is a diff or `WORKTREE`.

`pr-review` is different. It does not bind from `canon-input/` at all. It only
accepts explicit base and head refs, or `WORKTREE` as the head ref.

When a run starts from authored files, Canon snapshots those files under
`.canon/runs/<RUN_ID>/inputs/` and records digest-backed provenance in
`.canon/runs/<RUN_ID>/context.toml`.

## Mode: discovery

### Use It For

Explore a blurry problem space before you can write trustworthy requirements.

### Input Shape

A short discovery brief or exploratory note.

### Good Input Should Include

- the problem you are trying to understand
- the current context
- what is already known
- what is still unclear
- constraints
- what is in scope
- what is out of scope
- alternative directions worth exploring
- decisions that need to be prepared, but not finalized yet

### Questions This Mode Answers

- what is the real problem to solve
- what are the boundaries of the problem
- which unknowns block a credible next step
- which assumptions are implicit
- which options are worth exploring
- what needs to be decided now versus later

### Questions This Mode Does Not Answer Well

- what the final implementation plan should be
- what the final architecture should be
- what exact delivery sequence to execute
- what code should change

### What Canon Emits

Discovery produces an exploratory packet with these artifacts:

- `problem-map.md`
- `unknowns-and-assumptions.md`
- `context-boundary.md`
- `exploration-options.md`
- `decision-pressure-points.md`

Use that bundle when you need to make the unknowns, assumptions, boundaries,
and decision pressure explicit before a later mode becomes trustworthy.

### Typical Handoff After This Mode

- move to `requirements` when the problem boundary is stable enough for explicit framing
- move to `system-shaping` when the team now understands the problem and is ready to shape a new capability
- move to `architecture` when the key remaining work is choosing among structural options

### Common Mistakes

- using Discovery when the problem is already bounded enough for Requirements
- treating Discovery as a place to choose a final implementation plan
- giving it a solution memo instead of a problem-exploration brief

### Minimal Template

```md
# Discovery Brief

## Problem
What are we trying to understand?

## Current Context
What is the current situation?

## Known Facts
- Fact 1
- Fact 2

## Unknowns
- Unknown 1
- Unknown 2

## Constraints
- Constraint 1
- Constraint 2

## In Scope
- Item 1
- Item 2

## Out of Scope
- Item 1
- Item 2

## Exploration Options
- Option 1
- Option 2

## Questions
- Question 1
- Question 2
```

## Mode: requirements

### Use It For

Bound a problem before code, design, or scope drift starts.

### Input Shape

A short initiative brief, idea note, or product framing document.

### Good Input Should Include

- the problem to solve
- the desired outcome
- known constraints
- important tradeoffs
- what is out of scope
- the decisions that must be made before implementation

### Questions This Mode Answers

- what exactly are we trying to do
- what constraints and tradeoffs matter
- what options exist
- what should be cut from scope
- what open questions remain before planning or execution

### Questions This Mode Does Not Answer Well

- how a new system should be shaped internally
- which architecture option should win
- how to safely change a legacy surface
- what changed in a pull request

### What Canon Emits

Requirements produces a bounded framing packet with these artifacts:

- `problem-statement.md`
- `constraints.md`
- `options.md`
- `tradeoffs.md`
- `scope-cuts.md`
- `decision-checklist.md`

Use that bundle when the team needs one durable packet that explains the
problem, the constraints, the available options, the tradeoffs, and what still
has to be decided.

### Typical Handoff After This Mode

- move to `discovery` if the problem is still too fuzzy and the unknowns are more important than the framing
- move to `system-shaping` if the problem is bounded and the next step is shaping a new capability
- move to `brownfield-change` if the work is clearly about a bounded change in an existing system
- move to `architecture` if the main unresolved issue is a structural decision rather than product framing

### Common Mistakes

- using Requirements when the real problem is still ambiguous and needs exploration first
- using Requirements as a substitute for architecture decisions
- putting implementation details into the brief before the scope is credibly bounded

### Minimal Template

```md
# Requirements Brief

## Problem
Describe the problem in one or two paragraphs.

## Outcome
What should be true if this work succeeds?

## Constraints
- Constraint 1
- Constraint 2

## Tradeoffs
- Tradeoff 1
- Tradeoff 2

## Out of Scope
- Item 1
- Item 2

## Open Questions
- Question 1
- Question 2
```

## Mode: system-shaping

### Use It For

Shape a new capability or new system once the intent is bounded.

### Input Shape

A system-shaping brief for the `system-shaping` mode with explicit intent and
constraints.

### Important Note

This mode works best when the brief includes explicit markers like:

- `Intent:`
- `Constraint:`

### Good Input Should Include

- the capability to create
- the goal of the system
- explicit intent
- explicit constraints
- key domain responsibilities
- major delivery concerns
- main risks or unknowns

### Questions This Mode Answers

- what shape the new system should take
- what boundaries should exist early
- what capabilities are needed
- what delivery options exist
- where the risk hotspots are

### Questions This Mode Does Not Answer Well

- how to change a legacy system safely
- which exact code edits to make now
- how to review an existing diff
- how to govern a live migration or incident

### What Canon Emits

The `system-shaping` mode produces an early system-shaping packet with these
artifacts:

- `system-shape.md`
- `architecture-outline.md`
- `capability-map.md`
- `delivery-options.md`
- `risk-hotspots.md`

This mode includes mandatory critique, so the emitted packet is not just a raw
design sketch. It is a challenged first structure for a new capability.

### Important Runtime Constraint

This system-shaping mode works best when the input brief includes explicit
`Intent:` and `Constraint:` markers. If those anchors are missing, the emitted
artifacts can carry insufficient-evidence warnings instead of a strong
system-shaping result.

### Typical Handoff After This Mode

- move to `architecture` when the next step is to settle structural tradeoffs or invariants
- move to `requirements` only if the intent turned out not to be bounded enough after all
- move to downstream execution planning later, once the shaped capability is stable enough to implement

### Common Mistakes

- using `system-shaping` before the problem is bounded
- omitting explicit `Intent:` and `Constraint:` anchors in the brief
- treating system shaping as if it were already implementation planning

### Minimal Template

```md
# System Shaping Brief

Intent: Build a bounded capability for ...
Constraint: Keep the first release limited to ...

## Goal
What new capability are we creating?

## Users or Stakeholders
Who is this for?

## Domain Responsibilities
- Responsibility 1
- Responsibility 2

## Constraints
- Constraint 1
- Constraint 2

## Risks
- Risk 1
- Risk 2

## Open Questions
- Question 1
- Question 2
```

## Mode: architecture

### Use It For

Make explicit structural decisions about boundaries, invariants, and tradeoffs.

### Input Shape

An architecture brief describing the decision surface, competing options, and
important constraints.

### Good Input Should Include

- the design problem
- the main structural options
- important constraints
- candidate boundaries
- invariants that must hold
- tradeoffs that matter
- risks of the decision

### Questions This Mode Answers

- what boundaries should exist
- what invariants must hold
- what tradeoffs separate the options
- which option is strongest under explicit criteria
- what blockers or accepted risks remain

### Questions This Mode Does Not Answer Well

- what the product problem is if it is still unclear
- how to shape a brand new capability from scratch
- how to preserve legacy behavior in an existing system
- how to review an actual code diff

### What Canon Emits

Architecture produces a structural decision packet with these artifacts:

- `architecture-decisions.md`
- `invariants.md`
- `tradeoff-matrix.md`
- `boundary-map.md`
- `readiness-assessment.md`

This mode includes mandatory critique and is designed to leave behind a
decision bundle that later work can implement or review without relying on chat
history.

### Approval and Risk Behavior

Architecture can stop in `AwaitingApproval` when the run is `systemic-impact`
or in the `red` zone. That is expected behavior for a structural decision flow,
not a failure in the mode itself.

### Typical Handoff After This Mode

- move to `brownfield-change` when the structural decision now needs a bounded change plan in an existing system
- move to later implementation work once the boundaries and invariants are accepted
- return to `discovery` or `requirements` only if the decision surface itself was not actually bounded

### Common Mistakes

- using Architecture before the problem or capability is clear enough
- confusing architecture tradeoff work with product framing
- asking Architecture to inspect a real code diff instead of using `pr-review`

### Minimal Template

```md
# Architecture Brief

## Decision
What structural decision are we making?

## Options
- Option 1
- Option 2

## Constraints
- Constraint 1
- Constraint 2

## Candidate Boundaries
- Boundary 1
- Boundary 2

## Invariants
- Invariant 1
- Invariant 2

## Evaluation Criteria
- Criterion 1
- Criterion 2

## Risks
- Risk 1
- Risk 2
```

## Mode: brownfield-change

### Use It For

Plan a bounded change in an existing system while preserving important existing
behavior.

### Input Shape

A brownfield brief with explicit change surface and invariants.

### Good Input Should Include

- `System Slice:`
- `Intended Change:`
- `Legacy Invariants:`
- `Change Surface:`
- `Implementation Plan:`
- `Validation Strategy:`
- `Decision Record:`
- `Owner:`
- `Risk Level:`
- `Zone:`

### Questions This Mode Answers

- what exact slice of the current system is changing
- what behavior must not be broken
- where the allowed change surface starts and stops
- how to sequence the change safely
- how to validate preserved behavior

### Questions This Mode Does Not Answer Well

- what new product problem to explore
- what broad architecture direction to invent from scratch
- what a pull request currently does

### What Canon Emits

Brownfield-Change produces a bounded change packet with these artifacts:

- `system-slice.md`
- `legacy-invariants.md`
- `change-surface.md`
- `implementation-plan.md`
- `validation-strategy.md`
- `decision-record.md`

Use that bundle when you need explicit evidence about what is changing, what
must stay stable, where the allowed change surface ends, and how the result
will be validated.

### Important Input Behavior

Brownfield is much stronger when the brief uses the marker-style fields shown
in the template. If key fields like `Legacy Invariants:` or `Change Surface:`
are missing, Canon can still emit artifacts, but they will make the missing
context explicit instead of pretending the change is well-bounded.

### Typical Handoff After This Mode

- inspect the emitted artifact bundle before approving any consequential follow-up
- move into later implementation work only after the preserved behavior and allowed change surface are explicit
- use `pr-review` later when there is a real diff to challenge

### Common Mistakes

- using Brownfield for a vague product idea that should start in Requirements or Discovery
- failing to name the preserved invariants
- leaving the allowed change surface implicit

### Minimal Template

```md
# Brownfield Brief

System Slice: The bounded subsystem or module to change.
Intended Change: The intended modification.
Legacy Invariants: The behaviors that must remain true.
Change Surface: Files, modules, APIs, and boundaries allowed to change.
Implementation Plan: The high-level change sequence.
Validation Strategy: Tests, checks, and evidence required.
Decision Record: Why this change is preferable.
Owner: staff-engineer
Risk Level: bounded-impact
Zone: yellow
```

## Mode: review

### Use It For

Review a bounded non-PR packet with governed evidence and explicit disposition
handling before downstream work proceeds.

### Typical Upstream Sources

- a `requirements` packet that needs acceptance before planning or implementation
- an `architecture` packet that needs explicit boundary and evidence review
- a `brownfield-change` packet that needs a go/no-go packet review before implementation
- a proposal, migration memo, readiness packet, or other non-PR artifact bundle

### Input Shape

A single canonical review brief or review packet under `canon-input/review.md`
or `canon-input/review/`.

### Important Runtime Constraint

Review is packet-backed, not diff-backed. Point it at an authored packet, not
at `src/`, the repo root, or a worktree snapshot. If the real target is a
diff or local code changes, use `pr-review`.

### Good Input Should Include

- what packet is being reviewed
- which artifacts or evidence are in scope
- the main boundary or ownership concern
- the acceptance question or pending decision
- any missing evidence or open concern that may require explicit disposition

### Questions This Mode Answers

- whether the packet stays within the intended review boundary
- what evidence is missing or weak
- what decision impact is implied by the package
- whether explicit review disposition is still required

### Questions This Mode Does Not Answer Well

- what changed in a real diff or worktree
- whether a claim is formally supported or contradicted end to end
- what implementation plan should be executed next

### What Canon Emits

Review produces a governed review packet with these artifacts:

- `review-brief.md`
- `boundary-assessment.md`
- `missing-evidence.md`
- `decision-impact.md`
- `review-disposition.md`

Use that bundle when you need a durable review record that keeps boundary
findings, evidence gaps, decision impact, and explicit disposition in one run
context.

Run and status summaries also surface `review-disposition.md` directly, so the
happy path or gated path is readable without a mandatory inspect step first.

### Typical Handoff After This Mode

- inspect the review packet when you need the full findings bundle
- approve `gate:review-disposition` only after the remaining risk is consciously accepted
- move to `pr-review` only when the real target becomes a diff or worktree instead of a file-backed packet

### Common Mistakes

- using Review for a diff-backed change that should go through `pr-review`
- treating Review as a substitute for verification of claims or invariants
- pointing Review at `src/`, the repo root, or another arbitrary code folder instead of authoring a packet under `canon-input/review.*`
- omitting the evidence basis and expecting Canon to infer it from generated artifacts

### Minimal Template

```md
# Review Brief

Review Target: The packet or proposal being reviewed.
Evidence Basis: The artifacts, tests, decision notes, or checks in scope.
Owner: reviewer
Boundary Concern: The boundary or ownership edge that must remain explicit.
Pending Decision: The decision this review is expected to accept, reject, or defer.
Open Concern: The gap or concern that may still require explicit disposition.

## In Scope Artifacts
- artifact 1
- artifact 2

## Acceptance Question
- Should this packet be accepted for downstream work?

## Out of Scope
- item 1
- item 2
```

### Minimal Usage

```bash
canon run \
  --mode review \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input canon-input/review.md
```

If you keep a multi-file packet instead of a single brief, pass
`canon-input/review/`.

## Mode: verification

### Use It For

Challenge bounded claims, invariants, contracts, or evidence directly with a
governed verification packet.

### Input Shape

A file-backed verification packet or short verification brief.

### Good Input Should Include

- the claims or invariants under test
- the evidence basis Canon should challenge
- any contract surface that must be checked
- the contradiction or risk boundary that matters most

### Questions This Mode Answers

- which claims stay supported by the available evidence
- where contradictions or unsupported assumptions remain
- which findings still block readiness
- what follow-up is required before the packet is treated as trustworthy

### Questions This Mode Does Not Answer Well

- what changed in a real diff or worktree
- whether a non-PR package should be disposition-reviewed rather than challenged
- what implementation or refactor plan should be executed next

### What Canon Emits

Verification produces a governed challenge packet with these artifacts:

- `invariants-checklist.md`
- `contract-matrix.md`
- `adversarial-review.md`
- `verification-report.md`
- `unresolved-findings.md`

Use that bundle when you need durable evidence about what is supported, what is
rejected, and what remains unresolved in the current verification target.

Run and status summaries also surface `verification-report.md` directly, so the
supported or blocked posture is visible without a mandatory inspect step first.

### Typical Handoff After This Mode

- inspect the verification packet when unresolved findings remain
- inspect evidence when you need provenance or validation lineage behind the verdict
- move to `review` only when the next step is disposition over a bounded package, not challenge of claims

### Common Mistakes

- using Verification for diff review instead of `pr-review`
- using Verification for generic package review instead of `review`
- expecting Verification to invent new evidence rather than challenge the evidence already supplied

### Minimal Template

```md
# Verification Brief

## Claims Under Test
- claim 1
- claim 2

## Evidence Basis
- artifact, test, or repository surface 1
- artifact, test, or repository surface 2

## Contract Surface
- the interface, invariant, or behavioral contract that must stay true

## Risk Boundary
- the contradiction, proof gap, or unsupported jump that should block readiness

## Challenge Focus
- the strongest claim Canon should try to falsify first
- any adversarial or compatibility angle that still needs evidence

## Out of Scope
- anything this packet is not trying to prove
```

### Minimal Usage

```bash
canon run \
  --mode verification \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input canon-input/verification/
```

## Mode: pr-review

### Use It For

Review a real diff or worktree with governed evidence.

### Input Shape

This mode usually does not want a markdown brief. It wants refs or a worktree
target.

### Good Input Should Include

- a base ref
- a head ref or `WORKTREE`

### Questions This Mode Answers

- what changed
- what boundaries were crossed
- whether logic duplication appeared
- whether interfaces drifted
- whether tests are missing or weak
- what implied design decisions are hiding in the diff

### Questions This Mode Does Not Answer Well

- what problem the team should solve
- what new system to design from scratch
- what architecture option to explore in the abstract

### What Canon Emits

PR-Review produces a governed review packet with these artifacts:

- `pr-analysis.md`
- `boundary-check.md`
- `duplication-check.md`
- `contract-drift.md`
- `missing-tests.md`
- `decision-impact.md`
- `review-summary.md`

Use that bundle when you need a durable review record that makes findings,
boundary crossings, contract drift, missing tests, and final disposition
inspectable after the run.

Run and status summaries also surface the review disposition directly, using
`review-summary.md` as the primary artifact, so the happy path does not require
a separate inspect step just to learn whether the diff is ready, awaiting
disposition, or accepted with approval.

### Input Pattern

Unlike the analysis modes, PR-Review usually takes refs rather than a markdown
brief. The common pattern is:

- base ref such as `refs/heads/main`
- head ref such as `HEAD`

Or, for uncommitted work:

- base ref such as `refs/heads/main`
- `WORKTREE`

### Typical Handoff After This Mode

- read the run or status result summary first, then inspect the emitted review packet when you need the full findings bundle
- approve a review-disposition gate only when the remaining risk is consciously accepted
- move back to Brownfield or Architecture only if the review shows that the underlying change plan or decision packet is still weak

### Common Mistakes

- using PR-Review before there is a real diff or worktree to inspect
- trying to use PR-Review as a substitute for Requirements or Discovery
- expecting PR-Review to design the solution instead of challenge the current change

### Minimal Usage

```bash
canon run \
  --mode pr-review \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input refs/heads/main \
  --input HEAD
```