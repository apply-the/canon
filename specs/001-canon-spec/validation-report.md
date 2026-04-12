# Validation Report: Canon v0.1 Planning Phase

## Validation Scope

This report defines how the v0.1 implementation will be validated without
collapsing generation and validation into the same reasoning path.

## Implementation Kickoff Evidence

- Phase 0 governance updates must land before product code behavior changes.
- Phase 1 setup artifacts must establish the workspace, toolchain, and default
  methods or policies before foundational behavior work starts.
- TDD becomes mandatory once behavior-bearing files move beyond empty
  scaffolding.

## Contract Verification Checkpoints

- CLI contract evidence must be captured in `tests/contract/cli_contract.rs`
  and the story-specific contract tests.
- Runtime filesystem evidence must be captured in
  `tests/contract/runtime_filesystem.rs`.
- End-to-end mode evidence must be captured in `tests/integration/*.rs`.
- Final independent review evidence must be attached before completion claims.

## MVP Acceptance Evidence

### Requirements Mode

- `tests/integration/requirements_run.rs` must prove that a requirements run
  persists a run folder, state files, an artifact contract, and the six
  required artifacts.
- `tests/contract/requirements_contract.rs` must prove that `inspect artifacts`
  exposes the requirements artifact bundle.
- `specs/001-canon-spec/decision-log.md` must record the MVP completion
  decision after the requirements slice is green.

### Requirements Mode Completion Evidence

- `cargo test -p canon-workspace --test requirements_run` passed on
  2026-03-27 and proved that `run --mode requirements` persists
  `run.toml`, `context.toml`, `artifact-contract.toml`, `state.toml`,
  `links.toml`, the gate directory, the verification records, and the six
  requirements artifacts.
- `cargo test -p canon-workspace --test requirements_contract`
  passed on 2026-03-27 and locked the `inspect artifacts` JSON output plus the
  persisted `artifact-contract.toml` into checked-in snapshots under
  `tests/contract/snapshots/`.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  and `cargo test` all passed on 2026-03-27 for the current repository state.
- Self-critique and adversarial-critique records are now emitted under
  `.canon/runs/<run-id>/verification/` as part of the requirements run bundle.

### Brownfield Change Mode

- `tests/integration/brownfield_run.rs` must prove that a brownfield run blocks
  when preserved behavior is underspecified and completes when the bounded
  change context is explicit.
- `tests/contract/brownfield_contract.rs` must prove that blocked brownfield
  runs use semantic exit codes, that systemic-impact work requires approval,
  and that `resume` rejects stale context while allowing re-evaluation of fixed
  artifacts.
- `specs/001-canon-spec/decision-log.md` must record why brownfield approval
  and resume semantics were implemented before mutating adapter execution.

### Brownfield Change Mode Completion Evidence

- `cargo test --test brownfield_run -- --nocapture` passed on 2026-03-27 and
  proved that `run --mode brownfield-change` persists a blocked run when
  `legacy-invariants.md` or `change-surface.md` are incomplete, while a
  complete bounded brief yields a `Completed` state and the full brownfield
  artifact bundle.
- `cargo test --test brownfield_contract -- --nocapture` passed on 2026-03-27
  and locked three contract behaviors: blocked brownfield runs exit with code
  `2`, systemic-impact brownfield runs exit with code `3` until `approve`
  records a risk approval, and `resume` exits with code `5` for stale input
  context while allowing repaired artifact bundles to complete.
- Brownfield runs now persist `ApprovalRecord` files under
  `.canon/runs/<run-id>/approvals/` and re-evaluate gates after both
  `approve` and `resume`.
- Recommendation-only enforcement for mutating adapters remains pending in
  the concrete adapter surfaces that now exist; the remaining brownfield work
  moves on to PR review rather than more foundational scaffolding.

### Foundational Policy and Adapter Evidence

- `cargo test --test policy_and_traces -- --nocapture` passed on 2026-03-27 and
  proved three foundational guarantees: schema-bound local policy overrides
  merge into the effective `PolicySet`, unknown override fields fail closed, and
  governed runs emit a JSONL filesystem trace under `.canon/traces/` that is
  linked from `links.toml`.
