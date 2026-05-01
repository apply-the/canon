# Contract: Scoop Manifest Artifact

## Purpose

Define the generated Scoop manifest artifact shape derived from the canonical
Windows release asset.

## Artifact Names

- Generated release artifact: `canon-<VERSION>-scoop-manifest.json`
- Bucket submission path after manual copy or rename: `bucket/canon.json`

## Required Manifest Fields

The generated JSON MUST include:

- `version`
- `description`
- `homepage`
- `license`
- `architecture.64bit.url`
- `architecture.64bit.hash`
- `bin`

## Canon-Specific Expectations

- `version` MUST equal the Canon workspace version for the release.
- `description` MUST describe Canon as a governed local-first CLI for
  AI-assisted software engineering.
- `homepage` MUST point to `https://github.com/apply-the/canon`.
- `license` MUST remain `MIT` unless the repository license changes in a future
  release.
- `architecture.64bit.url` MUST point to the canonical GitHub Release URL for
  `canon-<VERSION>-windows-x86_64.zip`.
- `architecture.64bit.hash` MUST match the SHA256 recorded in
  `canon-<VERSION>-SHA256SUMS.txt`.
- `bin` MUST expose `canon.exe` on PATH.

## Validation Rules

- Manifest generation MUST fail if the Windows asset URL or checksum is missing
  from distribution metadata.
- Release-surface verification MUST confirm the generated manifest still points
  to the release asset name and checksum for the same version.
- The generated manifest MUST remain reviewable JSON with deterministic field
  content for a given versioned release bundle.