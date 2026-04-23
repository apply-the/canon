# Validation Report Plan: Controlled Execution Modes

## Summary

Validation for this feature must prove that `implementation` and `refactor` become governed runtime modes without regressing identity, publish, inspect, or existing-mode behavior. Validation remains layered and explicitly separate from generation.

## Validation Ownership

- **Generation owners**: runtime implementation changes in `canon-cli`, `canon-engine`, defaults, skills, and docs
- **Structural validators**: formatter, linter, config/skill validation scripts
- **Logical validators**: contract tests, integration tests, direct runtime coverage tests, publish path checks
- **Independent validators**: reviewer-mode or separate-reader pass against emitted artifacts, summaries, and posture labeling

## Structural Validation

Run these after code changes land:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test
cargo nextest run
scripts/validate-canon-skills.sh
```

Structural validation must also confirm:

- updated `defaults/methods/implementation.toml` and `defaults/methods/refactor.toml` remain loadable through the embedded method store
- updated embedded skills and materialized `.agents/skills/` copies stay in sync
- no new unresolved placeholders remain in spec/plan artifacts for this feature

Structural validation completed for the closeout tranche on 2026-04-23:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
bash scripts/validate-canon-skills.sh
```

Observed result:

- all structural commands passed after formatting the touched Rust files
- shared runtime-hint scripts and the shared skill index continue to advertise `implementation` and `refactor` as `available-now`
- skill validation remained green after the carry-forward packet and quickstart documentation updates

## Logical Validation

### Contract Tests

Add dedicated suites covering:

- implementation artifact contract completeness and missing-section failures
- refactor artifact contract completeness and missing-section failures
- canonical authored-input binding for `canon-input/implementation.*` and `canon-input/refactor.*`
- persisted `context.toml` contents for mode-specific execution metadata

### Integration Tests

Add dedicated suites covering:

- successful bounded `implementation` run with explicit mutation bounds
- blocked `implementation` run when mutation bounds or plan linkage are missing
- successful `refactor` run with preserved behavior and structural rationale
- blocked `refactor` run when preservation evidence or no-feature-addition proof is missing
- recommendation-only fallback for red-zone/systemic-impact execution-heavy runs
- publish compatibility for both modes through existing destinations
- status/list/inspect compatibility for display id, UUID, short id, slug, and `@last`

### Non-Regression Checks

- update staged-depth assumptions in `tests/integration/mode_profiles.rs`
- preserve existing behavior for previously shipped full-depth modes
- preserve `change` recommendation-only mutation semantics while extending the model to `implementation` and `refactor`

Logical validation completed for the closeout tranche on 2026-04-23:

```bash
cargo test
cargo nextest run
```

Observed result:

- `cargo test` passed across the full workspace, including the new `inspect_evidence_surfaces_upstream_context_from_folder_packet` contract test
- `cargo nextest run` passed with 136 tests run and 136 passed
- non-regression coverage for promoted mode profiles and inspect surfaces remained green

## Independent Validation

An independent reviewer pass must verify:

- artifact bundles are distinguishable between `implementation` and `refactor` without reading only metadata
- recommendation-only posture is explicit in summaries, inspect output, and published output
- docs and skills no longer describe the promoted modes as `modeled-only`
- no existing run created before this feature loses lookup, inspect, or publish compatibility

### T044 Closeout (carry-forward to Feature 011)

T044 was executed by dogfooding the promoted `implementation` and
`refactor` modes against `/Users/rt/workspace/java-html-sanitizer/`
with real folder-backed packets. The dogfood confirmed bundle
distinctness, posture labelling, and publish/lookup compatibility, but
also surfaced three blocking gaps that this feature could not address
within its own scope:

1. Both modes auto-complete in `recommendation-only` with no
   `approval_targets`, so `$canon-approve` and `$canon-resume` have no
   target to act on.
2. The constraint profiles hardcode `recommendation_only = true`, so
   even an explicit human approval cannot promote the run to a
   bounded posture.
3. `mode_result` exposes no `action_chips`, so hosts cannot render the
   Approve / Resume / Inspect / Open chips defined in
   `defaults/embedded-skills/canon-shared/references/output-shapes.md`.

These findings are owned by Feature 011
(`specs/011-execution-mode-approval-and-chips/`) under decisions D-013,
D-014, and D-015 in this folder's decision log. D-011 is therefore
marked superseded. T044 is **closed with carry-forward to 011**; no
further work happens against this report for that gap.

