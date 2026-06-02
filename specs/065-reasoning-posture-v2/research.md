# Research: Governed Reasoning Posture v2

## Decision: Publish a new `governed_reasoning_posture_v2` contract line

- **Decision**: Introduce `governed_reasoning_posture_v2` as a new Canon-owned
  contract line rather than extending `governed_reasoning_posture_v1` in place.
- **Rationale**: The feature intentionally strengthens selector resolution,
  independence semantics, confidence handoff, provenance, coexistence rules,
  and fail-closed validation. Those are semantic changes, not additive wording
  refinements, so they require a new line to preserve the meaning of `v1`.
- **Alternatives considered**:
  - Extend `v1` with more optional fields: rejected because it would force
    consumers to guess which semantics are authoritative.
  - Keep `v1` and publish only migration guidance: rejected because the
    consumer still needs a machine-checkable stronger producer shape.

## Decision: Keep the stable integration doc canonical and the feature-local brief as a mirror

- **Decision**: Continue to publish the normative Canon contract from
  `tech-docs/integration/governed-reasoning-posture-contract.md`, while the
  feature-local contract files under `specs/065-reasoning-posture-v2/contracts/`
  mirror the planned `v2` semantics for review and delivery planning.
- **Rationale**: Downstream consumers need one stable contract path, while the
  feature branch still needs durable design artifacts and machine-checkable
  example definitions close to the spec.
- **Alternatives considered**:
  - Make the feature-local contract brief canonical: rejected because external
    consumers should not depend on a feature directory.
  - Skip feature-local contract artifacts and plan only from the stable doc:
    rejected because planning and review would lose local traceability.

## Decision: Require exactly one profile selector kind per `v2` payload

- **Decision**: Every `governed_reasoning_posture_v2` payload declares exactly
  one selector kind, either `required_profile_family` or
  `required_profile_id`; both-present and neither-present states fail closed.
- **Rationale**: This keeps selector resolution deterministic, avoids
  consumer-side precedence rules, and makes examples and validation harnesses
  straightforward.
- **Alternatives considered**:
  - Allow both when redundant: rejected because redundant dual encoding still
    forces the consumer to decide which field is normative.
  - Allow both with precedence: rejected because precedence rules hide
    ambiguity rather than removing it.

## Decision: Publish explicit active-versus-legacy coexistence semantics

- **Decision**: Canon may publish both `v1` and `v2` only when exactly one line
  is marked active and the other is explicitly marked legacy; there is no
  implicit fallback between lines.
- **Rationale**: Migration needs a bounded dual-line period, but consumers must
  still be able to identify one authoritative contract line and reject mixed or
  ambiguous release surfaces.
- **Alternatives considered**:
  - Atomic cutover with no coexistence: rejected because it would complicate
    staged rollout and cross-repo coordination.
  - Treat `v1` and `v2` as equally active alternatives: rejected because it
    would push contract selection back onto the consumer.

## Decision: Make `confidence_handoff` a mandatory typed block

- **Decision**: Every `v2` payload includes a typed `confidence_handoff` block
  with an explicit state such as `none` or `required`. If state is `required`,
  the block must include the required handoff fields, validation rules,
  evidence or provenance references, and explicit fail-closed behavior.
- **Rationale**: A mandatory block removes ambiguity between intentionally
  absent handoff semantics and accidentally omitted data.
- **Alternatives considered**:
  - Omit the block when no handoff is required: rejected because consumers
    would have to infer intent from absence.
  - Derive handoff behavior from other fields: rejected because the spec is
    explicitly tightening, not relying on implication.

## Decision: Make `provenance` a mandatory typed block

- **Decision**: Every `v2` payload includes a typed `provenance` block with an
  explicit state and stable reference-kind contract; provenance cannot be
  inferred only from the contract line or release metadata.
- **Rationale**: Confidence handoff and fail-closed validation need explicit
  provenance so the consumer can tell whether Canon supplied adequate evidence.
- **Alternatives considered**:
  - Require provenance only when handoff is required: rejected because payload
    absence would still blur intentional minimal provenance and producer error.
  - Treat release metadata as sufficient provenance: rejected because the spec
    needs payload-level validation, not only release-level statements.

## Decision: Model independence as hard minima plus optional guidance

- **Decision**: Every `v2` payload includes a typed `minimum_independence`
  block with required hard-minimum fields and an optional guidance sub-block;
  guidance may only strengthen or elaborate on the minima and cannot weaken or
  replace them.
- **Rationale**: Downstream routing needs one machine-checkable minimum bar,
  while producers may still publish stronger advisory posture without becoming
  the runtime router.
- **Alternatives considered**:
  - Keep a flat object like `v1`: rejected because it cannot separate hard
    requirements from softer guidance.
  - Ban guidance entirely: rejected because the feature explicitly wants a
    stronger-but-bounded advisory layer.

## Decision: Treat machine-checkable examples and release metadata as contract surfaces

- **Decision**: The example corpus and release-facing metadata are part of the
  executable compatibility surface for `v2`.
- **Rationale**: The spec requires valid and invalid example payloads, and the
  current contract tests already couple the stable doc to workspace version and
  metadata surfaces. Excluding either would leave drift undetected.
- **Alternatives considered**:
  - Keep examples as non-binding documentation: rejected because example
    outcomes are explicitly required to be machine-checkable.
  - Treat release metadata as housekeeping outside the feature: rejected
    because stale metadata already produces consumer-visible contract drift.

## Implementation Direction

- Update the stable contract doc to publish `v2` alongside explicit
  active-versus-legacy publication rules.
- Create a feature-local contract packet that captures the `v2` payload shape,
  migration contract, and example inventory before implementation begins.
- Extend the governed reasoning posture contract tests so they validate the new
  contract line, release alignment on Canon `0.64.0`, and the example corpus.
- Keep Boundline runtime ownership explicit by limiting Canon to producer-side
  contract semantics, validation, and release truth surfaces.

## Likely Touchpoints

- `tech-docs/integration/governed-reasoning-posture-contract.md`
- `tests/contract/governed_reasoning_posture_contract.rs`
- `tests/governed_reasoning_posture_contract.rs`
- `Cargo.toml`
- `CHANGELOG.md`
- `README.md`
- `ROADMAP.md`
- `docs/reference/cli.md`
- `assistant/plugin-metadata.json`
- `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`
- `crates/canon-engine/src/domain/publish_profile/authority.rs`
- `crates/canon-engine/src/domain/publish_profile/publication.rs`
- `crates/canon-engine/src/domain/publish_profile/semantic.rs`