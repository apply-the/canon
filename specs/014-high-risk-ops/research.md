# Research: High-Risk Operational Programs

## R-001: Deliver `incident` and `migration` as one mode-completion feature

- **Decision**: Complete `incident` and `migration` together under one
  high-risk operational feature rather than splitting them into unrelated
  delivery tracks.
- **Rationale**: The roadmap identifies both as the remaining modeled
  operational modes, and both depend on the same governance concerns:
  containment, compatibility, sequencing, fallback credibility, and stronger
  readiness gates than ordinary implementation flows.
- **Alternatives considered**:
  - Deliver `incident` first and defer `migration` to a later roadmap item.
    Rejected because Canon would still have an incomplete operational mode set
    and would likely duplicate the same runtime plumbing twice.
  - Treat `migration` as an extension of `change`. Rejected because explicit
    compatibility and fallback semantics are the core product value of a
    dedicated migration mode.

## R-002: Retain the current artifact family names and deepen them with
explicit contracts

- **Decision**: Keep the artifact families already declared in
  `defaults/methods/incident.toml` and `defaults/methods/migration.toml`, then
  make their required sections and gate expectations explicit.
- **Rationale**: The method metadata already names meaningful artifact shapes
  and publish paths. Renaming them now would widen the blast radius into mode
  discovery, documentation, tests, and future runs without adding governance
  value.
- **Alternatives considered**:
  - Rename the packets around a single shared operational vocabulary.
    Rejected because `incident` and `migration` have overlapping but not
    identical concerns, and the current artifact names already fit the domain.
  - Collapse both modes into a common operational packet. Rejected because it
    would blur containment-driven incident work with compatibility-driven
    migration work.

## R-003: Make missing evidence explicit through block-or-downgrade posture

- **Decision**: High-risk packets will expose missing evidence as explicit
  blockers or downgrade signals rather than filling gaps with optimistic
  readiness language.
- **Rationale**: Operational modes are valuable only if a reviewer can trust
  that the packet tells the truth about blast radius, compatibility, fallback,
  and residual uncertainty. Honest incompleteness is safer than polished
  fiction.
- **Alternatives considered**:
  - Require full completion only and avoid downgrade semantics. Rejected
    because Canon already uses explicit state and gate vocabulary, and some
    operational packets still provide decision value even when not ready to
    advance.
  - Emit best-effort packets without explicit gating consequences. Rejected
    because that weakens governance exactly where the feature is meant to
    strengthen it.

## R-004: Implement incident first, then migration, on top of shared runtime
surfaces

- **Decision**: Sequence implementation as shared runtime hooks first,
  incident-specific surfaces second, then migration using the same completed
  pipeline.
- **Rationale**: `incident` has the smaller gate set and the clearer response
  semantics. Completing it first reduces ambiguity in summarizers, status,
  publish, and packet readability before layering in migration-specific
  compatibility rules.
- **Alternatives considered**:
  - Implement migration first. Rejected because it introduces more gate states
    and compatibility nuances before the common operational plumbing is proven.
  - Implement both modes fully in one undifferentiated pass. Rejected because
    it increases the chance of mixing concerns and obscuring regressions.

## R-005: Replace modeled-only skill wrappers with runnable authored-body
guidance

- **Decision**: When the modes become full-depth, replace the current
  support-state-only `canon-incident` and `canon-migration` skills with
  authored-input guidance for real governed runs.
- **Rationale**: The current skill text explicitly says the modes are not yet
  runnable. Leaving that support-state messaging in place after runtime
  delivery would violate artifact-first honesty and mislead users.
- **Alternatives considered**:
  - Keep the support-state wording and update only the runtime. Rejected
    because docs and skills would then contradict the product.
  - Remove the skills entirely. Rejected because the modes still need guided,
    mode-specific authored-input expectations.