## Evidence Artifacts

Record evidence in the delivering Canon run under `.canon/runs/<RUN_ID>/evidence/` and cross-link the results from `specs/010-controlled-execution-modes/decision-log.md` and any downstream `tasks.md` entries.

## Implementation Checkpoints

- **Governance checkpoint**: Before runtime code changes, confirm `spec.md`, `plan.md`, `decision-log.md`, contracts, and `quickstart.md` still describe the same scope and invariants.
- **US1 checkpoint**: Before promoting `refactor`, confirm bounded `implementation` execution, emitted artifacts, and publish compatibility are independently validated.
- **US2 checkpoint**: Before enabling shared recommendation-only behavior, confirm preservation gates, drift blocking, and no-feature-addition evidence are independently validated.
- **Final review checkpoint**: Before declaring the feature complete, perform an independent review of artifact distinctness, recommendation-only labeling, and backward compatibility.

## Implementation Validation Evidence

### US1: Bounded Implementation Execution

Validated on the new `implementation` runtime slice with focused checks:

```bash
cargo test -p canon-engine apply_execution_posture_summary_reads_recommendation_only_from_run_context
cargo test -p canon-engine from_identity_keeps_execution_heavy_modes_on_the_existing_manifest_surface
cargo test --test direct_runtime_coverage implementation_direct_run_surfaces_recommendation_only_posture_and_bounded_artifacts
cargo test --test implementation_run run_implementation_completes_with_recommendation_only_execution_posture
cargo test --test policy_and_traces implementation_run_persists_recommendation_only_mutation_traces
cargo test --test mode_profiles
cargo test -p canon-engine materialized_methods_keep_promoted_execution_artifact_lists
```

Observed evidence:

- `implementation` now starts through the real runtime path instead of failing with `UnsupportedMode("implementation")`.
- Completed runs surface `execution_posture = recommendation-only` through run and status summaries.
- The emitted packet contains `task-mapping.md`, `mutation-bounds.md`, `implementation-notes.md`, `completion-evidence.md`, `validation-hooks.md`, and `rollback-notes.md`.
- Trace persistence records `ExecuteBoundedTransformation` with `RecommendationOnly` outcome for implementation runs.
- `canon publish <RUN_ID>` writes the default visible destination under `docs/implementation/<RUN_ID>/` without changing the governed `.canon/` copy.
- The typed mode profile and embedded method defaults now advertise `implementation` as `full` depth and use the promoted artifact bundle.

Residual note for later phases:

- The current US1 runtime slice keeps implementation workspace mutation recommendation-only; later phases still need explicit cross-mode fallback behavior.

### US2: Preservation-First Refactor Execution

Validated on the new `refactor` runtime slice with focused checks:

```bash
cargo test --test direct_runtime_coverage refactor_direct_run_surfaces_recommendation_only_posture_and_preservation_artifacts
cargo test --test refactor_run run_refactor_completes_with_recommendation_only_execution_posture
cargo test --test refactor_preservation_run run_refactor_blocks_when_preservation_and_feature_audit_inputs_are_missing
bash scripts/validate-canon-skills.sh
```

Observed evidence:

- `refactor` now starts through the real runtime path instead of failing with `UnsupportedMode("refactor")`.
- Completed runs surface `execution_posture = recommendation-only` through run and status summaries.
- The emitted packet contains `preserved-behavior.md`, `refactor-scope.md`, `structural-rationale.md`, `regression-evidence.md`, `contract-drift-check.md`, and `no-feature-addition.md`.
- Blocked runs surface missing-context markers when preserved behavior or feature-audit evidence is incomplete.
- `canon publish <RUN_ID>` writes the default visible destination under `docs/refactors/<RUN_ID>/` without changing the governed `.canon/` copy.
- The typed mode profile and embedded method defaults now advertise `refactor` as `full` depth and use the promoted artifact bundle.
- Shared skill/index validation now accepts `canon-refactor` as `available-now` and keeps the embedded and materialized skill surfaces aligned.

### US3: Recommendation-Only Fallback for High-Risk Execution

Validated on the shared recommendation-only slice with focused checks:

