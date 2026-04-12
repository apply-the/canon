# Quickstart: Canon Install-First Workflow

Canon is the product, repository, workspace, and CLI name.

## Prerequisites

- Canon installed from a release archive using the repository install guide
- Git available on the local machine
- Copilot CLI installed if AI generation or critique will be exercised
- optional local MCP-compatible tools if the MCP adapter will be enabled

## Verify the Installed CLI

```bash
canon --version
command -v canon
```

Expected result:

- the version command reports the installed Canon release
- the resolved command path points to the PATH-installed binary, not a Cargo
  build inside the repository

## Initialize Runtime State in a Target Repository

```bash
canon init --output json
```

Expected result:

- `.canon/` is created
- built-in method and policy files are materialized
- the current repository becomes an executable governed workspace

## Start a Requirements Run

```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner "product-lead" \
  --input docs/idea.md
```

Expected result:

- a new run directory appears under `.canon/runs/<run-id>/`
- an artifact contract is created before any gate passes
- the requirements artifact bundle is emitted under
  `.canon/artifacts/<run-id>/requirements/`

## Start a Brownfield Change Run

```bash
canon run \
  --mode brownfield-change \
  --risk bounded-impact \
  --zone yellow \
  --owner "staff-engineer" \
  --input path/to/change-request.md
```

Expected result:

- the run blocks if `legacy-invariants.md` and `change-surface.md` are missing
- systemic-impact or red-zone work moves into `AwaitingApproval`
- repaired artifact bundles can later be resumed without re-running from
  scratch

## Start a PR Review Run

```bash
canon run \
  --mode pr-review \
  --risk bounded-impact \
  --zone yellow \
  --owner "reviewer" \
  --input refs/heads/main \
  --input HEAD
```

Expected result:

- the review artifact set is emitted under `.canon/artifacts/<run-id>/pr-review/`
- changed surfaces are mapped into `pr-analysis.md`
- `review-summary.md` reflects unresolved must-fix findings explicitly

## Resume a Blocked Run

```bash
canon resume --run <run-id>
```

Expected result:

- the engine reloads the run state and fingerprints
- if the repository changed, the run is blocked until reuse, refresh, or fork
  is chosen
- otherwise execution continues from the first incomplete step

## Record an Approval

```bash
canon approve \
  --run <run-id> \
  --gate review-disposition \
  --by "staff-engineer" \
  --decision approve \
  --rationale "Boundary change accepted with explicit follow-up review ownership"
```

Expected result:

- the approval is persisted under `.canon/runs/<run-id>/approvals/`
- the run is re-evaluated against its current gates
- unresolved review or risk disposition can move from `AwaitingApproval` to
  `Completed`

## Contributor / Development Bootstrap

Use this only when developing Canon itself.

```bash
rustup toolchain install 1.94.1 --profile minimal --component rustfmt --component clippy
cargo build
./scripts/install-hooks.sh
```

Validation checkpoints during implementation:

- CLI contract coverage lives in `tests/contract/cli_contract.rs`
- Runtime filesystem coverage lives in `tests/contract/runtime_filesystem.rs`
- End-to-end mode flows live in `tests/integration/*.rs`

## Verification Command Status

The `verify` command exists in the CLI surface, but it is not implemented in
v0.1 yet. Verification records are currently emitted as part of the deep mode
run flows rather than through a standalone command.
