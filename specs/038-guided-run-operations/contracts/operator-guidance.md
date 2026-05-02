# Contract: Operator Guidance

## Purpose

Define the Canon-backed operator-guidance contract that must stay consistent
across runtime JSON, CLI markdown output, and skill-facing helper text.

## Inputs

- Canon-backed run state
- approval targets
- blocked gates
- readable artifact paths
- evidence-bundle presence
- `mode_result` and primary artifact metadata when present
- real run id

## Required Outcomes

1. Canon emits at most one `recommended_next_action` for the active run state.
2. Canon emits an ordered `Possible Actions:` surface that contains only valid
   follow-ups for the current run.
3. If action chips are present, each chip’s `text_fallback` must mirror either:
   - the recommended next step exactly, or
   - one valid entry from `Possible Actions:` exactly.
4. If a readable packet exists for a gated or blocked run, Canon prefers packet
   review before approval or deeper evidence inspection.
5. If no readable packet exists but evidence does, Canon prefers evidence
   inspection over approval-oriented language.
6. If no approval targets remain and the run is still `AwaitingApproval`, Canon
   recommends resume and removes stale approval actions.

## Required Text Sections

### Completed readable result

- `## Result` remains the primary section.
- `## Recommended Next Step` may be omitted or rendered as explicit `None`.
- `Possible Actions:` must still expose the valid drill-down actions.

### Blocked or gated result

- `## Result` and `## Blockers` remain visible when Canon has a readable packet.
- `## Recommended Next Step` remains mandatory.
- `Possible Actions:` remains mandatory.
- Approval-oriented actions appear only when Canon exposed a real approval
  target.

## Action Ordering Rules

### Approval-gated with readable packet

1. Inspect packet or primary artifact
2. Inspect evidence if policy rationale is still needed
3. Approve the real target
4. Resume or status only after approval is recorded

### Blocked with readable packet

1. Inspect packet or primary artifact
2. Inspect evidence when runtime lineage matters
3. Refresh status only after follow-up work changes expectations

### Awaiting approval with no remaining targets

1. Resume
2. Status refresh if confirmation is needed

### Completed readable result

1. Open primary artifact when available
2. Inspect artifacts for the wider packet
3. Inspect evidence only for lineage or policy rationale

## Non-Negotiable Honesty Rules

- Do not recommend approval when no real approval target exists.
- Do not recommend artifact inspection as the primary next step when Canon did
  not emit readable artifact paths.
- Do not show resume while approval targets are still outstanding.
- Do not make chips the only carrier of guidance.
- Do not imply automatic follow-up execution from any guidance surface.