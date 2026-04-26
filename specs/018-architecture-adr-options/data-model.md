# Data Model: Architecture ADR And Options

## Entity: Architecture Decision Record Shape

- Purpose: Represents the authored decision narrative for `architecture` mode.
- Core fields:
  - Context
  - Decision
  - Status
  - Consequences
- Relationships:
  - Consumes the authored architecture brief.
  - Feeds `architecture-decisions.md` as the primary decision-facing artifact.

## Entity: Option Analysis Section

- Purpose: Represents the authored comparison surface for evaluating viable architecture options.
- Core fields:
  - Decision Drivers
  - Options Considered
  - Pros
  - Cons
  - Recommendation
  - Why Not The Others
- Relationships:
  - Feeds `tradeoff-matrix.md` and decision-summary sections.
  - Depends on the same authored architecture brief as the ADR shape.

## Entity: Missing-Body Marker

- Purpose: Represents an explicit honesty block emitted when a required authored decision section is absent.
- Core fields:
  - Missing canonical heading name
  - Artifact location
  - Reviewer-visible remediation signal
- Relationships:
  - May appear in `architecture-decisions.md` or `tradeoff-matrix.md` when authored sections are absent.

## Entity: C4 Context Block

- Purpose: Represents the authored C4 sections already supported by `architecture` mode.
- Core fields:
  - System Context
  - Containers
  - Components
- Relationships:
  - Feeds `system-context.md`, `container-view.md`, and `component-view.md`.
  - Must remain behaviorally independent from the new decision-shape fields.

## State And Validation Rules

- Canonical authored headings are required for verbatim preservation.
- ADR-like and option-analysis sections are valid only when they map to the declared architecture decision surface.
- Missing authored sections do not block artifact emission, but they downgrade packet completeness through explicit missing-body markers.
- C4 sections and decision sections must coexist without one overwriting or weakening the other.