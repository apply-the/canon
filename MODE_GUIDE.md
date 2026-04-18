# Canon Mode Guide

This guide explains what each implemented Canon mode is for, what input it
expects, which questions it helps answer, and which questions belong in a
different mode.

Use this guide when you are deciding which mode to run or when you need a
starting template for the `--input` material that Canon consumes.

## Supported Today

- `discovery`
- `requirements`
- `system-shaping`
- `architecture`
- `brownfield-change`
- `pr-review`

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

Repo-local skills may auto-bind only from those canonical mode-specific
locations. They must not treat the active editor file, open tabs, generated
artifacts under `.canon/`, or any other incidental file as the current run
input.

If you prefer to keep a mode brief as a folder, Canon expands the files in that
folder into the run context and fingerprints each authored file separately.

`pr-review` is different. It does not bind from `canon-input/` at all. It only
accepts explicit base and head refs, or `WORKTREE` as the head ref.

When a run starts from authored files, Canon snapshots those files under
`.canon/runs/<RUN_ID>/inputs/` and records digest-backed provenance in
`.canon/runs/<RUN_ID>/context.toml`.

## Discovery

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

## Requirements

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

## System Shaping Mode

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

## Architecture

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

## Brownfield-Change

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

## PR-Review

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