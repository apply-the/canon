# Architecture Context Map Contract

## Additive Artifact Surface

| Artifact | Required sections | Gate impact | Purpose |
|----------|-------------------|-------------|---------|
| `context-map.md` | `Summary`, `Bounded Contexts`, `Context Relationships`, `Integration Seams`, `Anti-Corruption Candidates`, `Ownership Boundaries`, `Shared Invariants` | `Architecture`, `Risk` | Make architecture boundaries, shared invariants, and inter-context coupling explicit and reviewable |

## Existing Artifact Expectations

- `boundary-map.md` remains the general structural boundary artifact; `context-map.md` adds domain-aware ownership and relationship detail rather than replacing it.
- `architecture-decisions.md` and `tradeoff-matrix.md` must remain critique-first and should reference context-boundary tradeoffs where they influence the chosen design.
- Existing C4 artifacts (`system-context.md`, `container-view.md`, `component-view.md`) remain additive and should stay consistent with the new context map.

## Runtime Contract Expectation

- `context-map.md` must be added to the architecture artifact contract so inspect/publish flows surface it alongside the existing packet.
- The initial design expectation is that `Architecture` and `Risk` gates both require the context map because it carries ownership, invariants, and coupling rationale that affect review readiness.

## Skill and Docs Expectation

- The `canon-architecture` skill, template, and example must teach the authored sections for `context-map.md` alongside the already-delivered C4 sections.
- Missing authored sections follow the existing explicit missing-body policy rather than allowing inferred context relationships to appear as complete.