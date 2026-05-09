# Research: Pragmatic C4 Architecture Packets And Visual Artifacts

## Decision 1: Add one primary architecture handoff document

- **Decision**: Introduce `architecture-overview.md` as the primary human-readable handoff document for published architecture packets.
- **Rationale**: The current packet is technically rich but operationally fragmented. A primary document gives reviewers one stable entrypoint while preserving supporting artifacts for deeper inspection.
- **Alternatives considered**:
  - Keep only the current flat file set: rejected because it preserves the usability problem.
  - Replace all supporting files with one merged document: rejected because it would weaken machine readability and inspectability.

## Decision 2: Mermaid is the canonical machine-readable diagram source

- **Decision**: Emit Mermaid source artifacts for each included architectural view and treat them as the default machine-readable diagram contract.
- **Rationale**: Mermaid is text-based, repository-friendly, human-auditable, and widely renderable without requiring binary assets to exist in every environment.
- **Alternatives considered**:
  - Store only fenced Mermaid blocks inside markdown: rejected because dedicated diagram-source artifacts are easier for tooling to discover and render.
  - Generate SVG or PNG only: rejected because binary-only outputs reduce inspectability and depend too heavily on environment-specific rendering support.

## Decision 3: Extend the artifact model for optional view artifacts

- **Decision**: Extend the artifact contract or persistence model so architecture packets can declare optional visual artifacts such as `component-view.md`, `dynamic-view.md`, Mermaid files, and rendered SVG or PNG assets without forcing every run to emit all of them.
- **Rationale**: Pragmatic C4 output needs required and optional view depth. The current contract model is all-or-nothing and would otherwise force unnecessary files to exist in every packet.
- **Alternatives considered**:
  - Keep every current file mandatory and fill omitted views with placeholder markdown: rejected because it preserves documentation bureaucracy and conflicts with the pragmatic view-selection goal.
  - Remove all deeper-view artifacts entirely: rejected because complex systems still need bounded deeper views when justified.

## Decision 4: Deployment coverage is mandatory at the packet level, not necessarily as a rendered diagram

- **Decision**: Require deployment coverage in the packet, but allow it to appear either as a real deployment view artifact or as an explicit omission record when the authored evidence cannot support one.
- **Rationale**: Deployment is part of the recommended default packet, but the system must stay honest when the brief does not justify a trustworthy deployment diagram.
- **Alternatives considered**:
  - Always require a deployment diagram artifact: rejected because it would encourage fabricated infrastructure detail.
  - Make deployment entirely optional: rejected because that weakens the pragmatic default the feature is trying to establish.

## Decision 5: Rendered SVG and PNG assets are capability-dependent secondaries

- **Decision**: Treat SVG and PNG files as optional secondary outputs derived from the Mermaid source when rendering support is available and quality thresholds are met.
- **Rationale**: This preserves a human-friendly visual path without making the packet unusable in environments where rendering cannot be guaranteed.
- **Alternatives considered**:
  - Require rendered assets for every included view: rejected because that would make environment capability a hard blocker.
  - Omit rendered assets entirely: rejected because downloadable visuals are valuable for architecture reviews and external documentation handoff.