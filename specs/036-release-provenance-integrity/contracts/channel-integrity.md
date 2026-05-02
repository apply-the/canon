# Contract: Channel Integrity

## Purpose

Define how package-manager renderers and release verification logic consume the
canonical distribution metadata without drifting from the GitHub Release bundle.

## Channel Rules

- `homebrew` may only render from the asset ids declared in its channel
  contract.
- `winget` may only render from the asset ids declared in its channel contract.
- `scoop` may only render from the asset ids declared in its channel contract.
- Each renderer MUST confirm its expected generated artifact names are declared
  in its channel contract before writing output.

## Fail-Closed Requirements

- Rendering MUST fail when the channel contract is missing.
- Rendering MUST fail when a required asset id is absent from the channel
  contract.
- Rendering MUST fail when an expected generated artifact name is absent from
  the channel contract.
- Release verification MUST fail when top-level provenance fields are missing,
  when channel contracts contradict the asset inventory, or when the generated
  artifact set diverges from the declared contract.

## Verification Responsibilities

- `write-distribution-metadata.sh` owns producing a valid canonical metadata
  contract.
- Renderer scripts own checking that their channel contract is complete before
  rendering output.
- `verify-release-surface.sh` owns checking that the release bundle, metadata,
  and generated package-manager artifacts remain coherent.

## Non-Goals

- This contract does not introduce a new package-manager channel.
- This contract does not add binary signing, external attestations, or remote
  repository publication automation.
- This contract does not alter Canon runtime or governed execution behavior.