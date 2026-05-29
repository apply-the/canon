# Data Model: Mode Clarification And Run Refinement

**Branch**: `062-clarify-run-refinement`
**Date**: 2026-05-29

## Entities

### 1. Durable Draft Run

The existing Canon run entity while it is still in pre-execution refinement.
This feature does not introduce a second identity family; the durable draft is
the same run identity in `RunState::Draft` and later states.

**Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `run_id` | string | yes | Human-facing run identifier |
| `uuid` | string | yes | Stable machine identity |
| `mode` | enum | yes | Current governed mode for the draft or run |
| `state` | enum | yes | Lifecycle state; starts at `Draft` |
| `owner` | string | yes | Human owner for systemic-impact governance |
| `created_at` | timestamp | yes | Draft creation time |

**Validation rules**:
- A durable draft run MUST keep the same `run_id` and `uuid` through
  clarification and run start.
- Pre-start mode correction MAY change `mode` in place.
- Post-start mode correction MUST create a successor run instead.

### 2. Clarification Refinement Context

Typed run-scoped refinement state persisted on `RunContext`.

```toml
[clarification_refinement]
workflow_family = "planning"
current_mode = "requirements"
working_brief_path = ".canon/runs/R-20260529-ab12cd34/artifacts/requirements/working-brief.md"
template_ref = "docs/templates/canon-input/requirements.md"
status = "active"
explicit_continuation_required = true
authoritative_input_refs = ["canon-input/requirements/brief.md"]
supporting_input_refs = ["canon-input/requirements/context-links.md"]

[clarification_refinement.suggested_candidate]
run_id = "R-20260529-ab12cd34"
mode = "requirements"
state = "Draft"
match_reason = "same authoritative input fingerprint"
advisory = true
```

**Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `workflow_family` | enum | yes | Planning, execution, or assessment family |
| `current_mode` | enum | yes | Mode the refinement currently targets |
| `working_brief_path` | path | yes | Run-local working brief artifact |
| `template_ref` | path | yes | Template or method source used to seed the brief |
| `status` | enum | yes | Draft refinement state (`active`, `ready`, `superseded`) |
| `explicit_continuation_required` | boolean | yes | Always true when mutation of an existing candidate would require confirmation |
| `authoritative_input_refs` | array[path] | yes | Current authoritative brief inputs |
| `supporting_input_refs` | array[path] | yes | Supporting but non-authoritative inputs |
| `suggested_candidate` | object or null | no | Advisory candidate summary for continuation workflows |

**Validation rules**:
- `working_brief_path` MUST point under `.canon/runs/<RUN_ID>/artifacts/`.
- `authoritative_input_refs` MUST NOT point into `.canon/`.
- `supporting_input_refs` never replace `authoritative_input_refs`.
- `suggested_candidate.advisory` is always `true`; candidate detection never
  grants implicit mutation authority.

### 3. Clarification Record

The durable record for one question asked during refinement.

```toml
[[clarification_refinement.records]]
id = "cq-001"
prompt = "Which actor owns the problem?"
answer = "platform operators"
answer_kind = "explicit"
affected_sections = ["Actors", "Problem Statement"]
resolution_state = "resolved"
recorded_at = "2026-05-29T12:10:00Z"
```

**Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | yes | Stable question record identifier |
| `prompt` | string | yes | Question presented to the operator |
| `answer` | string | yes | Explicit answer or applied default |
| `answer_kind` | enum | yes | `explicit`, `defaulted`, or `deferred` |
| `affected_sections` | array[string] | yes | Working-brief sections updated or still blocked |
| `resolution_state` | enum | yes | `resolved`, `deferred`, or `superseded` |
| `recorded_at` | timestamp | yes | When the answer or default was recorded |

**Validation rules**:
- Records are append-only; later supersession creates a new record or marks the
  old one superseded.
- `answer_kind = defaulted` requires the answer text to describe the applied
  default.
- `affected_sections` MUST map to real sections in the working brief or
  readiness summary.

### 4. Readiness Delta Item

