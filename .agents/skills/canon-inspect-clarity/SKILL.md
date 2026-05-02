---
name: canon-inspect-clarity
description: Use when you need Canon-backed missing-context findings, explicit output-quality posture, materially-closed decision signals, and targeted clarification questions from authored file-backed mode inputs before starting a run.
---

# Canon Inspect Clarity

## Support State

- `available-now`
- `default visibility`: `discoverable-standard`

## Purpose

Inspect authored file-backed mode inputs for missing context, weak reasoning,
explicit output-quality posture, or materially-closed decisions before
starting a governed run.

## When To Trigger

- The user has a file-backed governed brief, but it is still underspecified,
  structurally shallow, or already materially closes the decision.
- The user wants Canon to reason over the current inputs and ask targeted
  clarification questions before run start.

## When It Must Not Trigger

- The user already has a run id and needs run-scoped inspection, approval, or
  resume behavior.
- The user already wants to start a run and the authored input is sufficiently
  clear.
- The mode is `pr-review` or another non-file-backed surface.

## Required Inputs

- `MODE` as one of `requirements`, `discovery`, `system-shaping`,
  `architecture`, `backlog`, `change`, `implementation`, `refactor`,
  `review`, `verification`, `incident`, `security-assessment`,
  `system-assessment`, `migration`, or `supply-chain-analysis`
- one canonical authored-input surface or one or more explicit `INPUT_PATH`
  values

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- `.canon/` is not required for this inspection surface.
- Reject unsupported modes. Today every file-backed governed mode except
  `pr-review` is valid for clarity inspection.
- For auto-binding only, use the canonical `canon-input/<MODE>.md` or
  `canon-input/<MODE>/` authored-input location for any supported file-backed
  mode, and prefer the directory when both exist so Canon inspects the full
  authored surface instead of a single child file.
- Keep the stricter `review` input contract: only `canon-input/review.md` or
  `canon-input/review/` is valid for that mode.
- Never infer `--input` from the active editor file, open tabs, recent
  `.canon/` artifacts, or any other path under `.canon/`.
- If the selected canonical location or explicit input is a folder, inspect the
  whole directory recursively, including authored files under subfolders.
- If the user provided multiple explicit files or folders, inspect all of them
  together in a single clarity result.
- Never narrow a canonical directory such as `canon-input/change/` to a single
  child file when the directory itself exists.
- If `MODE` is missing, ask only which supported file-backed mode Canon should
  inspect.
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
- explicit output-quality posture stating whether the packet is only
  `structurally-complete`, already `materially-useful`, or `publishable`
- evidence signals or downgrade reasons explaining that posture
- reasoning signals that explain why Canon is asking follow-up questions,
  why the packet is still weak, or why the decision is already materially
  closed
- prioritized clarification questions
- recommended focus area when Canon found one
- no run id, approval target, or evidence packet because this is a pre-run
  inspection surface

## Failure Handling Guidance

- If `MODE` is missing or unsupported, ask only for the intended file-backed
  mode and show the exact Canon CLI retry form `canon inspect clarity --mode
  <MODE> --input <INPUT_PATH> [<INPUT_PATH> ...]`. Make the `pr-review`
  exclusion explicit when relevant.
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
- If Canon reports a packet as only `structurally-complete`, say that directly
  and keep the next step on resolving the named downgrade reasons before run
  start.
- If Canon reports a packet as `materially-useful`, say what still prevents it
  from reading as fully publishable.
- If Canon reports that the authored packet already materially closes the
  decision, say so directly and keep the next step on preserving that closure
  in the matching governed run rather than inventing more balance.
- Once the top gaps are answered, recommend the matching governed mode skill
  for the analyzed file-backed mode.
- Use `$canon-inspect-evidence` or other run-scoped skills only after a real
  run exists.
- Do not fabricate a started run, pending approval, or emitted artifact set
  from clarity inspection.

## Related Skills

- `$canon-requirements`
- `$canon-change`
- `$canon-review`
- `$canon-verification`
- `$canon-init`