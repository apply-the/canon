# Data Model: High-Risk Operational Programs

## Overview

This feature completes two operational packet families inside Canon:
`incident` for bounded failure investigation and containment, and `migration`
for compatibility-sensitive transitions between states or systems. Both modes
share a high-risk operational core while preserving mode-specific artifacts and
gate semantics.

## Entities

### Operational Packet

- **Purpose**: Represents the durable packet emitted by a high-risk run.
- **Fields**:
  - `mode`: `incident` or `migration`
  - `summary_headline`: short packet summary used by status and inspect
  - `artifact_paths`: ordered packet artifact list
  - `gate_status`: current gate outcomes and blockers
  - `evidence_posture`: whether the packet is complete, downgraded, or blocked
- **Validation Rules**:
  - Must map to exactly one Canon run.
  - Must remain readable outside runtime manifests.
  - Must not imply readiness beyond what the gate status and evidence support.

### Incident Packet

- **Purpose**: Encodes operational investigation and containment work during an
  active or recent failure.
- **Derived Artifacts**:
  - `incident-frame.md`
  - `hypothesis-log.md`
  - `blast-radius-map.md`
  - `containment-plan.md`
  - `incident-decision-record.md`
  - `follow-up-verification.md`
- **Validation Rules**:
  - Must name the bounded incident scope and current trigger state.
  - Must include explicit blast radius and containment logic.
  - Must surface evidence gaps that materially affect containment safety.

### Migration Packet

- **Purpose**: Encodes compatibility-aware transition work between current and
  target states.
- **Derived Artifacts**:
  - `source-target-map.md`
  - `compatibility-matrix.md`
  - `sequencing-plan.md`
  - `fallback-plan.md`
  - `migration-verification-report.md`
  - `decision-record.md`
- **Validation Rules**:
  - Must define the current state, target state, and transition boundary.
  - Must include compatibility, sequencing, and fallback expectations.
  - Must state when coexistence or rollback credibility is insufficient.

### Blast Radius Assessment

- **Purpose**: Captures the potential spread and impact of an incident.
- **Fields**:
  - `impacted_surfaces`
  - `propagation_paths`
  - `confidence_level`
  - `unknowns`
- **Validation Rules**:
  - Must distinguish known impact from inferred impact.
  - Must call out when confidence is too low to justify a release-readiness
    claim.

### Containment Plan

- **Purpose**: Orders the steps used to stop or limit incident spread.
- **Fields**:
  - `immediate_actions`
  - `ordered_sequence`
  - `stop_conditions`
  - `owner_notes`
- **Validation Rules**:
  - Must present an operationally ordered plan, not an unordered checklist.
  - Must state when a step depends on unverified assumptions.

### Compatibility Assessment

- **Purpose**: States what must keep working during migration and what can
  temporarily diverge.
- **Fields**:
  - `guaranteed_compatibility`
  - `temporary_incompatibilities`
  - `coexistence_rules`
  - `boundary_assumptions`
- **Validation Rules**:
  - Must separate guaranteed compatibility from temporary exceptions.
  - Must identify the boundaries where wrong sequencing causes breakage.

### Sequencing Plan

- **Purpose**: Captures step ordering for either incident containment or
  migration rollout.
- **Fields**:
  - `ordered_steps`
  - `parallelizable_steps`
  - `gating_conditions`
  - `cutover_or_stop_points`
- **Validation Rules**:
  - Must make sequencing dependencies explicit.
  - Must show where progression should halt if preconditions fail.

### Fallback Plan

- **Purpose**: Defines rollback, stop, or alternate-path behavior when the
  primary plan cannot proceed safely.
- **Fields**:
  - `fallback_paths`
  - `rollback_triggers`
  - `re_entry_criteria`
  - `residual_risks`
- **Validation Rules**:
  - Must describe what happens when the primary plan fails.
  - Must not claim rollback credibility without a named trigger and path.

### Operational Evidence Gap

- **Purpose**: Records missing facts or unverifiable assumptions that affect
  readiness or containment safety.
- **Fields**:
  - `gap_statement`
  - `affected_artifacts`
  - `gate_impact`
  - `recommended_follow_up`
- **Validation Rules**:
  - Every material gap must identify which gate or readiness judgment it
    affects.
  - Evidence gaps must remain visible in the packet summary or supporting
    artifacts.

### Operational Gate Decision

- **Purpose**: Represents the gate outcome for a high-risk run.
- **Fields**:
  - `gate_kind`
  - `decision_state`
  - `blockers`
  - `approval_requirements`
  - `supporting_artifacts`
- **Validation Rules**:
  - Must align with the declared gate profile for the mode.
  - Must explain blockers or approval needs in artifact-backed terms.

## Relationships

- One `Operational Packet` belongs to exactly one Canon run.
- An `Incident Packet` or `Migration Packet` is a mode-specific form of
  `Operational Packet`.
- A packet may contain many `Operational Evidence Gap` records.
- `Blast Radius Assessment`, `Containment Plan`, `Compatibility Assessment`,
  `Sequencing Plan`, and `Fallback Plan` feed one or more
  `Operational Gate Decision` outcomes.

## State Transitions

- `Draft Input` -> `Generated Packet`: authored inputs and runtime generation
  produce the initial operational packet.
- `Generated Packet` -> `Blocked`: one or more gates fail because containment,
  compatibility, or evidence posture is insufficient.
- `Generated Packet` -> `AwaitingApproval`: gates require explicit human
  approval before progression.
- `Generated Packet` -> `Completed`: all required gates pass with sufficient
  evidence.
- `Completed` -> `Published`: packet is copied into `docs/incidents/<RUN_ID>/`
  or `docs/migrations/<RUN_ID>/`.

High-risk packets must never transition directly from `Draft Input` to
`Completed` without generated artifacts and recorded gate outcomes.