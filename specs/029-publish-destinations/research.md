# Research: Structured External Publish Destinations

## Decision 1: Keep existing family roots and change only the default leaf layout

- **Decision**: Preserve the current external family roots such as `specs/`,
  `docs/changes/`, `docs/reviews/prs/`, and `docs/incidents/`, and change only
  the default published leaf from `<RUN_ID>/` to
  `<YYYY-MM-DD>-<descriptor>/`.
- **Rationale**: This delivers readable browsing paths without forcing a wider
  migration of publish roots, docs structure, or CLI semantics.
- **Alternatives considered**:
  - Rework every publish root at the same time.
  - Keep run-id-only leaves and only add metadata sidecars.

## Decision 2: Derive the descriptor from persisted metadata in priority order

- **Decision**: Use an explicit persisted slug when available, fall back to a
  sanitized title when present, and fall back again to a stable mode-derived
  descriptor when no better label exists.
- **Rationale**: Existing run manifests already carry optional `slug` and
  `title` metadata, so this keeps the first slice within the current runtime
  model and avoids inventing new identity fields.
- **Alternatives considered**:
  - Derive the descriptor directly from the raw run id.
  - Require a new mandatory publish title before publishing.

## Decision 3: Preserve override behavior and materialize metadata as a sidecar

- **Decision**: Keep explicit `publish --to` overrides authoritative and add a
  dedicated metadata artifact alongside the published packet instead of
  rewriting every published markdown file.
- **Rationale**: This is the smallest implementation that preserves reviewer
  traceability, avoids broad artifact-rendering changes, and keeps override
  behavior stable.
- **Alternatives considered**:
  - Force overrides into the structured default destination.
  - Inject metadata headers into every published markdown artifact.

## Decision 4: Treat `0.29.0` release alignment and validation closeout as part of the feature

- **Decision**: Include version bump, impacted docs and changelog updates,
  coverage for modified or new Rust files, `cargo clippy`, and `cargo fmt`
  inside the feature task graph and validation report.
- **Rationale**: In this repository the release-facing docs and compatibility
  references are part of the delivered contract, not optional cleanup.
- **Alternatives considered**:
  - Defer version and doc alignment until after runtime changes merge.
  - Bump only manifests and leave docs or coverage closeout for follow-up.