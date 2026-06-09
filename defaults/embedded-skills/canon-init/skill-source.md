---
name: canon-init
description: Use when a repository does not have Canon runtime state yet and you need to initialize .canon before any governed workflow.
---

# Canon Init

## Support State

- `available-now`
- `default visibility`: `prominent`

## Purpose

Initialize `.canon/` for the target repository so Canon workflows can run and
persist durable evidence locally. The Canon runtime may live at the repository
root or at a parent workspace root passed through `--canon-root`. In Codex and
compatible Copilot environments, this skill also materializes the matching
repo-local skill surface.

## When To Trigger

- The current repository has no reachable `.canon/` directory yet at the repo
  root or any intended parent Canon workspace.
- A later Canon skill needs initialization before it can proceed.

## When It Must Not Trigger

- `.canon/` already exists and the user is asking for run status or inspection.
- The user is asking for a specific workflow such as requirements or pr-review.
- The user wants initialization plus another Canon workflow in the same turn;
  initialize first and stop.

## Required Inputs

- current repository context
- optional parent Canon workspace root when `.canon/` should be shared across sibling repos

## Preflight Profile

- Verify `canon` is on PATH. If missing, point to the install guide.
- Do not require `.canon/` to exist before initialization.
- Do not show preflight checks to the user.

## Canon Command Contract

- Canon command: `canon init --non-interactive --ai codex --repo-root <repo-root> [--canon-root <workspace-root>]`
- This skill is Canon-backed immediately and should not invent setup state.
- Execute only the non-interactive command above.
- Do not automatically start another Canon skill or `canon run` in the same turn.

## Expected Output Shape

- concise initialization summary
- whether `.canon/` was created or already present
- no run id
- no run state
- optional next-step suggestions pointing to `$canon-requirements` or `$canon-status`

## Failure Handling Guidance

- If `canon` is missing, show the install command from the shared compatibility reference.
- If the user is outside a Git repo, tell them to switch into the intended repository root before retrying.
- If the user wants a shared parent `.canon/`, tell them to rerun with `--canon-root <workspace-root>`.

## Next-Step Guidance

- After reporting the init result, suggest `$canon-requirements` or
  `$canon-status` only as manual next steps.
- Do not execute the follow-up skill automatically.

## Related Skills

- `$canon-requirements`
- `$canon-status`
