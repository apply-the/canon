# Governed Expertise Input Contract

- **Owner**: Canon
- **Status**: Derived, non-normative brief
- **Normative Stable Path**: `tech-docs/integration/governed-expertise-input-contract.md`
- **Current Contract Line**: v1

## Purpose

Summarize the Canon-owned governed expertise-input surface for downstream
consumers without turning Canon into a runtime selector.

## Authority Rule

- This brief mirrors the stable Canon expertise-input contract and exists for feature-local planning and validation.
- If this brief and the stable Canon integration contract diverge, the stable Canon contract wins.
- Publication and lineage semantics remain governed by `tech-docs/integration/project-memory-promotion-contract.md`.

## Supported Expertise Kinds

- `domain-language`: reusable governed vocabulary and language guidance emitted from Canon `domain-language` mode.
- `domain-model`: reusable governed concept, relationship, and invariant guidance emitted from Canon `domain-model` mode.

## Classification Rules

- A governed expertise input must have a supported expertise kind.
- A governed expertise input must remain traceable to Canon lineage and mode semantics.
- A published Canon artifact without supported expertise classification remains readable Canon output but is not a governed expertise input for this slice.

## Machine-Readable Carrier

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
required for the artifact to qualify as a governed expertise input.

## Required Classification And Lineage Fields

At minimum, consumers must be able to recover:
- `contract_version`
- `mode`
- `source_ref`
- `promotion_state`
- `expertise_input.expertise_kind`
- `expertise_input.domain_families`

Additional lineage metadata remains governed by
`tech-docs/integration/project-memory-promotion-contract.md`.

## Publication Surfaces And Target Classes

- Governed expertise inputs reuse Canon's current project-memory and
	evidence-facing publication semantics.
- Target classes such as stable, pending, proposal, evidence, and index remain
	defined by `tech-docs/integration/project-memory-promotion-contract.md`.
- The expertise-input contract does not redefine artifact classes or metadata
	carriers; consumers combine this contract with the stable promotion contract
	to locate and classify published expertise inputs.

## Consumer Matching Guidance

- Consumers match a governed expertise input through
	`expertise_input.domain_families`.
- A published Canon artifact without an intersecting domain family is readable
	Canon output but not an applicable expertise input for that consumer state.

## Compatibility Rules

- Additive expertise hints may be ignored by older consumers.
- Unknown expertise kinds are outside the supported v1 surface.
- Unsupported contract lines fail closed for consumers.

## Explicit Exclusions

- runtime role directives
- expert-pack activation instructions
- provider or model routing policy
- any Canon mode not explicitly classified as a governed expertise input in this contract line
