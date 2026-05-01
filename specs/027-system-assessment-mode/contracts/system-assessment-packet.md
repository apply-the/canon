# Contract: System Assessment Packet

## Purpose

Define the first-slice artifact bundle and authored-body expectations for the
`system-assessment` mode.

## Artifact Bundle

The mode MUST emit the following artifact files under the run-local
`system-assessment/` artifact directory:

- `assessment-overview.md`
- `coverage-map.md`
- `asset-inventory.md`
- `functional-view.md`
- `component-view.md`
- `deployment-view.md`
- `technology-view.md`
- `integration-view.md`
- `risk-register.md`
- `assessment-evidence.md`

## Required Sections By Artifact

- `assessment-overview.md`
  - `Summary`
  - `Assessment Objective`
  - `Stakeholders`
  - `Primary Concerns`
  - `Assessment Posture`
- `coverage-map.md`
  - `Summary`
  - `Stakeholder Concerns`
  - `Assessed Views`
  - `Partial Or Skipped Coverage`
  - `Confidence By Surface`
- `asset-inventory.md`
  - `Summary`
  - `Assessed Assets`
  - `Critical Dependencies`
  - `Boundary Notes`
  - `Ownership Signals`
- `functional-view.md`
  - `Summary`
  - `Responsibilities`
  - `Primary Flows`
  - `Observed Boundaries`
  - `Confidence Notes`
- `component-view.md`
  - `Summary`
  - `Components`
  - `Responsibilities`
  - `Interfaces`
  - `Confidence Notes`
- `deployment-view.md`
  - `Summary`
  - `Execution Environments`
  - `Network And Runtime Boundaries`
  - `Deployment Signals`
  - `Coverage Gaps`
- `technology-view.md`
  - `Summary`
  - `Technology Stack`
  - `Platform Dependencies`
  - `Version Or Lifecycle Signals`
  - `Evidence Gaps`
- `integration-view.md`
  - `Summary`
  - `Integrations`
  - `Data Exchanges`
  - `Trust And Failure Boundaries`
  - `Inference Notes`
- `risk-register.md`
  - `Summary`
  - `Observed Risks`
  - `Risk Triggers`
  - `Impact Notes`
  - `Likely Follow-On Modes`
- `assessment-evidence.md`
  - `Summary`
  - `Observed Findings`
  - `Inferred Findings`
  - `Assessment Gaps`
  - `Evidence Sources`

## Behavioral Rules

- The packet MUST remain as-is and MUST NOT render architecture decisions,
  recommendations, or change plans as if they were already selected.
- ISO 42010 language MUST be visible in coverage and viewpoint reporting.
- Missing or weak evidence MUST lower confidence or surface assessment gaps
  rather than being written as facts.
- When a required authored section is absent or empty, the corresponding
  artifact MUST emit the explicit `## Missing Authored Body` marker.
- When the authored section exists, the renderer MUST preserve it verbatim.