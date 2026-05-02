# Validation Report: Release Provenance And Channel Integrity

## Structural Validation

- Extended `scripts/release/write-distribution-metadata.sh` so the canonical
  distribution metadata now emits explicit `source_of_truth` and top-level
  `channels` contract records derived from the release asset inventory.
- Tightened `scripts/release/render-homebrew-formula.sh`,
  `scripts/release/render-winget-manifests.sh`, and
  `scripts/release/render-scoop-manifest.sh` so each renderer requires its own
  channel contract, declared asset ids, and declared generated-artifact names
  before it writes output.
- Tightened `scripts/release/verify-release-surface.sh` so release validation
  now rejects mismatched `source_of_truth` fields and top-level channel
  contracts before publication continues.
- Added `tests/release_036_release_provenance_integrity.rs` with positive and
  fail-closed coverage for the metadata writer, all three renderers, release
  verification, and release-facing version or documentation alignment.

## User Story 1 Validation Evidence

- `distribution_metadata_includes_provenance_and_channel_contracts` proves the
  canonical metadata now records the explicit provenance and channel-contract
  shape required by the spec.
- `release_surface_verifier_rejects_mismatched_source_of_truth` proves the
  verifier blocks publication when the canonical provenance declaration drifts
  from the real release bundle.

## User Story 2 Validation Evidence

- `homebrew_renderer_rejects_missing_homebrew_channel_contract`,
  `winget_renderer_rejects_missing_generated_artifact_expectation`, and
  `scoop_renderer_rejects_missing_scoop_channel_contract` prove the renderers
  fail closed when channel contracts are absent or incomplete.
- `release_surface_verifier_rejects_channel_contract_asset_drift` proves the
  verifier rejects channel contracts that contradict the canonical asset
  inventory.
- `canonical_release_contract_renders_and_verifies_all_channels` proves the
  canonical metadata can drive Homebrew, Winget, and Scoop artifacts end to end
  and that the final release verifier accepts the coherent bundle.

## User Story 3 Validation Evidence

- `release_docs_and_version_surfaces_align_on_0_36_0_provenance` proves the
  version bump, runtime compatibility references, README, publication guides,
  roadmap, and changelog now tell one coherent `0.36.0` provenance story.
- `tests/integration/skills_bootstrap.rs` continues to verify that `skills
  install` materializes the current runtime compatibility reference and now
  carries the `0.36.0` expectation.

## Executed Validation Commands

- `cargo test --test release_036_release_provenance_integrity --test skills_bootstrap`
  passed with `8/8` tests green in
  `release_036_release_provenance_integrity` and `15/15` tests green in
  `skills_bootstrap`.
- Direct shell validation passed against a synthetic `0.36.0` release bundle by
  exercising `scripts/release/write-distribution-metadata.sh`,
  `scripts/release/render-homebrew-formula.sh`,
  `scripts/release/render-winget-manifests.sh`,
  `scripts/release/render-scoop-manifest.sh`, and
  `scripts/release/verify-release-surface.sh` end to end.
- `cargo fmt` completed successfully.
- `cargo fmt --check` completed successfully.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  completed successfully.
- `cargo nextest run` completed successfully with `327/327` tests passed.
- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`
  completed successfully and wrote `lcov.info`.

## Coverage Notes

- `lcov.info` was generated successfully for the workspace.
- The changed Rust files in this slice are test-only:
  `tests/release_036_release_provenance_integrity.rs` and
  `tests/integration/skills_bootstrap.rs`.
- Those changed test files do not appear as source entries in `lcov.info`, so
  the coverage evidence for them is the direct focused test execution recorded
  above rather than per-file LCOV percentages.
- No non-test Rust source files changed in this slice.

## Independent Review

- Reviewed the changed surfaces for scope drift: release scripts, focused Rust
  tests, version manifests, shared runtime compatibility references, README,
  maintainer publication guides, roadmap text, changelog, and Speckit artifacts
  only.
- Confirmed GitHub Releases remain the canonical source of binaries,
  filenames, checksums, and release notes.
- Confirmed the implementation does not introduce Canon runtime, engine,
  adapter, policy, approval, or `.canon/` schema changes.

## Residual Validation Risks

- Live external publication to Homebrew taps, `winget-pkgs`, and Scoop main is
  still outside this repository, so this slice validates generated artifacts
  and maintainer workflows rather than an upstream merge.
- The workspace coverage artifact does not report per-file LCOV percentages for
  the changed test-only Rust files, so their coverage evidence remains tied to
  the focused direct test commands above.