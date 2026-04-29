# Decision Log: Supply Chain And Legacy Analysis Mode

## D-001 Publish Surface

- **Decision**: Publish supply-chain packets to `docs/supply-chain/<RUN_ID>/`.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: The path is readable, domain-focused, and avoids awkward
  pluralization while staying consistent with other packet publish surfaces.

## D-002 Adapter Strategy

- **Decision**: Reuse `ShellAdapter` and `FilesystemAdapter` rather than
  introducing a dedicated scanner adapter in the first slice.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: The existing adapters already support bounded repository reads,
  governed shell execution, evidence capture, and tool outcome recording.

## D-003 Canonical Input Binding

- **Decision**: Keep canonical input binding on the exact mode string:
  `canon-input/supply-chain-analysis.md` and `canon-input/supply-chain-analysis/`.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: Repo-local guidance for file-backed modes already standardizes
  on `<mode>.md` and `<mode>/` as the only canonical bind points.

## D-004 Missing Scanner Posture

- **Decision**: Model missing scanners as explicit coverage-gap evidence and
  scanner decision records rather than treating them as invisible warnings.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: The packet must remain honest about partial coverage while
  still preserving useful bounded findings.

## D-005 Closeout Contract

- **Decision**: Make `0.24.0` version synchronization the first task and the
  high-coverage plus docs/examples/roadmap/formatting/lint remediation the last
  task.
- **Status**: Accepted
- **Date**: 2026-04-29
- **Rationale**: The user explicitly requested this delivery order as part of
  the feature contract.

## User Story 1 Decisions

- **US1-D1 Artifact contract enforcement**: Supply-chain packet artifacts keep
  canonical H2 requirements aligned with contract validation, and missing
  authored decisions are rendered with explicit markers instead of hidden
  omissions.
- **US1-D2 Risk gate behavior**: Risk blockers and approval-needed outcomes are
  consolidated into a single gate-kind result to avoid duplicate `Risk` gate
  records with conflicting statuses.
- **US1-D3 Publish behavior**: `supply-chain-analysis` packets remain publishable
  to `docs/supply-chain/<RUN_ID>/` even when recommendation-only posture or
  approvals keep execution blocked.

## User Story 2 Decisions

- **US2-D1 Clarification posture**: `inspect clarity --mode supply-chain-analysis`
  asks targeted questions for licensing posture, distribution model, declared
  scope, and scanner/tool decisions before runtime execution proceeds.
- **US2-D2 Coverage-gap honesty**: When scanner decisions are skipped or
  replaced, runtime derives explicit `Coverage Gaps` entries and preserves those
  as first-class evidence in packet artifacts.
- **US2-D3 Shared helper parity**: Runtime compatibility references, runtime
  helper scripts, and skill indexes are kept in sync between
  `defaults/embedded-skills/` and `.agents/skills/` for
  `supply-chain-analysis` discoverability.

## User Story 3 Decisions

- **US3-D1 Release surface inclusion**: `README.md`, `docs/guides/modes.md`, and
  `ROADMAP.md` explicitly include `supply-chain-analysis` in the public mode and
  publishability surface.
- **US3-D2 Validator parity**: Canon skill validators now treat
  `canon-supply-chain-analysis` as `available-now` and enforce canonical input
  hints in both bash and PowerShell preflight checks.
- **US3-D3 Coverage gate scope**: Full-file line coverage for several shared
  orchestrator files remains below the 85% closeout target despite broad suite
  execution; this is tracked as an open closeout item under `T037`.