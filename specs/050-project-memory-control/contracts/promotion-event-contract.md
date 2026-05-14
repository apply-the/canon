# Promotion Event Contract

## Purpose

Describe a Canon-owned project-memory or evidence promotion event in a
consumer-readable shape.

## Shape

```json
{
  "contract_version": "v1",
  "kind": "promotion_event",
  "source": "canon",
  "event_type": "project_memory_promoted",
  "run_ref": "RUN-123",
  "mode": "requirements",
  "target": "docs/project/product-context.md",
  "strategy": "managed-blocks",
  "promotion_state": "auto-if-approved",
  "lineage_ref": "docs/project/product-context.md#lineage",
  "content_digest": "sha256:..."
}
```

## Semantics

- Event ownership stays with Canon.
- `kind` is recommended in V1 for standalone serialization and logging, but it
  is not yet a required validated field.
- `target` names the repo-visible surface affected by the promotion.
- `strategy` reports how the update was applied, not how a consumer must act.