# Decision Log: Decision Alternatives, Pattern Choices, And Framework Evaluations

## D-001: Keep live evidence collection out of the first 028 slice

- **Decision**: Keep the slice authored and evidence-grounded rather than
  adding new authenticated registry, GitHub, or release collectors.
- **Rationale**: This bounds the blast radius and keeps the feature shippable
  in one repository slice.

## D-002: Split the feature into structural and framework-evaluation families

- **Decision**: Use one shared feature with two packet families: structural
  decision modes (`system-shaping`, `change`) and framework evaluation modes
  (`implementation`, `migration`), with `architecture` retained as the
  regression anchor.
- **Rationale**: The same reviewer need exists across both groups, but the
  authored sections and evidence expectations differ enough that one identical
  packet would be misleading.

## D-003: Keep architecture as the reference implementation

- **Decision**: Treat the already-delivered architecture ADR/options slice as
  the reference pattern for the wider feature.
- **Rationale**: This keeps the new work aligned with proven repository
  behavior instead of inventing a second incompatible model.

## D-004: Treat `0.28.0` release surfaces and validation closeout as part of the feature contract

- **Decision**: Include version bump, runtime compatibility references,
  impacted docs, changelog alignment, Rust coverage evidence, `cargo clippy`,
  and `cargo fmt` inside the feature's implementation and validation plan.
- **Rationale**: The repository treats release-facing docs and compatibility
  references as part of the shipped contract, not as incidental release admin.

## User Story 1 Decisions

### D-005: Reuse the existing structural artifact family for decision analysis

- **Decision**: Keep `system-shaping` and `change` on their existing artifact
  families and canonical structural headings rather than introducing a new
  packet family.
- **Rationale**: The structural modes already carry reviewable decision and
  tradeoff sections; the 028 work should strengthen discoverability and
  coverage, not create a second incompatible shape.

## User Story 2 Decisions

### D-006: Enrich implementation and migration inside their current packet families

- **Decision**: Keep the `implementation` and `migration` artifact file names
  stable while adding explicit candidate, evidence, ecosystem-health, and
  adoption or rollback sections.
- **Rationale**: This preserves publish paths, gate behavior, and run summaries
  while making stack and migration choices more reviewable.

## User Story 3 Decisions

### D-007: Make task-level release closeout non-optional

- **Decision**: The final task graph must contain dedicated tasks for version
  bump, impacted docs plus changelog alignment, coverage review for touched
  Rust files, `cargo clippy`, and `cargo fmt`.
- **Rationale**: The user explicitly requested these tasks, and the repository
  has already shown release drift when they were treated as incidental.