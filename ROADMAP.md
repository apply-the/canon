# Next Features

This file captures only the remaining product work after the delivered
`037-architecture-clarification-readiness` slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `backlog`, `change`, `implementation`,
`refactor`, `review`, `verification`, `pr-review`, `incident`,
`security-assessment`, `system-assessment`, `migration`, and
`supply-chain-analysis`.

Delivered work through `037` already covers the governed mode surface,
publishable packet families, clarity and readiness honesty, the governance
adapter, structured publish destinations, and release provenance across
distribution channels.

The roadmap is intentionally capped at two macrofeatures, to be delivered whole
and not as slices.

## Feature 038: Operator Workflow And Run Control Surface

### Desired Outcome

Canon should expose one coherent operator surface for understanding, steering,
approving, resuming, reviewing, and publishing a run end-to-end.

### Why This Is Next

- The governed runtime and packet contracts are now broad enough that the main
  friction is operator navigation rather than missing mode coverage.
- Maintainers still reconstruct state across `status`, `inspect`, `approve`,
  `resume`, and `publish` instead of working from one coherent control story.
- Approval-gated, blocked, and conditionally useful packets need stronger
  review and handoff ergonomics.

### Macrofeature Scope

- Unify run state, packet readiness, blocker explanation, approval targets,
  artifact availability, publish readiness, and next-step guidance across CLI
  and skill-facing outputs.
- Make review and remediation flows readable without weakening the underlying
  governance contract.
- Strengthen operator ergonomics for approval-gated, blocked, partially
  publishable, and resumed runs.
- Keep artifacts, evidence, approvals, and publish surfaces cross-linked and
  lossless.

### Invariants

- Canon must not become a hidden planner or opaque agent loop.
- Approval, evidence, and recommendation-only semantics must remain explicit.
- The CLI and governance adapter remain the canonical control surface.

## Feature 039: Authoring System And Packet Maturation

### Desired Outcome

Canon should make the path from weak authored input to publishable packet
explicit, consistent, and durable across all file-backed modes.

### Why This Is Next

- Mode coverage is now broad; the next leverage point is better authored input
  quality rather than more mode slices.
- Templates, examples, skill guidance, and clarity output exist, but they still
  behave more like separate assets than one authoring system.
- Maintainership still depends too much on repo knowledge when moving from a
  shallow brief to a strong, publishable packet.

### Macrofeature Scope

- Define one consistent authored-input lifecycle across file-backed modes:
  template, example, clarity inspection, run, critique, and publish.
- Tighten template, example, skill, and doc sync so each mode teaches one
  canonical packet shape and one honest readiness story.
- Improve carry-forward of missing context, defaults, assumptions, unresolved
  questions, and readiness deltas without inventing certainty.
- Strengthen ergonomics for directory-backed packets and multi-file authored
  inputs while preserving explicit boundaries.

### Invariants

- `## Missing Authored Body` remains stronger than generated filler.
- Materially closed decisions remain preserved rather than reopened for theater.
- Canon must not rewrite `canon-input/` automatically or introduce a new mode
  family.

There are no other active roadmap entries beyond Features 038 and 039.