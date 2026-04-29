# Validation Report: Distribution Channels Beyond GitHub Releases

## Validation Summary

- **Feature**: `025-distribution-channels`
- **Release Target**: `0.25.0`
- **Status**: Completed
- **Validation owner**: feature implementation plus separate review of the
  generated metadata, formula, and install surface

## Structural Validation Plan

| Check | Purpose | Planned Evidence | Status |
|------|---------|------------------|--------|
| Distribution metadata contract | Ensure the machine-readable artifact is complete and stable | generated metadata file plus focused contract assertions | Passed |
| Formula rendering contract | Ensure URLs, checksums, and platform branches are correct | rendered formula artifact plus formula-focused tests | Passed |
| Release workflow and script integrity | Ensure added automation is syntactically and structurally valid | shell syntax checks, workflow review, and release-surface script runs | Passed |
| Documentation sync | Ensure Homebrew instructions and fallback docs remain aligned | README and release-facing docs review | Passed |

## Logical Validation Plan

| Check | Purpose | Planned Evidence | Status |
|------|---------|------------------|--------|
| Metadata generation test | Validate metadata creation from a verified release fixture | focused `cargo test --test release_025_distribution` and release-facing regression coverage | Passed |
| Formula render test | Validate Homebrew formula URLs and `sha256` values against metadata | focused release-surface test coverage | Passed |
| Publication fallback test | Validate artifact-only behavior when tap publication is unavailable | workflow and script-level test coverage | Passed |
| Homebrew smoke validation | Validate the user-facing install path when Homebrew is available, or record the explicit environment gap when it is not | local `brew install --formula` smoke check and basic Canon command execution | Gap recorded |

## Coverage Goal

- Any new or modified Rust source files introduced by this feature should
  retain at least 85% line coverage.
- Script, workflow, and generated-artifact logic should be covered by focused
  fixture-based validation and syntax checks even where Rust coverage is not
  applicable.

## Independent Validation Plan

- Review the generated distribution metadata against the verified release bundle
  and checksum manifest.
- Review the rendered Homebrew formula against the metadata contract and the
  documented install path.
- Review the tap publication output or artifact-only fallback to confirm a
  durable distribution artifact is always preserved.

## Evidence Log

| Evidence Item | Path Or Command | Result | Notes |
|--------------|------------------|--------|-------|
| Specification review | `specs/025-distribution-channels/spec.md` | Passed | No unresolved clarification markers remain |
| Plan review | `specs/025-distribution-channels/plan.md` | Passed | Governance context, invariants, and validation ownership are explicit |
| Task review | `specs/025-distribution-channels/tasks.md` | Passed | Task ordering and story coverage align with spec and plan |
| Release metadata evidence | generated metadata artifact and focused validation output | Passed | Metadata enumerates all release assets and preserves Homebrew-only channel mapping |
| Formula evidence | rendered Homebrew formula and focused validation output | Passed | Formula URLs and checksums align with metadata and exclude Windows assets |
| Workflow closeout | focused tests, shell checks, fmt, clippy, docs review | Passed | Release workflow, docs, and quality gates all passed locally |

## Executed Evidence

| Evidence Item | Path Or Command | Result | Notes |
|--------------|------------------|--------|-------|
| Spec placeholder review | `rg '\[FEATURE NAME\]|\[DATE\]|\$ARGUMENTS|NEEDS CLARIFICATION' specs/025-distribution-channels/spec.md` | Passed | No unresolved spec template markers remain |
| Plan scaffold generation | `.specify/scripts/bash/setup-plan.sh --json` | Passed | Plan path resolved for the feature branch |
| Focused release distribution suite | `cargo test --test release_021_docs --test release_022_docs --test release_024_docs --test release_025_distribution` | Passed | 12 release-facing tests passed, including metadata, formula, sync, workflow, and docs coverage |
| Script syntax validation | `bash -n scripts/release/write-distribution-metadata.sh scripts/release/render-homebrew-formula.sh scripts/release/sync-homebrew-tap.sh scripts/release/verify-release-surface.sh` | Passed | All modified release scripts parsed cleanly |
| Editor diagnostics | `get_errors` for `.github/workflows/release.yml`, `README.md`, and `tests/release_025_distribution.rs` | Passed | No editor-reported errors for the modified workflow, docs, or focused tests |
| Formatting and lint gates | `cargo fmt && cargo fmt --check && cargo clippy --workspace --all-targets --all-features -- -D warnings` | Passed | Workspace formatting and lint gates are clean at `0.25.0` |
| Homebrew smoke validation | Not executed | Gap recorded | The generated formula targets unpublished `v0.25.0` GitHub Release assets; a real `brew install` would not be meaningful until release publication and would mutate the local Homebrew state |