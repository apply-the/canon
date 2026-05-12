# Decision Log: Ordered Artifact Filenames

## D-001 Two-digit prefix width

- **Date**: 2026-05-12
- **Decision**: Use two-digit zero-padded prefix (`01-` through `99-`).
- **Rationale**: Current maximum artifact count per mode is 15 (architecture). Two digits are sufficient, more readable than three, and leave room for growth to 99 artifacts per mode.

## D-002 Ordering source

- **Date**: 2026-05-12
- **Decision**: Derive artifact ordering from `artifact_families` in `ModeProfile`.
- **Rationale**: The mode profile already encodes the intended packet structure and reading order. Using it as the single source of truth avoids a separate ordering table.

## D-003 Contiguous numbering

- **Date**: 2026-05-12
- **Decision**: Use contiguous numbering when optional artifacts are omitted.
- **Rationale**: Gaps in numbering confuse readers and suggest missing files. Contiguous numbering makes every present artifact feel intentional.

## D-004 Mermaid sidecar sharing

- **Date**: 2026-05-12
- **Decision**: Mermaid `.mmd` sidecars share the same numeric prefix as their companion markdown artifact.
- **Rationale**: Keeps related files adjacent in directory listings and makes the pairing obvious.
