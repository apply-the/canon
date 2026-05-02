# Data Model: Guided Run Operations And Review Experience

## Overview

Feature `038` does not introduce a new persistence family. It tightens the
derived operator guidance Canon exposes from existing runtime facts.

## Entities

### 1. Operator Runtime Facts

- **Purpose**: the canonical, already-known run-state inputs used to derive
  operator guidance.
- **Source fields**:
  - `state`
  - `approval_targets`
  - `blocked_gates`
  - `artifact_paths`
  - evidence-bundle presence
  - `mode_result`
  - `run_id`
  - `blocking_classification`
- **Constraints**:
  - facts must remain Canon-backed, not inferred from docs or expected packet
    shapes
  - `approval_targets` remain the authority for approval-oriented actions
  - `blocked_gates` remain the authority for artifact-blocked or remediation
    flows

### 2. Readable Result Surface

- **Purpose**: the result-first summary already exposed by `ModeResultSummary`.
- **Fields**:
  - `headline`
  - `artifact_packet_summary`
  - `execution_posture`
  - `primary_artifact_title`
  - `primary_artifact_path`
  - `primary_artifact_action`
  - `result_excerpt`
  - `action_chips`
- **Constraints**:
  - the primary artifact target must come from Canon-backed runtime output
  - the readable result remains the happy path for completed runs
  - action chips remain optional progressive enhancement

### 3. Recommended Next Action

- **Purpose**: the single best follow-up Canon recommends from the current run
  state.
- **Current fields**:
  - `action`
  - `rationale`
  - `target` (optional)
- **Constraints**:
  - at most one recommended action may exist
  - the recommended action must preserve the current run context
  - approval or resume must never be recommended when Canon has not exposed the
    required run-state facts

### 4. Possible Action List

- **Purpose**: the ordered textual follow-ups valid from the current run state.
- **Expected semantics**:
  - first entry is the strongest non-mandatory drill-down or remediation action
  - later entries remain valid but lower-priority follow-ups
  - entries may point to artifact inspection, evidence inspection, approval,
    resume, status refresh, or direct primary-artifact open depending on state
- **Constraints**:
  - every entry must remain valid for the active run state
  - chip-backed actions must mirror one textual possible action or the
    recommended action exactly
  - `Possible Actions:` remains mandatory even when chips are present

### 5. Action Chip Contract

- **Purpose**: structured host affordances that preserve the same Canon-backed
  follow-up meaning as the text contract.
- **Fields**:
  - `id`
  - `label`
  - `skill`
  - `intent`
  - `prefilled_args`
  - `required_user_inputs`
  - `visibility_condition`
  - `recommended`
  - `text_fallback`
- **Constraints**:
  - chips are progressive enhancement only
  - `Approve generation...` requires a real `RUN_ID` and `TARGET`
  - `Resume run` appears only when continuation is still valid
  - `Inspect evidence` should outrank approval when no readable packet exists

## Relationships

- Operator Runtime Facts derive both the Recommended Next Action and the
  Possible Action List.
- The Readable Result Surface can change the guidance ordering by making packet
  review the honest first move.
- Action Chips mirror the text contract rather than define a new one.
- The CLI markdown renderer and shared next-step scripts are separate renderers
  over the same logical operator guidance.

## State Rules

- **Completed + readable result**:
  - recommended next action may be absent or explicit `None`
  - possible actions should remain inspection-oriented only
- **AwaitingApproval + approval targets + readable packet**:
  - recommend packet review first
  - approval remains available but secondary
- **AwaitingApproval + no approval targets**:
  - recommend `resume`
  - approval actions must disappear
- **Blocked + readable packet**:
  - recommend artifact review or remediation-oriented inspection
  - do not imply approval is valid
- **Blocked + no readable packet + evidence exists**:
  - recommend evidence inspection

## Derived Integrity Rules

- `recommended_next_action` must never contradict the ordered possible actions.
- `action_chips[*].recommended = true` may only appear when that chip mirrors
  the same move as the recommended next step.
- `primary_artifact_action` and `open-primary-artifact` remain coherent: both
  targets must point to the same Canon-backed artifact path.
- Any roadmap, README, changelog, or helper-script narrative must describe the
  same operator guidance ordering the runtime actually emits.