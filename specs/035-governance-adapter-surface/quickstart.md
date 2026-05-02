# Quickstart: Governance Adapter Surface For External Orchestrators

## Goal

Validate that Canon exposes a real machine-facing governance adapter surface
for external orchestrators, keeps domain outcomes machine-readable, preserves
strict readiness semantics, and ships `0.35.0` with contract docs, quality
gates, and touched-file Rust coverage above 95%.

## Recommended Validation Flow

1. Inspect the published capabilities response and confirm Canon advertises the
   supported schema versions, operations, governance modes, and exact outcome
   vocabularies.
2. Send a well-formed start request that intentionally omits one domain field
   such as `owner` and confirm Canon returns a blocked domain outcome with a
   machine-usable `reason_code`.
3. Send a valid start request and confirm Canon returns a run reference when it
   materializes or advances governed work.
4. Refresh the same governed run and confirm the response stays idempotent when
   the underlying run state does not change.
5. Validate that `governed_ready` only appears when the response also carries a
   reusable packet projection with canonical workspace-relative refs.
6. Run focused producer-side contract and integration tests for start,
   refresh, capabilities, and response normalization.
7. Run `cargo fmt --check`, `cargo clippy --workspace --all-targets
   --all-features -- -D warnings`, `cargo nextest run`, and a coverage command
   that demonstrates more than 95% line coverage for every modified or newly
   created Rust source file.
8. Execute one live consumer-driven smoke against the current Synod adapter
   expectations using a real Canon binary and record the result in the feature
   validation report.

## Representative Walkthroughs

- Use `capabilities` first and confirm a consumer can decide compatibility
  before attempting any governed work.
- Use a blocked start request with missing domain context and confirm the
  outcome is domain-readable rather than a transport failure.
- Use a refresh flow where packet output is incomplete and confirm Canon does
  not emit `governed_ready`.
- Use a refresh flow with a reusable packet and confirm Canon emits
  `governed_ready`, a packet reference, and non-empty document refs.
- Review the returned packet and document refs and confirm they stay
  workspace-relative and stable across repeated runs on the same workspace.