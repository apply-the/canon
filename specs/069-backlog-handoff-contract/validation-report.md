# Validation Report Plan: Backlog Handoff Contract

## Summary

Implementation completed in the isolated Canon worktree with runtime, publish,
lookup, docs, and skill surfaces aligned to the additive backlog handoff
contract. Validation proved three distinct outcomes remain honest:

- full planning packet plus `execution-handoff.md`
- full planning packet with handoff unavailable made explicit
- closure-limited packet with no handoff artifact

## Structural Validation

- Validate the new backlog handoff contract document and additive artifact list.
- Validate docs and skills wording so backlog mode is still described as
  planning-only while acknowledging governed handoff signals.
- Validate any packet templates or summaries that mention handoff availability.

### Structural Evidence

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `bash scripts/validate-canon-skills.sh`

All four checks passed after the runtime, docs, and test updates landed.

## Logical Validation

- Add contract coverage for:
  - stable `slice_id` presence and reuse across packet artifacts
  - `execution-handoff.md` emission only when a slice is handoff-capable
  - handoff withholding for risk-only and handoff-unavailable packets
- Add integration coverage for:
  - published full packet with handoff available
  - published full packet with handoff unavailable
  - closure-limited packet with no handoff artifact

### Logical Evidence

- `cargo test --test backlog_contract --test backlog_run --test backlog_closure_run --test run_lookup --test skills_bootstrap`
- `cargo nextest run`

Both commands passed. `cargo nextest run` finished with `422/422` tests green.

Focused regression evidence now covers:

- stable `slice_id` propagation across backlog packet artifacts
- `execution-handoff.md` presence only when implementation refs and independent
  verification anchors exist
- honest published full packets that keep handoff unavailable explicit when the
  planning packet is otherwise sound
- closure-limited packets that emit only `backlog-overview.md` and
  `planning-risks.md`
- materialized backlog skill wording that documents `execution-handoff.md` and
  handoff-unavailable behavior

## Independent Validation

- Perform a separate review of emitted sample packets to confirm:
  - handoff-capable packets remain above task-level detail
  - handoff-unavailable packets explain the absence honestly
  - no packet implies Canon-owned execution authority
  - implementation artifact refs are concrete enough to identify a bounded
  downstream change surface
  - independent verification anchors describe proof targets rather than generic
    review language

### Independent Review Notes

- The selected handoff slice stays at slice granularity and does not collapse
  into ticket-level or task-level decomposition.
- `execution-handoff.md` names bounded implementation surfaces and proof
  obligations rather than granting automatic execution authority.
- Handoff-unavailable full packets remain reusable planning artifacts because
  they keep sequencing, dependencies, and acceptance anchors while explicitly
  refusing to overclaim downstream readiness.
- Closure-limited packets remain visibly different from both full-packet paths.

## Evidence Artifacts

- Sample published packet with `execution-handoff.md`
- Sample published full packet without `execution-handoff.md`
- Sample closure-limited packet
- Review notes showing that stable `slice_id` values remain coherent across
  packet artifacts
- Review notes showing why the selected handoff slice is admissible while the
  packet still stays above task-level decomposition
- Full validation transcript:
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --test backlog_contract --test backlog_run --test backlog_closure_run --test run_lookup --test skills_bootstrap`
  - `bash scripts/validate-canon-skills.sh`
  - `cargo nextest run`
