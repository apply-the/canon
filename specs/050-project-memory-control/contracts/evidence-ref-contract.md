# Evidence Ref Contract

## Purpose

Connect readable repo-visible evidence back to authoritative Canon or Boundline
runtime sources.

## Shape

```json
{
  "contract_version": "v1",
  "kind": "evidence_ref",
  "source": "boundline",
  "source_ref": "boundline-trace:TRACE-123",
  "evidence_type": "validation",
  "target": "docs/evidence/validation.md",
  "status": "failed",
  "summary": "Validation failed after retry budget"
}
```

## Semantics

- `source` may be `canon` or `boundline`.
- `kind` is recommended in V1 for standalone serialization and logging, but it
  is not yet a required validated field.
- The ref is for attribution and traceability, not ownership transfer.
- Evidence refs may appear inside shared `docs/evidence/` documents under the
  producer-neutral managed-block format.
- Boundline-authored evidence blocks may carry attribution, status, summary,
  and source references for `source = "boundline"` entries.
- Canon-only promotion fields such as `promotion_state`, `approval_state`,
  `packet_readiness`, and target-routing semantics remain outside Boundline
  evidence refs and continue to be defined only by Canon-owned contracts.