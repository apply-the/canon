# Decision Log: Industry-Standard Artifact Shapes Follow-On

## D-001: Reuse the existing target-mode renderer surfaces

- **Decision**: Deliver the follow-on by extending the existing
  `discovery`, `system-shaping`, and `review` skill plus renderer contracts
  instead of introducing a new packet-shape abstraction layer.
- **Rationale**: The repository already preserves canonical authored sections
  for these modes, so the bounded work is to improve the authored shape fit,
  persona guidance, docs, and validation rather than to widen the runtime
  model.

## D-002: Keep persona guidance advisory only

- **Decision**: Persona language may improve packet voice, audience fit, and
  critique posture but may not override required sections, evidence posture,
  missing-body markers, or downstream approval semantics.
- **Rationale**: Canon's governance value depends on visible honesty when the
  authored input is incomplete.

## D-003: Make release alignment part of the feature

- **Decision**: Include the `0.30.0` version bump, impacted docs plus
  changelog, touched-Rust-file coverage, `cargo clippy`, and `cargo fmt` as
  explicit deliverables of the slice.
- **Rationale**: This repository treats release-facing docs and validation
  evidence as part of the shipped contract.

## User Story 1 Decisions

### D-004: Shape discovery as bounded exploratory research, not pseudo-requirements

- **Decision**: Author `discovery` as an exploratory research lead packet that
  reads like an Opportunity Solution Tree plus Jobs-To-Be-Done seed while
  keeping the existing canonical H2 contract unchanged.
- **Rationale**: The follow-on value is readability for ambiguous problem-space
  work, not a new requirements surface or a new renderer contract.

### D-005: Reuse the existing discovery preservation path

- **Decision**: Keep the existing discovery renderer behavior and prove the
  follow-on through updated skill guidance, templates, examples, docs, and
  contract-oriented tests.
- **Rationale**: The current renderer already preserves the required authored
  sections and missing-body honesty for discovery.

## User Story 2 Decisions

### D-006: Make system-shaping read like a domain map plus structural-options packet

- **Decision**: Treat `domain-model.md` as the domain-map spine and the shaping
  architecture outline as the explicit structural-options comparison surface.
- **Rationale**: This makes the packet useful to downstream architects and
  implementers without inventing a new artifact family.

### D-007: Keep the shaping persona bounded to framing and tradeoff language

- **Decision**: The bounded system designer persona may sharpen boundary,
  tradeoff, and rejection logic, but it may not invent architecture detail or
  relax missing-section behavior.
- **Rationale**: The packet still needs to surface weak authored input
  honestly.

## User Story 3 Decisions

### D-008: Make review findings-first without weakening disposition discipline

- **Decision**: Push reviewer-native severity, location, rationale, and
  recommended-change structure into the canonical review sections while keeping
  `## Final Disposition` and accepted-risk handling as the bounded decision
  summary.
- **Rationale**: Review packets are more usable when findings are concrete, but
  Canon still needs explicit closure and evidence posture.

### D-009: Reuse the current review renderer and enforce the contract in docs/tests

- **Decision**: Keep review rendering behavior unchanged and enforce the new
  findings-first shape through skill, template, example, docs, and regression
  coverage.
- **Rationale**: The current review artifact preservation logic already
  supports the required canonical section honesty.

## User Story 4 Decisions

### D-010: Lock `0.30.0` alignment through the current test surface, not a dedicated release doc test

- **Decision**: Treat the current tree's docs regressions plus
  `tests/skills_bootstrap.rs` and full-workspace regression as the release
  alignment guard for 030.
- **Rationale**: The final branch state no longer keeps dedicated README or
  changelog release tests, so the enforceable contract lives in the broader
  authoring-doc and compatibility coverage that remains.

### D-011: Treat touched Rust-file coverage as test-surface evidence for this slice

- **Decision**: Record `cargo llvm-cov` output plus passing targeted test
  binaries as the coverage evidence because the final 030 tree changes only
  test Rust files and no production Rust source files.
- **Rationale**: Workspace `lcov.info` remains the required coverage artifact,
  but test-only files do not carry the same line-reporting semantics as library
  or binary sources.