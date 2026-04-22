# Validation Report: Mode Context Split

## Scope

Track structural, logical, and independent validation for the `change` rename, explicit `system_context` contract, public-surface cleanup, and coverage recovery.

## Implementation Kickoff

- **Started**: 2026-04-20 via `/speckit.implement`
- **Checklist Status**:

  | Checklist | Total | Completed | Incomplete | Status |
  |-----------|-------|-----------|------------|--------|
  | requirements.md | 20 | 20 | 0 | PASS |

- **Pre-Hook Status**: no `.specify/extensions.yml` file was present, so no `before_implement` hooks executed
- **Setup Verification**: `.gitignore` already covered Rust build, coverage, and universal editor junk patterns; no additional ignore files were required for Docker, JS, Terraform, or Helm tooling

## Structural Validation

- **Status**: pass
- **Verified**:
  - mode parsing, help text, inspect output, skill metadata, and documentation no longer expose `brownfield` or `greenfield` terminology on public repo surfaces
  - canonical input hints and emitted artifact paths use `change`
  - `context.toml`, status output, and inspect summaries surface `system_context` consistently when present and omit it when not supplied
- **Evidence**:
  - `cargo fmt --check`: passed
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed
  - `/bin/bash scripts/validate-canon-skills.sh`: passed
  - `pwsh -File scripts/validate-canon-skills.ps1`: passed
  - public-surface audit across docs, skills, defaults, and validators: clean after the rename and context split

## Logical Validation

- **Status**: pass
- **Verified**:
  - required-versus-optional `system_context` enforcement across the mode matrix
  - legacy-name rejection and explicit rejection of `change + new`
  - `change + existing` end-to-end artifact emission, preserved gate behavior, and persisted `context.toml`
  - renamed artifact links and surfaced `system_context` in status and inspect output
  - renderer and summary branches exercised in CLI output, markdown artifacts, and inspection flows
- **Evidence**:
  - `cargo nextest run`: passed, `109` tests run and `109` passed
  - renamed and expanded suites include `change_contract`, `change_invocation_contract`, `change_run`, `change_governed_execution`, architecture and system-shaping contract coverage, and inspect/runtime filesystem coverage
  - `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`: completed successfully and generated `lcov.info`

## Independent Validation

- **Prepared Review Packet**:
  - semantic split remains explicit at the key boundaries in `Mode`, classifier validation, persisted run context, and change gate evaluation
  - documentation truth and runtime truth agree on `change`, `system_context`, and the required-versus-optional mode matrix
  - coverage evidence is recorded with both overall workspace coverage and changed-line patch coverage so the acceptance basis is explicit
- **External Gate Still Required**:
  - named human merge ownership and an independent human review remain required because this feature stays in the systemic-impact class
  - this report prepares the review packet but does not self-approve that gate

## Review Checkpoints

- **Checkpoint A**: review foundational mode/context plumbing before story-specific behavior ships
- **Checkpoint B**: review `change + existing`, rejected `change + new`, and the required-context matrix before documentation cleanup is considered final
- **Checkpoint C**: review coverage evidence, public-surface truth, and modified-file inventory before merge approval

## Merge Ownership

- **Required**: named human owner before merge because the feature remains systemic-impact work
- **Implementation Role**: prepare evidence, task completion state, and modified-file inventory without self-approving the review gate

## Coverage Hotspots To Prove

- `crates/canon-engine/src/orchestrator/service.rs`
- `crates/canon-engine/src/orchestrator/gatekeeper.rs`
- `crates/canon-adapters/src/copilot_cli.rs`
- `crates/canon-engine/src/artifacts/contract.rs`
- `crates/canon-engine/src/artifacts/markdown.rs`
- `crates/canon-cli/src/app.rs`
- `crates/canon-cli/src/commands/run.rs`
- `crates/canon-engine/src/persistence/store.rs`

## Coverage Evidence

- **Workspace line coverage**: `83.43%` (`11276 / 13515`)
- **Tracked hotspot aggregate**: `81.13%` (`8079 / 9958`)
- **Changed executable-line coverage under `crates/`**: `87.84%` (`354 / 403`)
- **Acceptance interpretation**:
  - the overall workspace line rate remains below the provisional `85%` mark
  - the feature's touched executable patch exceeds the agreed threshold, which is the relevant gate for this refactor's changed surface
  - `lcov.info` is preserved as the durable artifact for review

## Modified File Inventory

- **Runtime and CLI surfaces**: `crates/canon-engine/src/domain/{mode,run,execution,gate}.rs`, `crates/canon-engine/src/orchestrator/{classifier,gatekeeper,invocation,service,verification_runner}.rs`, `crates/canon-engine/src/artifacts/{contract,markdown}.rs`, `crates/canon-engine/src/persistence/{manifests,store}.rs`, `crates/canon-engine/src/modes/{change.rs,modes.rs}`, `crates/canon-cli/src/{app.rs,commands/run.rs,output.rs}`
- **Public docs, skills, and defaults**: `README.md`, `MODE_GUIDE.md`, `NEXT_FEATURES.md`, `AGENTS.md`, `.specify/templates/{spec-template.md,plan-template.md}`, `.specify/memory/constitution.md`, `.agents/skills/**`, `defaults/embedded-skills/**`, `defaults/methods/**`, `defaults/policies/{adapters.toml,gates.toml}`
- **Validators and scripts**: `scripts/validate-canon-skills.sh`, `scripts/validate-canon-skills.ps1`, `.agents/skills/canon-shared/scripts/{check-runtime.sh,check-runtime.ps1}`, `defaults/embedded-skills/canon-shared/scripts/{check-runtime.sh,check-runtime.ps1}`
- **Tests**: renamed `brownfield_*` suites to `change_*`, expanded contract and integration coverage under `tests/contract/**` and `tests/integration/**`, and updated runtime/CLI regression coverage including `tests/approve_resolution.rs`, `tests/direct_runtime_coverage.rs`, `tests/inspect_modes.rs`, `tests/mode_profiles.rs`, and related filesystem/evidence suites

## Invariant Check

- mode and `system_context` remain separate axes everywhere Canon parses, persists, validates, or explains a run
- public Canon surfaces no longer teach `brownfield` or `greenfield` naming
- `change + existing` preserves the bounded-change gate stack and rejects `change + new`
- required-context modes fail before run creation when `system_context` is missing
- optional-context modes persist no invented default

## Results

- **Status**: implementation complete; external independent human review pending
- **Phase 0**: complete
- **Phase 1**: complete
- **Phase 2**: complete
- **Phase 3 / US1**: complete
- **Phase 4 / US2**: complete
- **Phase 5 / US3**: complete
- **Phase 6 / US4**: complete
- **Exit Criteria**:
  - no public `brownfield` or `greenfield` vocabulary remains: pass
  - `change + existing` preserves bounded-change behavior: pass
  - missing required `system_context` and invalid combinations fail before run creation: pass
  - optional-context modes persist no invented default: pass
  - touched patch reaches at least `85%` line coverage: pass on changed executable lines (`87.84%`)
  - named human owner performs independent review before merge: pending external gate