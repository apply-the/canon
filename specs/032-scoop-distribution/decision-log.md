# Decision Log: Scoop Distribution Follow-On

## D-001: Choose Scoop as the next concrete distribution follow-on

- **Status**: Accepted
- **Rationale**: Scoop extends the existing Windows release surface into another
  familiar package-manager path without leaving the bounded packaging and
  documentation domain.

## D-002: Reuse the existing Windows zip instead of adding a second installer

- **Status**: Accepted
- **Rationale**: The canonical Windows archive already contains `canon.exe` and
  carries the versioned URL and checksum surface that Scoop needs.

## D-003: Keep final Scoop bucket submission manual in the first slice

- **Status**: Accepted
- **Rationale**: Repository-owned manifest generation and verification are
  enough to create a durable artifact and review surface without adding
  external bucket automation.

## D-004: Extend the shared distribution metadata instead of creating a
Scoop-only metadata shape

- **Status**: Accepted
- **Rationale**: The current distribution metadata already owns the canonical
  Windows filename, URL, and checksum. Reusing it preserves GitHub Releases as
  the single source of truth.

## User Story 1 Decisions

- Publish a versioned Scoop manifest artifact alongside the canonical Windows
  zip rather than replacing the existing release asset.
- Derive the Scoop manifest URL and checksum strictly from repository-owned
  distribution metadata so the renderer and verifier share the same contract.

## User Story 2 Decisions

- Document Scoop install and upgrade guidance alongside the existing Windows
  paths instead of replacing `winget`.
- Keep the direct Windows zip fallback visible anywhere Scoop guidance appears
  so bucket-propagation delays do not block installation.

## User Story 3 Decisions

- Remove already delivered feature blocks from the active remaining-candidates
  section in `ROADMAP.md`.
- Update the delivered-distribution narrative to include the Scoop follow-on
  once this slice lands.