# Decision Log: Adaptive Governance Semantics

## Purpose

Record the Canon-owned contract decisions that govern the S4 semantic surface
for adaptive governance.

## Active Decisions

### D-001 Required posture baseline stays unchanged

- Status: accepted
- Date: 2026-05-16
- Decision: `authority-governance-v1` remains the required S3 posture baseline
  for downstream S4 consumers.
- Rationale: preserving the required baseline keeps existing consumers usable
  and prevents S4 adoption from depending on a second required machine
  contract.
- Consequences: missing required posture metadata remains a fail-closed
  condition; optional adaptive metadata cannot repair a missing baseline.
- Related artifacts: `spec.md`, `research.md`, `contracts/`

### D-002 Companion semantics use a separate contract line

- Status: accepted
- Date: 2026-05-16
- Decision: Canon emits S4 machine-readable adaptive semantics, when present,
  under `adaptive-governance-v1` rather than overloading
  `authority-governance-v1`.
- Rationale: a separate contract line preserves additive evolution and avoids
  silent meaning drift in the required posture contract.
- Consequences: incompatible semantic changes require a new contract line;
  older consumers may continue to rely on the required baseline while ignoring
  unsupported optional companion metadata.
- Related artifacts: `spec.md`, `research.md`,
  `contracts/adaptive-governance-v1-contract.md`

### D-003 Adaptive semantics remain semantic only

- Status: accepted
- Date: 2026-05-16
- Decision: Canon defines governance-state and rollout-profile meaning, but it
  does not assign runtime confidence, trust, councils, reviewers, routes,
  overrides, or stop transitions.
- Rationale: the roadmap and paired Boundline spec require Canon to remain the
  semantic authority while downstream runtimes keep operational control.
- Consequences: any proposal that introduces runtime directives into Canon S4
  companion metadata is out of scope for this slice.
- Related artifacts: `spec.md`, `research.md`,
  `contracts/adaptive-governance-adapter-projection.md`

### D-004 Companion semantics reuse existing publication surfaces

- Status: accepted
- Date: 2026-05-16
- Decision: Canon publishes optional adaptive semantics through existing
  governed packet metadata and governance-adapter projection surfaces instead
  of introducing a second publication workflow.
- Rationale: the existing packet metadata and adapter surfaces are already the
  stable downstream integration boundary.
- Consequences: publication, service, and adapter docs must remain aligned
  about required-baseline and optional-companion availability.
- Related artifacts: `research.md`, `plan.md`,
  `docs/integration/governance-adapter.md`

### D-005 The first companion envelope stays minimal

- Status: accepted
- Date: 2026-05-16
- Decision: the first `adaptive-governance-v1` envelope keeps its required
  shape minimal, centered on `contract_line`, `governance_state`, and
  `rollout_profile`, with any explanation fields remaining additive.
- Rationale: a minimal envelope communicates the core S4 posture without
  leaking runtime directives into the semantic contract.
- Consequences: additive fields are compatible only while they preserve the
  existing meaning of the vocabulary and do not assign runtime behavior.
- Related artifacts: `research.md`, `data-model.md`,
  `contracts/adaptive-governance-v1-contract.md`

### D-006 Rollout profiles stay distinct from authority zones and councils

- Status: accepted
- Date: 2026-05-16
- Decision: Canon treats rollout profiles as governance-maturity labels,
  distinct from S3 authority zones and downstream council profiles.
- Rationale: S4 concerns progressive governance adoption; it does not replace
  S3 posture classification or downstream council assembly.
- Consequences: docs and machine-readable semantics must avoid reusing council
  or zone vocabulary for rollout profiles.
- Related artifacts: `research.md`, `spec.md`,
  `docs/governance-semantics-and-authority-zones.md`

## Pending Follow-Up

- Record independent cross-repo review findings in `validation-report.md`
  before final closeout.
- Extend this log if additive fields or a successor companion contract line is
  introduced.