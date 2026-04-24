# Contract: Backlog Authored Input

## Summary

`backlog` mode turns bounded upstream artifacts and explicit planning intent into governed delivery decomposition. The authored input contract defines the minimum information Canon needs to produce a credible backlog packet or to explain why decomposition must be blocked or downgraded.

## Canonical Authored Inputs

- Single-file canonical input: `canon-input/backlog.md`
- Folder-backed canonical input: `canon-input/backlog/`
- When both canonical locations exist, the runtime should prefer `canon-input/backlog/` so the full authored packet is read and snapshotted

## Folder-Backed Packet Shape

Preferred directory layout:

```text
canon-input/
  backlog/
    brief.md
    priorities.md
    context-links.md
```

- `brief.md` is the authoritative current-mode brief
- `priorities.md` is optional supporting authored input for sequencing and delivery intent
- `context-links.md` is optional supporting authored input for upstream references and narrowed source context

## Required Authored Content

The authored input must provide, directly or by explicit reference:

- source artifact references for the upstream work being decomposed, or explicit bounded justification when no prior packet exists
- delivery intent and desired granularity
- enough scope boundary information to distinguish in-scope, out-of-scope, and deferrable work

The authored packet should also provide when available:

- planning horizon
- priorities
- known constraints
- known dependencies or unresolved planning gaps

## Runtime Expectations

- Authored inputs are read-only and must be snapshotted immutably under `.canon/runs/<RUN_ID>/inputs/`
- The current backlog brief remains authoritative for the run
- Supporting inputs enrich provenance and planning context, but do not silently override the current backlog brief
- Missing delivery intent, desired granularity, or meaningful source references must produce explicit validation reasons rather than silent defaults

## CLI Surface

- Start path remains `canon run --mode backlog ...`
- Existing lifecycle surfaces remain authoritative for visibility:
  - `canon status --run <RUN_ID>`
  - `canon inspect artifacts --run <RUN_ID>`
  - `canon inspect evidence --run <RUN_ID>`
  - `canon inspect invocations --run <RUN_ID>`
  - `canon publish <RUN_ID>`
- No new top-level command is introduced for backlog mode

## Validation and Failure Semantics

- Blocking or downgrade-triggering authored-input failures include:
  - missing delivery intent
  - missing desired granularity
  - source artifacts too vague or contradictory for credible decomposition
  - missing scope boundaries severe enough to make decomposition misleading
  - authored requests that exceed backlog granularity by asking for task-level planning
- When these failures occur, Canon must record explicit closure findings or planning risks rather than invent missing structure