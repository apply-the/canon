# Decision Log: Guided Run Operations And Review Experience

## D-001: Keep operator guidance in the existing runtime summary path

- **Date**: 2026-05-02
- **Status**: Accepted
- **Decision**: Extend the current `service.rs` runtime-details assembly,
  `next_action.rs`, `summarizers.rs`, and CLI rendering path instead of adding a
  separate operator-control subsystem.
- **Rationale**: The active run state, blocked gates, approval targets, and
  result packet facts already converge there. A new subsystem would duplicate
  authority and increase drift risk.

## D-002: Treat possible actions as mandatory text, chips as optional enhancement

- **Date**: 2026-05-02
- **Status**: Accepted
- **Decision**: Preserve `Possible Actions:` and `Recommended Next Step:` as the
  canonical operator contract, with action chips only mirroring those actions.
- **Rationale**: Existing skill and frontend contracts already require this.
  Hosts without chip rendering must still receive the full guided flow.

## D-003: Prefer packet review before approval when Canon already emitted a readable packet

- **Date**: 2026-05-02
- **Status**: Accepted
- **Decision**: When a gated or blocked run has a readable packet, the first
  recommended action is packet review rather than direct approval or deeper
  evidence inspection.
- **Rationale**: This keeps the approval boundary honest and matches the shared
  output-shape guidance already present in the repository.

## D-004: Keep version, docs, and roadmap alignment inside the feature slice

- **Date**: 2026-05-02
- **Status**: Accepted
- **Decision**: Treat `0.37.0` alignment, changelog updates, roadmap cleanup,
  and release-surface validation as required parts of `038`, not post-feature
  cleanup.
- **Rationale**: This feature is primarily operator-facing. Repository guidance
  that disagrees with runtime behavior would leave the slice incomplete.