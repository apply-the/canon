# Data Model: Governed Expertise Inputs

## GovernedExpertiseKind

Represents the Canon-owned expertise categories supported in this contract line.

Values:
- `domain-language`
- `domain-model`

## ExpertiseModeClassification

Represents the stable mapping from a Canon mode to expertise-input semantics.

Fields:
- `mode`: Canon mode being classified.
- `expertise_kind`: supported expertise kind or no expertise classification.
- `domain_families`: Boundline-facing domain-family identifiers the expertise input applies to.
- `artifact_families`: human-readable artifact families associated with the classified mode.
- `publication_surface`: publication or lineage surface the consumer must use.

## GovernedExpertiseInput

Represents a Canon-published artifact that downstream consumers may treat as a
governed expertise input.

Fields:
- `expertise_input.expertise_kind`: Canon-owned expertise category.
- `expertise_input.domain_families`: non-empty list of applicable domain-family identifiers.
- `mode`: governing Canon mode.
- `contract_version`: supported expertise-input contract line.
- `promotion_state`: Canon publication state for the artifact.
- `publication_target_class`: Canon target class such as stable, pending, proposal, evidence, or index.
- `source_ref`: originating Canon packet or source reference.
- `artifact_class`: Canon artifact class or target class used for publication.

## ExpertiseInputCompatibility

Represents the rules consumers rely on when expertise-input semantics evolve.

Rules:
- supported contract line plus supported expertise kind is consumable
- unknown expertise kind is ignorable without changing stable lineage meaning
- missing required classification metadata makes the artifact unavailable as an expertise input
- incompatible contract line fails closed
