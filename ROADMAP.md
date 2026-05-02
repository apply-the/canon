# Next Features

This file captures only the remaining product work after the delivered
`038-guided-run-operations` slice.

Current end-to-end depth exists for `requirements`, `discovery`,
`system-shaping`, `architecture`, `backlog`, `change`, `implementation`,
`refactor`, `review`, `verification`, `pr-review`, `incident`,
`security-assessment`, `system-assessment`, `migration`, and
`supply-chain-analysis`.

Delivered work through `037` already covers the governed mode surface,
publishable packet families, clarity and readiness honesty, the governance
adapter, structured publish destinations, and release provenance across
distribution channels.

Delivered work through `038` now also covers the operator-facing run and status
control story, ordered next-step guidance, and coherent review-first ergonomics
across completed, blocked, approval-gated, and resumed runs.

The roadmap is intentionally capped at the single remaining macrofeature, to be
delivered whole and not as slices.

## Feature 039: Authoring Experience And Packet Readiness

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

There are no other active roadmap entries beyond Feature 039.