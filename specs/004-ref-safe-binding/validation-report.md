# Validation Report Plan: Runnable Skill Interaction and Ref-Safe Input Binding

## Status

Planned. No implementation validation has executed yet for this feature.

## Validation Ownership

- generation side:
  - updated skill contracts
  - updated shared preflight helpers
- validation side:
  - shell validator
  - PowerShell validator
  - runnable walkthrough review against real Canon CLI behavior
  - independent contract review against current Canon runtime docs and engine
    expectations

## Structural Validation

Planned checks:

- `scripts/validate-canon-skills.sh`
- `pwsh -File scripts/validate-canon-skills.ps1`
- `git diff --check`
- repo-relative path hygiene for new feature docs

Expected assertions:

- `canon-pr-review` preflight uses `--ref` for base/head
- path-oriented runnable skills continue to use `--input`
- affected runnable skills describe incremental correction and preserved valid
  inputs
- modeled-only skills remain unchanged in support-state honesty

## Shared Preflight Validation Matrix

| Case | Expected Result |
| --- | --- |
| missing `zone` after valid `owner` and `risk` | `STATUS=missing-input`, `FAILED_SLOT=zone`, preserved owner/risk |
| invalid risk token | `STATUS=invalid-input`, `FAILED_SLOT=risk` |
| invalid zone token | `STATUS=invalid-input`, `FAILED_SLOT=zone` |
| missing run id | `STATUS=missing-input`, `FAILED_SLOT=run-id` |
| unresolved run id | `STATUS=invalid-input`, `FAILED_SLOT=run-id` |
| missing input path | `STATUS=missing-file`, `FAILED_SLOT=input-path` |
| `canon-pr-review` with `master` and `HEAD` | `STATUS=ready`, normalized base/head returned |
| `canon-pr-review` with `main` and `HEAD` in master-only local repo | `STATUS=invalid-ref`, no file-path wording |
| `canon-pr-review` with `origin/main` | `STATUS=invalid-ref`, `FAILED_KIND=unsupported-remote-ref` |
| `canon-pr-review` with same normalized base/head | `STATUS=malformed-ref-pair` |

These cases must pass in both Bash and PowerShell.

## Runnable Walkthrough Validation

### `canon-pr-review`

Required walkthroughs:

- `base master, head HEAD`
- `base main, head HEAD` in a repo whose usable local branch is `master`
- one ref missing while the other remains valid
- remote-like ref rejected deterministically
- Canon execution begins only after typed preflight returns `ready`

### `canon-requirements`

Required walkthroughs:

- valid `owner` and `risk`, missing `zone`
- missing input path
- single corrected field retry without re-entering valid fields

### `canon-brownfield`

Required walkthroughs:

- valid ownership metadata, missing brief path
- corrected path only, with ownership metadata preserved

### Run-id-Oriented Skills

Required walkthroughs:

- `canon-status` asks only for missing `RUN_ID`
- `canon-resume` asks only for missing `RUN_ID`
- `canon-approve` preserves valid non-run-id fields when only `RUN_ID` needs
  correction

## Independent Validation Focus

Independent review must confirm:

- semantically valid refs are never treated as file paths
- retry guidance matches the exact accepted Canon binding form
- preflight failures and Canon-execution failures are explicitly separated
- shell and PowerShell behavior remain materially identical

## Evidence To Record After Implementation

- validator command outputs
- targeted shell preflight outputs
- targeted PowerShell preflight outputs
- runnable walkthrough notes for affected skills
- any discovered deviations and corrective follow-up
