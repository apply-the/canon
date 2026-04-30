# Contract: Homebrew Formula Surface

## Purpose

Define the generated Homebrew artifact and its publication semantics for Canon's
first package-manager channel.

## Artifact Identity

- **Rendered artifact**: `canon-<VERSION>-homebrew-formula.rb`
- **Tap destination**: `Formula/canon.rb`
- **Producer**: Canon release workflow or release-surface scripts
- **Consumer**: Homebrew users installing Canon from the dedicated tap

## Formula Requirements

The rendered formula MUST:

- define `class Canon < Formula`
- declare a canonical `desc`, `homepage`, `version`, and `license`
- select the correct archive URL and `sha256` for macOS and Linux on `arm64`
  and `x86_64`
- install the `canon` binary from the extracted archive without rebuilding it
- avoid Windows-specific logic in this slice
- define a `test do` block that exercises a real Canon CLI path rather than a
  version-only check

## Platform Mapping Rules

- macOS `arm64` maps to the canonical `macos-arm64` asset
- macOS `x86_64` maps to the canonical `macos-x86_64` asset
- Linux `arm64` maps to the canonical `linux-arm64` asset
- Linux `x86_64` maps to the canonical `linux-x86_64` asset
- Any missing URL or checksum for these mappings MUST block formula generation

## Publication Rules

- If tap publication is configured, automation MUST attempt to update the tap's
  `Formula/canon.rb` from the rendered artifact.
- If tap publication is not configured, the rendered artifact MUST still be
  retained as a durable release output for manual application.
- Failed tap publication MUST NOT discard the rendered artifact.

## Validation Rules

- The rendered formula's URLs and `sha256` values MUST match the release
  metadata artifact exactly.
- The formula MUST remain installable from a local file for smoke validation.
- The formula output MUST preserve the direct-download fallback in the user
  documentation instead of replacing it.