# Decision Log: Release Provenance And Channel Integrity

## D-001: Extend the existing distribution metadata instead of inventing a second provenance manifest

- **Status**: Accepted
- **Rationale**: Canon already has one repository-owned metadata surface for
  release artifacts. Extending it avoids duplicate inventories and keeps one
  canonical contract.

## D-002: Make package-manager channels explicit top-level contracts

- **Status**: Accepted
- **Rationale**: Channel expectations must be auditable and machine-readable so
  renderers and verifiers can fail closed on drift.

## D-003: Keep GitHub Releases explicit as the single source of truth

- **Status**: Accepted
- **Rationale**: The feature is about provenance clarity. The canonical release
  host, checksums, and release notes cannot stay implicit.

## D-004: Strengthen the existing verifier rather than adding a second release gate

- **Status**: Accepted
- **Rationale**: `scripts/release/verify-release-surface.sh` already owns the
  release bundle gate and should validate provenance and channel integrity from
  the same contract.

## D-005: Use focused Rust release tests plus direct script execution as the validation split

- **Status**: Accepted
- **Rationale**: This preserves the requested generation versus validation
  separation while keeping the feature bounded to release automation instead of
  adding a new runtime command surface.

## User Story 1 Decisions

- Add explicit source-of-truth fields to the canonical metadata so release
  provenance can be inspected without reverse-engineering the scripts.
- Keep asset-level channel membership and top-level channel contracts aligned in
  the same metadata document.

## User Story 2 Decisions

- Make each renderer validate its own channel contract before rendering output.
- Keep exact generated artifact names in the canonical contract so renderers
  and the verifier can reject drift without hand-maintained fallback logic.
- Keep package-manager outputs derived from existing archives and checksums
  rather than adding new build products.

## User Story 3 Decisions

- Align docs, roadmap, changelog, version surfaces, and validation artifacts on
  the same provenance language delivered by the metadata contract.
- Remove roadmap language that still implies this release-hardening slice is
  pending once the feature lands.