# Data Model: Requirements PRD Publishing And Chat Publish Skill

## Requirements Publication Bundle

- **Purpose**: Represents the full published requirements packet for one completed run.
- **Fields**:
  - `run_id`: Canon run identifier used for traceability.
  - `mode`: Always `requirements` for this slice.
  - `destination`: Published repository path where the packet is copied.
  - `section_artifacts`: Existing sectional markdown files such as `problem-statement.md`, `constraints.md`, and `decision-checklist.md`.
  - `consolidated_prd`: The additive `prd.md` artifact built from the same authoritative packet content.
  - `packet_metadata`: Existing `packet-metadata.json` with source-artifact lineage.
- **Relationships**:
  - Includes one `Consolidated PRD`.
  - Includes multiple `Section Artifact` entries.
  - Is referenced by publish summaries and metadata.
- **Validation rules**:
  - The bundle remains publishable only when the run is already publishable under existing Canon rules.
  - `consolidated_prd` cannot replace or omit any existing section artifact from the bundle.

## Consolidated PRD

- **Purpose**: A single readable markdown document for requirements-mode users who want a product-facing packet view.
- **Fields**:
  - `title`: Human-readable PRD title.
  - `summary`: High-level framing copied from the requirements packet.
  - `sections`: Ordered PRD sections derived from authored requirements bodies and missing-body markers.
  - `source_sections`: Traceable mapping to canonical authored requirement headings.
- **Relationships**:
  - Belongs to one `Requirements Publication Bundle`.
  - Derives from the same authored brief and evidence inputs as the sectional artifacts.
- **Validation rules**:
  - Missing authored sections must remain explicit in the consolidated output.
  - Section order must stay stable across renders and publishes.

## Chat Publish Skill

- **Purpose**: Repo-local skill package that exposes the existing `canon publish` contract to chat-first users.
- **Fields**:
  - `name`: Canonical skill name matching the directory.
  - `description`: One-sentence trigger guidance.
  - `support_state`: Availability and visibility guidance.
  - `instructions`: Publish command usage, run-id requirements, destination behavior, and gate reminders.
  - `mirror_source`: Embedded skill source path for `canon init --ai ...` flows.
- **Relationships**:
  - Mirrors between `.agents/skills/canon-publish/` and `defaults/embedded-skills/canon-publish/`.
  - Refers to the CLI publish command and requirements publication bundle.
- **Validation rules**:
  - Must pass repo skill validation.
  - Must not imply publish works for incomplete or approval-gated runs.