# Research: Governed Expertise Inputs

## Current Stable Surfaces

- `crates/canon-engine/src/domain/mode.rs` already defines `DomainLanguage` and `DomainModel` plus their current mode profiles and artifact families.
- `crates/canon-engine/src/domain/publish_profile.rs` already defines lineage fields, metadata carriers, and indexable artifact classes for Canon publication.
- `crates/canon-engine/src/orchestrator/publish.rs` already emits packet metadata and repo-visible sidecars for governed output.
- `tech-docs/integration/project-memory-promotion-contract.md` already governs Canon publication and lineage semantics for repo-visible promotion.

## Boundaries Confirmed During Planning

- Canon should publish expertise semantics, not runtime directives.
- `domain-language` and `domain-model` are the narrow initial expertise kinds with the clearest downstream value.
- Expertise inputs should reuse existing lineage and target-class semantics instead of inventing a second packet channel.
- Unsupported expertise kinds or incompatible contract lines must fail closed for downstream consumers.

## Implementation Direction

- Add explicit source-level expertise classification aligned with `DomainLanguage` and `DomainModel`.
- Publish a stable expertise-input integration contract that references the existing project-memory promotion contract instead of redefining publication semantics.
- Keep expertise inputs readable to humans and machine-classifiable to consumers.
- Preserve the Canon and Boundline ownership boundary by excluding runtime-role and provider-routing directives.

## Likely Touchpoints

- `crates/canon-engine/src/domain/mode.rs`
- `crates/canon-engine/src/domain/publish_profile.rs`
- `crates/canon-engine/src/orchestrator/publish.rs`
- `tech-docs/integration/project-memory-promotion-contract.md`
- `tech-docs/integration/governed-expertise-input-contract.md`
