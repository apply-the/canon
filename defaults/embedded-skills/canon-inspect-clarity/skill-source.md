---
name: canon-inspect-clarity
description: Use when you need Canon-backed missing-context findings and targeted clarification questions from authored requirements or discovery inputs before starting a run.
---

# Canon Inspect Clarity

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Inspect authored requirements or discovery inputs for missing context before
starting a governed run.

## When To Trigger

- The user has a requirements or discovery brief, but it is still underspecified.
- The user wants Canon to reason over the current inputs and ask targeted
  clarification questions before run start.

## When It Must Not Trigger

- The user already has a run id and needs run-scoped inspection, approval, or
  resume behavior.
- The user already wants to start a run and the authored input is sufficiently
  clear.
- The mode is not `requirements` or `discovery`.

## Required Inputs

- `MODE` as `requirements` or `discovery`
- one canonical authored-input surface or one or more explicit `INPUT_PATH`
  values

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- `.canon/` is not required for this inspection surface.
- Reject unsupported modes. Today only `requirements` and `discovery` are
  valid.
- For auto-binding only, treat `canon-input/requirements.md` or
  `canon-input/requirements/` as the canonical authored-input locations for
  `requirements`, and prefer the directory when both exist so Canon inspects
  the full authored surface instead of a single child file.
- For auto-binding only, treat `canon-input/discovery.md` or
  `canon-input/discovery/` as the canonical authored-input locations for
  `discovery`, and prefer the directory when both exist so Canon inspects the
  full authored surface instead of a single child file.
- Never infer `--input` from the active editor file, open tabs, recent
  `.canon/` artifacts, or any other path under `.canon/`.
- If the selected canonical location or explicit input is a folder, inspect the
  whole directory recursively, including authored files under subfolders.
- If the user provided multiple explicit files or folders, inspect all of them
  together in a single clarity result.
- Never narrow a canonical directory such as `canon-input/requirements/` to a
  single child file when the directory itself exists.
- If `MODE` is missing, ask only whether the user wants `requirements` or
  `discovery` clarification.
- If the input path is missing, ask only for the authored folder path or one or
  more authored file paths that Canon should inspect.
- Verify each selected input exists before invoking Canon.
- Do not show preflight checks to the user. Report only the specific missing
  input.

## Canon Command Contract

- Canon command: `canon inspect clarity --mode <MODE> --input <INPUT_PATH> [<INPUT_PATH> ...]`
- One `--input` group can carry multiple explicit files or folders and still
  produce one aggregated clarity result.
- Repeating `--input` remains accepted for compatibility, but it is not
  required for multi-path inspection.
- Any directory input is read recursively.

## Expected Output Shape

- concise clarity summary
- analyzed mode
- source input paths
- Canon-backed missing-context findings grounded in the authored inputs
- reasoning signals that explain why Canon is asking follow-up questions
- prioritized clarification questions
- recommended focus area when Canon found one
- no run id, approval target, or evidence packet because this is a pre-run
  inspection surface

## Failure Handling Guidance

- If `MODE` is missing or unsupported, ask only for `requirements` or
  `discovery` and show the exact Canon CLI retry form `canon inspect clarity
  --mode <MODE> --input <INPUT_PATH> [<INPUT_PATH> ...]`.
- If the input path is missing or invalid, ask only for the exact authored
  folder path or exact authored file path set and show the exact Canon CLI
  retry form `canon inspect clarity --mode <MODE> --input <INPUT_PATH>
  [<INPUT_PATH> ...]`.
- Preserve the already valid mode or input selection inside the current
  interaction when retrying the missing slot.
- If Canon fails after preflight succeeds, report it as a Canon-execution
  outcome rather than a preflight failure.
- Do not widen recovery into generic brainstorming when Canon already returned
  targeted clarification prompts.

## Next-Step Guidance

- If Canon reports that clarification is still required, surface the top
  questions directly and keep the next step on answering them.
- If the analyzed inputs are for `requirements`, recommend `$canon-requirements`
  once the top gaps are answered.
- If the analyzed inputs are for `discovery`, recommend `$canon-discovery` once
  the top gaps are answered.
- Use `$canon-inspect-evidence` or other run-scoped skills only after a real
  run exists.
- Do not fabricate a started run, pending approval, or emitted artifact set
  from clarity inspection.

## Related Skills

- `$canon-requirements`
- `$canon-discovery`
- `$canon-init`