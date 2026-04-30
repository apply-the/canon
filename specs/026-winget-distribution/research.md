# Research: Winget Distribution And Roadmap Refocus

## Decision 1: Reuse the existing Windows zip as a winget archive installer

**Decision**: Treat the current `canon-<VERSION>-windows-x86_64.zip` asset as the
installer artifact for `winget` and describe it using installer schema v1.12.0
with `InstallerType: zip`, `NestedInstallerType: portable`, and a nested
`canon.exe` entry with a command alias.

**Rationale**: The existing packaging flow already produces a versioned Windows
zip that contains `canon.exe`. The winget installer schema explicitly supports
archive installers with nested portable executables, so Canon can add a Windows
package-manager channel without inventing a second Windows binary format.

**Alternatives considered**:

- Generate a second Windows installer format just for `winget`: rejected
  because it widens packaging scope and introduces parallel Windows artifacts
  without proven user value.
- Rely only on `wingetcreate` interactive authoring: rejected because it does
  not create a durable repository-owned artifact contract for release review.

## Decision 2: Generate a multi-file manifest bundle, not a singleton manifest

**Decision**: Emit the recommended multi-file winget manifest bundle with
separate `version`, `defaultLocale`, and `installer` files.

**Rationale**: The multi-file layout is the recommended shape for richer
metadata, keeps installer concerns separate from package metadata, and makes it
easier to review generated publication artifacts in-repo and in release
artifacts.

**Alternatives considered**:

- Use a singleton manifest: rejected because it compresses concerns into one
  file and provides less room for durable metadata and validation.
- Defer machine-readable manifests and document a manual maintainer checklist:
  rejected because the whole point of the slice is to eliminate hand-derived
  publication steps.

## Decision 3: Keep publication artifact generation repo-owned, but keep final winget submission human-driven

**Decision**: Canon will generate the manifest bundle and release-ready
publication inputs in-repo, but it will not automate submission to the
`winget-pkgs` repository in this slice.

**Rationale**: Repository-owned manifest generation gives maintainers a durable,
reviewable output while staying within Canon's bounded release posture. Final
submission still requires repository, credential, and external workflow choices
that are better left explicit rather than hidden behind automation.

**Alternatives considered**:

- Open or merge `winget-pkgs` pull requests automatically: rejected because it
  introduces external-state automation, token management, and failure handling
  that are unnecessary for the first slice.
- Leave publication entirely manual with no generated artifact: rejected
  because it fails the artifact-first requirement and preserves checksum drift
  risk.

## Decision 4: Remove Protocol Interoperability from the active roadmap rather than renaming it

**Decision**: Remove the Protocol Interoperability / MCP feature from
`ROADMAP.md` instead of rewording or parking it as near-term work.

**Rationale**: The roadmap should advertise concrete next-value slices. No
named MCP server or consumer target currently unlocks immediate value for Canon,
while Windows distribution and authoring/evidence quality have clearer product
traction.

**Alternatives considered**:

- Keep the section but mark it deferred: rejected because it still spends
  roadmap attention on a speculative direction.
- Replace it with a generic interoperability note: rejected because that would
  remain too abstract to guide next delivery work.