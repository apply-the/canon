# Governed Stage Ref Contract

## Purpose

Provide a Canon-owned consumer-facing summary of a governed stage outcome.

## Shape

```json
{
  "contract_version": "v1",
  "kind": "governed_stage_ref",
  "source": "canon",
  "run_ref": "RUN-123",
  "mode": "architecture",
  "state": "Completed",
  "approval_state": "approved",
  "packet_readiness": "ready",
  "primary_artifact": "01-architecture-overview.md",
  "artifact_order": ["01-architecture-overview.md", "02-decisions.md"],
  "promotion_refs": [],
  "risk": "bounded-impact",
  "zone": "yellow"
}
```

## Semantics

- Produced by Canon, consumed by Boundline.
- Describes producer facts only.
- `kind` is recommended in V1 for standalone serialization and logging, but it
  is not yet a required validated field.
- Does not encode Boundline stop, retry, or orchestration policy.