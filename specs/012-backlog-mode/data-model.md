# Data Model: Backlog Mode (Delivery Decomposition)

## Overview

Backlog mode extends Canon's existing run model rather than introducing a second planning runtime. The core data design adds backlog-specific planning context, closure findings, and decomposition entities to the existing run, artifact, inspect, and publish surfaces.

Folder-backed `canon-input/backlog/` packets split the authored planning surface into `brief.md` plus optional `priorities.md` and `context-links.md`. The current backlog brief remains authoritative; upstream artifacts remain provenance and evidence, not hidden execution inputs.

## Entities

### Backlog Planning Context

- **Purpose**: Captures the machine-readable planning context required for a backlog run.
- **Persistence**: Added to `.canon/runs/<RUN_ID>/context.toml` as an optional mode-specific block.
- **Fields**:
  - `mode`: fixed to `backlog`
  - `delivery_intent`: concise statement of what the backlog packet is meant to support
  - `desired_granularity`: one of `epic-only`, `epic-plus-slice`, or `epic-plus-slice-plus-story-candidate`
  - `planning_horizon`: optional timeframe or horizon statement from the authored brief
  - `source_refs`: explicit upstream artifact references
  - `priority_inputs`: normalized priorities captured from the authored packet
  - `constraints`: explicit planning constraints such as time, dependency, team, or sequencing pressure
  - `out_of_scope`: explicitly excluded or deferrable work
  - `closure_assessment`: `ClosureAssessment`
- **Validation Rules**:
  - `delivery_intent` must not be empty
  - `desired_granularity` must be present and must not exceed backlog mode boundaries
  - `source_refs` must be present unless the brief provides explicit bounded planning justification
  - `out_of_scope` may be empty, but the runtime must surface missing boundaries as a planning weakness rather than inventing exclusions

### Source Trace Link

- **Purpose**: Connects a backlog output element to the upstream artifact, authored priority, or planning gap that justifies it.
- **Persistence**: Stored in the backlog planning context and referenced from emitted artifacts.
- **Fields**:
  - `source_kind`: `upstream-artifact`, `authored-priority`, or `planning-gap`
  - `source_ref`: repo-relative path, heading, or authored reference
  - `justification`: brief explanation of why the source supports the output element
  - `confidence`: `bounded`, `partial`, or `at-risk`
- **Validation Rules**:
  - every epic and delivery slice must resolve to at least one trace link
  - `planning-gap` trace links must carry a visible risk marker and cannot masquerade as bounded source proof

### Closure Assessment

- **Purpose**: Captures whether upstream architecture or system shape is credible enough for decomposition.
- **Persistence**: Stored in run context and surfaced in summaries plus `planning-risks.md`.
- **Fields**:
  - `status`: `sufficient`, `downgraded`, or `blocked`
  - `findings`: one or more `ClosureFinding` entries
  - `decomposition_scope`: `full-packet` or `risk-only-packet`
  - `notes`: optional explanation for human readers
- **Validation Rules**:
  - `blocked` and `downgraded` states must include at least one finding
  - `risk-only-packet` may not emit a misleading full decomposition artifact set

### Closure Finding

- **Purpose**: Records a specific reason why decomposition is unsafe or incomplete.
- **Persistence**: Stored in run context and surfaced in summaries plus `planning-risks.md`.
- **Fields**:
  - `category`: `missing-capability-boundary`, `contradictory-source`, `unresolved-dependency`, `missing-exclusion`, `unclear-ownership`, or other bounded category
  - `severity`: `warning` or `blocking`
  - `affected_scope`: epic, capability, dependency, or whole-run scope
  - `recommended_followup`: concise next step, usually return to architecture or strengthen the backlog brief
- **Validation Rules**:
  - blocking findings require `status = blocked` unless policy explicitly allows downgrade

### Epic Tree Node

- **Purpose**: Represents a bounded planning unit within the initiative/epic/sub-epic hierarchy.
- **Persistence**: Emitted in `epic-tree.md` and referenced from other backlog artifacts.
- **Fields**:
  - `id`: stable packet-local identifier
  - `title`: concise planning name
  - `level`: `initiative`, `epic`, or `sub-epic`
  - `scope_boundary`: concise statement of what the node includes and excludes
  - `trace_links`: one or more `SourceTraceLink` records
  - `dependencies`: zero or more `DependencyEdge` references
- **Validation Rules**:
  - every node must have a bounded scope statement
  - every node must have at least one trace link

