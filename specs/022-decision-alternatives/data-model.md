# Data Model: Decision Alternatives, Pattern Choices, And Framework Evaluations

## Entity: Mode Decision Profile

- **Purpose**: Declares how a specific mode expresses alternatives and decision
  rationale in its authored packet.
- **Fields**:
  - `mode`: targeted Canon mode
  - `decision_family`: structural decision or framework evaluation
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

## Entity: Evaluation Dimension

- **Purpose**: Captures one decision driver or comparison axis used across
  options.
- **Fields**:
  - `dimension_name`: performance, operational burden, migration cost,
    maintainability, ecosystem health, or similar driver
  - `applicability`: which modes or packet family use the dimension
  - `evidence_basis`: authored reasoning, cited repo evidence, or explicit gap
  - `weighting_note`: optional note explaining importance relative to other
    drivers

## Entity: Mode Persona Profile

- **Purpose**: Declares the bounded authored counterpart a mode should emulate.
- **Fields**:
  - `mode`: the Canon mode the persona applies to
  - `counterpart`: architecture-decision, system-design, change-planning,
    delivery-lead, migration-lead, lead or staff reviewer, adversarial
    verifier, or incident commander counterpart
  - `intended_audience`: who the packet is written for
  - `critique_posture`: how the persona frames risks, tradeoffs, and gaps
  - `authority_boundaries`: what the persona may not claim or override

## Entity: Validation Scenario

- **Purpose**: Represents one positive-path, negative-path, or release-surface
  proof for the feature.
- **Fields**:
  - `mode_group`: structural decision modes, framework evaluation modes, or
    persona-only guidance modes
  - `scenario_type`: positive path, negative path, sync check, or release check
  - `input_condition`: authored completeness or evidence-gap condition
  - `expected_behavior`: preserved analysis, explicit gap marker, persona fit,
    or synchronized release surface
  - `evidence_path`: where the validation result is recorded

## Relationships

- Each **Mode Decision Profile** belongs to exactly one targeted mode.
- Each **Option Candidate** belongs to one **Mode Decision Profile** and is
  evaluated across one or more **Evaluation Dimensions**.
- Each **Mode Persona Profile** can support one **Mode Decision Profile**, or
  stand alone for guidance-only modes in this slice.
- Each **Validation Scenario** validates one combination of decision profile,
  persona profile, and expected honesty behavior.
