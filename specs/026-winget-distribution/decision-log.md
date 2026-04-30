# Decision Log: Winget Distribution And Roadmap Refocus

## D-001: Choose `winget` as the next concrete distribution slice

- **Status**: Accepted
- **Rationale**: Windows distribution through `winget` has immediate user value,
  fits the existing GitHub Releases packaging posture, and is more concrete
  than speculative protocol work.

## D-002: Reuse the existing Windows zip as an archive installer with a nested portable executable

- **Status**: Accepted
- **Rationale**: The current release artifact already contains `canon.exe`, and
  the winget installer schema supports `zip` installers with
  `NestedInstallerType: portable` and nested file aliases.

## D-003: Keep final `winget-pkgs` submission manual in the first slice

- **Status**: Accepted
- **Rationale**: Repository-owned manifest generation is enough to create a
  durable artifact and release proof surface without pulling external-state
  automation into the first packaging increment.

## D-004: Remove Protocol Interoperability / MCP from the active roadmap

- **Status**: Accepted
- **Rationale**: No concrete MCP consumer or server target currently unlocks
  enough value to justify roadmap priority over packaging and authoring quality.

## User Story 1 Decisions

- Publish the generated `winget` manifest bundle alongside the canonical
  Windows zip instead of replacing the existing GitHub Release asset.
- Derive installer URLs and checksums from repository-owned distribution
  metadata so release workflow wiring does not duplicate Windows artifact facts.

## User Story 2 Decisions

- Present `winget` as the primary Windows install and upgrade path in user
  documentation.
- Keep the direct `windows-x86_64.zip` fallback visible anywhere `winget`
  guidance appears so Windows users are not blocked by package-index latency.

## User Story 3 Decisions

- Remove Protocol Interoperability / MCP from the active roadmap instead of
  keeping it as a near-term placeholder.
- Keep the roadmap focused on concrete packaging and authoring/evidence work
  until a named interoperability target produces immediate delivery value.