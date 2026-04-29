# Research: Distribution Channels Beyond GitHub Releases

## Decision 1: Generate Distribution Metadata From The Verified Release Bundle

- **Decision**: Emit a machine-readable artifact named
  `canon-<VERSION>-distribution-metadata.json` from the already verified release
  bundle instead of maintaining per-channel package metadata by hand.
- **Rationale**: The existing release workflow already verifies canonical
  archive filenames, release notes, version evidence, and checksums. Reusing
  that bundle keeps package-manager data anchored to the same audited surface.
- **Alternatives considered**:
  - Parse release notes downstream in each channel: rejected because it couples
    channel automation to human-facing prose.
  - Maintain separate Homebrew variables by hand: rejected because it creates
    checksum and URL drift risk at every release.

## Decision 2: Use A Dedicated Homebrew Tap Formula Rather Than A Second Packaging Path

- **Decision**: Publish Canon's Homebrew surface through a dedicated tap
  formula that installs the canonical release archives instead of introducing a
  new build path or relying on `homebrew/core` submission as the first slice.
- **Rationale**: Canon is a CLI distributed as versioned archives today. A
  dedicated tap keeps control of update timing in Canon's release process and
  avoids coupling the initial slice to `homebrew/core` governance or a new
  packaging pipeline.
- **Alternatives considered**:
  - `homebrew/core`: rejected for the first slice because it adds external
    review cadence and policy constraints before the metadata contract is
    proven.
  - Homebrew cask: rejected because Canon is a CLI tool with a formula-shaped
    installation surface and no GUI app bundle.

## Decision 3: Render One Formula Artifact Per Release And Make Tap Sync Optional

- **Decision**: Always render a release-specific formula artifact named
  `canon-<VERSION>-homebrew-formula.rb`, and only attempt tap synchronization
  when the destination repository and credentials are configured.
- **Rationale**: The artifact-first approach preserves a durable package output
  even when cross-repository publication is unavailable, while still supporting
  automation when repository secrets are present.
- **Alternatives considered**:
  - Require live tap publication for every release: rejected because it makes
    the release pipeline brittle when credentials or repository access are not
    configured yet.
  - Manual formula editing only: rejected because it reintroduces repetitive
    checksum editing and weakens traceability.

## Decision 4: Keep The Metadata Contract Channel-Neutral

- **Decision**: Include every canonical release asset in the metadata contract,
  including Windows assets that Homebrew does not consume in this slice.
- **Rationale**: The next channel slices should read the same artifact inventory
  rather than re-scraping release notes or introducing another manifest format.
  A channel-neutral manifest keeps Homebrew as the first consumer, not the only
  consumer.
- **Alternatives considered**:
  - Emit Homebrew-only metadata: rejected because it would force future
    `winget` and Scoop work to invent another parallel inventory contract.

## Decision 5: Keep The Manual Download Fallback Visible

- **Decision**: Update install documentation to add Homebrew while preserving
  the existing direct archive instructions as a fallback path.
- **Rationale**: GitHub Releases remain canonical, and the manual archive path
  continues to matter for users who cannot or do not want to use Homebrew.
- **Alternatives considered**:
  - Replace the archive instructions with Homebrew-only guidance: rejected
    because it hides the canonical fallback and narrows the supported install
    story unnecessarily.

## Decision 6: Use A Real CLI Smoke Test In The Formula Contract

- **Decision**: The rendered formula will define a `test do` block that checks
  a basic working Canon workflow such as `canon init` in a temporary test path,
  rather than relying only on `canon --version`.
- **Rationale**: Homebrew's guidance favors a basic functional test over a pure
  version check. `canon init` is a lightweight, deterministic smoke path that
  exercises real CLI behavior without external dependencies.
- **Alternatives considered**:
  - `canon --version` only: rejected because it is a weaker signal and does not
    prove the binary can perform basic work.
  - Full governed run invocation: rejected because it would make the formula
    test much heavier and harder to keep deterministic.

## Decision 7: Validate Scripts And Rendered Outputs With Fixture-Style Release Tests

- **Decision**: Cover the new channel surface with focused release tests that
  validate metadata generation, formula rendering, URL and checksum alignment,
  and artifact-only fallback behavior, alongside syntax checks for new shell
  scripts.
- **Rationale**: Most of this feature lives in release automation and generated
  artifacts rather than runtime Rust logic, so fixture-backed release tests and
  script checks provide better evidence than generic CLI smoke coverage alone.
- **Alternatives considered**:
  - Rely only on GitHub Actions execution: rejected because it delays feedback
    and weakens local validation.
  - Add a new Rust release crate: rejected because the feature does not justify
    new runtime architecture.