# Data Model: Governance Runtime Framing

## Entity: Runtime Positioning Surface

- **Represents**: The set of user-facing docs that define what Canon is, what it is not, and how a human should approach the product.
- **Fields**:
  - `primary_statement`: the opening identity statement for Canon
  - `non_goals`: explicit statements of what Canon does not claim to be
  - `human_happy_path`: the ordered human-driven flow from init to publish
  - `delivery_line`: the currently advertised release version in public docs
- **Relationships**:
  - Must stay aligned with `Release Alignment Surface`
  - Must remain consistent with `Governance Adapter Guide`

## Entity: Governance Adapter Guide

- **Represents**: The machine-facing integration document for `canon governance`.
- **Fields**:
  - `commands`: capabilities, start, refresh
  - `stable_fields`: status, approval_state, packet_readiness, reason_code, canonical refs
  - `usage_rule`: when to use human CLI vs machine adapter
  - `mode_examples`: representative examples for change, implementation, verification, pr-review
- **Relationships**:
  - Must describe the same runtime as the `Runtime Positioning Surface`
  - Must not contradict the adapter contract already described in existing docs

## Entity: Release Alignment Surface

- **Represents**: The repository files that advertise the delivered feature and version line.
- **Fields**:
  - `workspace_version`: version in Cargo workspace surfaces
  - `readme_delivery_line`: release version mentioned in README
  - `changelog_entry`: delivered feature summary for the release line
  - `roadmap_state`: explicit statement that no active roadmap entries remain
- **Relationships**:
  - Must stay coherent with the implementation artifacts and validation evidence

## Validation Rules

- A runtime positioning surface is invalid if it describes Canon as a generic agent framework or separate orchestration layer.
- A governance adapter guide is invalid if it omits one of the three commands or the stable lifecycle fields.
- A release alignment surface is invalid if version, changelog, README delivery line, and roadmap do not refer to the same delivered feature state.