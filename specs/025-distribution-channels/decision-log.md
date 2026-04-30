# Decision Log: Distribution Channels Beyond GitHub Releases

## D-001 Canonical Distribution Source

- **Decision**: Keep GitHub Releases and the verified release bundle as the
  single source of truth for distribution assets, checksums, and filenames.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: The feature should extend the existing release surface rather
  than creating a second packaging authority.

## D-002 Distribution Metadata Contract

- **Decision**: Emit `canon-<VERSION>-distribution-metadata.json` as the
  channel-neutral machine-readable artifact for downstream package managers.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: A channel-neutral metadata file lets Homebrew ship now while
  future `winget` and Scoop work reuse the same verified asset inventory.

## D-003 Homebrew Delivery Strategy

- **Decision**: Use a dedicated tap formula that installs Canon from canonical
  prebuilt archives instead of introducing a new build path or starting with
  `homebrew/core`.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: The first slice needs controlled release automation and a
  stable `brew install` surface without depending on external core-repo review.

## D-004 Publication Fallback

- **Decision**: Always render a release-specific formula artifact and treat tap
  synchronization as optional automation when repository configuration exists.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: Artifact-first delivery preserves traceability and avoids
  making release correctness depend on cross-repository credentials.

## D-005 Validation Strategy

- **Decision**: Validate the feature with focused release-surface tests,
  metadata and formula consistency checks, script syntax checks, and optional
  local Homebrew smoke validation.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: Most of the feature lives in workflow, scripts, and generated
  artifacts rather than runtime Rust paths, so validation must center on the
  release surface itself.

## D-006 Release Workflow Staging

- **Decision**: Keep `assemble-release` as the single release-bundle assembly
  point, but extend it to generate checksums first, then distribution metadata,
  then the Homebrew formula, and finally re-run release-surface verification
  against those generated artifacts before publication.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: This preserves one canonical release pipeline while ensuring
  channel artifacts are derived from already verified archives instead of from
  parallel packaging inputs.

## D-007 Tap Synchronization Boundary

- **Decision**: Treat tap synchronization as a workflow-level optional job that
  syncs into a checked-out tap repository when repository configuration exists,
  while preserving the rendered formula artifact as the durable fallback in all
  cases.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: This keeps cross-repository automation bounded and avoids
  making release correctness depend on live tap access.

## D-008 Local Smoke Validation Posture

- **Decision**: Record the Homebrew smoke-install step as an explicit
  environment gap until `v0.25.0` assets exist on GitHub Releases, rather than
  mutating the local Homebrew environment against unpublished release URLs.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: The real formula points at canonical release URLs, so a true
  `brew install` check is only meaningful once those assets are published.

## User Story 1 Decisions

- **US1-D1 Asset reuse**: Homebrew installs Canon from the same release archives
  already verified by the existing release pipeline.
- **US1-D2 Fallback visibility**: README install guidance keeps the archive
  download path visible alongside Homebrew.

## User Story 2 Decisions

- **US2-D1 Formula rendering**: Release automation derives formula content from
  distribution metadata rather than hand-maintained URL and checksum blocks.
- **US2-D2 Publication mode**: The workflow can either sync to a dedicated tap
  repository or preserve an artifact-only output when sync is unavailable.
- **US2-D3 Workflow gate**: Tap synchronization only runs when the release is
  being published and both the tap repository and token are configured.

## User Story 3 Decisions

- **US3-D1 Contract neutrality**: The distribution metadata includes Windows
  assets even though the first consumer is Homebrew.
- **US3-D2 Smoke validation**: Formula validation uses a basic working Canon CLI
  path instead of a version-only test.
- **US3-D3 Release-facing docs**: README, release notes, changelog, and roadmap
  all describe the Homebrew-first slice while leaving `Protocol
  Interoperability` as the next follow-on roadmap item.