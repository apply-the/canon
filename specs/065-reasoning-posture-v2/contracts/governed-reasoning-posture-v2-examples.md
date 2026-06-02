# Contract: Governed Reasoning Posture v2 Example Corpus

## Purpose

Define the machine-checkable example set required to validate the new
`governed_reasoning_posture_v2` contract line.

## Required Example Inventory

The delivered example corpus must include these canonical cases:

1. `valid-v2-posture`: one fully valid `governed_reasoning_posture_v2` payload
2. `invalid-selector-both-present`: both selector kinds are present
3. `invalid-selector-neither-present`: neither selector kind is present
4. `invalid-independence-missing-block`: `minimum_independence` is omitted
5. `invalid-independence-contradictory`: hard minima contradict each other
6. `invalid-independence-impossible-minima`: hard minima are impossible to
   satisfy
7. `invalid-independence-guidance-override`: guidance attempts to weaken or
   replace the hard minima
8. `invalid-confidence-missing-block`: `confidence_handoff` is omitted
9. `invalid-confidence-none-contradictory`: `confidence_handoff.state = none`
   still carries required-handoff fields or contradictory semantics
10. `invalid-confidence-required-missing-fields`: handoff is `required` but the
    required handoff fields are incomplete
11. `invalid-provenance-missing-block`: `provenance` is omitted
12. `invalid-provenance-missing-reference-kind`: provenance exists but omits
    the reference-kind contract
13. `invalid-provenance-incompatible-handoff`: provenance is incompatible with
    a required handoff
14. `invalid-provenance-stale`: provenance relies on stale evidence or stale
    references
15. `invalid-provenance-contradictory`: provenance supplies contradictory
    evidence or references
16. `invalid-unsupported-vocabulary`: the payload uses unsupported vocabulary
    values for the published contract line
17. `invalid-compatibility-window`: the version window contradicts the
    published contract line or release alignment
18. `invalid-release-metadata-stale`: release metadata lags behind the
    published contract line or compatibility window
19. `invalid-release-metadata-contradictory`: release metadata contradicts the
    published contract line, examples, or compatibility window
20. `dual-line-coexistence-valid`: one active line and one explicit legacy line
21. `dual-line-coexistence-ambiguous`: dual-line publication without exactly
    one active line and one legacy line
22. `migration-rejection-v2-to-v1-consumer`: a `v2` payload reaches a
    `v1`-only consumer
23. `migration-rejection-v1-to-v2-required`: a `v1` payload reaches a workflow
    that explicitly requires `v2`

## Example Shape Requirements

Every example must declare:

- `example_id`
- the payload or publication surface path
- the intended contract line or lines involved
- the expected validation result (`accept` or `reject`)
- the expected reason for acceptance or rejection

## Fixture Reference Rules

- TOML fixtures live under `tests/fixtures/governed_reasoning_posture_v2/` with
    filenames matching `<example_id>.toml`.
- JSON fixtures live under `tests/fixtures/governed_reasoning_posture_v2/` with
    filenames matching `<example_id>.json` for release-metadata drift cases.

## Expected Outcome Inventory

- `valid-v2-posture`: accept because the payload satisfies the
    `governed_reasoning_posture_v2` contract.
- `invalid-selector-both-present`: reject because selector conflict publishes
    both selector families.
- `invalid-selector-neither-present`: reject because selector data is missing.
- `invalid-independence-missing-block`: reject because
    `minimum_independence` is missing.
- `invalid-independence-contradictory`: reject because independence
    requirements contradict each other.
- `invalid-independence-impossible-minima`: reject because the hard minima are
    impossible to satisfy.
- `invalid-independence-guidance-override`: reject because guidance weakens the
    hard minima.
- `invalid-confidence-missing-block`: reject because `confidence_handoff` is
    missing.
- `invalid-confidence-none-contradictory`: reject because `state = none`
    publishes contradictory required-handoff semantics.
- `invalid-confidence-required-missing-fields`: reject because the required
    confidence handoff omits mandatory fields.
- `invalid-provenance-missing-block`: reject because `provenance` is missing.
- `invalid-provenance-missing-reference-kind`: reject because a provenance
    reference omits `reference_kind`.
- `invalid-provenance-incompatible-handoff`: reject because provenance is
    incompatible with a required handoff.
- `invalid-provenance-stale`: reject because provenance references stale
    evidence.
- `invalid-provenance-contradictory`: reject because provenance state and
    evidence contradict each other.
- `invalid-unsupported-vocabulary`: reject because the payload uses unsupported
    vocabulary.
- `invalid-compatibility-window`: reject because the compatibility window
    contradicts the published contract line.
- `invalid-release-metadata-stale`: reject because release metadata still
    advertises stale versions.
- `invalid-release-metadata-contradictory`: reject because release metadata
    disagrees across surfaces.
- `dual-line-coexistence-valid`: accept because one line is `active` and the
    other is `legacy`.
- `dual-line-coexistence-ambiguous`: reject because more than one line is
    treated as active.
- `migration-rejection-v2-to-v1-consumer`: reject because a `v1`-only consumer
    cannot accept `v2`.
- `migration-rejection-v1-to-v2-required`: reject because a `v2`-required
    workflow cannot accept `v1`.

## Validation Expectations

- Valid examples must pass without consumer-side interpretation rules.
- Invalid examples must fail for the reason encoded in the example definition.
- Example outcomes are part of the contract surface and must be executable in
  CI once the fixture harness exists.

## Delivery Notes

- Example payloads live under `tests/fixtures/governed_reasoning_posture_v2/`
    so the contract harness can execute them directly.
- The stable contract doc should reference the example corpus, but the fixture
  files themselves may remain feature-local if the release docs already link to
  the normative payload shape and validation harness.