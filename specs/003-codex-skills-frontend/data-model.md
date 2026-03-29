# Data Model: Codex Skills Frontend for Canon

## 1. SkillContract

- **Purpose**: canonical definition of a repo-local Canon skill
- **Fields**:
  - `id`: stable skill identifier such as `canon-requirements`
  - `kind`: `ExecutableWrapper` or `SupportStateWrapper`
  - `support_state`: `AvailableNow`, `ModeledOnly`, `IntentionallyLimited`,
    or `Experimental`
  - `default_visibility`: `Prominent` or `DiscoverableStandard`
  - `purpose`: short workflow statement
  - `triggers`: positive trigger boundaries
  - `must_not_trigger`: explicit exclusions
  - `required_inputs`: named user inputs or repo prerequisites
  - `preflight_profile`: link to runtime dependency rules
  - `command_binding`: canonical Canon command or support-state check
  - `output_profile`: link to the expected result shape
  - `next_step_profile`: related inspection, approval, or resume guidance

## 2. PreflightProfile

- **Purpose**: deterministic runtime dependency requirements for a skill
- **Fields**:
  - `requires_cli`: boolean
  - `version_policy`: minimum or expected Canon version
  - `requires_repo_context`: boolean
  - `requires_initialized_runtime`: boolean
  - `required_inputs`: run id, input file, refs, or none
  - `failure_codes`: deterministic mapping to user-facing failure responses

## 3. CommandBinding

- **Purpose**: bind a skill to the real Canon surface it is allowed to drive
- **Fields**:
  - `binding_type`: `CanonCommand` or `SupportStateCheck`
  - `command`: canonical invocation string
  - `output_mode`: `text`, `json`, `yaml`, or `markdown`
  - `runtime_authority`: always `CanonCLI`
  - `creates_run`: boolean

## 4. SupportStatePolicy

- **Purpose**: map support state to visibility and messaging
- **Fields**:
  - `support_state`
  - `default_visibility`
  - `discoverable_via_dollar`
  - `required_label`
  - `known_scope_statement`
  - `missing_capability_statement`
  - `nearest_supported_alternative`

## 5. OutputProfile

- **Purpose**: shape what a skill returns to the user in Codex
- **Fields**:
  - `summary`
  - `run_id` (optional)
  - `state` (optional)
  - `evidence_pointers`
  - `approval_guidance` (optional)
  - `resume_guidance` (optional)
  - `support_state_notice` (optional)
  - `failure_guidance` (optional)

## 6. AmbiguityBoundary

- **Purpose**: preserve clear selection boundaries between overlapping skills
- **Fields**:
  - `primary_skill`
  - `adjacent_skill`
  - `distinguishing_rule`
  - `fallback_rule`

## 7. SharedReference

- **Purpose**: deterministic, repo-local metadata used across many skills
- **Fields**:
  - `path`
  - `kind`: `Compatibility`, `SupportState`, `OutputExample`, `SkillIndex`
  - `owner`: Canon skill layer
  - `consumed_by`: list of skills or scripts
