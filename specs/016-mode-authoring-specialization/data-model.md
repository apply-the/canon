# Data Model: Mode Authoring Specialization

## Entity: Mode Authoring Profile

- **Purpose**: captures the specialization contract for one delivered mode in this slice.
- **Fields**:
  - `mode_name`: one of `requirements`, `discovery`, `change`
  - `artifact_files`: emitted markdown artifacts for the mode
  - `skill_source_path`: embedded skill source location
  - `materialized_skill_path`: `.agents/skills/` mirror path
  - `template_path`: docs template path
  - `example_path`: docs example path
  - `orchestrator_path`: mode-specific orchestrator source path
  - `renderer_entrypoint`: renderer function responsible for the packet

## Entity: Authored Section Contract

- **Purpose**: defines which canonical authored H2 sections the renderer expects and how they map into one artifact.
- **Fields**:
  - `artifact_file`: emitted markdown artifact filename
  - `canonical_headings`: required H2 headings in authored input
  - `optional_aliases`: accepted backward-compatible headings, if any
  - `fallback_policy`: `missing-authored-body`
  - `preservation_rule`: `verbatim-body`

## Entity: Missing Body Policy

- **Purpose**: standardizes how absent authored sections are surfaced.
- **Fields**:
  - `marker_text`: literal markdown heading emitted when content is missing
  - `missing_heading_reference`: reviewer-visible text naming the canonical heading that was absent
  - `missing_condition`: heading absent or body empty after trimming
  - `artifact_behavior`: emit the artifact anyway, but with explicit incompleteness
  - `review_expectation`: reviewers treat marker presence as honest incompleteness, not successful authoring

## Entity: Evidence Pass-Through Contract

- **Purpose**: records which modes require authored `context_summary` to flow through to rendering alongside derived critique or validation summaries.
- **Fields**:
  - `mode_name`: one of the first-slice modes
  - `authored_source`: original input context read from the brief or folder
  - `derived_context`: generated, critique, or validation evidence summaries
  - `combined_render_input`: renderer input that preserves authored body visibility

## Relationships

- One `Mode Authoring Profile` has many `Authored Section Contract` entries.
- One `Mode Authoring Profile` has exactly one `Missing Body Policy`.
- One `Mode Authoring Profile` has exactly one `Evidence Pass-Through Contract`.

## Entity: Example Realism Criteria

- **Purpose**: keeps worked examples strong enough to exercise the authored-body contract rather than acting as placeholder filler.
- **Fields**:
  - `heading_coverage_threshold`: example covers the required canonical headings for the mode packet
  - `substantive_body_rule`: each major heading contains non-placeholder content grounded in a plausible scenario
  - `negative_fixture_derivation`: an incomplete validation fixture is produced by removing one required heading from the worked example
  - `success_condition`: the complete example renders with zero `## Missing Authored Body` blocks