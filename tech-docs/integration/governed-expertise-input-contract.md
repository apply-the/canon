# Governed Expertise Input Contract

## Contract Identity

- `owner`: `canon`
- `current_contract_line`: `v1`
- `stable_doc`: `tech-docs/integration/governed-expertise-input-contract.md`
- `primary_consumer`: `boundline`

## Purpose

Define the Canon-owned producer semantics for governed expertise inputs that
Boundline or another downstream consumer may classify and reuse without turning
Canon into a runtime selector, provider router, or delivery orchestrator.

## Authority And Sync Rules

- This stable document is the normative source for expertise-input ownership,
  supported kinds, compatibility, and consumer classification rules.
- Publication, target-class, artifact-class, and metadata-carrier semantics
  remain governed by `tech-docs/integration/project-memory-promotion-contract.md`.
- Feature-local contracts may elaborate this contract with examples and derived
  shapes, but they do not supersede it.
- If a conflict appears between this stable document and a feature-local brief,
  this stable document wins and the feature-local brief must be realigned in
  the same change before merge.

## Supported Expertise Kinds

- `domain-language`: reusable governed vocabulary and language guidance emitted
  from Canon `domain-language` mode.
- `domain-model`: reusable governed concept, relationship, and invariant
  guidance emitted from Canon `domain-model` mode.

## Classification Rules

- A governed expertise input MUST have a supported expertise kind.
- A governed expertise input MUST remain traceable to Canon lineage and mode
  semantics.
- A published Canon artifact without supported expertise classification remains
  readable Canon output but is not a governed expertise input for this contract
  line.

## Required Classification And Lineage Fields

At minimum, consumers must be able to recover:
- `contract_version`
- `mode`
- `source_ref`
- `promotion_state`

The expertise-specific machine-readable carrier is the `expertise_input` object
serialized in `packet-metadata.json` or `<surface>.packet-metadata.json`:

```json
{
  "expertise_input": {
    "expertise_kind": "domain-language",
    "domain_families": ["react", "web_ui"]
  }
}
```

`expertise_input.expertise_kind` and `expertise_input.domain_families` are
required for a published artifact to qualify as a governed expertise input.
`domain_families` uses the current Boundline-facing domain-family identifiers
expected by the primary consumer.

Additional lineage metadata remains governed by
`tech-docs/integration/project-memory-promotion-contract.md`.

## Publication Surfaces And Target Classes

- Governed expertise inputs reuse Canon's current project-memory and
  evidence-facing publication semantics.
- Target classes such as `stable`, `pending`, `proposal`, `evidence`, and
  `index` remain defined by `tech-docs/integration/project-memory-promotion-contract.md`.
- This contract does not redefine artifact classes or metadata carriers;
  consumers combine this contract with the stable promotion contract to locate
  and classify published expertise inputs.

## Consumer Matching Guidance

- Consumers match a governed expertise input to their current domain selection
  through `expertise_input.domain_families`.
- A governed expertise input without an intersecting domain family is readable
  Canon output but not an applicable expertise input for that consumer state.
- This contract does not define consumer readiness policy for blocked,
  conflicting, pending, proposal, evidence, or index publication outcomes.
  Consumers must make that policy explicit on their own side.

## Compatibility Policy

- Additive expertise hints or metadata are backward-compatible when the meaning
  of an existing expertise kind does not change.
- Unknown expertise kinds are outside the supported `v1` surface and must be
  ignored or rejected by consumers.
- Removing or renaming required classification fields is breaking and requires a
  new major contract line.
- Changing the meaning of an existing supported expertise kind is breaking and
  requires a new major contract line.

## Explicit Exclusions

- runtime role directives
- expert-pack activation instructions
- provider or model routing policy
- any Canon mode not explicitly classified as a governed expertise input in this
  contract line

## Non-Goals

- Selecting runtime roles or models for Boundline
- Creating a Canon runtime registry, plugin system, or marketplace
- Replacing Canon's existing project-memory promotion contract
- Expanding the initial expertise-input surface beyond `domain-language` and
  `domain-model` in `v1`