Structured readiness blocker or follow-up item that can be rendered into the
existing flat readiness-delta summary.

```toml
[[clarification_refinement.readiness_delta]]
id = "rd-001"
section = "Validation Strategy"
summary = "Independent validation owner is not yet named."
blocking = true
source_kind = "missing-context"
default_available = false
resolved = false
```

**Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | yes | Stable readiness item identifier |
| `section` | string | yes | Affected section or lifecycle area |
| `summary` | string | yes | Human-readable blocker or delta summary |
| `blocking` | boolean | yes | Whether the item blocks readiness |
| `source_kind` | enum | yes | `authority-gap`, `missing-context`, `clarification-gap`, or `supporting-input-warning` |
| `default_available` | boolean | yes | Whether a default can be applied safely |
| `resolved` | boolean | yes | Whether the item still blocks or warns |

**Validation rules**:
- The derived flat readiness-delta summary MUST remain consistent with the
  structured items.
- `blocking = true` and `resolved = true` may only appear when the item is
  retained for audit history rather than active readiness.

### 5. Continuation Candidate Summary

Advisory projection surfaced to operators when Canon finds a likely existing
draft or run.

**Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `run_id` | string | yes | Candidate run identifier |
| `mode` | enum | yes | Candidate mode |
| `state` | enum | yes | Candidate lifecycle state |
| `match_reason` | string | yes | Why Canon considers it a likely continuation |
| `advisory` | boolean | yes | Always true |

**Validation rules**:
- A single candidate may be suggested, but Canon still requires explicit
  continuation intent before mutation.
- Multiple candidates trigger disambiguation instead of a single suggestion.

### 6. Run Lineage Link

Structured successor linkage stored on the successor manifest when a started
run is redirected to a new mode.

```toml
[lineage]
carried_from = "R-20260529-ab12cd34"
supersedes = "R-20260529-ab12cd34"
mode_change_reason = "Clarification redirected the work from architecture to change."
```

**Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `carried_from` | string | yes | Prior run whose context is being carried forward |
| `supersedes` | string | yes | Prior run replaced for forward work |
| `mode_change_reason` | string | yes | Explicit explanation for the redirection |

**Validation rules**:
- Present only on successor runs created after the original run has started.
- `carried_from` and `supersedes` point to the original run in this slice.

### 7. Working Brief Artifact

The run-local markdown artifact that holds the authoritative current-mode brief
plus refinement provenance.

**Fields**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `path` | path | yes | Artifact path under `.canon/runs/<RUN_ID>/artifacts/` |
| `mode` | enum | yes | Current mode of the working brief |
| `authority_source_refs` | array[path] | yes | Immutable authored inputs that seeded the brief |
| `clarification_record_ids` | array[string] | yes | Associated clarification records |
| `readiness_delta_ids` | array[string] | yes | Associated readiness items |

**Validation rules**:
- The artifact MUST preserve mode-specific template sections.
- The artifact MUST include additive provenance and unresolved-question
  sections; it MUST NOT overwrite `canon-input/`.

## Relationships

```text
Durable Draft Run
  ├── 1:1 Clarification Refinement Context
  │     ├── 1:N Clarification Record
  │     ├── 1:N Readiness Delta Item
  │     └── 0:1 Continuation Candidate Summary
  ├── 0:1 Working Brief Artifact
  └── 0:1 Run Lineage Link (successor only)
```

## Entity Lifecycle

1. Canon creates a run in `RunState::Draft` and captures input fingerprints.
2. Clarification materializes a `ClarificationRefinementContext` and a
   run-local working brief artifact.
3. Clarification answers append `ClarificationRecord` entries and update
   `ReadinessDeltaItem` collections on the same run identity.
4. If the correct mode changes before run start, Canon updates the same draft
   run and refinement context in place.
5. When the operator explicitly continues the work and starts execution, the
   same run identity proceeds into governed lifecycle states.
6. If the correct mode changes after run start, Canon creates a successor run
   with a `RunLineageLink` back to the original run and carries forward the
   typed refinement context into the successor.