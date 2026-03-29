# Validation Report: Governed Execution Adapters

## Tranche Status

- This tranche is complete through `T064`.
- Governed execution is validated end to end for `requirements`,
  `brownfield-change`, and `pr-review`.
- MCP remains modeled in policy and domain surfaces, but runtime MCP execution
  is still explicitly denied in this tranche.

## Structural Validation

Completed on `2026-03-28`.

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo nextest run`
- contract coverage passed in
  `tests/contract/runtime_evidence_contract.rs`
- contract coverage passed in
  `tests/contract/invocation_cli_contract.rs`
- contract coverage passed in
  `tests/contract/requirements_evidence_contract.rs`
- contract coverage passed in
  `tests/contract/brownfield_invocation_contract.rs`
- contract coverage passed in
  `tests/contract/pr_review_evidence_contract.rs`

Validated filesystem and contract shape:

- `.canon/runs/<run-id>/evidence.toml` is persisted for governed
  `requirements` runs
- `.canon/runs/<run-id>/invocations/<request-id>/request.toml` is persisted
- `.canon/runs/<run-id>/invocations/<request-id>/decision.toml` is persisted
- `.canon/runs/<run-id>/invocations/<request-id>/attempt-01.toml` is persisted
  both for executed requests and for policy-only denied or approval-gated
  requests
- `.canon/runs/<run-id>/links.toml` links `evidence` and `invocations`
- `.canon/traces/<run-id>.jsonl` remains append-only and summary-first
- `.config/nextest.toml` serializes the root package integration binaries so
  `cargo nextest run` stays stable even though those tests invoke `cargo run`
  internally

## Logical Validation

Completed on `2026-03-28`.

- governed `requirements` runs persist four request classes:
  repository context, AI generation, AI critique, and denied workspace
  mutation
- bounded-impact `requirements` runs complete with invocation counts, denied
  counts, evidence bundle linkage, and artifact provenance
- systemic-impact `requirements` runs enter `AwaitingApproval` before
  generation dispatch
- invocation-scoped approval with
  `approve --target invocation:<request-id>` unblocks the pending generation
  request
- `resume --run <run-id>` reuses the existing run id and re-checks input
  freshness before continuing
- denied and approval-gated requests persist explicit policy attempts with
  `Denied` or `AwaitingApproval` outcomes so the per-request evidence contract
  stays complete
- `inspect invocations` exposes policy decision, approval state, latest
  outcome, and evidence links
- `inspect evidence` exposes generation paths, validation paths, denied
  invocation refs, approval refs, decision refs, and artifact provenance refs
- requirements artifacts are derived from governed execution evidence rather
  than from renderer-only state

- governed `brownfield-change` runs persist repository-context capture, bounded
  generation, non-generative validation, and recommendation-only mutation as
  separate invocations
- systemic `brownfield-change` runs require invocation-scoped approval before
  consequential generation can proceed, and `resume` completes the same run id
- `brownfield-change` release readiness blocks when validation independence or
  evidence completeness is missing
- governed `pr-review` runs persist diff inspection and critique as separate
  invocations with a run-level evidence bundle
- `pr-review` retains bounded diff payload references under the invocation
  payload directory and links review artifacts back to the evidence bundle
- existing `review-disposition` approval flow still completes high-impact
  `pr-review` runs after explicit reviewer acceptance

## Contract Alignment

Confirmed on `2026-03-28`.

- `specs/002-governed-execution-adapters/contracts/cli-contract.md`
  matches the implemented `inspect invocations`, `inspect evidence`, and
  invocation-target approval surface across `requirements`,
  `brownfield-change`, and `pr-review`
- `specs/002-governed-execution-adapters/contracts/runtime-evidence-contract.md`
  matches the implemented invocation manifests, `evidence.toml`, summary-first
  trace linkage, retained payload refs, and policy-attempt persistence

## Independent Validation

Completed on `2026-03-28`.

- reviewed lineage separation in
  `crates/canon-engine/src/orchestrator/evidence.rs`
  plus the `requirements`, `brownfield-change`, and `pr-review` service paths:
  AI-originated generation or critique remains challenged by either
  non-generative shell validation or gate-scoped human review
- reviewed constraint enforcement in
  `crates/canon-engine/src/orchestrator/invocation.rs`
  and `defaults/policies/adapters.toml`:
  red or systemic mutation stays blocked or recommendation-only, and retained
  payload posture remains policy-bounded
- reviewed MCP runtime exclusion in
  `defaults/policies/adapters.toml` and
  `crates/canon-engine/src/orchestrator/invocation.rs`:
  `McpStdio` remains modeled but runtime-denied
- residual risk: AI critique remains synthetic in this tranche; deeper semantic
  analysis is still future work

## Milestone Acceptance

Accepted on `2026-03-28`.

- Increment `002-governed-execution-adapters` is accepted as complete for its
  stated scope.
- Acceptance is based on explicit structural validation, logical validation for
  `requirements`, `brownfield-change`, and `pr-review`, recorded independent
  review, and contract conformance against both closeout contracts.
- Artifacts in the delivered slices are confirmed to derive from governed
  execution evidence and persisted provenance, not from renderer-only state.

Confirmed exclusions and residual backlog:

- no MCP runtime execution in this increment
- no expansion into the remaining nine modeled modes
- no implementation of `verify`
- deeper semantic critique and richer non-synthetic AI validation remain future
  work