- `cargo test --test adapter_policy -- --nocapture` passed on 2026-03-27 and
  proved that `ShellAdapter` distinguishes read-only from mutating requests,
  blocks mutating execution when permission is denied, and marks systemic or
  red-zone brownfield mutation as `RecommendationOnly` rather than implicitly
  executable.
- The foundational dependency chain for `T051` is now closed: `T025`, `T028`,
  and `T029` are complete, so the next pending work item in dependency order is
  the PR review validation slice (`T053`-`T055`).

### PR Review Mode

- `tests/integration/pr_review_run.rs` must prove that a PR review run emits
  the full review packet, maps changed surfaces into `pr-analysis.md`, and
  persists the bundle under `.canon/artifacts/<run-id>/pr-review/`.
- `tests/contract/pr_review_contract.rs` must prove that unresolved high-impact
  findings return exit code `3`, that `review-summary.md` keeps the must-fix
  disposition explicit, and that `approve --gate review-disposition` can move a
  run to `Completed`.
- `specs/001-canon-spec/decision-log.md` must record why the first PR review
  implementation uses typed diff heuristics and approval-driven disposition
  instead of generic autonomous review behavior.

### PR Review Mode Completion Evidence

- `cargo test --test pr_review_run -- --nocapture` passed on 2026-03-27 and
  proved that `run --mode pr-review` emits the full review packet, maps changed
  source surfaces into `pr-analysis.md`, and completes when the diff stays
  bounded and test-aware.
- `cargo test --test pr_review_contract -- --nocapture` passed on 2026-03-27
  and locked three review behaviors: high-impact contract or boundary changes
  return exit code `3`, `review-summary.md` keeps unresolved must-fix findings
  explicit until disposition, and `approve --gate review-disposition` moves the
  run to `Completed`.
- PR review runs now persist shell-adapter diff traces into
  `.canon/traces/<run-id>.jsonl`, emit the seven review artifacts under
  `.canon/artifacts/<run-id>/pr-review/`, and record verification evidence
  through the same verification directory used by the other deep modes.
- The next pending tasks in dependency order are the final verification and
  compliance slice (`T062`-`T068`).

### Mode Taxonomy and Contract Coverage

- `cargo test --test mode_profiles -- --nocapture` passed on 2026-03-27 and
  proved that all twelve modes have typed `ModeProfile` coverage while the
  nine non-MVP modes remain explicitly staged as `ContractOnly` or `Skeleton`.
- `cargo test --test inspect_modes -- --nocapture` passed on 2026-03-27 and
  locked the public `inspect modes` contract to the full twelve-mode taxonomy
  in the expected order.

### Repository Quality and CI Evidence

- `.github/workflows/ci.yml` now enforces five repository gates:
  `fmt-clippy`, `test`, `msrv`, `deny`, and `cross-platform-build`.
- The cross-platform build matrix is staged exactly as planned: Linux GNU and
  MUSL, macOS `aarch64` and `x86_64`, and Windows `x86_64`. Linux ARM and
  Windows ARM remain explicitly staged for later work rather than being
  implied without evidence.
- `deny.toml` now encodes the permissive-license policy with an allowlist plus
  explicit copyleft denial, and the CI workflow runs
  `cargo deny check licenses advisories bans sources`.
