# Research: Architecture ADR And Options

## Decision 1: Keep the slice architecture-only

- Decision: Limit the combined roadmap package to `architecture` rather than touching `requirements` or `change` in the same feature.
- Rationale: This still combines authored-body specialization, standard artifact shape, and option-analysis behavior, but keeps the blast radius small enough to implement and validate in one slice.
- Alternatives considered:
  - Extend `requirements`, `architecture`, and `change` together. Rejected because the combined docs/renderer/test surface becomes too large for one bounded feature.
  - Implement only ADR shape without option analysis. Rejected because it would leave the strongest review value of the package on the table.

## Decision 2: Strengthen existing architecture artifacts instead of adding a new artifact family

- Decision: Preserve the existing `architecture-decisions.md`, `tradeoff-matrix.md`, and `readiness-assessment.md` files and strengthen their authored-section contract rather than introducing a brand new packet layout.
- Rationale: Existing tests, publish behavior, and user expectations already anchor on those artifact names. The feature goal is stronger decision shape, not packet sprawl.
- Alternatives considered:
  - Add a new `adr.md` artifact. Rejected because it increases contract surface, migration burden, and documentation churn for limited additional value.
  - Collapse decision and tradeoff artifacts into one file. Rejected because it weakens separation between the chosen decision and the option comparison.

## Decision 3: Keep C4 behavior unchanged

- Decision: Do not modify the authored contract or rendering rules for `system-context.md`, `container-view.md`, or `component-view.md` in this slice.
- Rationale: The package should deepen decision fidelity without reopening already-delivered C4 behavior.
- Alternatives considered:
  - Reformat the C4 artifacts to align visually with ADR output. Rejected because it risks regression in a stable slice without improving decision capture directly.

## Decision 4: Missing authored decision sections remain explicit failures of authored completeness

- Decision: Use the existing `## Missing Authored Body` honesty pattern for required decision sections.
- Rationale: This keeps the mode critique-first and consistent with earlier authoring-specialization work.
- Alternatives considered:
  - Generate fallback summary prose when authored sections are missing. Rejected because it obscures the gap the feature is trying to make visible.

## Decision 5: Represent option analysis with authored H2 sections rather than external evidence collectors

- Decision: The first slice relies on authored option-analysis sections already present in the architecture brief, not live ecosystem lookups or registry evidence.
- Rationale: This satisfies the roadmap package while staying repository-local and compatible with existing renderer patterns.
- Alternatives considered:
  - Add GitHub or package-registry evidence collectors now. Rejected because that pushes the slice toward a much larger adapter feature.