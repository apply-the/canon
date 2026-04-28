# Research: Decision Alternatives, Pattern Choices, And Framework Evaluations

## Decision 1: Keep the first slice authored and evidence-grounded, not adapter-driven

- **Decision**: Reuse Canon's authored-section preservation model for option
  matrices, tradeoff analysis, ecosystem-health notes, and adoption burden
  instead of adding live registry or GitHub evidence collectors in this slice.
- **Rationale**: The user asked for the next feature end to end now. Keeping the
  slice authored and evidence-grounded preserves a bounded blast radius and
  lets the repo prove the packet contract before introducing new adapters.
- **Alternatives considered**:
  - Add live registry, GitHub, or release-note collectors immediately.
  - Keep ecosystem-health entirely out of scope and only compare structural
    options.

## Decision 2: Use two packet families under one feature

- **Decision**: Treat `system-shaping`, `architecture`, and `change` as the
  structural decision-alternatives family, and treat `implementation` and
  `migration` as the framework-evaluation family.
- **Rationale**: The same high-level behavior applies across the feature, but
  the concrete authored sections and reviewer expectations differ between
  structural design decisions and tool or platform selection.
- **Alternatives considered**:
  - Force every in-scope mode into one identical artifact list.
  - Split the feature into two separate roadmap items immediately.

## Decision 3: Treat architecture as the reference pattern, not the only mode

- **Decision**: Preserve `architecture` as the already-proven reference for ADR
  and options behavior while extending the same decision-discipline to the
  other targeted modes.
- **Rationale**: Feature `018` already proved the authored comparison pattern.
  Reusing that vocabulary reduces design risk and keeps the new feature
  anchored in existing behavior.
- **Alternatives considered**:
  - Rebuild the architecture contract from scratch to match a new generic
    packet family.
  - Leave architecture out of scope entirely even though it is the clearest
    current example of the desired outcome.

## Decision 4: Complete adjacent persona guidance through skills and docs first

- **Decision**: Expand persona guidance for `review`, `pr-review`,
  `verification`, and `incident` through skill and documentation surfaces in
  this slice, while keeping their runtime artifact families unchanged.
- **Rationale**: The user explicitly called out missing persona guidance for
  reviewer-like modes. Guidance-only completion addresses discoverability and
  audience fit without reopening governance semantics or packet layouts.
- **Alternatives considered**:
  - Leave the remaining persona mapping in the roadmap only.
  - Add runtime-visible persona metadata under `.canon/` now.

## Decision 5: Treat `0.22.0` as part of the feature contract

- **Decision**: Include version bump and release-surface synchronization in the
  feature itself rather than treating them as incidental follow-up.
- **Rationale**: The user explicitly requested the first task to be the version
  bump and the last task to update docs, examples, and roadmap. Capturing the
  release boundary in the design avoids those steps becoming an afterthought.
- **Alternatives considered**:
  - Defer version and release-surface changes until after implementation.
  - Update only `Cargo.toml` and leave other compatibility surfaces for later.
