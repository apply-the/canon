# Research: Cybersecurity Risk Assessment Mode

## Decision 1: Keep the first slice authored-input driven and offline

- **Decision**: Build `security-assessment` on Canon's existing authored-input
  preservation model rather than adding live scanners, registry lookups, or
  external network calls.
- **Rationale**: The user requested an end-to-end deliverable in the current
  release boundary. Reusing the existing artifact pipeline keeps the blast
  radius bounded while still delivering a real new mode.
- **Alternatives considered**:
  - Add live vulnerability or dependency scanning in the same slice.
  - Delay the mode until external evidence collectors exist.

## Decision 2: Model the mode operationally on incident and migration

- **Decision**: Reuse the operational-mode pattern established by `incident`
  and `migration`: required system context, recommendation-only posture,
  publishable packet behavior, and explicit risk gating.
- **Rationale**: `security-assessment` is a bounded operational analysis mode,
  not a planning-only or mutating execution mode. The existing operational
  pattern is the nearest stable fit.
- **Alternatives considered**:
  - Model the feature on `architecture` even though it lacks publishable
    recommendation-only operational behavior.
  - Treat the feature as a new mutating execution mode.

## Decision 3: Use the roadmap artifact family as the first packet contract

- **Decision**: Implement the packet with the roadmap-aligned artifacts
  `assessment-overview.md`, `threat-model.md`, `risk-register.md`,
  `mitigations.md`, `assumptions-and-gaps.md`, `compliance-anchors.md`, and
  `assessment-evidence.md`.
- **Rationale**: This keeps the delivered mode aligned with the documented next
  feature instead of inventing a second security packet shape.
- **Alternatives considered**:
  - Collapse the mode into four generic artifacts.
  - Reuse incident artifact names with security-specific prose only.

## Decision 4: Reuse existing gate kinds for the first slice

- **Decision**: Gate `security-assessment` with existing `Risk`, `Architecture`,
  and `ReleaseReadiness` semantics rather than adding a new gate kind in the
  first slice.
- **Rationale**: The mode needs explicit risk, structural review, and evidence
  readiness checks, but a new gate kind would widen the blast radius without a
  strong behavioral benefit in this release.
- **Alternatives considered**:
  - Add a new security-specific gate kind and update policy files.
  - Skip architecture-style gating and rely only on risk plus release checks.

## Decision 5: Keep the mode scoped to existing systems in this release

- **Decision**: Require `--system-context existing` for `security-assessment`
  in `0.22.0`.
- **Rationale**: The mode assesses a bounded existing surface with trust
  boundaries, assets, and operational findings. New-system security shaping can
  remain future work through architecture or later dedicated expansion.
- **Alternatives considered**:
  - Allow both `new` and `existing` immediately.
  - Make system context optional.

## Decision 6: Treat `0.22.0` verification work as part of delivery

- **Decision**: Keep the version bump, release-surface sync, coverage growth,
  formatting, linting, and full regression pass in scope as explicit feature
  work.
- **Rationale**: The user requested those items directly, and the release
  contract is part of the feature's observable output.
- **Alternatives considered**:
  - Defer release hygiene until after the mode lands.
  - Treat targeted tests as sufficient and leave full workspace validation for
    another change.
