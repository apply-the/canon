# Quickstart: High-Risk Operational Programs

## Goal

Exercise the first full-depth `incident` and `migration` slices and confirm the
packets are readable, approval-aware, and publishable.

## Preconditions

- Canon CLI is installed for the workspace.
- The repository contains authored operational briefs for an incident and a
  migration.
- The run is started with explicit system context because both modes operate on
  an existing system surface.

## Incident Flow

1. Run a bounded incident packet:

```bash
canon run \
  --mode incident \
  --risk systemic-impact \
  --zone red \
  --owner incident-commander \
  --system-context existing \
  --input incident.md
```

2. Confirm the packet contains:

- `incident-frame.md`
- `hypothesis-log.md`
- `blast-radius-map.md`
- `containment-plan.md`
- `incident-decision-record.md`
- `follow-up-verification.md`

3. Confirm the run exposes explicit gate posture for containment and release
   readiness rather than implying that all operational questions are already
   resolved.

4. Inspect the incident packet and verify:

- blast radius and containment steps are visible
- evidence gaps are explicit when confidence is incomplete
- no artifact suggests autonomous action by Canon

## Migration Flow

1. Run a bounded migration packet:

```bash
canon run \
  --mode migration \
  --risk systemic-impact \
  --zone yellow \
  --owner migration-lead \
  --system-context existing \
  --input migration.md
```

2. Confirm the packet contains:

- `source-target-map.md`
- `compatibility-matrix.md`
- `sequencing-plan.md`
- `fallback-plan.md`
- `migration-verification-report.md`
- `decision-record.md`

3. Confirm the run exposes compatibility, sequencing, and fallback posture
   explicitly and blocks or awaits approval when migration safety is not
   credible.

4. Inspect the migration packet and verify:

- compatibility guarantees and temporary incompatibilities are separated
- sequencing and fallback are operationally ordered
- residual risks remain visible in the verification report

## Publish Check

1. Publish each completed run:

```bash
canon publish <RUN_ID>
```

2. Confirm the published packets under `docs/incidents/<RUN_ID>/` and
   `docs/migrations/<RUN_ID>/` contain all expected artifacts and remain
   readable without internal runtime manifests.