### Delivery Slice

- **Purpose**: Represents a bounded vertical delivery slice that later implementation work can consume.
- **Persistence**: Emitted in `delivery-slices.md` and linked from sequencing and acceptance artifacts.
- **Fields**:
  - `id`: stable slice identifier
  - `title`: concise slice name
  - `parent_epic`: the owning epic or sub-epic
  - `slice_type`: `foundation`, `feature`, or `risk-reduction`
  - `trace_links`: one or more `SourceTraceLink` records
  - `dependencies`: zero or more `DependencyEdge` references
  - `acceptance_anchor_refs`: zero or more `AcceptanceAnchor` references
- **Validation Rules**:
  - slices must remain above task level
  - slices must remain bounded enough for downstream execution planning

### Dependency Edge

- **Purpose**: Records a dependency relationship relevant to sequencing or scope credibility.
- **Persistence**: Emitted in `dependency-map.md` and referenced from sequencing and epic/slice artifacts.
- **Fields**:
  - `from`: source epic or slice
  - `to`: dependent epic, slice, or external dependency
  - `dependency_type`: `cross-epic`, `intra-epic`, or `external`
  - `reason`: concise explanation
  - `blocking`: boolean
- **Validation Rules**:
  - blocking dependencies must appear in the sequencing plan
  - external dependencies must remain visible in planning risks when they threaten credibility

### Acceptance Anchor

- **Purpose**: Captures a bounded planning-level signal that an epic or slice is complete.
- **Persistence**: Emitted in `acceptance-anchors.md`.
- **Fields**:
  - `target_id`: epic or slice identifier
  - `anchor_text`: concise planning-level completion statement
  - `trace_links`: one or more `SourceTraceLink` records
- **Validation Rules**:
  - anchors must not expand into full acceptance criteria or executable test plans

### Planning Risk

- **Purpose**: Records risks that weaken sequencing or delivery credibility.
- **Persistence**: Emitted in `planning-risks.md` and referenced in run summaries.
- **Fields**:
  - `risk_id`: stable identifier
  - `category`: `closure-gap`, `oversized-epic`, `hidden-dependency`, `sequencing-uncertainty`, or similar bounded label
  - `severity`: `warning` or `blocking`
  - `description`: concise risk statement
  - `mitigation_hint`: suggested follow-up
- **Validation Rules**:
  - blocking risks must align with closure findings when they prevent full decomposition

### Backlog Artifact Bundle

- **Purpose**: Names the durable markdown artifacts required for a successful or degraded backlog run.
- **Persistence**: Existing artifact bundle under `.canon/runs/<RUN_ID>/artifacts/` with explicit contract requirements in the mode artifact contract.
- **Successful Bundle**:
  - `backlog-overview.md`
  - `epic-tree.md`
  - `capability-to-epic-map.md`
  - `dependency-map.md`
  - `delivery-slices.md`
  - `sequencing-plan.md`
  - `acceptance-anchors.md`
  - `planning-risks.md`
- **Risk-Only Bundle**:
  - `backlog-overview.md`
  - `planning-risks.md`

## Relationships

- A `RunManifest` identifies the backlog run and its canonical identity.
- A `RunContext` stores generic run metadata plus an optional `BacklogPlanningContext`.
- `ClosureAssessment` determines whether the run may emit a full backlog packet or only a risk-focused packet.
- `EpicTreeNode`, `DeliverySlice`, `DependencyEdge`, and `AcceptanceAnchor` all depend on `SourceTraceLink` to remain governance-backed.
- `PlanningRisk` draws from both authored constraints and closure findings.

## State Transitions

### Backlog Run Lifecycle

1. `Draft` -> `ContextCaptured`: authored inputs are bound, snapshotted, and persisted
2. `ContextCaptured` -> `Classified`: mode, risk, zone, and initial planning context are established
3. `Classified` -> `ClosureChecked`: source closure and granularity discipline are evaluated
4. `ClosureChecked` -> `Decomposing` or `Blocked`: successful closure proceeds to full packet generation; insufficient closure yields downgrade or block behavior
5. `Decomposing` -> `Completed`: backlog packet is emitted and linked from the run manifest

## Compatibility Notes

- No new run identity fields are added; display id, UUID, short id, slug, and `@last` behavior remain unchanged.
- No new publish model is added; backlog packets publish through the existing flow to `docs/planning/<RUN_ID>/` by default.
- No authored-input file is rewritten; immutable snapshots remain the runtime record.