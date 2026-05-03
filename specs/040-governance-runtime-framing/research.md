# Research: Governance Runtime Framing

## Decision 1: Frame Canon as the governed packet runtime

- **Decision**: Use "governed packet runtime for AI-assisted engineering" as the primary product framing.
- **Rationale**: This wording keeps Canon anchored to bounded runs, approvals, evidence, artifacts, and publishable packets without implying higher-level orchestration ownership.
- **Alternatives considered**:
  - "product entrypoint": rejected because it becomes ambiguous when Canon sits under an orchestrator such as Synod.
  - "local-first method engine": retained as supporting language only, because it is true but less specific about Canon's runtime role.

## Decision 2: Keep the governance adapter as the machine-facing boundary

- **Decision**: Document `canon governance capabilities|start|refresh --json` as the machine-facing boundary around the same runtime rather than as a separate subsystem.
- **Rationale**: The current runtime already exposes stable lifecycle, approval, and packet-readiness fields; the missing piece is a dedicated integration surface that external tools can read without scraping human CLI prose.
- **Alternatives considered**:
  - Fold all adapter details into README and mode docs only: rejected because the integration boundary stays too diffuse.
  - Add Synod-specific mapping to Canon core docs: rejected because it would blur product ownership boundaries.

## Decision 3: Treat release alignment as part of the feature, not aftercare

- **Decision**: Include explicit version bump, changelog, roadmap cleanup, and validation evidence in the same macrofeature.
- **Rationale**: Public framing changes are not credible unless the repository advertises one coherent post-feature state.
- **Alternatives considered**:
  - Leave version or roadmap cleanup to a follow-up slice: rejected because the user explicitly asked for one end-to-end feature with top output quality.

## Decision 4: Add Rust guardrail tests instead of relying on manual docs review only

- **Decision**: Use Rust-based documentation guardrail tests to keep the new framing and governance adapter guide synchronized with the existing docs surface.
- **Rationale**: The repo already uses Rust tests to protect documentation and contract coherence; reusing that pattern preserves auditability and makes drift visible in CI.
- **Alternatives considered**:
  - Manual documentation review only: rejected because it would weaken the artifact-first and layered-verification posture.

## Decision 5: Document the current `pr-review` adapter boundary honestly

- **Decision**: Treat `pr-review` as an explicit current-boundary note in the adapter guide instead of fabricating a first-class JSON example.
- **Rationale**: The current `v1` governance request envelope is built around workspace-relative document refs for file-backed modes. It does not yet expose dedicated diff-ref binding fields for `pr-review`, so the docs should keep that limit explicit instead of inventing support.
- **Alternatives considered**:
  - Invent a synthetic `pr-review` request example: rejected because it would misrepresent the current contract.
  - Omit `pr-review` entirely: rejected because external orchestrator readers still need to know where the present boundary sits.