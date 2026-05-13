# Quickstart: Project Memory Promotion Policy

## Scenario 1: Promote completed architecture output to stable project memory

```bash
# Run a completed architecture session
canon run --mode architecture --risk bounded-impact

# Publish with the project-memory profile
canon publish <RUN_ID> --profile project-memory
```

**Expected result**: Canon promotes architecture artifacts to
`docs/project/architecture-map.md` using the managed-block update strategy,
emits `architecture-map.packet-metadata.json` alongside, and preserves any
human-authored content outside the Canon-managed range.

## Scenario 2: Publish pending output to audit surface only

```bash
# Run a session that ends in AwaitingApproval
canon run --mode change --risk bounded-impact
# (run ends without approval)

# Publish with project-memory profile
canon publish <RUN_ID> --profile project-memory
```

**Expected result**: Promotion policy resolves to `PendingIndex` because the
run is not approved. Canon updates `docs/project/pending-decisions.md` via
managed blocks and does not touch stable project-memory targets.

## Scenario 3: Blocked run publishes evidence only

```bash
# Run a completed review session
canon run --mode review --risk bounded-impact

canon publish <RUN_ID> --profile project-memory
```

**Expected result**: Promotion policy resolves to `EvidenceOnly`. Canon writes
to `docs/project/audit-log.md`, preserves supporting artifacts under
`docs/evidence/review/<RUN_ID>/`, and leaves stable project-memory surfaces
unchanged.

## Scenario 4: Managed-block preserves human content

Given a `docs/project/domain-language.md` with:

```markdown
# Domain Language

Human-authored introduction paragraph.

<!-- canon:managed-block:R-20260513-arch:start -->
Old Canon-generated content.
<!-- canon:managed-block:R-20260513-arch:end -->

Human-authored notes and context.
```

After `canon publish <RUN_ID> --profile project-memory` re-publishes that same
run, the file becomes:

```markdown
# Domain Language

Human-authored introduction paragraph.

<!-- canon:managed-block:R-20260513-arch:start -->
New Canon-generated content from latest run.
<!-- canon:managed-block:R-20260513-arch:end -->

Human-authored notes and context.
```

## Scenario 5: Proposal file for unsafe update

Given a target that has no Canon-managed markers and Canon cannot safely
determine a merge boundary:

```bash
canon publish <RUN_ID> --profile project-memory
```

**Expected result**: A pending incident-style promotion using `proposal-files`
emits `docs/project/open-risks.proposal.md` with the full promoted content and
lineage metadata, leaving the existing `docs/project/open-risks.md` unchanged.
The operator reviews and merges manually.

## Scenario 6: Lineage metadata inspection

After any project-memory publish, inspect the lineage sidecar:

```bash
cat docs/project/architecture-map.packet-metadata.json
```

```json
{
  "contract_version": "0.1.0",
  "source_run": "019738a4-...",
  "mode": "architecture",
  "profile": "project-memory",
  "promotion_state": "auto",
  "approval_state": "Completed",
  "readiness": "complete",
  "published_at": "2026-05-13T14:30:00Z",
  "update_strategy": "managed-blocks",
  "source_artifacts": ["architecture-overview.md", "architecture-decisions.md"]
}
```
