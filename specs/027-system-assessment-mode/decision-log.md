# Decision Log: System Assessment Mode

## D-001: Introduce `system-assessment` as a dedicated existing-system mode

- **Status**: Accepted
- **Rationale**: The roadmap and product discussion explicitly separate as-is
  assessment from decision-shaped architecture work. A dedicated mode keeps the
  contracts, summaries, and follow-on posture clear.

## D-002: Require `system-context existing`

- **Status**: Accepted
- **Rationale**: The packet is only credible when grounded in present-tense
  system evidence. Supporting `new` would collapse this slice into
  future-state system-shaping or architecture work.

## D-003: Use ISO 42010 language for coverage and viewpoints

- **Status**: Accepted
- **Rationale**: ISO 42010 gives the packet a defensible way to describe what
  concerns were assessed, which views were covered, and what remains partial or
  skipped.

## D-004: Make observed findings, inferred findings, and assessment gaps first-class output categories

- **Status**: Accepted
- **Rationale**: Honest bounded coverage is more valuable than implied
  certainty, especially for large or partially observable repositories.

## D-005: Publish under `docs/architecture/assessments/`

- **Status**: Accepted
- **Rationale**: The packet belongs in the architecture family, but it must
  stay visibly separate from decision packets under
  `docs/architecture/decisions/`.

## User Story 1 Decisions

- Reuse the operational analysis mode pipeline already used by
  `security-assessment` and `supply-chain-analysis` instead of inventing a new
  orchestration path.
- Keep the first slice to five high-value view families plus coverage, asset,
  risk, and evidence artifacts.
- Surface `assessment-overview.md` as the primary summary artifact and keep the
  emitted execution posture explicitly `recommendation-only`.

## User Story 2 Decisions

- Make the authoring contract explicit through a dedicated skill, template, and
  realistic example instead of relying on runtime errors alone.
- Keep the publish root within the architecture docs family for discoverability.
- Materialize shared runtime hints, skill indexes, and validator rules so
  `canon init`, `skills install`, and the repo-local skill checks all recognize
  `system-assessment` as a first-class authored-input mode.

## User Story 3 Decisions

- Treat the `0.26.0` version bump as the first implementation task.
- Keep roadmap, docs, changelog, coverage, formatting, and clippy clean-up in
  the final closeout task rather than scattering them across the feature.
- Lock the public release surface with focused release-doc regression tests and
  a `canon-cli` summary-rendering test rather than relying on manual release
  review alone.