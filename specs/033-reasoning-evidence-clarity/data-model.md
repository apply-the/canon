# Data Model: Cross-Mode Reasoning Evidence And Clarity Expansion

## Entity: Mode Reasoning Profile

- **Purpose**: Declares how one governed mode exposes reasoning evidence,
  honest gaps, and clarity posture.
- **Fields**:
  - `mode`: targeted Canon mode
  - `intake_family`: file-backed clarity, diff-backed review, or runtime-only
    packet posture
  - `reasoning_signals`: required signal categories for that mode
  - `gap_markers`: explicit honesty signals that remain authoritative
  - `summary_posture`: how run summaries distinguish grounded vs shallow output

## Entity: Clarity Inspect Summary

- **Purpose**: Represents the durable inspect result returned before a run.
- **Fields**:
  - `mode`: targeted mode
  - `summary`: bounded description of the authored input surface
  - `source_inputs`: concrete authored inputs read by Canon
  - `requires_clarification`: whether missing context or open questions still
    block confident use
  - `missing_context`: explicit missing-context findings
  - `clarification_questions`: targeted questions Canon wants answered
  - `reasoning_signals`: explicit statements describing the current reasoning
    posture
  - `recommended_focus`: next action for the maintainer

## Entity: Reasoning Signal

- **Purpose**: Captures one inspectable statement about reasoning strength or
  weakness in an authored packet.
- **Fields**:
  - `signal_type`: grounded, shallow, materially-closed, contradiction, or
    evidence-gap
  - `source_basis`: authored input, packet artifact, diff, or derived runtime
    observation
  - `message`: reviewer-visible explanation
  - `severity`: informational, cautionary, or blocking

## Entity: Honesty Marker

- **Purpose**: Represents one explicit signal that Canon must preserve instead
  of papering over missing support.
- **Fields**:
  - `marker_kind`: missing-authored-body, missing-evidence, blocked,
    unsupported, unresolved-findings, or closure-limited
  - `artifact_surface`: artifact or summary surface where it appears
  - `trigger_condition`: authored-section absence, weak evidence, contradiction,
    or bounded closure
  - `required_reader_outcome`: what the reviewer must understand immediately

## Entity: Fallback Packet Surface

- **Purpose**: Identifies a rendered artifact section that currently uses
  generic fallback prose and must be tightened by this feature.
- **Fields**:
  - `mode`: owning mode
  - `artifact_path`: emitted artifact file name
  - `fallback_type`: generic placeholder, generic sequencing, generic anchor,
    or generic comparison
  - `replacement_posture`: explicit missing-body, closure finding, or grounded
    authored preservation

## Entity: Release Surface Anchor

- **Purpose**: Declares one repository-facing surface that must align to
  `0.33.0` and the delivered reasoning contract.
- **Fields**:
  - `path`: repository path
  - `surface_type`: manifest, compatibility reference, skill mirror, guide,
    template, example, roadmap, or changelog
  - `expected_version`: required release value
  - `expected_behavior_note`: what the surface must say about reasoning
    evidence or honest gaps

## Relationships

- Each **Mode Reasoning Profile** owns zero or more **Reasoning Signals** and
  zero or more **Honesty Markers**.
- Each **Clarity Inspect Summary** belongs to one **Mode Reasoning Profile**.
- Each **Fallback Packet Surface** belongs to one **Mode Reasoning Profile**
  and resolves into one or more **Honesty Markers** or preserved authored
  sections.
- Each **Release Surface Anchor** is validated by one or more structural or
  logical validation scenarios recorded in `validation-report.md`.