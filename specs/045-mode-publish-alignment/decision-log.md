# Decision Log: Mode Publish Alignment

- **D-001**: Runtime behavior will be aligned to the documented `security-assessment` publish posture rather than narrowing the docs, **Rationale**: the current documentation already treats `security-assessment` as a publishable operational packet when readable artifacts exist.
- **D-002**: Assistant publish command drift will be corrected in assistant metadata and prompt packs instead of adding a second CLI syntax, **Rationale**: the CLI surface is already correct and should remain singular.
- **D-003**: The slice ships as `0.45.0` with direct release-surface closeout, **Rationale**: repository practice and the user request both require a versioned, validated finish.