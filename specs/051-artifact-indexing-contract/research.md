# Research: Artifact Indexing Contract

## Current Stable Contract Surface

- `docs/integration/project-memory-promotion-contract.md` is already the
  normative Canon-owned stable contract for project-memory publication.
- The stable contract already defines ownership, compatibility, target classes,
  managed-block markers, and required versus optional lineage fields.

## Current Code Surfaces

- `crates/canon-engine/src/domain/publish_profile.rs` already defines the V1
  lineage field sets, managed-block descriptor, and publish-profile semantics.
- `crates/canon-engine/src/orchestrator/publish.rs` already emits
  `packet-metadata.json` for packet roots and `<surface>.packet-metadata.json`
  for project-memory profile projections.
- `runtime_packet_metadata` already treats packet sidecars as the canonical
  packet-order source when present.

## Implementation Direction

- Extend the stable Canon contract instead of creating a second normative
  document.
- Make the supported V1 artifact classes explicit.
- Make metadata carrier and discovery rules explicit for each supported class.
- Exclude `safety-net packets` from Canon V1 vocabulary because the current
  producer does not publish that class.