# Data Model: Industry-Standard Artifact Shapes With Personas

## Entity: Mode Persona Profile

- **Purpose**: Declares the authored counterpart a mode should emulate for a
  bounded packet.
- **Fields**:
  - `mode`: the Canon mode the persona applies to
  - `counterpart`: the named role or persona for the packet
  - `intended_audience`: who the packet is written for
  - `critique_posture`: how the persona frames risks, tradeoffs, and gaps
  - `authority_boundaries`: what the persona may not claim or override

## Entity: Artifact Shape Contract

- **Purpose**: Declares the industry-standard packet shape a mode should follow.
- **Fields**:
  - `mode`: the Canon mode the shape applies to
  - `shape_name`: PRD, C4 plus ADR, ADR-style change packet, or later slice
    shape name
  - `required_sections`: canonical authored sections that must remain explicit
  - `preserved_artifacts`: emitted artifacts that must reflect the shape
  - `gap_behavior`: how missing authored content is surfaced

## Entity: Skill Materialization Pair

- **Purpose**: Tracks the relationship between embedded skill source and the
  mirrored AI-facing skill file.
- **Fields**:
  - `source_path`: embedded skill source path
  - `mirror_path`: mirrored skill path under `.agents/skills/`
  - `sync_expectation`: whether the two files must remain byte-aligned or
    semantically aligned for the first slice

## Entity: Validation Scenario

- **Purpose**: Represents one positive-path or negative-path proof for the
  feature.
- **Fields**:
  - `mode`: targeted mode under validation
  - `scenario_type`: positive path, negative path, or regression guard
  - `input_condition`: authored brief completeness or conflict condition
  - `expected_packet_behavior`: shaped output, preserved contract, or explicit
    missing-gap behavior
  - `evidence_path`: where the result is recorded

## Relationships

- Each **Mode Persona Profile** belongs to exactly one first-slice mode in this
  feature.
- Each **Artifact Shape Contract** is paired to one **Mode Persona Profile** for
  the same mode.
- Each **Skill Materialization Pair** must reflect one or more persona and shape
  decisions for a mode.
- Each **Validation Scenario** validates a specific combination of persona,
  shape, and preserved honesty behavior.
