# Decision Log: Output Quality Gates

## D-001: Deliver feature 034 as one end-to-end slice

- **Decision**: Keep runtime behavior, mirrored skills, docs, version bump,
  roadmap cleanup, changelog, coverage, `cargo clippy`, and `cargo fmt`
  inside one feature delivery.
- **Rationale**: The user explicitly requested that 034 not be split into
  slices, and output quality is incomplete if the authoring surfaces drift.

## D-002: Reuse the clarity plus placeholder-aware engine helpers as the main seam

- **Decision**: Build the shared output-quality assessment on top of existing
  authored-support, material-closure, and placeholder-count helpers rather than
  adding a parallel evaluator.
- **Rationale**: The current engine already has the right evidence signals; the
  defect is that they are not promoted into one explicit quality posture.

## D-003: Keep publishability posture descriptive, not operational

- **Decision**: Surface `publishable` as a packet-quality posture without
  changing publish destinations or publish command eligibility semantics.
- **Rationale**: The feature improves output honesty and reader guidance, not
  repository operations or approval boundaries.

## D-004: Prefer downgrade reasons over synthetic filler

- **Decision**: When a targeted packet cannot justify a stronger posture, keep
  downgrade reasons and honesty markers explicit in summaries or artifacts
  rather than smoothing the output with generic prose.
- **Rationale**: False confidence is the specific failure mode this feature is
  meant to eliminate.

## D-005: Do not punish materially closed packets for lacking artificial balance

- **Decision**: Treat materially closed decisions as a legitimate path to a
  stronger quality posture when the authored evidence supports the closure.
- **Rationale**: Canon should not encourage fabricated alternatives merely to
  satisfy quality heuristics.

## User Story 1 Decisions

- **Decision**: Extend inspect-facing data to include explicit output-quality
  posture and named downgrade reasons.
- **Rationale**: Maintainers need a cheap pre-read signal before trusting the
  packet or opening every artifact.

## User Story 2 Decisions

- **Decision**: Thread the shared posture through runtime summaries and at
  least one fallback-heavy artifact seam.
- **Rationale**: Summary language and rendered artifacts are where false
  confidence is most visible to readers.

## User Story 3 Decisions

- **Decision**: Treat version alignment, roadmap cleanup, docs, skill mirrors,
  coverage, `cargo clippy`, and `cargo fmt` as contractual closeout work.
- **Rationale**: Release drift is a recurring repo failure mode and must be
  closed inside the feature.

- **Decision**: Do not add brittle repository-doc content tests for the new
  wording.
- **Rationale**: The user explicitly asked to avoid those; runtime-backed checks
  and focused integration assertions remain the preferred validation path.