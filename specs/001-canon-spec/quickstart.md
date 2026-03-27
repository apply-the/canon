# Quickstart: Canon v0.1

Canon is the product, repository, workspace, and CLI name.

## Prerequisites

- Rust 1.94.0 installed through `rustup`
- Git available on the local machine
- Copilot CLI installed if AI generation or critique will be exercised
- optional local MCP-compatible tools if the MCP adapter will be enabled

## Bootstrap the Repository

```bash
rustup toolchain install 1.94.0 --profile minimal --component rustfmt --component clippy
cargo build
./scripts/install-hooks.sh
```

Validation checkpoints during implementation:

- CLI contract coverage lives in `tests/contract/cli_contract.rs`
- Runtime filesystem coverage lives in `tests/contract/runtime_filesystem.rs`
- End-to-end mode flows live in `tests/integration/*.rs`

## Initialize Runtime State in a Target Repository

```bash
cargo run -p canon-cli -- init
```

Expected result:

- `.canon/` is created
- built-in method and policy files are materialized
- the current repository becomes an executable governed workspace

## Start a Requirements Run

```bash
cargo run -p canon-cli -- run \
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
cargo run -p canon-cli -- run \
  --mode brownfield-change \
  --risk bounded-impact \
  --zone yellow \
  --owner "staff-engineer" \
  --input path/to/change-request.md
```

Expected result:

- the run blocks if `legacy-invariants.md` and `change-surface.md` are missing
- systemic-impact or red-zone work moves into `AwaitingApproval`
- repaired artifact bundles can later be resumed without re-running from scratch

## Start a PR Review Run

```bash
cargo run -p canon-cli -- run \
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
cargo run -p canon-cli -- resume --run <run-id>
```

Expected result:

- the engine reloads the run state and fingerprints
- if the repository changed, the run is blocked until reuse, refresh, or fork
  is chosen
- otherwise execution continues from the first incomplete step

## Record an Approval

```bash
cargo run -p canon-cli -- approve \
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

## Verification Command Status

The `verify` command exists in the CLI surface, but it is not implemented in
v0.1 yet. Verification records are currently emitted as part of the deep mode
run flows rather than through a standalone command.
