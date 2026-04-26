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

## Bounded Contexts
- Context 1 and its primary responsibility
- Context 2 and its primary responsibility

## Context Relationships
- How two contexts interact or depend on one another

## Integration Seams
- Named seam or translation boundary 1

## Anti-Corruption Candidates
- Where translation or protective boundaries may be needed

## Ownership Boundaries
- Context 1 owner
- Context 2 owner

## Shared Invariants
- Invariant 1 that every context crossing must preserve
- Invariant 2 that every context crossing must preserve

## System Context
<!--
  C4 Level 1. Author this section yourself before invoking Canon.
  - Name the bounded system the architecture run is shaping.
  - List the external actors (humans, systems) that interact with it.
  - Capture each external interaction as a short bullet.
-->
- System: <name and one-sentence purpose>
- External actors:
  - <actor>: <how they interact>

## Containers
<!--
  C4 Level 2. Enumerate the deployable / runnable units inside the system.
  - One bullet per container with technology and responsibility.
  - Note the persistence and integration containers explicitly.
-->
- `<container-name>` (<technology>): <responsibility>

## Components
<!--
  C4 Level 3. Decompose the most critical container into named components.
  - One bullet per component with its responsibility.
  - Keep this bounded to the components that carry the architectural intent.
-->
- `<component-name>`: <responsibility>