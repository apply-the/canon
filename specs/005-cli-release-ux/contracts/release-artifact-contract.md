# Contract: Release Artifact Surface

## Purpose

Define the exact public artifact set and archive rules for an install-first
Canon release.

## Public Artifact Matrix

| Public Artifact | Internal Build Target | Format | Binary Name |
| --- | --- | --- | --- |
| `canon-<VERSION>-macos-arm64.tar.gz` | `aarch64-apple-darwin` | `tar.gz` | `canon` |
| `canon-<VERSION>-macos-x86_64.tar.gz` | `x86_64-apple-darwin` | `tar.gz` | `canon` |
| `canon-<VERSION>-linux-arm64.tar.gz` | `aarch64-unknown-linux-gnu` | `tar.gz` | `canon` |
| `canon-<VERSION>-linux-x86_64.tar.gz` | `x86_64-unknown-linux-gnu` | `tar.gz` | `canon` |
| `canon-<VERSION>-windows-x86_64.zip` | `x86_64-pc-windows-msvc` | `zip` | `canon.exe` |
| `canon-<VERSION>-SHA256SUMS.txt` | N/A | text | N/A |

## Naming Rules

- every archive name must include Canon version, operating system, and
  architecture
- public names use user-facing os and architecture tokens, not raw target
  triples
- the checksum manifest must be versioned with the release

## Archive Content Rules

- Unix archives contain exactly one executable named `canon`
- the Windows archive contains exactly one executable named `canon.exe`
- archive roots remain flat and do not require users to drill through nested
  directories to reach the binary
- no archive may depend on Cargo or repository-local build products at install
  time

## Publication Rules

- a release is incomplete until all five platform archives and the checksum
  manifest are attached to the public release surface
- release notes must accompany the artifact set and reference the same version
  string as the archives
- partial artifact sets are not publishable under this contract

## Integrity Rules

- the checksum manifest must contain one entry for every published archive
- each line uses standard SHA256 formatting:
  `<sha256>  <filename>`
- checksum validation must complete before release publication

## Acceptance Checks

- all required artifacts exist once and only once
- every archive opens successfully and exposes the expected binary name
- the checksum manifest covers every archive exactly once
- public release notes reference the same version and matrix

## Implementation Notes

- release notes are rendered from `.github/release-notes-template.md` before
  release verification runs
- local or CI dry runs may generate internal `*.version.txt` sidecars for
  parity checks, but only the archives plus `canon-<VERSION>-SHA256SUMS.txt`
  are public release assets