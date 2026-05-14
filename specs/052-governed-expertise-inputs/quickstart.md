# Quickstart: Governed Expertise Inputs

## Goal

Validate that Canon exposes one bounded expertise-input contract for
`domain-language` and `domain-model` without taking over Boundline runtime
selection.

These are post-implementation validation steps and assume the stable expertise-
input contract has already been created.

## Steps

1. Review `docs/integration/governed-expertise-input-contract.md` and confirm it names the supported expertise kinds, classification rules, and ownership boundary.
2. Review `docs/integration/project-memory-promotion-contract.md` and confirm the expertise-input contract extends existing publication semantics instead of replacing them.
3. Run `cargo test --test governed_expertise_inputs --test governed_expertise_publish --test governed_expertise_input_contract` and confirm the expertise-classification, publication-boundary, and contract tests pass.
4. Review `specs/052-governed-expertise-inputs/validation-report.md` for the broader validation suite and closeout status.

## Expected Outcome

- Consumers can identify supported expertise inputs without reading Canon implementation code.
- Only `domain-language` and `domain-model` are supported expertise kinds in this slice.
- The Canon and Boundline ownership boundary remains explicit.
