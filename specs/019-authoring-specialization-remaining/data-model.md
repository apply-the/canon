# Data Model: Mode Authoring Specialization Follow-On

## Entity: Mode Authored-Body Contract

- Purpose: Represents the per-mode mapping from emitted artifact file to the canonical authored H2 sections required in the source brief.
- Core fields:
  - Mode name
  - Artifact file name
  - Canonical heading list
  - Approved alias list when explicitly supported
- Relationships:
  - Defined by the artifact contract in `crates/canon-engine/src/artifacts/contract.rs`
  - Consumed by renderer logic in `crates/canon-engine/src/artifacts/markdown.rs`
  - Reflected in skills, templates, and worked examples

## Entity: Authored Section Preservation Rule

- Purpose: Represents the rendering rule that preserves authored section bodies verbatim when the canonical heading is present and non-empty.
- Core fields:
  - Canonical heading
  - Source body text
  - Target artifact section heading
  - Alias compatibility behavior
- Relationships:
  - Uses `render_authored_artifact()` and `extract_authored_h2_section()`
  - Applies to `system-shaping`, `implementation`, and `refactor` packet artifacts in this slice

## Entity: Missing Authored Body Marker

- Purpose: Represents the explicit honesty block emitted when a required authored section is missing or empty.
- Core fields:
  - Missing canonical heading name
  - Artifact file where the gap appears
  - Reviewer-visible remediation cue
- Relationships:
  - Produced by renderer logic
  - Validated by focused renderer and run tests
  - Documented in skills, examples, and quickstart negative scenarios

## Entity: Authored Brief Handoff

- Purpose: Represents the source authored text passed from the orchestrator layer to the markdown renderer for section extraction.
- Core fields:
  - Original brief text
  - Derived evidence summary text
  - Mode-specific render call inputs
- Relationships:
  - Produced in orchestrator services
  - Consumed by `render_system_shaping_artifact()`, `render_implementation_artifact()`, and `render_refactor_artifact()`
  - Must preserve authored H2 visibility even when summary/evidence text is also generated

## Entity: Documentation Contract Surface

- Purpose: Represents the user-facing authored-contract description across skills, templates, examples, and docs.
- Core fields:
  - Mode-specific skill source
  - Mirrored `.agents` skill file
  - Starter template
  - Worked example
  - Guide and roadmap references
- Relationships:
  - Mirrors the Mode Authored-Body Contract
  - Supports P1 discoverability without source-code inspection

## State And Validation Rules

- Canonical authored headings are required for verbatim preservation unless an alias is explicitly documented.
- A required section that is absent, blank, or replaced by a near-match heading emits `## Missing Authored Body` naming the canonical heading.
- Renderer preservation and missing-body behavior must operate on the original authored brief, not only on evidence-mixed summary text.
- Execution posture, gate semantics, publish destinations, and non-target modes remain outside the state changes introduced by this feature.
- Skills, templates, examples, and tests are valid only when they describe the same artifact-to-heading contract as the runtime surfaces.