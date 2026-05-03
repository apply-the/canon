# Decision Log: Governance Runtime Framing

## D-001: Prefer runtime framing over generic entrypoint framing

- **Decision**: Reframe Canon publicly as the governed packet runtime for AI-assisted engineering.
- **Status**: Accepted
- **Rationale**: This communicates Canon's bounded role more clearly than product-entrypoint language once Canon is used beneath a higher-level orchestrator.
- **Consequences**: README, guides, integration docs, and release alignment surfaces must all reflect the same boundary.

## D-002: Document the governance adapter in one dedicated guide

- **Decision**: Add a dedicated integration guide for the governance adapter rather than relying on README and mode-guide fragments alone.
- **Status**: Accepted
- **Rationale**: Orchestrator maintainers need one stable machine-facing document for commands, lifecycle states, and response fields.
- **Consequences**: Rust guardrail tests should protect the new guide from drifting away from README and mode docs.

## D-003: Keep Synod mapping out of Canon core docs

- **Decision**: Keep external orchestrator stage mapping out of Canon core product docs.
- **Status**: Accepted
- **Rationale**: Canon must document its own runtime boundary without absorbing another system's orchestration model.
- **Consequences**: Canon docs can mention external orchestrators generically, but Synod-specific choreography belongs elsewhere.

## D-004: Keep `pr-review` as an explicit adapter-boundary note for now

- **Decision**: Document `pr-review` as a current boundary note in the governance adapter guide rather than pretending the `v1` request envelope already supports diff-ref binding directly.
- **Status**: Accepted
- **Rationale**: The current governance request shape accepts workspace-relative document refs for file-backed flows. Publishing a fake `pr-review` JSON example would lower contract honesty.
- **Consequences**: The guide names `pr-review`, explains the present limitation, and keeps higher-level diff-ref ownership outside Canon until the adapter surface grows that capability explicitly.