# Contract: System Assessment Publish Surface

## Purpose

Define the repository-visible publish destination for completed
`system-assessment` packets.

## Publish Root

Completed or publishable `system-assessment` runs MUST publish under:

```text
docs/architecture/assessments/<RUN_ID>/
```

## Publish Rules

- The publish destination MUST remain separate from
  `docs/architecture/decisions/<RUN_ID>/` so as-is assessment is not confused
  with decision packets.
- Publishing MUST reuse the existing Canon publish flow and MUST NOT require a
  new persistence schema.
- The full emitted artifact bundle for `system-assessment` MUST be copied to
  the publish root.
- Existing publish roots for other modes MUST remain unchanged.