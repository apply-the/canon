# Contract: Version Visibility

## Purpose

Ensure every public surface of a Canon release exposes the same version so a
user can trust what they downloaded and what they run.

## Required Version Surfaces

The following surfaces must match semver exactly for a release candidate:

- `Cargo.toml` workspace version
- release Git tag in the form `v<VERSION>`
- release notes title and installation snippets
- public archive names
- checksum manifest file name and entries
- `canon --version` output from every built binary
- `.agents/skills/canon-shared/references/runtime-compatibility.toml`

## Validation Rules

- release publication must fail if any surface differs
- the verification step must inspect archive names and extracted binary output,
  not just source metadata
- release notes must be validated before publication, not patched after the
  assets are live

## User-Facing Consequences

- users must never see one version in the download link and a different version
  in `canon --version`
- the skill compatibility helper must not expect a version that the public
  release surface does not provide

## Acceptance Checks

- tag, archive names, release notes, and CLI output are identical on version
  content
- checksum manifest references only the matching versioned archive names
- runtime-compatibility guidance names the same expected Canon version as the
  release

## Implementation Notes

- build and packaging jobs should emit version evidence before artifacts are
  assembled for publication
- release verification must inspect rendered release notes and generated
  checksum manifests, not only source templates