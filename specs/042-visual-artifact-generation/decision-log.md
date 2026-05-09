# Decision Log: Pragmatic C4 Architecture Packets And Visual Artifacts

## D-001 Primary architecture packet over flat file parity

- **Status**: Accepted
- **Date**: 2026-05-08
- **Decision**: Add `architecture-overview.md` as the default review entrypoint instead of keeping all architecture files at equal weight.
- **Rationale**: Reviewers need one coherent starting point for the packet.
- **Consequences**: The overview document must reference supporting artifacts and remain additive rather than replacing them.

## D-002 Mermaid as the canonical diagram source

- **Status**: Accepted
- **Date**: 2026-05-08
- **Decision**: Emit Mermaid source artifacts for included diagram views.
- **Rationale**: Mermaid is text-based, durable, and suitable for both human and machine use.
- **Consequences**: Rendered SVG or PNG assets become optional secondaries, not the system of record.

## D-003 Optional view artifacts need explicit contract support

- **Status**: Accepted
- **Date**: 2026-05-08
- **Decision**: Extend the architecture artifact model so deeper views and rendered formats can be optional and justified rather than mandatory in every packet.
- **Rationale**: Pragmatic C4 output requires depth-by-need, not depth-by-template.
- **Consequences**: Artifact persistence, publish behavior, manifests, and tests must all recognize optional outputs explicitly.