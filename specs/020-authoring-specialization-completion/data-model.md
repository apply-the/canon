# Data Model: Mode Authoring Specialization Completion

## Entity: Remaining Mode Authored Contract

- Purpose: Represents the per-mode mapping from each emitted artifact file to the canonical authored H2 sections that must exist in the source brief for `review`, `verification`, `incident`, and `migration`.
- Core fields:
  - Mode name
  - Artifact file name
  - Canonical heading list
  - Explicit alias list when compatibility support is documented
- Relationships:
  - Defined by `crates/canon-engine/src/artifacts/contract.rs` and this feature's contract document
  - Consumed by renderer logic in `crates/canon-engine/src/artifacts/markdown.rs`
  - Reflected in skills, templates, examples, and docs

## Entity: Authored Section Preservation Rule

- Purpose: Represents the rule that preserves a supplied authored H2 body verbatim in the emitted artifact when the canonical heading is present and non-empty.
- Core fields:
  - Canonical heading
  - Extracted authored body text
  - Target artifact section heading
  - Mode-specific summary/verdict metadata that must remain compatible with gate evaluation
- Relationships:
  - Uses `extract_authored_h2_section()` and/or `render_authored_artifact()`
  - Applies to all emitted artifacts in the four targeted modes

## Entity: Missing Authored Body Marker

- Purpose: Represents the explicit honesty block emitted when a required authored section is missing, blank, or replaced by an unauthorized near match.
- Core fields:
  - Missing canonical heading name
  - Artifact file where the gap appears
  - Reviewer-visible remediation cue
- Relationships:
  - Produced by the markdown renderer
  - Validated by focused renderer, run, and docs-sync tests
  - Referenced in skills, examples, and quickstart negative scenarios

## Entity: Authored Source Handoff

- Purpose: Represents the authored packet text that the orchestrator hands to the renderer for section extraction.
- Core fields:
  - Original authored brief text
  - Derived generation/critique/validation summaries
  - Mode-specific renderer inputs
- Relationships:
  - Produced in `mode_review.rs`, `mode_incident.rs`, and `mode_migration.rs`
  - Consumed by review/verification/incident/migration artifact renderers
  - Must keep authored H2 visibility intact even when summaries are also generated for evidence and result reporting

## Entity: Governance Posture Surface

- Purpose: Represents the mode-specific state and gate semantics that must remain stable while authored-body specialization lands.
- Core fields:
  - Primary artifact title
  - Blocked or approval-gated state trigger
  - Recommendation-only or critique-first posture
  - Gate targets and readiness expectations
- Relationships:
  - Evaluated by gatekeeper logic after artifacts are rendered
  - Verified by run and contract tests
  - Must remain unchanged by this feature except where missing authored bodies now surface more explicitly

## Entity: Release And Documentation Contract Surface

- Purpose: Represents the release-facing surfaces that tell users and maintainers which specialization slice is shipped.
- Core fields:
  - Workspace version
  - Changelog entry
  - Roadmap state
  - Mode-guide language
  - Runtime-compatibility references
- Relationships:
  - Mirrors the implemented runtime and user-facing authored contract
  - Must report `0.20.0` consistently when the rollout is complete

## State And Validation Rules

- Canonical authored headings are required for verbatim preservation unless an alias is explicitly documented.
- A required section that is absent, blank, or replaced by a near-match heading emits `## Missing Authored Body` naming the canonical heading.
- Renderer preservation must operate on the authored packet text, not only on evidence-mixed summary text.
- Review/verification critique posture and incident/migration recommendation-only posture remain unchanged while authored-body fidelity improves.
- Skills, templates, examples, and docs are valid only when they describe the same artifact-to-heading contract as the runtime surfaces.
- Version and roadmap surfaces are valid only when they describe the rollout state that the runtime and tests actually prove.