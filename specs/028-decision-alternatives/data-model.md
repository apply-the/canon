# Data Model: Decision Alternatives, Pattern Choices, And Framework Evaluations

## Entity: Mode Decision Profile

- **Purpose**: Declares how a specific mode expresses alternatives,
  recommendation rationale, and evidence posture in its authored packet.
- **Fields**:
  - `mode`: targeted Canon mode
  - `decision_family`: structural decision analysis or framework evaluation
  - `required_sections`: canonical headings required for the mode
  - `preserved_artifacts`: emitted artifacts that must reflect those headings
  - `gap_behavior`: explicit honesty behavior when required sections are absent

## Entity: Option Candidate

- **Purpose**: Represents one viable alternative being compared in the packet.
- **Fields**:
  - `name`: option label visible to reviewers
  - `category`: structural pattern, framework, platform, coexistence path, or
    replacement path
  - `fit_summary`: short description of why it is viable
  - `pros`: positive attributes grounded in the authored brief
  - `cons`: negative attributes or tradeoffs grounded in the authored brief
  - `rejection_rationale`: why it lost when not selected

## Entity: Decision Evidence Reference

- **Purpose**: Captures one evidence anchor that supports a comparison claim or
  exposes that such support is missing.
- **Fields**:
  - `label`: human-readable evidence label
  - `source_kind`: authored brief, repository artifact, release note,
    registry page, project-health reference, or missing-evidence marker
  - `reference`: path or URL when available
  - `claim_supported`: which option or tradeoff claim the reference supports
  - `confidence_note`: explicit note when the evidence is partial or absent

## Entity: Release Surface Anchor

- **Purpose**: Declares one repository-facing version or documentation surface
  that must stay aligned to `0.28.0`.
- **Fields**:
  - `path`: repository path
  - `surface_type`: manifest, compatibility reference, guide, template,
    example, or changelog
  - `expected_version`: release value required by the slice
  - `validation_owner`: command or review step that confirms the surface

## Entity: Validation Scenario

- **Purpose**: Represents one positive-path, negative-path, or release-surface
  proof for the feature.
- **Fields**:
  - `mode_group`: structural decision modes, framework evaluation modes, or
    release-surface alignment
  - `scenario_type`: positive path, negative path, regression, or release check
  - `input_condition`: authored completeness, closed decision, or evidence-gap
    condition
  - `expected_behavior`: preserved analysis, explicit gap marker, stable
    regression behavior, or synchronized release surface
  - `evidence_path`: where the validation result is recorded

## Relationships

- Each **Mode Decision Profile** belongs to exactly one targeted mode.
- Each **Option Candidate** belongs to one **Mode Decision Profile** and may be
  supported by zero or more **Decision Evidence References**.
- Each **Release Surface Anchor** is validated by one or more **Validation
  Scenarios**.
- Each **Validation Scenario** validates one combination of decision profile,
  evidence posture, and expected honesty behavior.