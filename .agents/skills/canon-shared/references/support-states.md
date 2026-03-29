# Canon Skill Support States

Use these labels exactly. Do not invent softer wording.

## `available-now`

- Say the workflow is backed by the real Canon CLI.
- Say what command or inspection surface it drives.
- Show the next inspection or unblock skill when useful.
- Do not imply more than the CLI actually returns.

## `modeled-only`

- Lead with: `This Canon workflow is modeled, but not runnable end to end yet.`
- Explain what Canon already knows about the mode today, not generic aspiration.
- Explain what is still missing before the mode becomes runnable.
- Point to the nearest runnable skill only when it is honest and useful.
- Never emit a run id, approval result, evidence summary, or fake Canon state.

## `intentionally-limited`

- Lead with: `This Canon workflow is intentionally limited in the current release.`
- Explain the current boundary.
- Name the closest usable alternative.
- Never promise a hidden or upcoming subcommand as if it already exists.

## `experimental`

- Lead with: `This Canon workflow is experimental.`
- Explain the unstable boundary and what is not guaranteed.
- Do not present the behavior as default or production-ready.

## Cross-State Rules

- All Canon skills remain discoverable through `$`.
- Trust comes from explicit support-state messaging, not from hiding skills.
- Support-state wrappers never fabricate Canon runtime output.
- Runnable wrappers may only claim behavior that maps cleanly to the current Canon CLI.
