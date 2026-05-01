# Decision Log: Structured External Publish Destinations

## D-001: Keep the first slice inside the current publish CLI contract

- **Decision**: Preserve the existing `publish` command shape and explicit
  `--to` override behavior while changing only the default destination logic
  and published metadata output.
- **Rationale**: This delivers the repository-navigation value without opening
  a broader CLI design or migration problem.

## D-002: Treat structured default paths and metadata as one feature

- **Decision**: Deliver readable default destinations and published metadata in
  the same slice instead of shipping them separately.
- **Rationale**: A descriptor-based path without metadata would weaken
  traceability, while metadata alone would leave the main browsing problem
  unresolved.

## D-003: Keep `.canon/` runtime-only

- **Decision**: Preserve `.canon/` as runtime and evidence storage only and do
  not move published packets back into that tree.
- **Rationale**: The feature exists to improve external repository browsing,
  not to redefine Canon runtime storage.

## User Story 1 Decisions

### D-004: Preserve existing family roots in the first slice

- **Decision**: Keep the current family roots under `specs/` and `docs/` and
  change only the default leaf structure.
- **Rationale**: This bounds migration cost while still delivering readable
  publish paths.

### D-007: Derive the leaf from persisted descriptors and suffix collisions

- **Decision**: Build the default leaf from the run date plus a descriptor
  derived in priority order from persisted slug, persisted title, then
  mode-derived fallback, and append a short-id suffix only when another run
  already occupies the same destination.
- **Rationale**: This keeps the common path readable while preserving stable
  deterministic naming and avoiding destination collisions across runs.

## User Story 2 Decisions

### D-005: Emit a dedicated metadata artifact with the published packet

- **Decision**: Materialize a dedicated metadata artifact alongside the packet
  rather than injecting traceability headers into every published markdown file.
- **Rationale**: This is the smallest slice that restores auditability after
  the path contract changes.

### D-008: Name the sidecar `packet-metadata.json` and keep source lineage explicit

- **Decision**: Publish a `packet-metadata.json` sidecar that records source
  artifacts as their canonical `.canon/artifacts/<RUN_ID>/<mode>/<file>`
  lineage rather than inventing a second runtime identity scheme.
- **Rationale**: Reviewers can recover provenance from the published packet
  without blurring the boundary between external materialization and runtime
  storage.

## User Story 3 Decisions

### D-006: Make release and validation closeout non-optional

- **Decision**: Include version bump, impacted docs and changelog alignment,
  coverage review for touched Rust files, `cargo clippy`, and `cargo fmt` as
  explicit implementation tasks.
- **Rationale**: This repository treats release-facing docs and quality-gate
  evidence as part of the shipped contract.

### D-009: Lock the release surface with repo-level regression tests

- **Decision**: Keep the `0.29.0` release alignment enforceable through a
  dedicated release-surface regression test plus skills-bootstrap coverage for
  mirrored runtime compatibility references.
- **Rationale**: The publish contract spans runtime code, mirrored skill
  materialization, and public docs, so release drift needs an executable guard.