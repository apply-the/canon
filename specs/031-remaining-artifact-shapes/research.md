# Research: Remaining Industry-Standard Artifact Shapes

## Decision 1: Reuse the existing markdown renderer and artifact contracts

- **Decision**: Deliver the remaining rollout by reusing the existing
  `render_implementation_artifact()`, `render_refactor_artifact()`, and
  `render_verification_artifact()` hooks in
  `crates/canon-engine/src/artifacts/markdown.rs` and the existing mode
  contract definitions in `crates/canon-engine/src/artifacts/contract.rs`
  instead of introducing a new packet-shape abstraction.
- **Rationale**: The target modes already have canonical file names and exact
  H2 preservation behavior. The remaining value is to tighten the authored
  packet shape and persona contract and prove it with focused tests, not to
  widen the rendering architecture.
- **Alternatives considered**:
  - Introduce a new mode-to-shape registry in Rust: rejected because the
    renderer and contract surfaces already encode the canonical section
    contracts and the feature is bounded to three existing modes.
  - Limit the slice to docs only: rejected because the user explicitly asked
    for implementation plus validation, and renderer-backed coverage is needed
    to keep the packet-shape contract enforceable.

## Decision 2: Treat skill source, mirrors, templates, and examples as one authoring surface

- **Decision**: Update the source-of-truth skill files under
  `defaults/embedded-skills/`, keep the mirrored `.agents/skills/` files in
  lockstep, and align the corresponding templates and worked examples inside
  the same slice.
- **Rationale**: The repository ships embedded skill source, AI-facing mirrors,
  input templates, and examples as part of the contract. Release alignment and
  `scripts/validate-canon-skills.sh` already assume those surfaces stay
  synchronized.
- **Alternatives considered**:
  - Edit only `.agents/skills/`: rejected because those mirrors are derived and
    would drift from the embedded source of truth.
  - Edit only the docs and examples: rejected because runtime validation would
    no longer prove the shipped packet contract.

## Decision 3: Keep release alignment and validation closeout inside the feature boundary

- **Decision**: Include the `0.31.0` version bump, impacted docs plus
  changelog closeout, coverage for touched Rust files, `cargo clippy`, and
  `cargo fmt` as first-class deliverables of the slice.
- **Rationale**: This repository treats release-facing docs, mirrored runtime
  compatibility references, and validation evidence as part of the shipped
  contract rather than optional cleanup.
- **Alternatives considered**:
  - Defer the version bump and docs to a later release-only patch: rejected
    because it would leave the artifact-shape contract partially delivered and
    poorly auditable.

## Decision 4: Bound the rollout to implementation, refactor, and verification

- **Decision**: Deliver the remaining artifact-shape work only for
  `implementation`, `refactor`, and `verification` and explicitly avoid
  widening the slice to unrelated roadmap items.
- **Rationale**: The roadmap now points to these as the remaining high-value
  follow-on modes. Closing them in one bounded slice completes the current
  feature family without reopening packaging, new-mode, or publish-surface
  work.
- **Alternatives considered**:
  - Split the work into three smaller feature slices: rejected because the
    shared authoring, docs, release-alignment, and validation surfaces would be
    duplicated with little additional risk reduction.
  - Extend the slice to more release or roadmap work: rejected because that
    would mix separate contracts into one feature and weaken validation focus.

## Decision 5: Preserve each target mode's existing governance semantics

- **Decision**: Let shape guidance improve packet readability for the targeted
  audience while preserving mode-specific honesty rules: bounded or closed
  decision language for `implementation`, preserved-behavior posture for
  `refactor`, and unresolved or unsupported evidence posture for
  `verification`.
- **Rationale**: The feature succeeds only if the packet becomes more readable
  without becoming less trustworthy.
- **Alternatives considered**:
  - Allow persona guidance to infer stronger closure: rejected because it would
    undermine Canon's explicit gap reporting and evidence semantics.