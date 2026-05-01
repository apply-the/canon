# Decision Log: Remaining Industry-Standard Artifact Shapes

## D-001: Reuse the existing target-mode renderer and contract surfaces

- **Decision**: Deliver the remaining rollout by extending the existing
  `implementation`, `refactor`, and `verification` skill plus renderer and
  artifact-contract surfaces instead of introducing a new packet-shape
  abstraction layer.
- **Rationale**: The repository already preserves canonical authored sections
  for these modes, so the bounded work is to improve the authored shape fit,
  persona guidance, docs, and validation rather than widen the runtime model.

## D-002: Keep persona guidance advisory only

- **Decision**: Persona language may improve packet voice, audience fit, and
  critique posture but may not override required sections, evidence posture,
  missing-body markers, or downstream approval semantics.
- **Rationale**: Canon's governance value depends on visible honesty when the
  authored input is incomplete.

## D-003: Make release alignment part of the feature

- **Decision**: Include the `0.31.0` version bump, impacted docs plus
  changelog, touched-Rust-file coverage, `cargo clippy`, and `cargo fmt` as
  explicit deliverables of the slice.
- **Rationale**: This repository treats release-facing docs and validation
  evidence as part of the shipped contract.

## User Story 1 Decisions

### D-004: Shape implementation as bounded delivery planning, not generic execution prose

- **Decision**: Author `implementation` as a delivery lead packet that reads
  like a task-mapped implementation plan with contract-test intent,
  implementation notes, and bounded framework evaluation when the authored
  brief presents a real choice.
- **Rationale**: The remaining value is readability for delivery work, not a
  new execution surface or a weaker governance contract.

### D-005: Keep implementation decision closure explicit when no real option set exists

- **Decision**: If the authored implementation brief is materially closed to
  one stack or delivery path, the packet states that the decision is bounded
  instead of fabricating an options matrix.
- **Rationale**: Canon should not pretend there is more decision flexibility
  than the authored material actually supports.

## User Story 2 Decisions

### D-006: Make refactor read like a preserved-behavior matrix plus rationale record

- **Decision**: Treat `refactor` as a preservation-first maintenance artifact
  with explicit invariant versus mechanism mapping and structural rationale.
- **Rationale**: This makes the packet useful to maintainers without weakening
  the no-feature-addition posture.

### D-007: Keep refactor scope pressure explicit

- **Decision**: Any authored signal that risks turning the refactor into
  feature work must remain visible as scope pressure rather than being
  normalized into the packet as acceptable expansion.
- **Rationale**: The refactor contract is only trustworthy when preserved
  behavior stays inspectable.

## User Story 3 Decisions

### D-008: Make verification claims-first and evidence-native

- **Decision**: Push verifier-native claims, evidence, independence posture,
  and unresolved findings into the canonical verification sections while
  keeping blocked or unsupported outcomes explicit.
- **Rationale**: Verification packets are more reusable when evidentiary status
  is concrete, but Canon still needs explicit honesty when support is weak.

### D-009: Preserve current verification honesty semantics

- **Decision**: Keep existing missing-evidence, unresolved-finding, and
  missing-body behavior intact while improving packet shape readability.
- **Rationale**: The current renderer already encodes the governance posture;
  this slice should improve fit, not lower skepticism.

## User Story 4 Decisions

### D-010: Lock `0.31.0` alignment through the current doc, skill, and test surface

- **Decision**: Treat the existing doc regressions, `tests/skills_bootstrap.rs`,
  focused targeted tests, and workspace quality gates as the enforceable
  release-alignment surface for `0.31.0`.
- **Rationale**: The repository already uses those surfaces to detect drift, so
  the most durable closeout is to keep them aligned instead of inventing a new
  one-off release test.

### D-011: Repair shared contract drift discovered during release validation instead of waiving the gates

- **Decision**: When final validation exposed shared drift in
  `crates/canon-engine/src/artifacts/contract.rs` and
  `crates/canon-engine/src/orchestrator/service/summarizers.rs`, repair those
  surfaces in place using the existing contract and runtime tests as the source
  of truth instead of deferring the fixes outside feature 031.
- **Rationale**: `cargo llvm-cov` and `cargo nextest` are release gates for the
  shipped tree. Leaving compile-blocking or contract-blocking drift unresolved
  would have made the 031 release artifacts claim readiness that the workspace
  could not actually prove.

### D-012: Treat LCOV percentages and direct executable test runs as one coverage evidence set

- **Decision**: Use `cargo llvm-cov --workspace --all-features --lcov` as the
  line-coverage artifact for changed engine sources and record direct
  executable evidence for changed `tests/*.rs` files that the selected LCOV run
  does not emit.
- **Rationale**: The final `lcov.info` captured the changed runtime sources but
  omitted the changed integration-test source files entirely. Recording both
  LCOV data and targeted test execution keeps the closeout honest without
  inventing coverage numbers that the chosen artifact does not contain.