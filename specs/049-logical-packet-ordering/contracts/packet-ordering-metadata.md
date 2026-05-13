# Contract: Packet Ordering Metadata

## Purpose

Define the Canon-owned packet metadata shape that exposes logical packet order for new packets and preserves backward-readable behavior for historical packets.

## Contract Fields

### Required for new packets

```json
{
  "primary_artifact": "01-architecture-overview.md",
  "artifact_order": [
    "01-architecture-overview.md",
    "02-architecture-decisions.md",
    "03-invariants.md"
  ]
}
```

### Optional compatibility fields

```json
{
  "publish_order": [
    "01-architecture-overview.md",
    "02-architecture-decisions.md",
    "03-invariants.md"
  ],
  "legacy_aliases": {
    "architecture-overview.md": "01-architecture-overview.md"
  }
}
```

## Contract Rules

- `primary_artifact` identifies the first reader-facing artifact a reviewer or downstream tool should open.
- `artifact_order` lists the emitted reader-facing artifacts in Canon-owned reading order.
- `primary_artifact` must be the first entry in `artifact_order`.
- `publish_order` is optional and is emitted only when Canon needs an explicit publish-facing index order beyond `artifact_order`; when present, it must preserve the same logical sequence used for published outputs and generated indexes.
- `legacy_aliases` is optional and is emitted only when Canon needs compatibility mappings for historical packet names; when present, it maps historical artifact names to their ordered equivalents without requiring historical run rewrites.

## Non-Rules

- Sidecars are not required to appear in `artifact_order` unless Canon explicitly declares them part of the packet body.
- Historical packets are not required to adopt ordered names retroactively.