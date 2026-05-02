# Research: Release Provenance And Channel Integrity

## Decision 1: Extend the existing distribution metadata instead of creating a second manifest family

- **Decision**: Keep `distribution-metadata.json` as the canonical release
  manifest and extend it with explicit provenance and per-channel contract
  fields.
- **Rationale**: Canon already has one repository-owned metadata artifact that
  ties version, URLs, checksums, and channels back to GitHub Releases. A second
  manifest family would duplicate inventory and create a new drift surface.
- **Alternatives considered**:
  - Add a second provenance-only JSON file beside `distribution-metadata.json`.
  - Keep provenance implicit in scripts and docs without expanding the JSON
    contract.

## Decision 2: Model Homebrew, Winget, and Scoop as explicit channel contracts

- **Decision**: Add top-level channel-contract records that name each channel,
  the canonical asset ids it may consume, and the generated artifact shapes it
  expects.
- **Rationale**: Current renderers and verifiers still encode channel behavior
  implicitly. Explicit channel contracts make the source-of-truth auditable and
  give scripts one place to fail closed when channel assumptions drift.
- **Alternatives considered**:
  - Continue deriving channel behavior only from per-asset `channels` arrays.
  - Hardcode channel expectations independently inside each renderer and the
    verifier.

## Decision 3: Keep verification in the existing shell verifier and strengthen it from the contract

- **Decision**: Reuse `scripts/release/verify-release-surface.sh` as the main
  release-surface gate, but extend it so it validates provenance and
  channel-contract fields from the canonical metadata.
- **Rationale**: The verifier already gates the release bundle and package
  artifacts. Extending it preserves the current automation flow and avoids a
  second release-validation entrypoint.
- **Alternatives considered**:
  - Add a brand new verifier script for provenance only.
  - Defer channel-contract validation to renderer-specific scripts.

## Decision 4: Use focused Rust release tests for automation evidence rather than adding a new runtime surface

- **Decision**: Add repository-owned Rust release tests that stage a synthetic
  release bundle, generate metadata and channel artifacts, and assert fail-
  closed behavior for mismatched channel contracts.
- **Rationale**: The requested coverage, `cargo fmt`, and `cargo clippy`
  closeout fit naturally with focused Rust tests, while the feature itself
  remains bounded to release automation rather than expanding Canon's runtime
  CLI.
- **Alternatives considered**:
  - Add only shell walkthroughs without any Rust validation.
  - Introduce a new runtime inspect or verify command solely for this slice.

## Decision 5: Keep GitHub Releases explicit as the source of truth in docs and metadata

- **Decision**: Record the source-of-truth declaration both in the metadata
  contract and in release-facing docs.
- **Rationale**: This slice is about provenance clarity. Leaving the source of
  truth implicit would weaken the main product outcome.
- **Alternatives considered**:
  - Assume the existing docs are sufficient and skip explicit provenance text.
  - Describe channel derivation only in scripts and validation evidence.