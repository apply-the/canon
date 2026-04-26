# Decision Log: Mode Authoring Specialization Follow-On

- **D-001**: Keep the feature bounded to `system-shaping`, `implementation`, and `refactor`.
  - Rationale: These three modes are the remaining high-value authored-body gaps that still fit one bounded-impact slice.
  - Consequence: Additional mode rollout remains explicit follow-on work.

- **D-002**: Reuse the existing authored-section renderer helper and missing-body marker.
  - Rationale: The current helper already defines the desired honesty pattern in prior delivered modes.
  - Consequence: Validation must prove the new slice behaves consistently with the earlier specialization pattern.

- **D-003**: Treat canonical H2 headings as the only accepted contract for this slice unless an alias is explicitly documented.
  - Rationale: Honest contract enforcement is more important than permissive heading recovery.
  - Consequence: Tests must cover absent, blank, and near-match headings.

- **D-004**: Restore original authored brief text to the renderer for `implementation` and `refactor`.
  - Rationale: Preservation cannot work reliably if renderers only receive evidence-mixed summaries.
  - Consequence: The plan must include orchestrator handoff changes alongside renderer changes.

- **D-005**: Keep existing artifact families, gate semantics, and recommendation-only posture unchanged.
  - Rationale: The slice improves packet honesty and discoverability, not runtime governance.
  - Consequence: Validation must include explicit non-regression checks for execution posture and non-target modes.

- **D-006**: Treat skills, templates, examples, and roadmap text as first-class contract surfaces.
  - Rationale: The authored contract is not usable if it only exists in code.
  - Consequence: Docs-sync validation is part of the feature definition, not optional polish.

- **D-007**: Treat recommendation-only posture, non-target mode stability, and explicit missing-body markers as hard non-regression boundaries for implementation.
  - Rationale: The follow-on slice is allowed to deepen authored-body fidelity only if it does not reopen execution governance or silently change unaffected modes.
  - Consequence: Baseline and closeout validation must explicitly check execution posture, missing-body honesty, and non-target-mode stability.

- **D-008**: Keep the existing runtime artifact family in `contract.rs` and concentrate foundational changes in renderer scaffolding and authored-source handoff.
  - Rationale: The implementation and refactor contract sections were already aligned with the planned canonical headings, so the real foundational gap lived in `markdown.rs` and `mode_change.rs`.
  - Consequence: Phase 2 validation must prove no contract schema drift while renderer and handoff behavior change underneath it.

- **D-009**: Make the authored H2 contract explicit in skills, starter templates, and worked examples rather than relying on inline labels or legacy discovery wording.
  - Rationale: US1 fails if users still have to read Rust or reverse-engineer packet files to know what to author.
  - Consequence: Docs-sync validation must enforce canonical H2 parity across embedded skills, mirrored skills, starter templates, and worked examples.

- **D-010**: Treat missing canonical authored sections as both an emitted honesty marker and a real gate blocker for the affected packet.
  - Rationale: The runtime should expose the missing section explicitly in the artifact while still preventing downstream approval or completion semantics from advancing on incomplete authored input.
  - Consequence: US2 validation must cover both the emitted `## Missing Authored Body` marker and the blocked gate/result classification for incomplete packets.

- **D-011**: Close feature 019 as a bounded follow-on slice and narrow the remaining authoring-specialization roadmap to `review`, `verification`, `incident`, and `migration`.
  - Rationale: The delivered work now covers the remaining high-value execution and shaping modes without implying the broader specialization rollout is done.
  - Consequence: Roadmap, guide, changelog, and runtime-compatibility references must all state the delivered slice and the residual scope explicitly.