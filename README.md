# Canon

Canon is a local-first CLI for governed AI-assisted software engineering. It
does not try to replace your editor, shell, test runner, or coding assistant.
It sits above them and forces work through explicit modes, risk
classification, artifact contracts, gates, approvals, and durable run memory.

It is not a generic agent framework, not a prompt library, and not a Copilot
replacement.

## Install

### Prerequisites

- Rust `1.94.0`
- Git
- a local repository or folder where you want Canon to write `.canon/`

### Install the CLI

Clone the repository, then install the `canon` binary from source:

```bash
git clone https://github.com/apply-the/canon.git
cd canon
cargo +1.94.0 install --path crates/canon-cli --bin canon
```

That installs `canon` into Cargo's bin directory, usually `~/.cargo/bin`.
Make sure that directory is on your `PATH`.

If you do not want to install the binary yet, you can run Canon directly from
the source tree instead:

```bash
cargo +1.94.0 run -p canon-cli -- --help
```

### Distribution Status

Today, Canon is installed from source with Cargo.

The right no-Cargo distribution path for a future release pipeline is:

- GitHub Releases with prebuilt binaries and installer scripts
- Homebrew for macOS and Linux
- winget, and optionally Scoop, for Windows
- Debian packages later, once there is a real apt repository to publish to

That packaging pipeline is not wired yet, so this README documents the install
path that works today.

## First Use

Canon is meant to be used inside a target project, not only inside the Canon
source repository.

Create or enter the project where you want to run it:

```bash
mkdir -p ~/tmp/canon-demo
cd ~/tmp/canon-demo
git init
```

Create a small input file:

```bash
cat > idea.md <<'EOF'
# Idea

Define requirements for a new internal CLI tool without letting scope drift.
EOF
```

Initialize Canon's runtime state:

```bash
canon init --output json
```

Expected result:

- `.canon/` is created in the current directory
- built-in methods are written under `.canon/methods/`
- built-in policies are written under `.canon/policies/`

Run the working `requirements` flow:

```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input idea.md \
  --output json
```

Expected result:

- Canon prints a JSON object containing a `run_id`
- the run is persisted under `.canon/runs/<run-id>/`
- the requirements artifact bundle is written under
  `.canon/artifacts/<run-id>/requirements/`

Check the run state:

```bash
canon status --run <run-id> --output json
```

Inspect the generated artifact bundle:

```bash
canon inspect artifacts --run <run-id> --output json
```

For a successful requirements run, you should see:

- `problem-statement.md`
- `constraints.md`
- `options.md`
- `tradeoffs.md`
- `scope-cuts.md`
- `decision-checklist.md`

## Other Working Flows

### Brownfield Change

```bash
canon run \
  --mode brownfield-change \
  --risk bounded-impact \
  --zone yellow \
  --owner staff-engineer \
  --input brownfield.md \
  --output json
```

What it does:

- emits the brownfield artifact bundle under
  `.canon/artifacts/<run-id>/brownfield-change/`
- blocks the run if preserved behavior is underspecified
- moves systemic-impact or red-zone work into `AwaitingApproval`

### PR Review

```bash
canon run \
  --mode pr-review \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input refs/heads/main \
  --input HEAD \
  --output json
```

What it does:

- collects changed surfaces from a git diff
- emits the review packet under `.canon/artifacts/<run-id>/pr-review/`
- keeps high-impact findings explicit in `review-summary.md`
- requires explicit disposition for unresolved must-fix review findings

### Approve a Gated Run

```bash
canon approve \
  --run <run-id> \
  --gate review-disposition \
  --by principal-engineer \
  --decision approve \
  --rationale "Accept the bounded review risk with named ownership."
```

What it does:

- persists an approval record under `.canon/runs/<run-id>/approvals/`
- re-evaluates the run against its current gates

### Resume a Blocked Run

```bash
canon resume --run <run-id>
```

What it does:

- reloads persisted artifacts and gate state
- refuses to continue silently if the input context changed
- allows repaired blocked runs to move to `Completed`

## What Canon Writes

Canon stores its durable runtime state under `.canon/`:

```text
.canon/
├── artifacts/
├── decisions/
├── methods/
├── policies/
├── runs/
├── sessions/
└── traces/
```

At a high level:

- `runs/` stores per-run manifests, state, gates, approvals, and verification
- `artifacts/` stores emitted artifact bundles
- `methods/` stores materialized method definitions
- `policies/` stores materialized policy definitions
- `decisions/` is durable decision memory
- `traces/` stores adapter and execution evidence

The point is that Canon does not treat chat as system memory. The filesystem is
the system of record.

## Current v0.1 Status

Working today:

- `init`
- `run --mode requirements`
- `run --mode brownfield-change`
- `run --mode pr-review`
- `status`
- `inspect modes|methods|policies|artifacts`
- `approve`
- `resume`

Mode depth today:

- `requirements`, `brownfield-change`, and `pr-review` are fully runnable
- the other nine modes already exist in the typed domain model and defaults
- those nine are not yet implemented as full runtime flows

Not implemented yet:

- `verify`
- deeper semantic review beyond the current local diff heuristics
- end-to-end runtime execution for `discovery`, `greenfield`, `architecture`,
  `implementation`, `refactor`, `verification`, `review`, `incident`, and
  `migration`

## Development

Useful local commands:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo nextest run
cargo +1.94.0 test --all-targets --all-features
```

Install the local git hooks with:

```bash
./scripts/install-hooks.sh
```

## Further Reading

- Constitution: [`.specify/memory/constitution.md`](.specify/memory/constitution.md)
- Product specification: [`specs/001-canon-spec/spec.md`](specs/001-canon-spec/spec.md)
- Implementation plan: [`specs/001-canon-spec/plan.md`](specs/001-canon-spec/plan.md)
