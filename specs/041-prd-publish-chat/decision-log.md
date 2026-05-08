# Decision Log: Requirements PRD Publishing And Chat Publish Skill

## D-001 Additive PRD artifact instead of publish-only synthesis

- **Status**: Accepted
- **Date**: 2026-05-07
- **Decision**: Add `prd.md` to the persisted requirements artifact contract so the file participates in inspect, publish, and metadata flows automatically.
- **Rationale**: This preserves one artifact lifecycle and avoids special publish-only behavior.
- **Consequences**: Tests and metadata assertions must expand to include the new artifact without regressing the existing file set.

## D-002 Chat publish support ships as a repo-local skill

- **Status**: Accepted
- **Date**: 2026-05-07
- **Decision**: Add a new `canon-publish` skill and embedded mirror instead of creating a second runtime interface.
- **Rationale**: The CLI already owns publish semantics; chat needs discoverability, not a duplicate control surface.
- **Consequences**: Skill validation must pass and docs must mention the chat-first publish path explicitly.

## D-003 Requirements remains the only consolidated PRD mode in this slice

- **Status**: Accepted
- **Date**: 2026-05-07
- **Decision**: Limit the consolidated single-document artifact to `requirements` for this feature.
- **Rationale**: That directly addresses the user pain while keeping compatibility risk and validation scope bounded.
- **Consequences**: Other modes may be discussed in docs or roadmap but are not changed in the code path for this release.

## D-004 Publish UX stays explicit about runtime artifacts versus published docs

- **Status**: Accepted
- **Date**: 2026-05-07
- **Decision**: Update the README, mode guide, roadmap, changelog, and release guardrails to state that artifacts land under `.canon/artifacts/` first and become visible repository docs only after publish.
- **Rationale**: The reported user confusion was not about missing runtime artifacts; it was about the visible handoff surface being unclear and fragmented.
- **Consequences**: The `0.41.0` release line must stay synchronized across version-bearing files and doc guardrails, and requirements docs must mention `prd.md` as an additive published output.