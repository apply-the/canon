# Semantic Artifact Contract

- **Owner**: Canon
- **Status**: Draft, producer brief
- **Planned Stable Path**: `tech-docs/integration/semantic-artifact-contract.md`
- **Current Draft Contract Line**: V1

## Purpose

Define the Canon-owned producer semantics that allow repo-visible Canon
artifacts to participate in downstream semantic retrieval without making Canon
responsible for retrieval runtime behavior, ranking, or local index ownership.

## Authority Rule

- This brief is the feature-local source for Canon semantic artifact semantics
  until the stable integration path is promoted.
- The existing project-memory promotion contract and artifact-indexing contract
  remain authoritative for publication, artifact classes, metadata carriers,
  and compatibility unless this brief realigns them explicitly.
- Consumers may depend on Canon semantic metadata but may not redefine Canon's
  producer semantics.

## Relationship To Existing Contracts

This semantic contract extends, and does not replace, these Canon surfaces:

- `tech-docs/integration/project-memory-promotion-contract.md`
- `specs/050-project-memory-control/contracts/project-memory-promotion-contract.md`
- `specs/051-artifact-indexing-contract/contracts/artifact-indexing-contract.md`

The semantic descriptor travels through the same documented metadata carrier as
Canon artifact indexing metadata. Canon does not introduce a second discovery
path in this slice.

## Supported Semantic Eligibility Surface

The draft V1 semantic contract applies to these artifact classes:

- `managed-surface`: semantically eligible when Canon-produced content is
  present and a semantic provenance boundary is available
- `proposal-artifact`: semantically eligible at the published proposal surface
  boundary
- `evidence-bundle`: semantically eligible when Canon can point consumers to a
  stable evidence-facing provenance boundary
- `index-surface`: semantically excluded from similarity selection and reserved
  for discoverability-only or visibility-only use

If an artifact class is not listed here, it is outside the draft semantic
surface and consumers MUST NOT infer semantic eligibility.

## Semantic Descriptor

Canon V1 semantic metadata is expressed as a typed `semantic_descriptor`
payload with these required fields:

- `semantic_contract_line`
- `semantic_eligibility`
- `semantic_provenance_boundary`
- `semantic_provenance_ref`

Additive optional fields may include:

- `semantic_labels`
- `semantic_exclusion_reason`

Example shape:

```json
{
  "semantic_descriptor": {
    "semantic_contract_line": "v1",
    "semantic_eligibility": "eligible",
    "semantic_provenance_boundary": "managed_block",
    "semantic_provenance_ref": "tech-docs/evidence/example.md#project-memory-managed:block-3",
    "semantic_labels": ["review", "architecture"]
  }
}
```

## Metadata Carrier And Discovery

The semantic descriptor travels through the same Canon-owned metadata carrier
used by the artifact-indexing contract. This slice does not introduce a second
semantic discovery path.

- `managed-surface` uses the managed-surface envelope plus the adjacent
  `<surface>.packet-metadata.json` sidecar that already carries Canon-owned
  publication metadata.
- `proposal-artifact` and `evidence-bundle` use the canonical
  `packet-metadata.json` sidecar for packet roots or the adjacent published
  surface sidecar documented by the existing promotion and indexing contracts.
- `index-surface` remains on the same metadata carrier family but is
  semantically excluded.

Consumers must read publication routing and artifact carrier semantics from the
existing promotion and indexing contracts, then read `semantic_descriptor`
from that same Canon-owned metadata surface.

## Semantic Provenance Boundaries

Canon V1 recognizes these provenance boundary values:

- `surface`: the entire published surface is the producer semantic boundary
- `managed_block`: a Canon-managed block inside a readable document is the
  producer semantic boundary
- `section`: a named Canon-authored section inside a readable artifact is the
  producer semantic boundary

Consumers may derive smaller local fragments from these boundaries, but Canon
only owns the boundary itself and the provenance reference attached to it.

## Mixed-Producer Rule

When a readable surface contains Canon and non-Canon content:

- Canon semantic metadata applies only to Canon-produced content
- Canon does not claim ownership over non-Canon fragments or consumer-derived
  local fragment identifiers
- consumers must not generalize Canon eligibility from a mixed document to
  unrelated non-Canon content on the same page

## Compatibility Policy

- Additive optional semantic fields are backward-compatible.
- Removing or renaming a required semantic field is breaking.
- Changing the meaning of an existing eligibility state or provenance boundary
  is breaking.
- Breaking changes require a new major semantic contract line.

## Unsupported And Rejection Conditions

Consumers must treat a Canon candidate as unsupported for semantic retrieval
when any of the following is true:

- the artifact class is explicitly semantically excluded, including
  `index-surface`
- the `semantic_contract_line` is not supported by the consumer
- a required semantic descriptor field is missing or empty
- the `semantic_provenance_ref` does not resolve to a Canon-owned surface,
  managed block, or section boundary
- semantic metadata is being inferred for non-Canon content inside a
  mixed-producer readable surface

When Canon emits an excluded semantic surface, it may include
`semantic_exclusion_reason` to make the rejection explicit without changing the
meaning of the required fields.

## Explicit Exclusions

This contract does not define:

- consumer fragment identifiers
- embedding model selection
- vector store ownership
- hybrid ranking or reranking policy
- retrieval depth or fallback policy
- remote-provider or transmission policy
- Boundline stop semantics or delivery adjudication
