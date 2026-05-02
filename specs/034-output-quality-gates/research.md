# Research: Output Quality Gates

## Decision 1: Build output quality on the existing clarity and placeholder-aware seams

- **Decision**: Introduce a shared output-quality assessment in the existing
  engine surfaces that already understand authored support, material closure,
  and placeholder filtering rather than inventing a parallel scoring service.
- **Rationale**: `clarity.rs` already exposes `weak_reasoning` and
  `materially_closed`, while `context_parse.rs` and `summarizers.rs` already
  distinguish real context items from placeholders. Reusing that seam keeps the
  feature inspectable and minimizes mode-specific drift.
- **Alternatives considered**:
  - Add a new repository-wide quality score detached from clarity and
    summarizer logic.
  - Patch each mode summary with bespoke wording heuristics.

## Decision 2: Model posture explicitly instead of relying on good-sounding summaries

- **Decision**: Represent output quality with explicit postures such as
  `structurally-complete`, `materially-useful`, and `publishable`, plus named
  downgrade reasons or evidence signals.
- **Rationale**: The product defect is false confidence. A named posture is
  easier to inspect, serialize, test, and mirror in docs or skills than ad hoc
  prose upgrades.
- **Alternatives considered**:
  - Keep posture implicit inside summary copy.
  - Use a numeric score or percentage-based grade.

## Decision 3: Keep publishability posture separate from publish command eligibility

- **Decision**: Surface whether a packet reads as publishable without changing
  the underlying publish command semantics or `.canon/` artifact contract.
- **Rationale**: The feature is about honesty of output quality, not about
  changing operational permissions or publish destinations.
- **Alternatives considered**:
  - Gate `publish` directly on the new posture.
  - Avoid the word `publishable` entirely and only report generic readiness.

## Decision 4: Treat materially closed packets as justified quality, not weak reasoning

- **Decision**: Distinguish materially closed decisions from shallow packets so
  Canon can preserve bounded closure without inventing alternatives merely to
  satisfy quality heuristics.
- **Rationale**: Some packets are strong precisely because the decision is
  tightly constrained. Penalizing them would encourage synthetic balance.
- **Alternatives considered**:
  - Require multiple options or tradeoffs for every strong classification.
  - Collapse materially closed and weakly reasoned packets into one cautionary
    bucket.

## Decision 5: Use downgrade reasons, not filler, at the renderer boundary

- **Decision**: Where a targeted artifact cannot justify a stronger posture,
  render explicit missing-body or downgrade language instead of synthetic prose
  that looks like approved reasoning.
- **Rationale**: The renderer boundary is where false confidence becomes most
  visible to end readers. Honest downgrade language is safer than placeholder
  completion.
- **Alternatives considered**:
  - Keep fallback artifact prose and only downgrade inspect output.
  - Omit the artifact entirely when support is weak.

## Decision 6: Ship runtime, authoring surfaces, and roadmap cleanup as one 0.34.0 slice

- **Decision**: Keep version bump, skill mirrors, docs, changelog, roadmap
  cleanup, coverage, `cargo clippy`, and `cargo fmt` inside the same feature
  delivery as the runtime changes.
- **Rationale**: The user explicitly requested one unsliced feature. Output
  quality is a product contract, so runtime and authoring surfaces must land
  together.
- **Alternatives considered**:
  - Land runtime changes first and clean docs or roadmap later.
  - Leave the roadmap with a follow-on candidate block after delivery.