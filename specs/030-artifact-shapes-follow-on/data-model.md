# Data Model: Industry-Standard Artifact Shapes Follow-On

## Entities

### Mode Persona Profile

- **Purpose**: Defines the bounded authored counterpart for one Canon mode.
- **Fields**:
  - `mode`: one of `discovery`, `system-shaping`, or `review`
  - `persona_name`: short authored role label used in skill guidance
  - `intended_audience`: who should be able to read the packet without chat context
  - `presentation_emphasis`: the kinds of framing the persona may strengthen
  - `prohibited_implications`: what the persona must never imply or override

### Artifact Shape Contract

- **Purpose**: Defines the packet shape Canon preserves for one mode.
- **Fields**:
  - `mode`: owning mode
  - `artifact_files`: emitted markdown files for the mode bundle
  - `canonical_sections`: exact authored H2 headings preserved by the renderer
  - `industry_shape`: the reviewer-native or industry-standard framing the
    packet should read like
  - `missing_gap_behavior`: explicit marker behavior when authored sections are
    absent
  - `renderer_surface`: the Rust function that preserves the contract

### Release Alignment Surface

- **Purpose**: Tracks repo-visible versioned surfaces that must align to
  `0.30.0`.
- **Fields**:
  - `path`: repository path
  - `required_version`: `0.30.0`
  - `surface_type`: manifest, lockfile, mirrored runtime reference, doc, or changelog
  - `validation_owner`: test or review surface that detects drift

### Validation Evidence Record

- **Purpose**: Captures how one story proves its contract.
- **Fields**:
  - `story_id`: `US1`, `US2`, `US3`, or `US4`
  - `target_files`: code, doc, or skill files under test
  - `command_or_review`: executable command or independent review step
  - `expected_claim`: what the evidence proves
  - `evidence_location`: `validation-report.md`, test output, or `lcov.info`

## Relationships

- Each **Mode Persona Profile** owns exactly one **Artifact Shape Contract** in
  this slice.
- Each **Artifact Shape Contract** is implemented across one skill source file,
  one mirrored skill file, and one renderer surface.
- Each **Artifact Shape Contract** must be covered by at least one
  **Validation Evidence Record** for positive-path behavior and one for
  negative-path honesty.
- **Release Alignment Surface** records are cross-cutting and apply to the
  whole slice, especially User Story 4.

## Contract Mapping

| Mode | Persona | Industry Shape | Renderer Surface |
|------|---------|----------------|------------------|
| `discovery` | exploratory research lead | Opportunity Solution Tree seed plus JTBD-flavored discovery brief | `render_discovery_artifact()` |
| `system-shaping` | bounded system designer | domain-map plus structural-options packet | `render_system_shaping_artifact()` |
| `review` | skeptical packet reviewer | findings-first review bundle | `render_review_artifact()` |