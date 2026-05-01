# Research: Decision Alternatives, Pattern Choices, And Framework Evaluations

## Decision 1: Keep the first 028 slice authored and evidence-grounded, not adapter-driven

- **Decision**: Reuse Canon's authored-section preservation model for decision
  alternatives, framework evaluation, and decision-evidence references instead
  of adding live authenticated collectors in this slice.
- **Rationale**: This keeps the blast radius bounded enough to ship the packet
  contract, renderer, documentation, and test changes end to end on one branch
  without introducing new adapter or network behavior.
- **Alternatives considered**:
  - Add live registry, GitHub, or release-note collectors immediately.
  - Leave evidence posture out of scope and only compare options structurally.

## Decision 2: Use two packet families under one feature

- **Decision**: Treat `system-shaping` and `change` as the structural
  decision-analysis family, and treat `implementation` and `migration` as the
  framework-evaluation family.
- **Rationale**: The same high-level review need exists across the feature, but
  the authored sections and evidence posture differ between structural design
  choices and concrete stack or migration decisions.
- **Alternatives considered**:
  - Force every targeted mode into one identical packet shape.
  - Split the work into two separate features immediately.

## Decision 3: Keep architecture as the regression anchor, not the main delivery target

- **Decision**: Preserve `architecture` as the already-delivered reference
  implementation for option-analysis vocabulary and use it as a regression
  anchor rather than a primary runtime change surface.
- **Rationale**: The repository already proved the option-analysis pattern in
  architecture. Reusing that baseline reduces design risk and keeps the new
  feature anchored in existing behavior.
- **Alternatives considered**:
  - Rework architecture into a new generic packet family.
  - Remove architecture entirely from the feature, losing the clearest
    regression reference.

## Decision 4: Treat version bump, docs sweep, and validation closeout as first-class scope

- **Decision**: Include the `0.28.0` version bump, impacted docs and changelog
  updates, coverage for touched Rust files, `cargo clippy`, and `cargo fmt` in
  the feature task graph rather than as post-feature cleanup.
- **Rationale**: This repository treats versioned documentation and runtime
  compatibility references as contract surfaces. Keeping them outside the task
  graph creates avoidable release drift.
- **Alternatives considered**:
  - Defer version and docs work until after runtime changes land.
  - Update only `Cargo.toml` and leave other release surfaces for later.