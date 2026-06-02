# Quickstart: Governed Reasoning Posture v2

## Goal

Review, implement, and validate the `governed_reasoning_posture_v2` contract
without relying on Canon implementation code as the system of record.

## Prerequisites

- Be on branch `065-reasoning-posture-v2`
- Review the feature spec and plan in `specs/065-reasoning-posture-v2/`
- Use Canon workspace version `0.64.0` when validating release alignment

## Review The Contract Shape

1. Review the stable Canon integration contract in
   `tech-docs/integration/governed-reasoning-posture-contract.md` and confirm
   that `governed_reasoning_posture_v2` is published without weakening the
   meaning of `v1`.
2. Review `contracts/governed-reasoning-posture-v2.md` and confirm that the
   payload shape requires exactly one selector kind plus explicit
   `minimum_independence`, `confidence_handoff`, `provenance`, and
   `compatibility_window` blocks.
3. Review `contracts/governed-reasoning-posture-v2-migration.md` and confirm
   that dual-line publication allows exactly one active line and one legacy
   line, with no implicit fallback.
4. Review `contracts/governed-reasoning-posture-v2-examples.md` and confirm
   that the example set covers one valid payload plus all required invalid and
   migration scenarios.
5. Inspect the executable fixtures under
   `tests/fixtures/governed_reasoning_posture_v2/`, including:
   - `valid-v2-posture.toml`
   - `invalid-selector-both-present.toml`
   - `invalid-selector-neither-present.toml`
   - the malformed-case fixtures for independence, confidence handoff,
     provenance, vocabulary, and compatibility drift
   - the JSON release-metadata drift fixtures
   - the dual-line and migration rejection fixtures

## Validate Release Alignment

1. Confirm `Cargo.toml` advertises Canon `0.64.0`.
2. Confirm release-facing metadata and user-facing docs align with the same
   contract line and compatibility window.
3. Confirm README, CHANGELOG, and impacted `tech-docs` describe the new line as
   a substantive protocol evolution rather than a minor extension of `v1`.

## Run Validation

Run the focused validation path for the contract and release surfaces:

```bash
cargo test --test governed_reasoning_posture_contract
cargo test --no-run --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
cargo fmt --check
```

The fixture harness now reads the concrete files under
`tests/fixtures/governed_reasoning_posture_v2/`, so reviewers should cross-check
expected reasons in the fixture metadata against the stable contract text.

## Expected Outcomes

- `v2` payload validation is deterministic and fail-closed
- `v1` remains semantically frozen and cannot be silently reinterpreted
- dual-line publication is unambiguous and explicitly active-versus-legacy
- machine-checkable examples and release metadata stay aligned with the stable
  contract