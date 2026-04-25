# Decision Log: Stronger Architecture Outputs (C4 Model)

- **D-001**: Use textual C4 model artifacts (system context, container, component) rather than diagram-syntax artifacts.
  - **Rationale**: keeps the artifact set reviewable as plain Markdown, avoids a new diagram render pipeline, and matches the authored-body contract used elsewhere in Canon.

- **D-002**: Emit the C4 artifacts alongside the existing five legacy architecture artifacts; do not replace any of them.
  - **Rationale**: preserves the critique-first invariant. C4 is a communication shape, not a critique, so it must augment rather than replace decisions, invariants, tradeoffs, boundaries, and readiness artifacts.

- **D-003**: When an authored C4 section is absent in the brief, emit the artifact with a `## Missing Authored Body` marker rather than skipping the artifact or fabricating content.
  - **Rationale**: matches the truthfulness pattern already used by `canon-backlog` and the operational packets. Skipping changes the artifact set shape; fabricating misleads reviewers; an explicit marker preserves predictability.

- **D-004**: Extract authored content using exact canonical H2 headings (`## System Context`, `## Containers`, `## Components`).
  - **Rationale**: matches the existing `extract_marker` mechanism used by other architecture and brief renderers, keeps the contract self-describing through the skill and template, and avoids fragile fuzzy matching.

- **D-005**: Associate `system-context.md` with `Architecture` and `Exploration` gates, `container-view.md` with `Architecture`, and `component-view.md` with `Architecture` and `ReleaseReadiness`.
  - **Rationale**: integrates the new artifacts with the existing gate evaluation surfaces and makes downstream consumption gating consistent with the current architecture artifact set, without expanding the gate vocabulary.

- **D-006**: Defer Mermaid/PlantUML/SVG generation, C4 Level 4 (Code) views, and DDD artifacts.
  - **Rationale**: keeps this slice bounded to the recognized industry C4 shape and maintains the existing publish layout. Diagram rendering and DDD live in distinct roadmap features.
