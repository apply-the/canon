# Design Decision Log: High-Risk Operational Programs

## D-001: Complete `incident` and `migration` together

- **Status**: Accepted
- **Context**: Both modes are already modeled in Canon, both are still
  skeleton depth, and the roadmap treats them as the next missing operational
  coverage.
- **Decision**: Deliver them together as one high-risk operational completion
  feature.
- **Consequences**: Shared runtime plumbing and documentation can be designed
  once; delivery scope is larger, but the roadmap gap closes coherently.

## D-002: Retain existing artifact family names

- **Status**: Accepted
- **Context**: Method metadata and publish paths already declare incident and
  migration artifact families.
- **Decision**: Keep those artifact names and deepen them with explicit
  contract sections instead of renaming the packet shapes.
- **Consequences**: Runtime, tests, and docs align with existing mode metadata;
  the feature avoids unnecessary churn in artifact naming.

## D-003: Use explicit block-or-downgrade posture for missing evidence

- **Status**: Accepted
- **Context**: High-risk operational work is unsafe if packets imply readiness
  beyond what blast radius, compatibility, fallback, or evidence actually
  support.
- **Decision**: Surface missing evidence as explicit blockers or downgrade
  signals in packet and summary behavior.
- **Consequences**: Readers get honest packet posture; mode summaries and
  tests must recognize blocked and degraded operational outcomes.

## D-004: Implement incident before migration on shared runtime hooks

- **Status**: Accepted
- **Context**: Neither mode currently has runtime dispatch or summary support,
  and migration adds a broader gate profile than incident.
- **Decision**: Build the shared runtime hooks first, complete incident, then
  use the same surfaces to complete migration.
- **Consequences**: Delivery becomes easier to validate incrementally, and the
  shared operational plumbing is proven before the more compatibility-heavy
  mode lands.

## D-005: Replace support-state skills with runnable authored-input guidance

- **Status**: Accepted
- **Context**: Current `canon-incident` and `canon-migration` skills declare
  the modes as modeled-only.
- **Decision**: Update both skills to require authored mode bodies for real
  governed runs once the runtime is full-depth.
- **Consequences**: Docs, skills, and runtime stay consistent; skill validation
  becomes part of the feature's exit criteria.

## D-006: Reconfirm governance artifacts at implementation start

- **Status**: Accepted
- **Context**: `spec.md` and `plan.md` already capture mode, systemic-impact
  risk, scope boundaries, invariants, and split validation ownership.
- **Decision**: Treat the current governance context as implementation-ready
  without widening scope or changing the execution posture.
- **Consequences**: Phase 0 can close without reopening specification or plan
  debates, and implementation proceeds against the recorded invariants.

## D-007: Keep the incident MVP analysis-heavy and risk-gated

- **Status**: Accepted
- **Context**: The first runnable `incident` slice needs to publish a readable
  containment packet without implying that Canon is executing operational
  remediation.
- **Decision**: Implement `incident` as a generation + critique + validation
  packet that stays recommendation-only, emits all six readable artifacts, and
  uses the `Risk` gate as the approval boundary for systemic/red runs rather
  than introducing an execution/resume path.
- **Consequences**: Incident runs can become `Completed` immediately after
  gate approval; publishability depends on honest packet artifacts and
  containment/readiness evidence instead of workspace mutation semantics.

## D-008: Keep the migration MVP analysis-heavy with explicit safety packet semantics

- **Status**: Accepted
- **Context**: The first runnable `migration` slice must expose compatibility,
  sequencing, fallback, and residual risk directly in the packet without
  implying automatic cutover execution.
- **Decision**: Implement `migration` as a generation + critique + validation
  packet that stays recommendation-only, emits all six rollout artifacts, and
  uses explicit `MigrationSafety`, `Architecture`, `ReleaseReadiness`, and
  `Risk` gates instead of an execution/resume workflow.
- **Consequences**: Migration runs become `Completed` after explicit risk
  approval when the packet is otherwise credible; reviewers can inspect
  compatibility and fallback posture directly from readable artifacts and
  published docs.

## D-009: Allow readable operational packets to publish before full completion

- **Status**: Accepted
- **Context**: User Story 3 requires downstream reviewers to assess readiness,
  gaps, and fallback posture from the packet alone, including approval-gated
  and blocked high-risk runs.
- **Decision**: Permit `incident` and `migration` packets to publish when Canon
  emitted a readable artifact set, even if the run is still `AwaitingApproval`
  or `Blocked`.
- **Consequences**: Publish remains strict for ordinary modes, while
  operational docs under `docs/incidents/` and `docs/migrations/` can carry
  honest gate posture outside the runtime.

## D-010: Treat missing operational body sections as explicit gate blockers

- **Status**: Accepted
- **Context**: High-risk operational packets must fail closed when blast-radius,
  containment, compatibility, sequencing, or fallback content is still
  `NOT CAPTURED`.
- **Decision**: Make `incident` and `migration` containment/safety and
  release-readiness gates treat `NOT CAPTURED` markers as blocking evidence
  gaps rather than passing on section presence alone.
- **Consequences**: Blocked operational packets stay honest in run, status,
  publish, and inspect surfaces; reviewers can distinguish readable packets
  from credible ready-to-advance packets.

## D-011: Promote incident and migration skills to authored-body executable wrappers

- **Status**: Accepted
- **Context**: The runtime now ships both modes end to end, but the skill and
  index surfaces still described them as modeled-only wrappers.
- **Decision**: Reclassify `canon-incident` and `canon-migration` as
  `available-now` executable wrappers and require authored incident/migration
  bodies before invoking Canon.
- **Consequences**: Runtime truth, repo-local skills, support-state indexes,
  and validation scripts stay aligned.