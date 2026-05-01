# Research: Industry-Standard Artifact Shapes Follow-On

## Decision 1: Reuse the existing markdown renderer preservation hooks

- **Decision**: Deliver the follow-on slice by reusing the existing
  `render_discovery_artifact()`, `render_system_shaping_artifact()`, and
  `render_review_artifact()` hooks in
  `crates/canon-engine/src/artifacts/markdown.rs` instead of creating a new
  renderer abstraction.
- **Rationale**: The target modes already have canonical file names and exact
  H2 preservation behavior. The follow-on value is to tighten the authored
  packet shape and persona contract and prove it with focused tests, not to
  widen the rendering architecture.
- **Alternatives considered**:
  - Introduce a new mode-to-shape registry in Rust: rejected because the
    renderer already encodes the canonical section contracts and the feature is
    bounded to three existing modes.
  - Limit the slice to docs only: rejected because the user explicitly asked
    for implementation plus validation, and renderer-backed coverage is needed
    to keep the packet-shape contract enforceable.

## Decision 2: Treat skill source and mirror pairs as the authoring surface

- **Decision**: Update the source-of-truth skill files under
  `defaults/embedded-skills/` and keep the mirrored `.agents/skills/` files in
  lockstep inside the same feature.
- **Rationale**: The repository ships both the embedded skill source and the
  AI-facing mirrors as part of the contract. Release alignment and
  `scripts/validate-canon-skills.sh` already assume those surfaces stay
  synchronized.
- **Alternatives considered**:
  - Edit only `.agents/skills/`: rejected because those mirrors are derived and
    would drift from the embedded source of truth.
  - Edit only `defaults/embedded-skills/`: rejected because repository tests
    and release surfaces validate the mirrored skills as shipped artifacts.

## Decision 3: Keep release alignment inside the feature boundary

- **Decision**: Include `0.30.0` version alignment, docs plus changelog
  closeout, coverage for touched Rust files, `cargo clippy`, and `cargo fmt`
  as first-class deliverables of the slice.
- **Rationale**: This repository treats release-facing docs, mirrored runtime
  compatibility references, and validation evidence as part of the shipped
  contract rather than optional cleanup.
- **Alternatives considered**:
  - Defer the version bump and docs to a later release-only patch: rejected
    because it would leave the skill and artifact-shape contract partially
    delivered and poorly auditable.

## Decision 4: Bound the rollout to discovery, system-shaping, and review

- **Decision**: Deliver the follow-on only for `discovery`, `system-shaping`,
  and `review` and explicitly defer `implementation`, `refactor`, and
  `verification` shape expansion.
- **Rationale**: The three selected modes broaden the roadmap materially while
  remaining close to already-existing renderer and test surfaces. This keeps
  the slice small enough to validate without reopening unrelated modes.
- **Alternatives considered**:
  - Include `verification` in the same pass: rejected because the roadmap can
    still evolve independently there, and widening now risks mixing separate
    contracts into one feature.