```bash
cargo test --test cli_contract run_implementation_auto_binds_canonical_input_before_runtime_support_check
cargo test --test invocation_cli_contract inspect_invocations_and_evidence_are_user_visible_and_populated
cargo test --test inspect_modes inspect_modes_text_output_keeps_execution_heavy_modes_visible
cargo test --test adapter_policy change_red_or_systemic_work_becomes_recommendation_only
cargo test -p canon-engine implementation_risk_gate_keeps_systemic_red_runs_in_recommendation_only_posture
cargo test -p canon-engine refactor_risk_gate_keeps_systemic_red_runs_in_recommendation_only_posture
cargo test --test implementation_run systemic_implementation_run_remains_recommendation_only_and_publishable
cargo test --test refactor_run red_zone_refactor_run_remains_recommendation_only_and_publishable
cargo test --test policy_and_traces red_zone_refactor_run_persists_recommendation_only_mutation_traces
cargo test --test run_lookup recommendation_only_implementation_runs_remain_resolvable_via_last_alias
```

Observed evidence:

- `implementation` canonical authored-input auto-binding now succeeds through the real runtime path and still surfaces `execution_posture = recommendation-only`.
- `inspect invocations` exposes `recommendation_only = true` for constrained execution requests, and `inspect evidence` exposes `execution_posture = recommendation-only` from persisted run context.
- High-risk `implementation` and `refactor` runs no longer stop in `AwaitingApproval`; they complete as recommendation-only packets with the same bounded artifact bundles.
- `canon publish <RUN_ID>` and `canon publish @last` continue to route recommendation-only implementation/refactor runs through the existing `docs/implementation/<RUN_ID>/` and `docs/refactors/<RUN_ID>/` destinations.
- Trace persistence still records `ExecuteBoundedTransformation` with `RecommendationOnly` outcome for recommendation-only execution-heavy runs.
- `inspect modes` text output and existing mode taxonomy remain stable while the promoted execution-heavy modes stay visible.

Residual note for later phases:

- Independent review remains the only open final-phase closeout item.

### Closeout Tranche: Carry-Forward Lineage and Folder-Backed Walkthroughs

Validated the folder-backed carry-forward additions with focused and end-to-end checks:

```bash
cargo test -p canon-engine run_context_serializes_mode_specific_execution_blocks_when_present
cargo test -p canon-engine persist_and_load_run_context_round_trip_mode_specific_execution_metadata
cargo test -p canon-engine build_run_context_extracts_upstream_context_from_folder_packet
cargo test -p canon-cli evidence_markdown_renders_sections_for_available_lineage
cargo test --test invocation_cli_contract inspect_evidence_surfaces_upstream_context_from_folder_packet
```

Quickstart walk-through validation executed against a temporary git workspace with real folder-backed packets for both modes using:

```bash
canon run --mode implementation --system-context existing --risk bounded-impact --zone yellow --owner staff-engineer --input canon-input/implementation
canon inspect evidence --run <RUN_ID> --output json
canon publish <RUN_ID>

canon run --mode refactor --system-context existing --risk bounded-impact --zone yellow --owner staff-engineer --input canon-input/refactor
canon inspect evidence --run <RUN_ID> --output json
canon publish <RUN_ID>
```

Observed evidence:

- `.canon/runs/<RUN_ID>/context.toml` now round-trips optional `upstream_context` provenance alongside existing mode-specific execution metadata
- `canon inspect evidence --output json` now exposes `upstream_feature_slice`, `primary_upstream_mode`, `upstream_source_refs`, `carried_forward_items`, and `excluded_upstream_scope` when the authored packet provides those markers
- the quickstart examples were tightened to include the required runtime markers for both modes, not only the new carry-forward lineage markers
- real folder-backed `implementation` and `refactor` runs completed, surfaced lineage in evidence inspection, and published to the existing `docs/implementation/<RUN_ID>/` and `docs/refactors/<RUN_ID>/` destinations
- the carry-forward model remained explicit: current-mode `brief.md` governed readiness, while `source-map.md` populated provenance-only lineage

Residual note for later phases:

- Independent review remains the only open final-phase closeout item.

## Exit Criteria

- dedicated implementation/refactor contract and integration tests exist and pass
- mode profiles no longer describe the promoted modes as staged
- skill validation passes with runtime-accurate wording
- publish, inspect, list, resume, and status remain backward-compatible
- recommendation-only posture is explicit and test-covered