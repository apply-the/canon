# Data Model: Runnable Skill Interaction and Ref-Safe Input Binding

## Overview

This patch does not add persistent product data. It adds an explicit design
model for the transient data a runnable skill must handle before invoking
Canon.

## Entities

### RunnableSkillInteraction

Represents the current active interaction for one executable Canon skill.

| Field | Type | Notes |
| --- | --- | --- |
| `skill_name` | string | e.g. `canon-pr-review` |
| `command_name` | string | e.g. `pr-review`, `requirements`, `status` |
| `phase` | enum | `collecting`, `preflight-failed`, `ready`, `canon-executing`, `canon-returned` |
| `slots` | list of `TypedInputSlot` | Required and optional slots for the active skill |
| `preserved_valid_values` | map | Ephemeral values retained only within the current interaction |

**Validation rules**:

- exists only for one active runnable-skill interaction
- must not persist into `.canon/`, repo files, or cross-skill memory

### TypedInputSlot

Represents one named input requirement for a runnable skill.

| Field | Type | Notes |
| --- | --- | --- |
| `slot_id` | string | `owner`, `risk`, `zone`, `run-id`, `input-path`, `base-ref`, `head-ref` |
| `kind` | enum | `OwnerField`, `RiskField`, `ZoneField`, `RunIdInput`, `FilePathInput`, `RefInput` |
| `required` | boolean | Whether the slot must be valid before Canon starts |
| `raw_value` | string | User-provided form |
| `normalized_value` | string | Canonical accepted form, if validation succeeds |
| `status` | enum | `missing`, `valid`, `invalid` |
| `error_class` | optional string | Filled when validation fails |
| `semantic_retry_value` | optional string | Human-oriented retry rendering |
| `cli_retry_value` | optional string | Exact Canon CLI rendering |

**Validation rules**:

- slot identity is stable within a skill
- normalized value must never change command intent without explicit user
  confirmation

### RefPairInput

Represents the ordered base/head requirement for `canon-pr-review`.

| Field | Type | Notes |
| --- | --- | --- |
| `base_ref` | `TypedInputSlot` | `kind=RefInput` |
| `head_ref` | `TypedInputSlot` | `kind=RefInput` |
| `pair_status` | enum | `missing`, `valid`, `invalid` |
| `pair_error_class` | optional string | `missing-input` or `malformed-ref-pair` |

**Validation rules**:

- both sides must be present before Canon runs
- each side validates as a ref, not a path
- pair order is preserved
- if both normalized refs are identical, pair validation fails as
  `malformed-ref-pair`

### PreflightOutcome

Represents the deterministic result returned by the shared preflight layer.

| Field | Type | Notes |
| --- | --- | --- |
| `status` | string | e.g. `ready`, `invalid-ref`, `missing-file` |
| `code` | integer | Shared shell/PowerShell status code |
| `phase` | string | `preflight` or `canon-execution` |
| `failed_slot` | optional string | Slot id for targeted retry |
| `failed_kind` | optional string | Slot kind or subtype |
| `message` | string | Specific actionable explanation |
| `action` | string | Exact next correction or install step |
| `normalized_values` | map | Normalized slots returned on success |

### RetryGuidance

Represents what the skill shows after a preflight failure or normalization.

| Field | Type | Notes |
| --- | --- | --- |
| `preserved_slots` | list of strings | Slots that remain valid |
| `requested_slot` | string | Slot or pair being requested next |
| `semantic_prompt` | string | Human-oriented prompt |
| `cli_form` | string | Exact Canon CLI form accepted by binding |
| `phase_label` | string | `before Canon execution` or `inside Canon execution` |

## Input-Class Rules

### OwnerField

- Validation: non-empty after trim
- Normalization: trim outer whitespace only
- Retry rendering:
  - semantic: `owner <VALUE>`
  - CLI: `--owner <VALUE>`

### RiskField

- Validation: must match Canon runtime risk tokens:
  - `low-impact`
  - `bounded-impact`
  - `systemic-impact`
  - plus runtime-recognized aliases such as `LowImpact`
- Normalization: convert accepted alias to canonical hyphenated token
- Retry rendering:
  - semantic: `risk bounded-impact`
  - CLI: `--risk bounded-impact`

### ZoneField

- Validation: must match Canon runtime zone tokens:
  - `green`
  - `yellow`
  - `red`
  - plus runtime-recognized aliases such as `Yellow`
- Normalization: convert accepted alias to canonical lowercase token
- Retry rendering:
  - semantic: `zone yellow`
  - CLI: `--zone yellow`

### RunIdInput

- Validation: non-empty, then repo-local existence under `.canon/runs/<RUN_ID>`
- Normalization: trim only
- Retry rendering:
  - semantic: `run id <RUN_ID>`
  - CLI: `--run <RUN_ID>`

### FilePathInput

- Validation: existing path relative to repo root or explicit absolute path
- Normalization: preserve repo-relative form when possible
- Retry rendering:
  - semantic: `input path docs/brief.md`
  - CLI: `--input docs/brief.md`

### RefInput

- Validation: Git ref resolution only
- Accepted raw forms in this patch:
  - `HEAD`
  - `refs/heads/<name>`
  - short local branch names resolving to `refs/heads/<name>`
- Rejected raw forms in this patch:
  - `refs/remotes/*`
  - remote-like `<remote>/<branch>` without matching local branch resolution
- Normalization:
  - `HEAD` stays `HEAD`
  - short local names normalize to `refs/heads/<name>`
  - explicit local refs stay explicit
- Retry rendering:
  - semantic: `base ref master`
  - CLI: `--input refs/heads/master`

## Relationships

- `RunnableSkillInteraction` owns many `TypedInputSlot` values.
- `canon-pr-review` builds one `RefPairInput` from two `TypedInputSlot`
  instances.
- `PreflightOutcome` references one or more `TypedInputSlot` values through
  `failed_slot` and `normalized_values`.
- `RetryGuidance` is derived from `PreflightOutcome` plus the preserved slots in
  `RunnableSkillInteraction`.

## State Transitions

```text
collecting
  -> preflight-failed
  -> ready
ready
  -> canon-executing
canon-executing
  -> canon-returned
preflight-failed
  -> collecting
```

Rules:

- transition to `canon-executing` only after every required slot is valid
- returning to `collecting` preserves only slots that were already valid in the
  same interaction
