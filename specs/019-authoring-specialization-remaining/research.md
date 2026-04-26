# Research: Mode Authoring Specialization Follow-On

## Decision 1: Keep the slice bounded to the three remaining high-value modes

- Decision: Limit the follow-on feature to `system-shaping`, `implementation`, and `refactor` rather than widening to every unspecialized mode.
- Rationale: These three modes carry the clearest remaining authored-body gap while still sharing one coherent renderer-and-doc synchronization pattern.
- Alternatives considered:
  - Extend every remaining mode in one rollout. Rejected because the contract, docs, and validation surface becomes too broad for one bounded-impact slice.
  - Focus only on `implementation` and `refactor`. Rejected because `system-shaping` still has five packet artifacts that remain summary-driven and would leave the slice visibly incomplete.

## Decision 2: Reuse the existing authored-section helper and missing-body marker

- Decision: Expand the existing `render_authored_artifact()` and `## Missing Authored Body` pattern to the targeted mode artifacts instead of inventing a new renderer path.
- Rationale: The helper already anchors the delivered behavior in `requirements`, `discovery`, `change`, and part of `architecture`; reuse keeps the behavior consistent and lowers regression risk.
- Alternatives considered:
  - Introduce a new mode-specific preservation helper. Rejected because it duplicates behavior already proven in the current renderer.
  - Keep summary-driven rendering and only strengthen docs. Rejected because the feature goal is runtime honesty, not documentation alone.

## Decision 3: Preserve canonical headings as the contract and treat near matches as missing

- Decision: The plan keeps canonical H2 headings as the only authored contract for the new slice, with missing-body markers emitted when a required heading is absent, empty, or replaced by a near match.
- Rationale: The spec explicitly requires honest incompleteness, and silent acceptance of near-match headings would weaken that invariant.
- Alternatives considered:
  - Fuzzy-match similar headings automatically. Rejected because it hides authoring drift and makes the contract harder to review.
  - Add broad alias support for convenience. Rejected because there is no approved alias set for the targeted modes beyond explicitly documented compatibility cases.

## Decision 4: Restore authored brief text to the renderer for execution-heavy modes

- Decision: The implementation plan includes a targeted orchestrator handoff change so `implementation` and `refactor` renderers can extract authored H2 sections from the original brief text rather than from evidence-mixed summaries.
- Rationale: Current `mode_change` flow mixes generated framing and validation text into the string passed to `render_implementation_artifact()` and `render_refactor_artifact()`, which prevents reliable authored-section extraction.
- Alternatives considered:
  - Parse authored sections back out of the mixed evidence summary. Rejected because it is brittle and fights the current data flow.
  - Limit the feature to docs and contract changes until a larger orchestrator rewrite. Rejected because that would leave the core runtime behavior unchanged.

## Decision 5: Keep existing artifact families and execution posture unchanged

- Decision: Preserve current artifact file names, publish destinations, gate semantics, and recommendation-only posture while strengthening authored-body preservation inside the existing packet shapes.
- Rationale: The feature is about contract honesty and authored fidelity, not packet expansion or execution-governance change.
- Alternatives considered:
  - Rename artifacts to reflect the new authored-body emphasis. Rejected because it widens migration burden without improving the core behavior.
  - Change execution posture for `implementation` or `refactor` while touching those modes. Rejected because the spec explicitly bounds that out of scope.

## Decision 6: Treat docs and examples as first-class contract surfaces

- Decision: The feature plan includes synchronized updates across embedded skills, mirrored `.agents` skill files, starter templates, worked examples, mode guidance, and roadmap text.
- Rationale: Users currently discover the contract through skills and examples first; leaving those surfaces unsynchronized would recreate the same ambiguity even if the renderer is fixed.
- Alternatives considered:
  - Update only source code and leave docs follow-up for later. Rejected because the feature's P1 user story depends on contract discoverability without code inspection.
  - Update only embedded skills and rely on materialization later. Rejected because the repository already treats mirrored `.agents` skills as a user-facing surface under validation.