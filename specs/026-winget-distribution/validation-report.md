# Validation Report: Winget Distribution And Roadmap Refocus

## Planned Structural Validation

- Verify generated Windows package-manager artifacts against the declared
  release metadata and checksum surface.
- Validate release workflow, release helper scripts, and documentation edits for
  formatting or schema drift.

## Planned Logical Validation

- Add a focused release-surface test that exercises Windows metadata, manifest
  generation, and verification behavior.
- Extend documentation validation so Windows install guidance, changelog, and
  roadmap expectations remain aligned.

## Planned Independent Validation

- Perform an independent review of roadmap cleanup and Windows distribution
  evidence after code and docs are updated.
- Confirm the feature stays bounded to packaging and documentation surfaces and
  does not reintroduce protocol or MCP work through implementation drift.

## Executed Foundational Packaging Evidence

- Added `tests/release_026_winget_distribution.rs` to cover Windows
  distribution metadata, deterministic manifest rendering, release-surface
  verification, workflow wiring, install guidance, changelog entries, and
  roadmap cleanup.
- Extended `scripts/release/write-distribution-metadata.sh` so the Windows
  asset advertises the `winget` channel and added
  `scripts/release/render-winget-manifests.sh` plus `packaging/winget/*.tpl`
  for deterministic multi-file manifest output.
- Extended `scripts/release/verify-release-surface.sh` so the release contract
  verifies both the Windows distribution metadata entry and the generated
  `winget` manifest bundle.

## User Story 1 Validation Evidence

- `.github/workflows/release.yml` now generates
  `canon-<VERSION>-distribution-metadata.json`, renders `dist/winget/*.yaml`,
  verifies both surfaces, and publishes the manifest bundle with the canonical
  release artifacts.
- `.github/release-notes-template.md` and `CHANGELOG.md` now describe the
  Windows package-manager publication artifacts and the `0.25.0` winget slice.

## User Story 2 Validation Evidence

- `README.md` now presents `winget install ApplyThe.Canon` and
  `winget upgrade ApplyThe.Canon` as the primary Windows path while preserving
  the direct archive fallback.
- The focused release/documentation tests assert both the primary `winget`
  path and the continued presence of the Windows zip fallback.

## User Story 3 Validation Evidence

- `ROADMAP.md` no longer presents Protocol Interoperability or MCP as active
  next work and keeps Windows `winget` distribution as the concrete release
  follow-on.
- The focused release/documentation tests assert the absence of the Protocol
  Interoperability section and the continued visibility of the Windows
  distribution priority.

## Executed Validation Commands

- `cargo test --test release_026_winget_distribution`
- `cargo test --test release_024_docs --test release_026_winget_distribution --test skills_bootstrap`
- `/bin/bash scripts/validate-canon-skills.sh`
- Direct shell validation of `scripts/release/write-distribution-metadata.sh`,
  `scripts/release/render-winget-manifests.sh`, and
  `scripts/release/verify-release-surface.sh` against a synthetic `0.25.0`
  release bundle
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Independent Review

- Reviewed the changed surfaces for scope drift: versioning, release scripts,
  workflow publication, documentation, roadmap text, and focused regression
  tests only.
- Confirmed the implementation does not introduce engine, adapter, policy, or
  runtime schema changes beyond the explicit `0.25.0` compatibility/version
  surface needed for the release slice.
- Confirmed Windows package-manager support remains recommendation-only and
  repository-local; live `winget-pkgs` submission stays manual and out of
  scope for this feature.

## Residual Validation Risks

- Windows Package Manager submission mechanics are external to this repository,
  so the first slice validates generated manifests and documented maintainer
  steps rather than live community-repository submission.
- The Windows archive currently packages a portable executable inside a zip, so
  manifest validation must explicitly confirm nested portable-file metadata.