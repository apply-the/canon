# Decision Log: Industry-Standard Artifact Shapes With Personas

## D-001: Keep the first slice to three modes

- **Status**: Accepted
- **Decision**: Limit implementation to `requirements`, `architecture`, and
  `change`.
- **Rationale**: These modes are the roadmap's highest-leverage authoring and
  decision surfaces and provide enough coverage to prove the feature without
  widening the runtime surface.
- **Consequences**: Other modes remain explicitly deferred and must not receive
  partial persona support by implication.

## D-002: Put persona semantics in skill guidance first

- **Status**: Accepted
- **Decision**: Implement persona guidance in embedded skill source, mirrored
  skills, and operator-facing docs before considering runtime metadata.
- **Rationale**: The user request is explicitly about personas on skills or
  where they belong, and this route preserves the current runtime model.
- **Consequences**: The feature can ship without `.canon/` schema changes, but
  future work may revisit whether persona metadata should become runtime-visible.

## D-003: Preserve explicit honesty markers

- **Status**: Accepted
- **Decision**: `## Missing Authored Body`, evidence gaps, and approval/risk
  posture remain authoritative even when persona guidance is active.
- **Rationale**: Canon's governance value depends on visible honesty when
  authored input is incomplete.
- **Consequences**: Renderer and contract tests must include negative-path
  coverage proving the persona layer does not fabricate completeness.

## D-004: Accept the existing renderer and contract as sufficient for the first slice

- **Status**: Accepted
- **Decision**: Keep `crates/canon-engine/src/artifacts/markdown.rs` and
  `crates/canon-engine/src/artifacts/contract.rs` unchanged for this feature
  and prove first-slice support through focused validation instead.
- **Rationale**: Existing requirements, architecture, and change coverage
  already preserves the intended authored sections, emits explicit missing-body
  markers, and keeps the current approval and evidence semantics intact.
- **Consequences**: The delivered slice closes through skill guidance, docs,
  and evidence capture rather than a Rust runtime or `.canon/` schema change.

## D-005: Label the roadmap mapping as roadmap vision

- **Status**: Accepted
- **Decision**: Keep the broader mode-to-shape and mode-to-persona mapping in
  `ROADMAP.md`, but label it as roadmap vision and scope the delivered first
  slice separately.
- **Rationale**: The roadmap should describe the intended direction without
  implying that deferred modes already ship persona support.
- **Consequences**: Future slices can expand coverage without blurring the
  boundary of the currently delivered `requirements`, `architecture`, and
  `change` slice.
