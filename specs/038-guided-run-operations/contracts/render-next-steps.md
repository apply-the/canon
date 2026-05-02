# Contract: Render Next Steps

## Purpose

Keep the shared Bash and PowerShell `render-next-steps` helpers aligned with the
runtime operator-guidance contract.

## Shared Rules

- `Recommended Next Step:` is always rendered first.
- `Possible Actions:` is rendered whenever valid follow-ups remain.
- Text must preserve the active run id and any real approval target.
- Output must never suggest invalid transitions for the current run state.
- The shell and PowerShell scripts must produce byte-for-byte equivalent text
  after normalizing line endings.

## Required Profiles

### `status-completed`

- Recommended text may explicitly say `None` when the result is already
  readable.
- Possible actions include:
  - direct primary-artifact open when available
  - `canon inspect artifacts`
  - `canon inspect evidence` only as optional lineage follow-up

### `gated` or `status-gated`

- Recommend inspection before approval.
- Include approval follow-up only when the real Canon target is present.
- Mention `canon resume` only after approval or as a contingent follow-up,
  never as the first move.

### `approval-recorded`

- Recommend `canon resume` only when continuation is still valid.
- Offer `canon status` as confirmation follow-up.

### `inspect-artifacts`

- Recommend direct packet review and keep evidence or status as secondary
  follow-ups.

### `resumed`

- Describe the new run state honestly.
- Do not retain stale approval instructions.

## Validation Requirements

- `tests/render_next_steps.rs` must cover all modified profiles.
- Both script copies under `.agents/skills` and `defaults/embedded-skills` must
  stay text-identical in behavior.
- Any wording change in these helpers must stay aligned with runtime guidance,
  docs, and action-chip fallback text.