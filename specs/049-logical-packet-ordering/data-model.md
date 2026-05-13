# Data Model: Logical Packet Ordering

## Entities

### PacketOrderingRegistry

Canon-owned registry that defines the intended reader-facing artifact sequence for each packet-emitting mode.

| Field | Type | Description |
|-------|------|-------------|
| mode | String | Canon mode identifier |
| primary_artifact_slug | String | Slug for the packet artifact that must become `01-*` |
| ordered_artifacts | Vec<OrderedArtifactRule> | Canon-owned reading sequence for reader-facing artifacts |
| sidecars | Vec<String> | Support files that remain outside the packet-body sequence |

**Validation rules**:
- Each packet-emitting mode has exactly one registry entry.
- `primary_artifact_slug` must appear in `ordered_artifacts`.
- Ordered artifact positions must remain unique within the mode.

### OrderedArtifactRule

Reader-facing artifact rule used to determine packet ordering and contiguous renumbering.

| Field | Type | Description |
|-------|------|-------------|
| slug | String | Canonical artifact slug without numeric prefix |
| required | Boolean | Whether the artifact is mandatory for the mode |
| optional_reason | String? | Why the artifact may be omitted |
| body_sequence | Integer | Canon-owned logical position before contiguous emission |

**Validation rules**:
- Required artifacts must always appear in emitted packets for supported scenarios.
- Optional artifacts may be omitted, but emitted numbering must remain contiguous.

### PacketOrderingMetadata

Ordering metadata emitted with a packet to preserve reading semantics for tools and summaries.

| Field | Type | Description |
|-------|------|-------------|
| primary_artifact | String | Prefixed filename to open first |
| artifact_order | Vec<String> | Ordered list of reader-facing emitted artifact names |
| publish_order | Vec<String>? | Optional publish-facing order when Canon needs an explicit external index contract |
| legacy_aliases | Map<String, String>? | Optional legacy-name to ordered-name compatibility map |

**Validation rules**:
- `primary_artifact` must be the first item in `artifact_order`.
- Every `artifact_order` entry must correspond to a real emitted artifact.
- `legacy_aliases` entries must not redefine current emitted names.

### LegacyArtifactAliasMap

Compatibility record that helps Canon resolve historical packet names for existing governed runs.

| Field | Type | Description |
|-------|------|-------------|
| aliases | Map<String, String> | Legacy artifact filename to ordered artifact filename mappings |

**Validation rules**:
- Alias mappings apply only to historical packets or compatibility surfaces.
- Alias mappings must not require historical run rewrites.
- Alias mappings must not redefine current emitted ordered names.

## Relationships

```text
PacketOrderingRegistry
  ├── defines OrderedArtifactRule[*]
  ├── identifies one primary artifact
  └── informs PacketOrderingMetadata

PacketOrderingMetadata
  └── may include LegacyArtifactAliasMap[*]
```

## State and Transition Notes

- New packet emission resolves a mode through `PacketOrderingRegistry`, filters any optional artifacts that are unsupported in the current packet, then assigns contiguous numeric filenames for the emitted `artifact_order`.
- Publish, status, and inspect consume `PacketOrderingMetadata` rather than rebuilding order heuristically from filenames alone.
- Legacy packets continue using historical filenames, but Canon may project them through `LegacyArtifactAliasMap.aliases` or equivalent compatibility behavior.