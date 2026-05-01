# Research: Cross-Mode Reasoning Evidence And Clarity Expansion

## Decision 1: Extend the existing clarity pipeline instead of inventing a second reasoning service

- **Decision**: Reuse `inspect_clarity`, `ClarityInspectSummary`, and
  mode-specific reasoning-signal helpers as the primary runtime surface for the
  feature instead of adding a new parallel scoring or quality-evaluation
  subsystem.
- **Rationale**: The repository already has a working clarity contract for
  `requirements`, `discovery`, and `supply-chain-analysis`. Extending that
  path keeps the behavior inspectable, reuse-friendly, and consistent with the
  existing CLI output contract.
- **Alternatives considered**:
  - Add a new repository-wide reasoning score independent from clarity.
  - Keep reasoning posture in skill text only and avoid runtime expansion.

## Decision 2: Treat file-backed modes and diff-backed review as different intake families

- **Decision**: Expand pre-run clarity inspection across the remaining
  file-backed governed modes, but keep `pr-review` on its diff-backed intake
  flow and align it through runtime review posture rather than forcing it into
  the same authored-input clarity contract.
- **Rationale**: `pr-review` does not read `canon-input/` and should not
  pretend to. The correct shared behavior is honest reasoning evidence in the
  review packet and summary, not a fake file-backed clarity path.
- **Alternatives considered**:
  - Force every mode, including `pr-review`, into one identical clarity
    interface.
  - Leave `pr-review` entirely out of the feature, even for summary and packet
    honesty alignment.

## Decision 3: Tighten placeholder-heavy renderers into explicit gap or closure language

- **Decision**: Replace or tighten generic placeholder fallback prose,
  especially in backlog-style packet surfaces, so emitted artifacts signal
  missing authored reasoning or closure findings instead of reading like
  authored content.
- **Rationale**: Generic fallback text is the most concrete source of the
  “template compiler” perception. The feature should improve honesty at the
  renderer boundary, not just in skill instructions.
- **Alternatives considered**:
  - Leave existing fallback prose in place and only downgrade summaries.
  - Remove fallback sections entirely and skip artifact emission when authored
    content is missing.

## Decision 4: Use shared runtime posture rather than mode-by-mode one-off wording

- **Decision**: Implement reasoning-evidence posture through shared engine
  paths such as clarity helpers, summarizers, renderer helpers, and gate or
  readiness logic before touching mode-specific prompt wording.
- **Rationale**: The product problem is cross-cutting. Fixing it mode by mode
  would produce drift and increase the number of PRs and regressions.
- **Alternatives considered**:
  - Patch only the highest-risk skills such as `requirements` and `review`.
  - Add bespoke rules independently inside every mode implementation.

## Decision 5: Keep runtime and authoring-surface synchronization inside one 033 delivery

- **Decision**: Deliver the runtime contract, mirrored skills, templates,
  examples, docs, version bump, changelog, coverage, `cargo clippy`, and
  `cargo fmt` as one end-to-end feature rather than a runtime slice followed by
  a separate authoring slice.
- **Rationale**: The user explicitly requested that 033 not be split. The
  runtime fix would be incomplete if the authoring UX still encouraged template
  filling.
- **Alternatives considered**:
  - Keep the roadmap's runtime and authoring work as separate candidate
    deliveries.
  - Defer docs, versioning, and validation closeout until after runtime code
    lands.