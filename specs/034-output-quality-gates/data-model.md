# Data Model: Output Quality Gates

## Entity: Output Quality Assessment

- **Purpose**: Represents the shared classification Canon computes for a packet
  or inspect result based on authored evidence and explicit gaps.
- **Fields**:
  - `mode`: Canon mode or mode family being assessed
  - `posture`: `structurally-complete`, `materially-useful`, or `publishable`
  - `materially_closed`: whether the packet is intentionally bounded to one
    viable path
  - `evidence_signals`: positive reasons supporting the posture
  - `downgrade_reasons`: explicit reasons that prevent a stronger posture
  - `honesty_markers`: preserved missing-body, missing-evidence, blocked, or
    unresolved signals affecting the posture

## Entity: Output Quality Posture

- **Purpose**: Defines the reviewer-visible meaning of each classification.
- **Fields**:
  - `status`: canonical posture label
  - `reader_interpretation`: what a maintainer should infer immediately
  - `minimum_support`: authored support needed before the posture is allowed
  - `forbidden_shortcuts`: conditions that must not qualify for the posture

## Entity: Quality Evidence Signal

- **Purpose**: Captures one positive indicator that the packet is more than
  structurally complete.
- **Fields**:
  - `signal_type`: boundary, rationale, tradeoff, preservation, verdict,
    contradiction, or closure
  - `source_basis`: authored input, emitted artifact, or derived runtime helper
  - `message`: reviewer-visible explanation
  - `strength`: informative or qualifying

## Entity: Quality Downgrade Reason

- **Purpose**: Captures one explicit reason a packet was downgraded.
- **Fields**:
  - `reason_type`: missing-authored-body, missing-evidence, placeholder-density,
    unresolved-gap, missing-disposition, or weak-support
  - `severity`: cautionary or blocking
  - `surface`: inspect, summary, or artifact location where it should remain
    visible
  - `message`: reviewer-visible explanation

## Entity: Inspect Quality Summary

- **Purpose**: Extends the clarity inspect result with output-quality posture
  and the reasons behind it.
- **Fields**:
  - `mode`: targeted mode
  - `summary`: bounded description of the authored surface
  - `reasoning_signals`: existing reasoning posture statements
  - `output_quality`: the shared posture classification
  - `evidence_signals`: concrete support for the current posture
  - `downgrade_reasons`: explicit gaps blocking a stronger posture
  - `recommended_focus`: next step to improve packet quality safely

## Entity: Mode Quality Contract

- **Purpose**: Describes how one mode family maps shared posture rules onto its
  own authored sections, summaries, and artifact surfaces.
- **Fields**:
  - `mode_family`: planning, execution, or assessment
  - `required_support`: authored signals expected for stronger posture
  - `closure_rules`: how materially closed decisions qualify
  - `fallback_policy`: how weak or missing content must be rendered honestly
  - `summary_mapping`: how posture appears in runtime mode results

## Entity: Release Surface Anchor

- **Purpose**: Represents one repository-facing surface that must align with
  `0.34.0` and the delivered quality contract.
- **Fields**:
  - `path`: repository path
  - `surface_type`: manifest, compatibility reference, skill mirror, guide,
    roadmap, or changelog
  - `expected_version`: `0.34.0`
  - `expected_posture_note`: the quality-gate behavior that surface must
    describe

## Relationships

- Each **Output Quality Assessment** belongs to one **Mode Quality Contract**.
- Each **Output Quality Assessment** contains zero or more **Quality Evidence
  Signals** and zero or more **Quality Downgrade Reasons**.
- Each **Inspect Quality Summary** includes one **Output Quality Assessment**.
- Each **Release Surface Anchor** is validated by the structural or logical
  checks recorded in `validation-report.md`.