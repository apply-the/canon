# Data Model: Cybersecurity Risk Assessment Mode

## Entity: Security Assessment Brief

- **Purpose**: The authored source packet Canon consumes for a bounded security
  review.
- **Fields**:
  - `scope`: named assessment boundary and out-of-scope surfaces
  - `assets`: in-scope systems, services, data stores, or integrations
  - `trust_boundaries`: explicit crossings or isolation boundaries
  - `assumptions`: authored confidence limits and unresolved questions
  - `evidence_sources`: code, config, docs, or run artifacts grounding the packet

## Entity: Asset Inventory Entry

- **Purpose**: Represents one in-scope surface the security packet evaluates.
- **Fields**:
  - `name`: asset or service label
  - `classification`: system, data, credential, process, or integration type
  - `owner`: accountable team or function when known
  - `sensitivity`: qualitative data or impact sensitivity note
  - `dependencies`: adjacent surfaces or trust relationships

## Entity: Trust Boundary

- **Purpose**: Captures a boundary across which data, access, or attacker
  movement changes risk posture.
- **Fields**:
  - `boundary_name`: human-readable label
  - `source_surface`: originating asset or zone
  - `target_surface`: receiving asset or zone
  - `boundary_type`: privilege, network, data, deployment, or ownership boundary
  - `security_assumption`: assumption required for the boundary to hold

## Entity: Threat Entry

- **Purpose**: Represents a concrete threat or attacker-goal hypothesis tied to
  an asset or trust boundary.
- **Fields**:
  - `title`: concise threat label
  - `category`: STRIDE or equivalent threat framing
  - `target`: asset or boundary under threat
  - `attack_path`: summarized adversarial path
  - `impact_summary`: likely consequence if realized
  - `confidence_note`: confidence and uncertainty statement

## Entity: Risk Register Entry

- **Purpose**: Represents one rated finding carried into the packet's risk
  register.
- **Fields**:
  - `risk_id`: durable packet-local identifier
  - `finding`: concise risk statement
  - `likelihood`: qualitative likelihood rating
  - `impact`: qualitative impact rating
  - `owner`: proposed owner when known
  - `status`: proposed remediation or acceptance status
  - `linked_threats`: related threat entries

## Entity: Mitigation Proposal

- **Purpose**: A recommendation-only control or change mapped to one or more
  risks.
- **Fields**:
  - `proposal`: recommended control or change
  - `linked_risks`: associated risk identifiers
  - `implementation_pressure`: urgency or sequencing note
  - `tradeoffs`: cost, complexity, or operational burden
  - `validation_hook`: how the mitigation would later be verified

## Entity: Compliance Anchor

- **Purpose**: Maps packet findings to applicable standards or control families
  without claiming audit completion.
- **Fields**:
  - `reference`: standard, article, or control-family identifier
  - `relevance`: why the anchor applies to the assessment
  - `scope_limit`: why this is guidance rather than certification

## Entity: Validation Scenario

- **Purpose**: Represents one proof point for runtime behavior, rendering,
  publish flow, or release closeout.
- **Fields**:
  - `scenario_type`: positive path, missing-body path, publish path, sync check, or release check
  - `input_condition`: authored completeness, risk posture, or compatibility state
  - `expected_behavior`: packet emission, explicit gap, approval gating, or publish success
  - `evidence_path`: where the result is recorded

## Relationships

- A **Security Assessment Brief** contains multiple **Asset Inventory Entries**,
  **Trust Boundaries**, and authored assumptions.
- Each **Threat Entry** targets one or more **Asset Inventory Entries** or
  **Trust Boundaries**.
- Each **Risk Register Entry** can be derived from one or more **Threat Entries**.
- Each **Mitigation Proposal** maps to one or more **Risk Register Entries**.
- **Compliance Anchors** reference the broader standards context for findings
  and mitigations without changing the recommendation-only runtime posture.
- Each **Validation Scenario** proves one expected behavior across the runtime,
  authoring, publish, or release surfaces.
