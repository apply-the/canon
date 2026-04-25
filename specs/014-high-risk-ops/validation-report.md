# Validation Report: High-Risk Operational Programs

## Summary

Validation for this feature will prove that `incident` and `migration` move
from modeled skeletons to full-depth governed modes without weakening Canon's
artifact, gating, or recommendation-only guarantees.

## Validation Ownership

- **Generation owners**: runtime dispatch, gate evaluation, artifact
  contracts, markdown rendering, summary surfaces, docs, and skill updates
- **Structural validators**: formatter, linter, repository consistency, and
  skill validation checks
- **Logical validators**: contract, integration, publish, and runtime coverage
  tests for incident and migration packet behavior
- **Independent validators**: separate review of emitted incident and
  migration packets to confirm readability, honesty about evidence gaps, and
  preservation of recommendation-only posture

## Implementation Kickoff

- Governance context in `spec.md` and `plan.md` was reconfirmed unchanged at
  implementation start.
- Decision ownership remains in `decision-log.md`; validation evidence remains
  in this file and the emitted `.canon/` run bundles.
- The first implementation slice will close Phase 1 setup and Phase 2 shared
  plumbing before entering the `incident` MVP.

## Structural Validation

- **Status**: Passed
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `/bin/bash scripts/validate-canon-skills.sh`
- `cargo test --test incident_contract --test migration_contract`
- Result expectation: method metadata, runtime dispatch, skill mirrors, and
  docs remain structurally consistent after incident and migration become
  runnable.
- Result: formatting, linting, and skill validation all passed after the
  operational publish/inspect and skill-surface changes; no warning-level
  regressions remained under the full workspace `clippy` gate.

## Logical Validation

- **Status**: Passed
- Targeted tests for `tests/incident_contract.rs`, `tests/incident_run.rs`,
  `tests/migration_contract.rs`, `tests/migration_run.rs`,
  `tests/integration/incident_publish.rs`, and
  `tests/integration/migration_publish.rs`
- Runtime coverage in `tests/direct_runtime_coverage.rs` exercising completed,
  blocked, downgraded, and approval-gated high-risk runs
- Non-regression validation in `tests/contract/inspect_modes.rs`,
  `tests/integration/mode_profiles.rs`, `tests/policy_and_traces.rs`, and
  `tests/artifact_confinement.rs`
- Result expectation: both modes emit honest packets with explicit gate and
  evidence posture, and existing modes remain unaffected.
- Result: the combined operational suite passed across contracts, runtime,
  publish, inspect, direct engine coverage, mode visibility, and artifact
  confinement. Approval-gated `incident` runs, blocked `migration` runs, and
  blocked `incident` evidence-gap paths all surfaced the expected state,
  gates, and publish behavior.

## Incident MVP Evidence

- **Status**: Passed
- `cargo test --test incident_contract`
- `cargo test --test incident_run`
- `cargo test --test direct_runtime_coverage incident_direct_run_produces_artifacts_and_completes_after_risk_approval`
- `cargo test -p canon-engine incident_gates_require_containment_artifacts_and_risk_approval`
- Result: `incident` now emits the six-artifact containment packet, preserves
  recommendation-only posture in run/status summaries, requires explicit risk
  approval for systemic/red runs, and becomes publishable after gate approval.

## Migration MVP Evidence

- **Status**: Passed
- `cargo test --test migration_contract`
- `cargo test --test migration_run`
- `cargo test --test direct_runtime_coverage migration_direct_run_produces_artifacts_and_completes_after_risk_approval`
- `cargo test -p canon-engine migration_gates_require_migration_safety_packet_and_pass_when_present`
- Result: `migration` now emits the six-artifact compatibility packet,
  preserves recommendation-only posture in run/status summaries, requires
  explicit risk approval for systemic runs, and becomes publishable after gate
  approval.

## Publish And Inspect Evidence

- **Status**: Passed
- `cargo test --test incident_publish --test migration_publish`
- `cargo test --test runtime_filesystem engine_publish_allows_approval_gated_operational_packets`
- `cargo test -p canon-engine resolved_execution_posture_label_for_mode_defaults_operational_modes_to_recommendation_only`
- `cargo test --test runtime_evidence_contract incident_evidence_surface_keeps_recommendation_only_posture_and_artifact_links`
- `cargo test --test policy_and_traces blocked_migration_run_persists_traces_and_operational_status_surfaces`
- `/bin/bash scripts/validate-canon-skills.sh`
- Result: approval-gated `incident` packets and blocked `migration` packets now
  publish through the operational docs surface, blocked gates stay explicit
  when `NOT CAPTURED` markers remain, `inspect evidence` preserves the
  `recommendation-only` posture for both modes, and repo-local skills now
  describe runnable authored-input behavior honestly.

## Independent Validation

- **Status**: Passed
- Generated a representative approval-gated `incident` packet and a blocked
  `migration` packet in separate temporary repositories, then published both
  to their public docs surfaces.
- Reviewed only the published markdown artifacts, not the internal manifests.
- Independent review findings:
  - the published `incident` packet makes scope, current state, containment
    order, stop conditions, verification checks, and recommendation-only
    release posture readable from the markdown alone.
  - the published `migration` packet makes the current/target boundary
    readable, and the missing fallback posture is explicit in both
    `fallback-plan.md` (`NOT CAPTURED`) and `migration-verification-report.md`
    (`fallback credibility is not yet established`).
  - neither packet implies that Canon executed remediation, cutover, rollback,
    or other privileged operational action.

## Exit Criteria

- **Passed**: `incident` emits all required artifacts and summary surfaces
- **Passed**: `migration` emits all required artifacts and summary surfaces
- **Passed**: gate outcomes remain explicit for blocked, downgraded, and
  approval-gated runs
- **Passed**: publish output remains readable under `docs/incidents/` and
  `docs/migrations/`
- **Passed**: docs and skill surfaces describe runnable behavior honestly

## Closeout

- **Status**: Passed
- High-Risk Operational Programs is complete: both operational modes are full
  depth, publishable, inspectable, recommendation-only, and documented
  honestly across runtime, skills, and public docs.