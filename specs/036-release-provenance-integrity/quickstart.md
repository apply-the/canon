# Quickstart: Release Provenance And Channel Integrity

## Goal

Validate that Canon can generate one canonical release provenance manifest and
derive Homebrew, Winget, and Scoop artifacts from explicit channel contracts.

## Prerequisites

- Repository checkout on branch `036-release-provenance-integrity`
- `jq`, `shasum`, `unzip`, and Rust toolchain available
- A writable temporary directory for synthetic release artifacts

## Walkthrough

1. Prepare a synthetic `dist/` release bundle containing the existing Canon
   archives, checksum manifest, and release notes file for one target version.
2. Run `scripts/release/write-distribution-metadata.sh` to generate
   `distribution-metadata.json` and confirm it records:
   - top-level version, release URL, and canonical artifact references
   - an explicit source-of-truth declaration
   - canonical asset inventory entries
   - explicit `homebrew`, `winget`, and `scoop` channel contracts
3. Run the renderer scripts against that metadata:
   - `scripts/release/render-homebrew-formula.sh`
   - `scripts/release/render-winget-manifests.sh`
   - `scripts/release/render-scoop-manifest.sh`
4. Run `scripts/release/verify-release-surface.sh` and confirm it accepts the
   generated metadata and channel artifacts.
5. Mutate the metadata to remove one required channel contract or generated
   artifact expectation, rerun the relevant renderer or verifier, and confirm
   it fails closed.
6. Run the focused Rust release test for this slice plus the version-alignment
   test surface.

## Expected Result

- All derived package-manager artifacts are rendered from the same canonical
  GitHub Release contract.
- Missing provenance fields or channel-contract drift cause renderer or
  verifier failure.
- Docs, changelog, roadmap, and version surfaces align on `0.36.0` and the
  release-provenance story.