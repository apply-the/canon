# Decision Log: Decision Alternatives, Pattern Choices, And Framework Evaluations

## D-001: Keep live external evidence collection out of the first slice

- **Decision**: Keep the first slice authored and evidence-grounded rather than
  adding new external collectors or adapters.
- **Rationale**: This bounds the blast radius and keeps the feature shippable in
  one repository slice.

## D-002: Split the feature into structural and framework-evaluation families

- **Decision**: Use one shared feature with two packet families: structural
  decision modes (`system-shaping`, `architecture`, `change`) and framework
  evaluation modes (`implementation`, `migration`).
- **Rationale**: The same reviewer need exists across both groups, but the
  authored sections and evidence expectations differ enough that one identical
  packet would be misleading.

## D-003: Use architecture as the reference implementation

- **Decision**: Treat the already-delivered architecture ADR/options slice as
  the reference pattern for the wider feature.
- **Rationale**: This keeps the new work aligned with proven repository
  behavior instead of inventing a second incompatible model.

## D-004: Complete adjacent persona guidance through skills and docs first

- **Decision**: Add persona coverage for `review`, `pr-review`, `verification`,
  and `incident` through skills and documentation in this slice, while keeping
  their runtime packet families unchanged.
- **Rationale**: The user explicitly called out the gap, and guidance-only
  completion avoids changing runtime semantics.

## D-005: Treat `0.22.0` release surfaces as part of the feature contract

- **Decision**: Include version bump and release-surface synchronization inside
  the feature's implementation and validation plan.
- **Rationale**: The repo treats versioned documentation and compatibility
  references as part of the delivered surface, not as incidental release admin.

## User Story 1 Decisions

### D-006: Reuse the existing structural artifact family for option analysis

- **Decision**: Keep `system-shaping`, `architecture`, and `change` on their
  existing artifact families and canonical structural headings rather than
  introducing a new option-analysis packet family.
- **Rationale**: The structural modes already carry reviewable decision and
  tradeoff sections; the 022 work should strengthen discoverability and
  coverage, not create a second incompatible shape.

### D-007: Make system-shaping persona and structural alternatives explicit

- **Decision**: Add explicit system-shaping persona framing in skills,
  templates, examples, and run assertions while keeping the structural option
  headings (`Structural Options`, `Selected Boundaries`, `Rationale`) intact.
- **Rationale**: This closes the only obvious discoverability gap in the
  structural family without widening runtime semantics.

## User Story 2 Decisions

### D-008: Extend framework evaluation by enriching existing artifact families

- **Decision**: Keep the `implementation` and `migration` artifact file names
  stable while adding explicit decision-alternative sections such as
  `Options Matrix`, `Recommendation`, `Adoption Implications`, and
  `Ecosystem Health`.
- **Rationale**: This preserves current publish paths, gate behavior, and run
  summaries while making stack and migration choices reviewable inside the
  existing packet families.

### D-009: Keep migration and implementation evaluation recommendation-only

- **Decision**: Add option-comparison and ecosystem-health reasoning without
  changing execution posture, approval targets, or migration cutover semantics.
- **Rationale**: The new authored sections improve decision quality without
  widening Canon's mutating authority or operational claims.

## User Story 3 Decisions

### D-010: Use skill-level persona blocks as the explicit discoverability surface

- **Decision**: Add `Packet Shape And Persona` guidance to the runtime-targeted
  modes missing it (`system-shaping`, `implementation`, `migration`) and to the
  review-like modes (`review`, `pr-review`, `verification`, `incident`).
- **Rationale**: Skill-level discoverability satisfies the release need without
  introducing new packet contracts or hidden runtime behavior.

### D-011: Keep persona guidance advisory-only across all affected modes

- **Decision**: Every persona block must state that persona wording is
  presentation only and must not backfill missing authored sections or imply
  new authority.
- **Rationale**: This preserves Canon's honesty contract and prevents persona
  language from fabricating completeness or approval.
