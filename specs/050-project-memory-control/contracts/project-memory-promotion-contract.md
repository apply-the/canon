# Project Memory Promotion Contract

- **Owner**: Canon
- **Current Contract Line**: V1
- **Stable Owner-Side Path**: `tech-docs/integration/project-memory-promotion-contract.md`
- **Primary Consumer**: Boundline

## Purpose

Define the Canon-owned producer semantics for repo-visible project memory and
evidence publication.

## Standalone JSON Payload Example

```json
{
  "contract_version": "v1",
  "kind": "project_memory_promotion",
  "producer": "canon",
  "source_ref": "canon-run:RUN-123",
  "source_artifacts": [".canon/runs/RUN-123/01-architecture-overview.md"],
  "promotion_state": "auto-if-approved",
  "promoted_at": "2026-05-13T10:00:00Z",
  "content_digest": "sha256:..."
}
```

The `kind` field is recommended in V1 when this payload is embedded or logged
outside the contract brief. It does not change ownership semantics or encode any
Boundline orchestration policy. Keep it optional until producer and consumer
validation is added in a compatible later contract minor.

## Default Targets

- `tech-docs/project/` for stable or pending project-memory surfaces
- `tech-docs/evidence/` for readable evidence summaries and mixed-producer evidence
  blocks

## Managed Block Format

```md
<!-- project-memory:managed:start producer="canon|boundline" source_ref="..." contract_version="v1" -->
...
<!-- project-memory:managed:end -->
```

Feature-local contracts elaborate this stable marker shape with supporting
entity contracts and examples, but the stable owner-side document remains the
normative source for ownership, compatibility, and publish-policy semantics.

If a feature-local brief and the stable owner-side path drift, the stable path
wins and the feature-local brief must be realigned in the same change.

## Required V1 Lineage Fields

- `contract_version`
- `producer`
- `source_ref`
- `source_artifacts`
- `promotion_state`
- `promoted_at`
- `content_digest`

## Optional V1 Lineage Fields

- `mode`
- `stage`
- `owner`
- `risk`
- `zone`
- `approval_state`
- `packet_readiness`
- `promotion_profile`

## Promotion State Expectations

- Stable managed-block updates are reserved for completed and policy-eligible
  knowledge.
- Pending, blocked, or conflicting knowledge uses pending indexes, proposal
  files, or evidence-only outputs rather than overwriting stable memory.
- Mixed-producer evidence authorship is allowed through the shared block
  marker, but Canon remains owner of Canon-produced promotion semantics.
- Boundline-managed evidence blocks may contribute attribution and readable
  evidence text, but they do not define Canon `promotion_state`,
  `approval_state`, `packet_readiness`, or target-routing rules.

## Compatibility Policy

- Additive V1 changes are backward-compatible.
- Removing or renaming required fields is breaking.
- Breaking changes require a new major contract line.
- Canon supports the previous minor published contract revision for one full
  minor release cycle.