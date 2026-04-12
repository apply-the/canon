# Contract: Installation and Runtime Compatibility

## Purpose

Define what a user must be able to do after downloading a Canon release and
what the skill-layer runtime checks must report when Canon is absent or
incompatible.

## Installation Contract

A supported installation flow must allow the user to:

1. choose the correct archive for their platform and architecture
2. download the archive and the checksum manifest
3. verify the selected archive checksum
4. extract the binary
5. place the binary in a PATH directory
6. run `canon --version`
7. run `canon init --output json` in a fresh Git repository

Normal daily use must not require:

- `cargo install`
- `cargo run`
- cloning the Canon source repository

## Platform Guidance Requirements

- macOS guidance must show a user or system PATH directory example
- Linux guidance must show a user or system PATH directory example
- Windows guidance must show a documented `%USERPROFILE%`-based or equivalent
  PATH directory example
- every platform flow must explain how to confirm that the installed binary is
  the one resolved on PATH

## Runtime Compatibility Contract

The shared Canon skill preflight helpers must:

- detect when `canon` is missing from PATH
- detect when the installed Canon binary is incompatible with the expected
  workspace version or command contract
- point users to release-based install or update guidance instead of Cargo
- preserve the existing `STATUS=ready` success contract when a valid binary is
  available

The shared compatibility reference must remain the source of truth for:

- expected Canon version
- install/update guidance location
- version probe and command-contract fallback

## Recovery Contract

When the installed binary is missing, incompatible, or shadowed by an older
PATH entry, the documented recovery path must tell the user to:

1. install or update the correct release artifact
2. ensure the intended binary directory appears before stale Canon locations on
   PATH
3. rerun `canon --version` and confirm the expected version

## Acceptance Checks

- README and quickstart surfaces use `canon` directly in the normal user path
- source-build guidance is isolated to contributor/development sections
- Bash and PowerShell preflight helpers emit release-based recovery guidance
- a fresh user can install, verify, and initialize Canon without Cargo

## Implementation Notes

- the shared preflight helpers remain unchanged in this increment; the
  release-based recovery message is supplied through
  `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- PATH troubleshooting must cover both stale binaries earlier on PATH and the
  not-yet-initialized `.canon/` repository case