# Decision Log: Cybersecurity Risk Assessment Mode

## D-001: Keep the first slice authored-input driven and recommendation-only

- **Decision**: Deliver `security-assessment` as an authored-input,
  recommendation-only governed mode in `0.22.0`.
- **Rationale**: This provides a real security workflow without expanding into
  live scanners, autonomous remediation, or a new persistence schema.

## D-002: Model the mode on incident and migration operational patterns

- **Decision**: Reuse the operational-mode execution posture and publishability
  pattern from `incident` and `migration`.
- **Rationale**: `security-assessment` is an operational analysis mode that
  needs gated risk posture and readable publishable packets.

## D-003: Use the roadmap packet shape as the first artifact family

- **Decision**: Implement the mode with `assessment-overview.md`,
  `threat-model.md`, `risk-register.md`, `mitigations.md`,
  `assumptions-and-gaps.md`, `compliance-anchors.md`, and
  `assessment-evidence.md`.
- **Rationale**: This keeps the delivered mode aligned with the documented
  roadmap contract and gives security reviewers a recognizable packet.

## D-004: Keep gate behavior explicit but reuse existing gate kinds

- **Decision**: Reuse `Risk`, `Architecture`, and `ReleaseReadiness` gate kinds
  for the first slice.
- **Rationale**: The mode needs those controls, but a new gate kind is not
  required to deliver credible behavior in this release.

## D-005: Keep `0.22.0` release work inside the feature boundary

- **Decision**: The version bump, release-surface updates, coverage growth, and
  full validation closeout remain explicit feature tasks.
- **Rationale**: The user requested them directly, and the release surface is
  part of what maintainers observe.

## User Story 1 Decisions

### D-006: Require `--system-context existing` for the first slice

- **Decision**: Restrict `security-assessment` to existing systems in this
  release.
- **Rationale**: The packet evaluates real assets, trust boundaries, and risks
  in a bounded existing surface rather than greenfield shaping.

### D-007: Treat missing security sections with the existing honesty marker

- **Decision**: Missing required sections surface `## Missing Authored Body`
  rather than fallback narrative.
- **Rationale**: Security work is especially vulnerable to false confidence;
  the packet must stay critique-first.

## User Story 2 Decisions

### D-008: Add a dedicated skill and canonical input locations

- **Decision**: Ship `canon-security-assessment` with canonical input paths at
  `canon-input/security-assessment.md` and `canon-input/security-assessment/`.
- **Rationale**: A first-class mode needs first-class authoring guidance and
  consistent auto-binding hints.

### D-009: Publish to a dedicated security-assessments directory

- **Decision**: Publish readable packets under `docs/security-assessments/<RUN_ID>/`.
- **Rationale**: Security packets need a stable, mode-specific destination that
  does not collide with incidents, migrations, or reviews.

## User Story 3 Decisions

### D-010: Treat coverage growth as explicit regression work, not a side effect

- **Decision**: Add dedicated tests for contract, renderer, docs, run, and
  publish behavior, then finish with full workspace regression.
- **Rationale**: The user explicitly asked for coverage and passing tests, and a
  new first-class mode needs proof at each runtime surface.
