# Data Model: Project Memory Promotion Policy

## Entities

### PublishProfile

Identifies the publish routing strategy for a given publish invocation.

| Field | Type | Description |
|-------|------|-------------|
| variant | enum | `Default` (existing behavior) or `ProjectMemory` (new) |

**Relationships**: Selected by operator via `--profile`; determines whether
promotion policy evaluation and lineage emission occur.

### PromotionState

Canon-owned vocabulary describing the publication outcome for a run under the
project-memory profile.

| Variant | Meaning |
|---------|---------|
| `Auto` | Promote to stable project memory unconditionally |
| `AutoIfApproved` | Promote only when approval gates have passed |
| `PendingIndex` | Update pending/audit surfaces only |
| `IndexOnly` | Record in audit surfaces without stable update |
| `EvidenceOnly` | Update evidence surfaces without stable promotion |
| `Manual` | Require explicit manual action |

**State transitions**: Resolved deterministically from `(Mode, RunState,
PromotionPolicy)`. No runtime mutation after resolution.

### UpdateStrategy

Canon-owned mechanism for modifying a project-visible document.

| Variant | Meaning |
|---------|---------|
| `ManagedBlocks` | Update Canon-managed range; preserve human content |
| `ProposalFiles` | Emit `.proposal.md` when in-place update is unsafe |
| `AppendOnlyIndex` | Append to index/audit log without rewriting |

**Relationships**: Configured per target path in the publish profile TOML.

### LineageMetadata

Durable sidecar record emitted alongside every promoted artifact.

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| contract_version | String | yes | Shared contract version (e.g. `1.0.0`) |
| source_run | UUID | yes | Originating Canon run identifier |
| mode | String | yes | Canon mode that produced the output |
| profile | String | yes | Publish profile used (`project-memory`) |
| promotion_state | PromotionState | yes | Resolved promotion outcome |
| approval_state | String | yes | `approved`, `pending`, `blocked`, `not-applicable` |
| readiness | String | yes | `stable`, `draft`, `evidence-only` |
| published_at | ISO 8601 | yes | UTC timestamp of publication |
| update_strategy | UpdateStrategy | yes | Strategy applied to the target |
| source_artifacts | Vec\<String\> | yes | Filenames of source `.canon/` artifacts |

**Validation rules**: All fields required; `contract_version` must follow
semver; `source_run` must reference an existing run ID; `published_at` must be
valid ISO 8601.

### PromotionPolicy

Per-mode configuration that maps run state to promotion state and update
strategy.

| Field | Type | Description |
|-------|------|-------------|
| mode | Mode | Canon mode this policy applies to |
| completed_state | PromotionState | Promotion state when run is Completed |
| approved_state | PromotionState | Promotion state when approved |
| blocked_state | PromotionState | Promotion state when run is Blocked |
| pending_state | PromotionState | Promotion state when AwaitingApproval |
| default_update_strategy | UpdateStrategy | Fallback strategy for this mode |
| target_overrides | Map\<Path, UpdateStrategy\> | Per-target strategy overrides |

**Relationships**: Loaded from `defaults/policies/publish-profiles.toml` with
workspace-local overrides from `.canon/config.toml`.

## Entity Relationship Summary

```text
Operator
  └── selects PublishProfile
        ├── Default → existing publish behavior (unchanged)
        └── ProjectMemory
              ├── evaluates PromotionPolicy(Mode, RunState)
              │     └── resolves PromotionState
              ├── selects UpdateStrategy(target path)
              └── emits LineageMetadata sidecar
```
