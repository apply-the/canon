# Research: Domain Modeling And Boundary Design

## Decision 1: Keep the first slice bounded to system-shaping, architecture, and change

- **Decision**: Deliver domain-modeling work only for `system-shaping`, `architecture`, and `change` in this slice.
- **Rationale**: These are the modes where domain boundaries, ownership, and preserved invariants provide the highest leverage without widening the runtime model or crossing into later implementation-heavy workflows.
- **Alternatives considered**:
  - Extend all remaining modes: rejected because it would exceed bounded-impact and blur the first useful slice.
  - Limit the slice to architecture only: rejected because the domain model would arrive too late, after shaping and without downstream change preservation.

## Decision 2: Represent domain modeling as additive packet surfaces, not a new Canon mode

- **Decision**: Keep domain modeling inside the existing mode model and add explicit artifact surfaces where needed instead of introducing a new standalone mode.
- **Rationale**: The roadmap frames this feature as strengthening `system-shaping`, `architecture`, and `change`, not inventing a separate planning vocabulary or runtime lifecycle.
- **Alternatives considered**:
  - New `domain-modeling` mode: rejected because it would duplicate upstream/downstream responsibilities and widen governance semantics.
  - Inline all domain content into summaries only: rejected because it would not make boundaries first-class or reviewable.

## Decision 3: Use one dedicated domain artifact for system-shaping and one context-map artifact for architecture

- **Decision**: Add a focused `domain-model.md` artifact to `system-shaping` and a focused `context-map.md` artifact to `architecture`, while keeping their existing artifact sets intact.
- **Rationale**: This makes the new material first-class and inspectable without fragmenting the packet into many tiny files.
- **Alternatives considered**:
  - Multiple new artifacts per mode: rejected because it raises review overhead and implementation size without first proving the narrower slice.
  - Only extend existing artifacts: rejected because the new domain surfaces would remain easy to miss and harder to test independently.

## Decision 4: Strengthen change through existing artifacts instead of introducing a second change packet family

- **Decision**: Extend the existing `change` artifacts with domain-slice, domain-invariant, ownership-boundary, and cross-context-risk sections instead of adding a new file family.
- **Rationale**: `change` already has a well-bounded packet structure around `system-slice.md`, `legacy-invariants.md`, `change-surface.md`, and `decision-record.md`; the domain-modeling slice is best delivered by strengthening those artifacts.
- **Alternatives considered**:
  - Add a new standalone `domain-slice.md` artifact: rejected because it duplicates existing packet intent and increases drift risk between artifacts.
  - Leave `change` unchanged and focus on upstream modes only: rejected because the spec requires preserved domain invariants and ownership boundaries at change time.

## Decision 5: Surface uncertainty explicitly instead of forcing crisp domain boundaries when the brief is weak

- **Decision**: Preserve critique-first behavior by making uncertain boundaries, overloaded terms, and questionable context splits explicit in the packet rather than normalizing them into a single confident answer.
- **Rationale**: The constitution and feature spec both prioritize honesty, bounded context awareness, and adversarial critique over plausible-looking certainty.
- **Alternatives considered**:
  - Always synthesize a single best boundary map: rejected because it hides missing evidence and weakens reviewer trust.
  - Push all ambiguity into open questions only: rejected because the packet still needs enough explicit structure to guide downstream architecture and change work.

## Decision 6: Reuse the existing docs, skills, and method metadata paths

- **Decision**: Update the current method metadata, embedded skill sources, materialized skills, templates, examples, and guide files in place.
- **Rationale**: The repository already has a stable per-mode discoverability path, and parallel hierarchies would create drift.
- **Alternatives considered**:
  - Create a second `domain-modeling/` docs tree: rejected because it adds maintenance overhead and splits user guidance.