# Domain Model Brief

System Surface: The bounded product area or domain whose concepts this packet formalizes.
Primary Upstream Mode: domain-language | architecture | direct
Upstream Sources:
- docs/domain/language/<RUN_ID>/02-domain-glossary.md
Carried-Forward Decisions:
- Existing domain constraints carried into this model packet.
Excluded Upstream Scope: Implementation details, data schemas, and API contracts that remain out of scope.

Published Canon packets use ordered filenames. Prefer the emitted `01-*`,
`02-*`, and later packet paths when this model references upstream Canon
artifacts.

## Domain Scope
- Name the bounded domain or product area whose concepts are being formalized.

## Model Maturity
- State the current maturity level (exploratory | evolving | stable).

## Upstream Sources
- List the upstream documents, language packets, or references that inform this model.

## Downstream Consumers
- Name the downstream modes or teams that will consume this model packet.

## Concepts
- List the domain concepts with brief definitions and ownership boundaries.

## Ownership Boundaries
- State which bounded context owns each concept.

## Open Gaps
- List concepts that lack clear ownership or definition.

## Relationships
- List the relationships between concepts.

## Cardinality Rules
- State the cardinality constraints for each relationship.

## Boundary Crossings
- Name relationships that cross bounded context boundaries.

## Bounded Contexts
- List the bounded contexts in the domain.

## Context Relationships
- Describe how bounded contexts relate to each other.

## Integration Seams
- Name the integration points between bounded contexts.

## Entity Lifecycles
- Describe the lifecycle of key domain entities.

## State Transitions
- List the valid state transitions for stateful concepts.

## Invariant Guards
- Name the guards that prevent invalid state transitions.

## Invariants
- List the domain invariants that must always hold.

## Enforcement Points
- Name where each invariant is enforced.

## Violation Consequences
- Describe what happens when an invariant is violated.

## Business Policies
- List the business policies that constrain the domain.

## Constraint Rules
- State the constraint rules derived from business policies.

## Exception Handling
- Describe how policy exceptions are handled.

## Impact Rules
- List rules that describe how features impact domain concepts.

## Affected Concepts
- Name which concepts are affected by each feature-impact rule.

## Downstream Effects
- Describe the downstream effects of feature-impact rules.

## Code Mapping
- Map domain concepts to their code representations.

## Data Store Mapping
- Map domain concepts to their data store representations.

## Alignment Gaps
- Note where code or data representations diverge from the domain model.

## Model Gaps
- List gaps in the model that need further investigation.

## Risk Signals
- Name risks that the current model introduces or exposes.

## Recommended Follow-Ups
- List the recommended follow-up actions.

## Consumer Modes
- Name the Canon modes that should consume this model packet.

## Handoff Expectations
- State what the downstream consumer should expect from this packet.

## Adoption Risks
- List risks that may prevent adoption of this domain model.
