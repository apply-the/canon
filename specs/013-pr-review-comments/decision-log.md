# Design Decision Log: PR Review Conventional Comments

## D-001: Additive artifact, not replacement summary

- **Status**: Implemented
- **Context**: `review-summary.md` already anchors current status and next-step
  surfaces.
- **Decision**: Add `conventional-comments.md` to the packet instead of
  replacing `review-summary.md`.
- **Consequences**: Existing approval and summary paths remain stable while the
  new reviewer-facing shape becomes publishable.

## D-002: Surface-scoped comments in the first slice

- **Status**: Implemented
- **Context**: The current review packet does not preserve durable inline diff
  coordinates.
- **Decision**: Emit file/surface-scoped comments only in the first slice.
- **Consequences**: The artifact remains honest and host-agnostic, but direct
  inline-host export stays deferred.

## D-003: Deterministic kind mapping

- **Status**: Implemented
- **Context**: Reviewer-facing output needs to be testable and stable.
- **Decision**: Map findings to Conventional Comments kinds deterministically
  from persisted severity/category signals.
- **Consequences**: Tests and published packets remain stable, and must-fix
  findings do not get softened accidentally.

## D-004: First-slice one-finding-to-one-comment model

- **Status**: Implemented
- **Context**: Aggregating findings into a single reviewer-facing comment would
  complicate traceability and make validation noisier in the first slice.
- **Decision**: Keep the first slice at one persisted `ReviewFinding` to one
  Conventional Comments entry.
- **Consequences**: The packet remains easy to validate; aggregation can be a
  future refinement if readability needs it.
