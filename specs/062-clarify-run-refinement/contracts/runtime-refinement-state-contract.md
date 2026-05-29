# Contract: Runtime Refinement State

**Feature**: `062-clarify-run-refinement`

## Purpose

Define the stable persisted runtime shape for clarification refinement state so
Canon can preserve same-work continuity, carry-forward lineage, and additive
working-brief artifacts without mutating `canon-input/`.

## Persisted Surfaces

- `.canon/runs/<RUN_ID>/context.toml`
- `.canon/runs/<RUN_ID>/manifest.toml`
- `.canon/runs/<RUN_ID>/artifacts/<mode>/working-brief.md`

No new persistence family is introduced in this slice.

## `context.toml` Additions

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

[[clarification_refinement.records]]
id = "cq-001"
prompt = "Which actor owns the problem?"
answer = "platform operators"
answer_kind = "explicit"
affected_sections = ["Actors", "Problem Statement"]
resolution_state = "resolved"
recorded_at = "2026-05-29T12:10:00Z"

[[clarification_refinement.readiness_delta]]
id = "rd-001"
section = "Validation Strategy"
summary = "Independent validation owner is not yet named."
blocking = true
source_kind = "missing-context"
default_available = false
resolved = false
```

### Field Rules

| Field | Rule |
|-------|------|
| `workflow_family` | Uses Canon family vocabulary already used by clarity surfaces (`planning`, `execution`, `assessment`). |
| `current_mode` | Tracks the mode currently owning the working brief. Pre-start mode correction updates this field in place. |
| `working_brief_path` | Must stay under `.canon/runs/<RUN_ID>/artifacts/`. |
| `template_ref` | Points to the mode template or equivalent guidance that seeded the working brief. |
| `status` | Draft refinement lifecycle only; does not duplicate run lifecycle state. |
| `explicit_continuation_required` | Always `true` when Canon is considering reuse of an existing run or draft. |
| `authoritative_input_refs` | Immutable authored inputs Canon treats as the current authority source. |
| `supporting_input_refs` | Supplemental inputs that never replace the authoritative brief. |
| `suggested_candidate.advisory` | Always `true`. Candidate detection never authorizes mutation on its own. |
| `records` | Append-only clarification history with explicit answer/default provenance. |
| `readiness_delta` | Structured readiness items that can be rendered as flat strings for existing output compatibility. |

## `manifest.toml` Additions

Successor runs created after a started run changes mode must carry typed
lineage.

```toml
[lineage]
carried_from = "R-20260529-ab12cd34"
supersedes = "R-20260529-ab12cd34"
mode_change_reason = "Clarification redirected the work from architecture to change."
```

### Field Rules

| Field | Rule |
|-------|------|
| `carried_from` | Required on successor runs created from an already-started run. |
| `supersedes` | Required on successor runs created from an already-started run. |
| `mode_change_reason` | Required human-readable explanation for the successor relationship. |

## Compatibility Rules

- Existing runs without `clarification_refinement` or `lineage` remain valid.
- New fields must be modeled with typed serde structs or enums.
- Existing authored-input fingerprints and upstream context remain unchanged.
- `canon-input/` content remains immutable and is referenced only through
  source paths and snapshots.

## Mutation Rules

- Canon MUST NOT update an existing draft or run only because a candidate was
  detected.
- Canon MUST record explicit continuation intent before mutating
  `clarification_refinement` on an existing run.
- Canon MAY update `current_mode` in place only while the run is still in
  pre-start draft refinement.
- Canon MUST create a successor run once the original run has started and the
  correct mode changes.

## Derived Output Rules

- The persisted `readiness_delta` items derive the existing flat readiness
  summary shown by inspect and status surfaces.
- The persisted clarification records derive counts for answered, defaulted,
  deferred, and unresolved refinement state.
- The working-brief artifact path stored here is the canonical source for
  run-scoped refinement inspection.