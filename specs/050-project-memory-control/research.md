# Research: Project Memory And Delivery Control Contracts

## Decision 1: Keep the canonical owner-side contract in `tech-docs/integration/`

- **Decision**: Keep `tech-docs/integration/project-memory-promotion-contract.md` as
  the stable Canon consumer-discovery path and mirror the same semantics in the
  feature-local `contracts/` directory.
- **Rationale**: Canon already exposes integration contracts under
  `tech-docs/integration/`; keeping that path avoids forcing Boundline or future
  consumers to discover the current contract by searching numbered specs.
- **Alternatives considered**:
  - Move the stable contract to `tech-docs/contracts/`: rejected because it creates
    a second stable convention without enough value in V1.
  - Keep the contract only under the feature-local spec: rejected because
    consumers need a stable path outside numbered feature folders.

## Decision 2: Use producer-neutral managed blocks

- **Decision**: Standardize repo-visible generated sections on a
  `project-memory:managed` marker family that includes `producer`, `source_ref`,
  and `contract_version`.
- **Rationale**: `tech-docs/evidence/` can legitimately contain Canon-produced and
  Boundline-produced blocks. Canon-specific markers would make mixed authorship
  look like Canon-only ownership even where that is false.
- **Alternatives considered**:
  - Keep Canon-specific managed markers: rejected because the evidence surface
    is intentionally mixed-producer.
  - Replace entire generated files: rejected because curated human-authored
    content must remain safe outside generated regions.

## Decision 3: Keep V1 lineage intentionally small

- **Decision**: Require only `contract_version`, `producer`, `source_ref`,
  `source_artifacts`, `promotion_state`, `promoted_at`, and `content_digest` in
  V1. Treat `mode`, `stage`, `owner`, `risk`, `zone`, `approval_state`,
  `packet_readiness`, and `promotion_profile` as optional.
- **Rationale**: The consumer needs a reliable minimum to implement against.
  Requiring the full long-form metadata set in V1 would make the first usable
  contract heavier than necessary.
- **Alternatives considered**:
  - Make the full metadata envelope required in V1: rejected because it adds
    implementation burden without changing the core ownership boundary.
  - Leave the required set implicit: rejected because it keeps the contract
    ambiguous.

## Decision 4: Strengthen compatibility policy

- **Decision**: Treat additive V1 changes as backward-compatible, require a new
  major contract line for removing or renaming required fields, and support the
  previous minor published contract revision for one full minor release cycle.
- **Rationale**: Weak wording such as "when feasible" is not a real contract.
  Consumers need deterministic rules for proceed, warn, or stop behavior.
- **Alternatives considered**:
  - Preserve the old pre-stable "best effort" wording: rejected because the new
    control layer is intended to be consumable, not advisory-only.
  - Freeze the contract entirely in V1: rejected because additive evolution must
    remain possible.

## Decision 5: Default to `tech-docs/project/` and `tech-docs/evidence/`

- **Decision**: Use `tech-docs/project/` and `tech-docs/evidence/` as the default
  repo-visible targets for V1, with managed blocks for stable promoted content
  and proposal or pending surfaces when stable promotion is not allowed.
- **Rationale**: The umbrella spec's main value is readable, visible project
  memory. Deferring target defaults would leave the first consumer slice without
  a stable repository shape.
- **Alternatives considered**:
  - Make target paths fully configurable in V1: rejected because it expands the
    surface before the contract semantics are even stable.
  - Co-locate all evidence inside `tech-docs/project/`: rejected because project
    memory and evidence have different credibility semantics.