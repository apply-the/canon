# Research: Scoop Distribution Follow-On

## Decision 1: Reuse the existing Windows zip as the Scoop install payload

**Decision**: Treat the current `canon-<VERSION>-windows-x86_64.zip` asset as
the Scoop installation payload and describe it through a manifest that points to
the canonical GitHub Release URL plus SHA256 checksum.

**Rationale**: The existing packaging flow already produces the Windows zip with
`canon.exe` at the archive root. Scoop can install portable tools directly from
zip archives, so Canon can add the follow-on Windows channel without creating a
second installer format or build path.

**Alternatives considered**:

- Generate a second Windows installer format just for Scoop: rejected because
  it widens packaging scope and creates a parallel release artifact.
- Bypass release metadata and hand-author Scoop URLs and hashes: rejected
  because it violates the source-of-truth invariant and increases checksum
  drift risk.

## Decision 2: Emit one versioned Scoop manifest artifact per release

**Decision**: Generate a single Scoop manifest JSON artifact for each release
as `canon-<VERSION>-scoop-manifest.json`, even though the bucket submission path
will rename or copy it to `canon.json`.

**Rationale**: A versioned release artifact is easier to publish, review, and
trace back to a specific GitHub Release than an unversioned file name. The
bucket-facing rename can happen during the manual submission step without
changing the generated manifest content.

**Alternatives considered**:

- Publish only `canon.json`: rejected because the release artifact would lose
  version traceability and be easier to confuse across releases.
- Generate a directory of multiple Scoop files: rejected because Scoop needs a
  single manifest for this bounded portable-zip case.

## Decision 3: Keep Scoop bucket submission manual in the first slice

**Decision**: Canon will generate and verify the Scoop manifest in-repo and as
part of the release bundle, but final submission to the Scoop bucket will remain
manual in this slice.

**Rationale**: Repository-owned generation and verification create the durable
artifact contract the feature needs. Automatic bucket mutation would add
external repository ownership, token management, and submission failure modes
that are unnecessary for the first Scoop slice.

**Alternatives considered**:

- Automate bucket synchronization from CI: rejected because it introduces
  external-state automation before the manifest contract and maintainer review
  steps are proven stable.
- Leave submission entirely manual with no generated artifact: rejected because
  it preserves the exact URL and checksum drift this slice is meant to remove.

## Decision 4: Extend the shared Windows distribution metadata instead of
creating a Scoop-only metadata path

**Decision**: Extend the existing Windows distribution metadata entry so the
same canonical asset advertises both `winget` and `scoop` channel membership.

**Rationale**: The existing metadata already owns the release version,
download URL, filename, and checksum. Reusing that shared record keeps GitHub
Releases as the single source of truth and avoids duplicating Windows artifact
facts into a second metadata shape.

**Alternatives considered**:

- Create a Scoop-specific metadata file beside the current distribution
  metadata: rejected because it duplicates canonical release facts.
- Hardcode Scoop asset details only inside the renderer: rejected because the
  verifier would lose a stable shared contract.

## Decision 5: Document Scoop as a supported Windows path while preserving
`winget` and archive fallback guidance

**Decision**: Add Scoop installation and maintainer guidance without replacing
the existing `winget` and direct-download Windows paths.

**Rationale**: The user value is additive reach and convenience, not replacing
an already-delivered Windows channel. Documentation should make the new path
discoverable while keeping the fallback honest for bucket-propagation delays.

**Alternatives considered**:

- Replace `winget` with Scoop as the primary Windows path: rejected because the
  delivered `winget` channel remains valid and already documented.
- Hide the archive fallback once Scoop is added: rejected because package-index
  propagation can lag a release and users still need a direct install escape
  hatch.