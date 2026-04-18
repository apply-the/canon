# Quickstart: Codex Skills Frontend for Canon

## Goal

Make Canon feel native in Codex without hiding the underlying CLI runtime.

## Preconditions

- `canon` is installed and on `PATH`
- you are inside the target repository
- the repository is the intended working context for the Canon run

## First Supported Flow

1. Invoke `$canon-init` if `.canon/` has not been initialized yet.
2. Invoke `$canon-requirements` with a bounded problem statement.
3. Use `$canon-status` to confirm the run state.
4. Use `$canon-inspect-invocations` or `$canon-inspect-evidence` to inspect
   what Canon actually did.

Canonical MVP skill sequence:

- `$canon-init`
- `$canon-requirements`
- `$canon-status`
- `$canon-inspect-invocations`
- `$canon-inspect-evidence`

This is the MVP slice for the skills frontend. It should be proven in Codex
before broader skill refinement continues.

## Higher-Value Follow-Up Flows

- `$canon-brownfield` for governed change planning in a live codebase
- `$canon-pr-review` for governed diff review
- `$canon-inspect-artifacts` for emitted file paths
- `$canon-approve` and `$canon-resume` for invocation-gated runs

## Available-Now Skills

- `$canon-init`
- `$canon-requirements`
- `$canon-status`
- `$canon-inspect-invocations`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
- `$canon-brownfield`
- `$canon-pr-review`

## Discoverable But Not Runnable End To End

- `$canon-discovery`
- `$canon-system-shaping`
- `$canon-architecture`
- `$canon-implementation`
- `$canon-refactor`
- `$canon-review`
- `$canon-incident`
- `$canon-migration`
- `$canon-verification` as intentionally limited

## Expected UX Pattern

- skill returns a concise summary
- skill points back to the Canon run id
- skill points to the next inspection or unblock action
- user can always verify the result in `.canon/`

## Modeled-Only Example

If a user invokes `$canon-architecture` or `$canon-review` in phase 1, the
skill should:

- state that the mode is modeled-only
- avoid starting a Canon run
- explain what Canon already knows about the mode
- explain what is still missing before runnable delivery
- point to the nearest supported alternative when one exists

## Failure Example

If `canon` is not installed, the skill should:

- state that Canon CLI is missing
- show the supported install path from README
- stop without pretending a workflow has started

## Discoverability Reminder

All Canon skills should exist and be discoverable through `$`, even when they
are not runnable yet. Trust comes from explicit support-state messaging, not
from hiding the taxonomy.
