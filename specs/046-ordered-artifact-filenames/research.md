# Research: Ordered Artifact Filenames

## Decision 1: Prefix format

- **Decision**: `NN-` format (e.g., `01-architecture-overview.md`).
- **Rationale**: Consistent, sortable, and human-readable.
- **Alternatives rejected**: letter prefixes (`a-`, `b-`), no prefix with a manifest-only ordering (not visible in file browsers).

## Decision 2: Ordering source

- **Decision**: Use `artifact_families` from `ModeProfile` as the canonical ordering.
- **Rationale**: Already maintained per mode, already reflects intended reading order.
- **Alternatives rejected**: alphabetical (loses semantic order), hardcoded ordering table (duplicates information).

## Decision 3: Impact on existing manifests

- **Decision**: Update `view-manifest.json` and `packet-metadata.json` to reference prefixed filenames.
- **Rationale**: Machine consumers must resolve files by name; stale references break tooling.
- **Alternatives rejected**: keep unprefixed names in manifests only (creates a split between filesystem and metadata).
