# Research: Industry-Standard Artifact Shapes With Personas

## Decision 1: Encode personas in skill guidance, not in runtime manifests

- **Decision**: Introduce the persona layer in the first-slice skill guidance,
  mirrored skills, and operator-facing docs rather than adding new runtime
  metadata under `.canon/`.
- **Rationale**: The user explicitly asked for personas on the skill side.
  Keeping persona semantics in authoring guidance satisfies that goal while
  preserving the existing persistence model and avoiding unnecessary runtime
  schema changes.
- **Alternatives considered**:
  - Add persona metadata to run manifests or `context.toml` now.
  - Document personas only in the roadmap without binding them to skill text.

## Decision 2: Limit the first slice to requirements, architecture, and change

- **Decision**: Prove the feature on `requirements`, `architecture`, and
  `change` before widening to other modes.
- **Rationale**: These modes already anchor the roadmap item, have strong
  existing packet contracts, and offer the highest leverage for testing the
  combined shape-plus-persona contract without broadening the blast radius.
- **Alternatives considered**:
  - Roll out personas to every supported mode in one pass.
  - Start with review-oriented modes instead of the roadmap's primary planning
    and decision surfaces.

## Decision 3: Treat personas as guidance-only and preserve Canon honesty rules

- **Decision**: Persona guidance may change voice, emphasis, and intended
  audience fit, but it may not override missing-authored-body markers, evidence
  posture, approval semantics, or recommendation-only boundaries.
- **Rationale**: Canon's product value depends on explicit truthfulness when the
  authored brief is incomplete. The feature must improve readability without
  weakening governance.
- **Alternatives considered**:
  - Allow persona language to synthesize missing narrative when the authored
    brief is thin.
  - Treat persona seniority as implicit authority in packet recommendations.

## Decision 4: Keep embedded skill source as the source of truth

- **Decision**: Update `defaults/embedded-skills/.../skill-source.md` first and
  keep `.agents/skills/.../SKILL.md` synchronized as the materialized mirror.
- **Rationale**: This matches the repository's existing authoring model and the
  current skill validation workflow.
- **Alternatives considered**:
  - Hand-edit only the mirrored skills.
  - Move first-slice persona guidance into a new shared abstraction before the
    slice is proven.
