# Decision Log: Artifact Indexing Contract

## 2026-05-14

- **D-001**: `docs/integration/project-memory-promotion-contract.md` remains
  the single normative stable contract surface.
  **Rationale**: 051 must extend the Canon-owned contract, not introduce a
  competing authority document.

- **D-002**: V1 indexable artifact classes are `managed-surface`,
  `proposal-artifact`, `evidence-bundle`, and `index-surface`.
  **Rationale**: these classes match current Canon publish behavior and avoid
  forcing consumers to infer semantics from implementation details.

- **D-003**: V1 metadata carriers are `managed-surface-envelope` and
  `packet-metadata-sidecar`.
  **Rationale**: current Canon publish behavior already emits managed-block
  markers plus sidecars for managed surfaces and packet sidecars for packet-like
  outputs.

- **D-004**: `safety-net packets` is excluded from V1 Canon contract
  vocabulary.
  **Rationale**: no current Canon publish surface emits that artifact class.

- **D-005**: Feature-local 051 contract briefs are derived and non-normative.
  **Rationale**: they support implementation and review but must not outrank the
  stable Canon contract.