# Contract: Status And Inspect Refinement Surfaces

**Feature**: `062-clarify-run-refinement`

## Purpose

Define the additive operator-facing CLI contract for same-work refinement.

## Existing Surface Boundaries

- `canon status --run <RUN_ID>` remains the compact run summary.
- `canon inspect clarity --mode <MODE> --input ...` remains the pre-run
  authored-input analysis surface.
- A new run-scoped refinement inspect target carries detailed working-brief and
  clarification state.

## `canon status --run <RUN_ID>` Additions

Status stays compact and additive. JSON or YAML output gains a top-level
`refinement_state` object when refinement state exists.

```json
{
  "run_id": "R-20260529-ab12cd34",
  "mode": "requirements",
  "state": "Draft",
  "refinement_state": {
    "active": true,
    "working_brief_path": ".canon/runs/R-20260529-ab12cd34/artifacts/requirements/working-brief.md",
    "authoritative_inputs": ["canon-input/requirements/brief.md"],
    "supporting_inputs": ["canon-input/requirements/context-links.md"],
    "clarification_records_total": 4,
    "clarification_records_unresolved": 1,
    "readiness_delta_total": 3,
    "explicit_continuation_required": true
  },
  "suggested_continuation": {
    "run_id": "R-20260529-ab12cd34",
    "mode": "requirements",
    "state": "Draft",
    "match_reason": "same authoritative input fingerprint",
    "advisory": true,
    "mutation_allowed": false
  }
}
```

### Rules

- Existing top-level status fields remain unchanged.
- `refinement_state` appears only when a run has persisted refinement context.
- `suggested_continuation` is optional and advisory; `mutation_allowed` is
  always `false` until explicit continuation intent is captured.
- Text and markdown status output must summarize the same information without
  hiding unresolved clarification counts.

## `canon inspect refinement --run <RUN_ID>`

Introduce a dedicated run-scoped inspection target for refinement state.

### Command Shape

```text
canon inspect refinement --run <RUN_ID> [--output text|json|yaml|markdown]
```

### JSON Payload

```json
{
  "run_id": "R-20260529-ab12cd34",
  "mode": "requirements",
  "state": "Draft",
  "working_brief_path": ".canon/runs/R-20260529-ab12cd34/artifacts/requirements/working-brief.md",
  "authoritative_inputs": ["canon-input/requirements/brief.md"],
  "supporting_inputs": ["canon-input/requirements/context-links.md"],
  "clarification_records": [
    {
      "id": "cq-001",
      "prompt": "Which actor owns the problem?",
      "answer": "platform operators",
      "answer_kind": "explicit",
      "affected_sections": ["Actors", "Problem Statement"],
      "resolution_state": "resolved",
      "recorded_at": "2026-05-29T12:10:00Z"
    }
  ],
  "readiness_delta": [
    {
      "id": "rd-001",
      "section": "Validation Strategy",
      "summary": "Independent validation owner is not yet named.",
      "blocking": true,
      "source_kind": "missing-context",
      "default_available": false,
      "resolved": false
    }
  ],
  "suggested_continuation": {
    "run_id": "R-20260529-ab12cd34",
    "mode": "requirements",
    "state": "Draft",
    "match_reason": "same authoritative input fingerprint",
    "advisory": true,
    "mutation_allowed": false
  },
  "lineage": null
}
```

### Rules

- `working_brief_path` is the canonical path for the run-local authoritative
  working brief.
- `clarification_records` preserve explicit answers, defaults, and deferred
  state.
- `readiness_delta` preserves structured items even if status shows only counts.
- `lineage` is non-null only for successor runs created after a started run is
  redirected to another mode.

## Ambiguity And Explicit Intent Rules

- Single-candidate detection may surface `suggested_continuation`, but it must
  not mutate an existing run by itself.
- Multiple plausible matches require a disambiguation response before any
  mutation; the inspect surface may show a candidate list or ambiguity flag in
  a future additive field, but mutation remains blocked until explicit choice.
- Fresh work without an explicit continuation signal starts a new run even when
  a single likely candidate exists.

## Markdown Output Requirements

Markdown output for `inspect refinement` must include these headings when data
exists:

- `## Refinement State`
- `## Working Brief`
- `## Clarification Records`
- `## Readiness Delta`
- `## Continuation Guidance`
- `## Lineage` (successor runs only)

## Compatibility Rules

- `inspect clarity` keeps its existing contract and does not accept run-scoped
  refinement output in this slice.
- Existing status consumers may ignore `refinement_state` and
  `suggested_continuation` safely.
- Text and markdown renderers must state clearly that candidate detection is
  advisory and continuation requires explicit intent.