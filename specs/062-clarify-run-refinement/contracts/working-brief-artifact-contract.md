# Contract: Working Brief Artifact

**Feature**: `062-clarify-run-refinement`

## Purpose

Define the run-local markdown artifact Canon refines during clarification for
the five targeted planning modes.

## Artifact Location

```text
.canon/runs/<RUN_ID>/artifacts/<mode>/working-brief.md
```

The working brief is additive runtime state. It never replaces or rewrites the
authored input under `canon-input/`.

## Shape Rules

- The artifact MUST preserve the current mode's canonical brief sections.
- Canon MAY seed missing sections from `defaults/templates/canon-input/<mode>.md`
  or equivalent mode guidance, but only the run-local working brief is mutated
  during clarification.
- Canon MUST append a standard refinement appendix so all targeted modes expose
  provenance and readiness consistently.

## Required Content

### Mode-Specific Body

The opening sections remain mode-specific and mirror the authoritative brief or
seed template for the current mode.

### Standard Refinement Appendix

Every targeted-mode working brief adds these sections after the mode-specific
body:

```markdown
## Clarification Provenance

### Applied Answers

- cq-001: Which actor owns the problem? -> platform operators

### Applied Defaults

- cq-004: Validation owner defaulted to repository maintainer review

## Source Snapshots

- canon-input/requirements/brief.md
- canon-input/requirements/context-links.md

## Unresolved Questions

- Which downstream team owns the rollout sign-off?

## Readiness Delta

- Independent validation owner is not yet named.

## Continuation State

- Same run identity retained during draft refinement.
- Candidate detection is advisory; continuation requires explicit intent.
```

## Rendering Rules

- `Applied Answers` reflects explicit operator answers.
- `Applied Defaults` reflects defaults Canon applied because a question was
  skipped or deferred.
- `Source Snapshots` lists immutable authored inputs or carried-forward source
  refs used to seed the working brief.
- `Unresolved Questions` remains explicit when clarification is still pending.
- `Readiness Delta` is derived from the structured readiness items stored in
  runtime context.
- `Continuation State` states whether the run is still draft-local, has been
  promoted into governed execution, or has been superseded by a successor.

## Validation Rules

- The artifact path MUST match the `working_brief_path` stored in runtime
  context.
- The artifact MUST reference authoritative and supporting inputs without
  claiming that supporting inputs are authoritative.
- The artifact MUST NOT drop unresolved questions or readiness blockers when
  the brief is materially incomplete.
- The artifact MUST stay readable as markdown and remain mode-appropriate for
  `requirements`, `discovery`, `system-shaping`, `architecture`, and `change`.

## Compatibility Rules

- Existing packet outputs and published artifacts stay unchanged unless they
  explicitly choose to surface refinement state.
- Non-targeted modes do not receive this first-class working-brief artifact in
  the initial slice, though they still participate in explicit continuation
  identity continuity.