- `.githooks/pre-commit` and `scripts/install-hooks.sh` now enforce a fail-fast
  local gate from the repository root for `cargo fmt --check`,
  `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- `README.md` and `specs/001-canon-spec/quickstart.md` now reflect the real
  v0.1 state: `requirements`, `brownfield-change`, and `pr-review` are runnable;
  `approve` and `resume` work; `verify` remains intentionally unimplemented.

### Final Completion Evidence

- `cargo fmt --check` passed on 2026-03-27 after the final phase changes.
- `cargo clippy --all-targets --all-features -- -D warnings` passed on
  2026-03-27 after the final phase changes.
- `cargo test` passed on 2026-03-27 after the final phase changes.
- `cargo nextest run` passed on 2026-03-27 with 20 tests run and 20 tests
  passed.
- `cargo +1.94.1 test --all-targets --all-features` passed on 2026-03-27 as
  the explicit local MSRV verification strategy.
- `./scripts/install-hooks.sh` passed on 2026-03-27 and installed the local
  `core.hooksPath` to `.githooks`.
- `cargo deny` and `cargo msrv` are not installed in the local environment, so
  their enforcement is delegated to the committed CI workflow rather than
  claimed as locally verified.

### Final Adversarial Closeout

- The final adversarial check challenged three likely drift points:
  stale README status, unguarded mode taxonomy drift, and unverifiable platform
  support claims.
- README and quickstart no longer claim that only `requirements` works or that
  `verify` is available.
- Mode taxonomy drift is now blocked by dedicated tests over both
  `ModeProfile` values and `inspect modes`.
- Platform support remains intentionally staged. Canon now claims verified CI
  smoke builds for Linux GNU and MUSL, macOS `aarch64` and `x86_64`, and
  Windows `x86_64`, while Linux ARM and Windows ARM remain future work.
- Residual v0.1 risk remains explicit: standalone verification execution is not
  yet implemented, so verification evidence is emitted during mode runs rather
  than through the `verify` command.

## 1. Structural Validation

Structural validation must confirm:

- mode, risk, zone, and artifact contract are required before a run can
  progress
- all twelve modes exist as first-class domain concepts
- the three MVP modes have complete artifact and gate definitions
- `.canon/` persistence layout is stable and versioned
- run manifests, gate outcomes, approvals, decisions, and traces are all
  independently persisted
- CLI contract and runtime filesystem contract remain compatible

Evidence sources:

- unit tests for domain rules and manifest schemas
- contract tests for CLI output and filesystem layout
- snapshot tests for emitted artifacts

## 2. Logical Validation

Logical validation must prove:

- the stricter of risk and zone governs adapter permissions
- gates fail closed when artifacts or approvals are missing
- `brownfield-change` blocks without preserved invariants
- `pr-review` cannot complete without the full review artifact bundle
- resume refuses to continue silently on stale repository context
- red-zone mutating execution degrades to recommendation-only behavior

Evidence sources:

- focused integration tests for run flow
- gate enforcement tests
- resume and stale-context tests
- adapter isolation tests

## 3. Independent Validation Layers

| Layer | Primary Owner | Evidence |
| --- | --- | --- |
| Self-critique | Engine or AI critique adapter | verification artifact under `.canon/runs/<run-id>/verification/` |
| Adversarial critique | Separate critique step or reviewer | adversarial review artifact linked from the run |
| Peer review | Human reviewer | review artifact or approval record |
| Architectural review | Human owner or architecture reviewer | decision record plus gate approval |

Rules:

- self-critique alone is never sufficient for bounded or systemic work
- generated artifacts agreeing with generated tests do not count as independent
  validation
- peer and architectural review are mandatory for structural or systemic impact

## 4. CI Validation

The CI pipeline must enforce:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo nextest run`
- `cargo msrv verify`, or the pinned `cargo +1.94.1 test` fallback strategy
- `cargo deny check licenses advisories bans sources`

## 5. Manual Review Expectations

Before implementation tasks are generated:

- the architecture in `plan.md` must receive human review
- the decision log must be checked for consistency with the product thesis
- the CLI and runtime filesystem contracts must be accepted as the v0.1 public
  surfaces

Before the implementation phase is marked complete:

- a code review run must exist
- verification evidence must be attached to the relevant run
- any gate override must have a linked approval and decision record

## 6. Residual Validation Risks

- external tool behavior, especially shell quoting and Copilot CLI output, may
  vary across platforms
- contract-only modes can drift if future tasks add behavior without updating
  `ModeProfile` definitions
- repository-local policy overrides can weaken governance if schema validation
  is too loose

These risks are acceptable for v0.1 only because the plan keeps the engine
fail-closed and treats platform parity as a staged release concern rather than a
silent assumption.
