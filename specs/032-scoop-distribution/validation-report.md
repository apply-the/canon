# Validation Report: Scoop Distribution Follow-On

## Structural Validation

- Added `tests/release_032_scoop_distribution.rs` to stage a synthetic release
  bundle and verify shared Windows distribution metadata, deterministic Scoop
  manifest rendering, workflow wiring, publication docs, roadmap delivery text,
  changelog delivery text, and `0.32.0` runtime-compatibility references.
- Extended `scripts/release/write-distribution-metadata.sh` so the canonical
  Windows asset advertises both `winget` and `scoop` channel membership.
- Extended `scripts/release/verify-release-surface.sh` so the release contract
  verifies the generated Scoop manifest alongside the existing distribution
  metadata and `winget` bundle.
- Added `scripts/release/render-scoop-manifest.sh` and
  `packaging/scoop/canon.json.tpl` for deterministic Scoop artifact rendering.

## User Story 1 Validation Evidence

- `.github/workflows/release.yml` now renders
  `canon-<VERSION>-scoop-manifest.json`, verifies it against the canonical
  release bundle, and uploads it with the GitHub Release assets.
- `.github/release-notes-template.md` and
  `docs/guides/publishing-to-scoop.md` now describe the Scoop install path,
  generated artifact, and manual bucket-submission flow.

## User Story 2 Validation Evidence

- `README.md` now presents Scoop as a supported Windows install and upgrade path
  while keeping `winget` and the direct archive fallback visible.
- `docs/guides/publishing-to-winget.md` now cross-links the parallel Scoop
  maintainer flow and uses the current `0.32.0` version examples.

## User Story 3 Validation Evidence

- `ROADMAP.md` no longer keeps already delivered distribution and
  `system-assessment` work in the active remaining-candidates section and now
  records `032` as the delivered Scoop follow-on.
- `CHANGELOG.md` now records the `0.32.0` Scoop release.
- `Cargo.toml`, `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`,
  `.agents/skills/canon-shared/references/runtime-compatibility.toml`, and
  `tests/integration/skills_bootstrap.rs` now align on `0.32.0`.
- `AGENTS.md` was refreshed from the 032 plan and now reflects the Scoop
  distribution slice in its recent changes.

## Executed Validation Commands

- `cargo test --test release_032_scoop_distribution --test skills_bootstrap`
  passed with `4/4` tests green in `release_032_scoop_distribution` and `15/15`
  tests green in `skills_bootstrap`.
- `/bin/bash scripts/validate-canon-skills.sh` passed.
- Direct shell validation passed against a synthetic `0.32.0` release bundle by
  exercising `scripts/release/write-distribution-metadata.sh`,
  `scripts/release/render-winget-manifests.sh`,
  `scripts/release/render-scoop-manifest.sh`, and
  `scripts/release/verify-release-surface.sh` end to end.
- `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`
  completed and generated `lcov.info`.
- `cargo fmt` completed successfully.
- `cargo fmt --check` completed successfully.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  completed successfully.
- `cargo nextest run --status-level fail --final-status-level fail --success-output never --failure-output immediate-final`
  completed successfully on the final rerun with `326/326` tests passed.

## Coverage Notes

- `lcov.info` was generated successfully for the workspace.
- The changed Rust files in this slice are test-only:
  `tests/release_032_scoop_distribution.rs` and
  `tests/integration/skills_bootstrap.rs`.
- Those changed test files do not appear as source entries in `lcov.info`, so
  the coverage evidence for them is the direct focused test execution recorded
  above rather than per-file LCOV percentages.
- No non-test Rust source files changed in this slice.

## Independent Review

- Reviewed the changed surfaces for scope drift: release scripts, packaging
  template, GitHub Actions release workflow, install and maintainer docs,
  roadmap text, changelog, runtime-compatibility references, focused release
  tests, and Speckit artifacts only.
- Confirmed the implementation does not introduce Canon runtime, engine,
  adapter, policy, approval, or `.canon/` schema changes.
- Confirmed GitHub Releases remain the canonical artifact source, the existing
  Windows zip remains the single Windows installation payload, and final Scoop
  bucket submission remains manual and out of scope.

## Residual Validation Risks

- Scoop bucket acceptance is external to this repository, so this slice
  validates generated artifacts and maintainer instructions rather than a live
  upstream bucket merge.
- The Windows archive remains a portable zip payload, so release validation must
  continue checking filename, URL, and checksum alignment explicitly.
- LCOV does not provide per-file source coverage for the changed test-only Rust
  files, so their coverage evidence remains tied to the focused direct test
  commands above.