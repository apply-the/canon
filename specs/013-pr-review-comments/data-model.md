# Data Model: PR Review Conventional Comments

## Overview

This feature extends the existing `pr-review` packet with one additive,
reviewer-facing artifact. The runtime continues to derive review state from the
existing `ReviewPacket` and `ReviewFinding` models, while introducing a stable
mapping layer for Conventional Comments output.

## Entities

### Conventional Comment Kind

- **Purpose**: Classifies a reviewer-facing entry using the Conventional
  Comments vocabulary.
- **Allowed Values**: `praise`, `nitpick`, `suggestion`, `issue`, `todo`,
  `question`, `thought`, `chore`
- **Validation Rules**:
  - Only the allowed values may appear in the emitted artifact.
  - Must-fix findings may map only to `issue`, `todo`, or `question` in the
    first slice.
  - Note findings may map only to `praise` or `thought` in the first slice.

### Conventional Comment Entry

- **Purpose**: Represents one reviewer-facing comment in the new artifact.
- **Derived From**: Exactly one persisted `ReviewFinding` record in the first
  slice.
- **Fields**:
  - `kind`: `ConventionalCommentKind`
  - `title`: short reviewer-facing summary
  - `details`: rationale/body text
  - `changed_surfaces`: file or surface list carried from the finding
  - `source_categories`: originating `FindingCategory` values
- **Validation Rules**:
  - Must reference at least one changed surface or explicitly state that no
    changed surfaces were detected.
  - Must remain attributable to exactly one persisted finding in the first
    slice.

### Comment Kind Mapping

- **Purpose**: Encodes the deterministic transformation from `ReviewFinding` to
  `ConventionalCommentEntry`.
- **Inputs**:
  - `FindingSeverity`
  - `FindingCategory`
  - `ReviewFinding.title`
  - `ReviewFinding.details`
  - `ReviewFinding.changed_surfaces`
- **Outputs**:
  - `ConventionalCommentKind`
  - reviewer-facing wording guidance
- **Validation Rules**:
  - The same input finding must map to the same kind across repeated runs.
  - Mapping must preserve severity intent.
  - The first-slice mapping table is:
    - `MustFix + BoundaryCheck` -> `issue`
    - `MustFix + ContractDrift` -> `issue`
    - `MustFix + MissingTests` -> `todo`
    - `MustFix + DecisionImpact` -> `question`
    - `Note + DuplicationCheck` -> `praise`
    - any other `Note` -> `thought`

## Relationships

- A `ReviewPacket` contains many `ReviewFinding` records.
- One `ReviewFinding` yields one Conventional Comments entry in the first
  slice.
- `review-summary.md` remains the canonical packet summary; the Conventional
  Comments artifact is additive and derived from the same packet.

## State and Publish Implications

- The new artifact does not change `RunState` semantics.
- Approval remains driven by unresolved must-fix findings in the existing
  summary/gate path.
- Publish behavior extends the existing `docs/reviews/prs/<RUN_ID>/` packet
  with one more markdown artifact.
