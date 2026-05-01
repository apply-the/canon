# Data Model: System Assessment Mode

## Entity: System Assessment Brief

- **Purpose**: The authored source packet Canon consumes for a bounded as-is
  system review.
- **Fields**:
  - `assessment_objective`: why the current-state assessment is needed
  - `stakeholders`: the named readers or decision-makers for the packet
  - `concerns`: the architecture concerns that drive the assessment
  - `scope`: included repositories, directories, services, or boundaries
  - `out_of_scope`: known exclusions
  - `evidence_sources`: source files, configs, diagrams, or docs that ground
    the packet

## Entity: Assessment Surface

- **Purpose**: One bounded repository or runtime surface included in the
  assessment.
- **Fields**:
  - `name`: human-readable label
  - `surface_type`: code path, deployment config, external integration, data
    flow, document, or runtime artifact
  - `path_or_reference`: repo path or authored reference
  - `owner`: owning team or role when known
  - `coverage_status`: assessed, partial, skipped, or unobservable

## Entity: Coverage Entry

- **Purpose**: Records ISO 42010-style coverage for one stakeholder concern or
  view.
- **Fields**:
  - `view_name`: functional, component, deployment, technology, or integration
  - `stakeholder_concern`: the concern the view addresses
  - `coverage_status`: assessed, partial, skipped, or unobservable
  - `confidence_level`: high, medium, or low
  - `coverage_rationale`: why the current status is justified

## Entity: Asset Inventory Entry

- **Purpose**: Represents one system element or dependency captured by the
  assessment.
- **Fields**:
  - `name`: asset or component label
  - `category`: service, executable, library, datastore, queue, integration,
    environment, or control plane
  - `responsibility`: what the asset appears to do
  - `dependencies`: adjacent assets or integrations
  - `evidence_note`: source grounding for the inventory entry

## Entity: Finding Entry

- **Purpose**: Represents one explicit packet finding classified by certainty.
- **Fields**:
  - `classification`: `observed`, `inferred`, or `assessment-gap`
  - `statement`: concise assessment claim
  - `evidence_refs`: files, configs, or authored references supporting the claim
  - `confidence_level`: high, medium, or low
  - `affected_surfaces`: views or assets the finding applies to

## Entity: View Assessment

- **Purpose**: Captures one emitted view artifact in the packet.
- **Fields**:
  - `view_name`: functional, component, deployment, technology, or integration
  - `summary`: concise overview of the current-state view
  - `primary_elements`: components, flows, or dependencies seen in that view
  - `boundaries`: seams, interfaces, or deployment partitions relevant to the view
  - `confidence_note`: why the view confidence is what it is

## Entity: Risk Register Entry

- **Purpose**: An observed risk or architecture concern surfaced by the
  assessment without implying a follow-on decision is already made.
- **Fields**:
  - `risk_id`: packet-local identifier
  - `risk_statement`: concise current-state concern
  - `trigger`: what evidence or gap caused the risk to be recorded
  - `impact_summary`: why the concern matters
  - `follow_on_mode`: likely next governed mode (`architecture`, `change`,
    `migration`, or `security-assessment`)

## Entity: Validation Scenario

- **Purpose**: Represents one proof point for mode registration, run behavior,
  authored-body honesty, publish behavior, or release closeout.
- **Fields**:
  - `scenario_type`: positive path, missing-body path, invalid-context path,
    publish path, shared-surface sync, or release validation
  - `input_condition`: authored completeness, system-context posture, or shared
    registry state
  - `expected_behavior`: packet emission, blocker, missing-body marker, or
    publish success
  - `evidence_path`: where the result is recorded

## Relationships

- A **System Assessment Brief** declares multiple **Assessment Surfaces** and
  stakeholder concerns.
- Each **Coverage Entry** maps one stakeholder concern to one **View
  Assessment** and one or more **Assessment Surfaces**.
- Each **Asset Inventory Entry** can appear in multiple **View Assessments**.
- Each **Finding Entry** can affect one or more **Coverage Entries** and
  **Asset Inventory Entries**.
- Each **Risk Register Entry** is grounded in one or more **Finding Entries**.
- Each **Validation Scenario** proves one expected behavior across runtime,
  authoring, publish, or release surfaces.