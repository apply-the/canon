# Canon

[![CI](https://github.com/apply-the/canon/actions/workflows/ci.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/ci.yml)
[![Lint](https://github.com/apply-the/canon/actions/workflows/lint.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/lint.yml)
[![Vulnerabilities](https://github.com/apply-the/canon/actions/workflows/vulnerabilities.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/vulnerabilities.yml)
[![Coverage](https://codecov.io/gh/apply-the/canon/graph/badge.svg?token=JZ4IPF51DH)](https://codecov.io/gh/apply-the/canon)

**Canon is a local CLI for governed AI-assisted software engineering. You run it inside a repository to start bounded work, record approvals and evidence, and publish durable packets when they are ready.**

The current delivery line in this repository targets Canon `0.24.0`.

## What Canon Does

Canon is the product entrypoint. The shipped binary is `canon`.

Use it when you want AI-assisted work to stay inspectable and bounded:

- Start a governed run with an explicit `mode`, `risk`, `zone`, `owner`, and authored input.
- Keep generated artifacts, evidence, approvals, and invocation history under `.canon/`.
- Inspect what happened with `status`, `inspect`, `approve`, `resume`, and `publish`.
- Work with an AI assistant through repo-local skills without hiding the CLI contract.

Canon is not a generic agent framework and it is not an opaque agent loop. It is a local-first method engine that keeps the control surface on disk.

## Install

Canon ships as a single binary named `canon`.

### Prebuilt Binary

Download the latest release from [Releases](https://github.com/apply-the/canon/releases).

**macOS / Linux**

```bash
VERSION=vX.Y.Z
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCHIVE="canon-${VERSION}-${OS}-${ARCH}.tar.gz"

curl -LO "https://github.com/apply-the/canon/releases/download/${VERSION}/${ARCHIVE}"
tar -xzf "${ARCHIVE}"
install -m 0755 canon "$HOME/.local/bin/canon"
```

**Windows (PowerShell)**

```powershell
$Version = 'vX.Y.Z'
$Archive = "canon-$Version-windows-x86_64.zip"

Invoke-WebRequest -Uri "https://github.com/apply-the/canon/releases/download/$Version/$Archive" -OutFile $Archive
Expand-Archive -Path $Archive -DestinationPath "$env:USERPROFILE\bin" -Force
```

### Build From Source

If you already have Rust `1.95.0`, you can install Canon from this repository:

```bash
cargo +1.95.0 install --path crates/canon-cli --bin canon
canon --help
```

## Use Canon

The short version is:

1. Initialize the repository.
2. Write authored input in `canon-input/`.
3. Run Canon with a mode, risk, and zone.
4. Inspect the run and publish the packet when ready.

### 1. Initialize The Repository

Inside the repository you want to govern:

```bash
canon init
```

If you want Canon to materialize repo-local AI skills as well:

```bash
canon init --ai codex
canon init --ai copilot
canon init --ai claude
```

### 2. Write Authored Input

Canon expects authored input in canonical locations under `canon-input/`. For example, a requirements run typically starts from `canon-input/requirements.md`.

```bash
mkdir -p canon-input
cat > canon-input/requirements.md <<'EOF'
# Requirements Brief

## Problem
Define requirements for a bounded internal CLI without letting scope drift.

## Outcome
Leave behind a governable requirements packet with explicit scope cuts,
tradeoffs, and open questions.
EOF
```

Canon now combines canonical packet shapes with bounded authoring personas
across planning, operational security, and review-heavy surfaces. The product lead,
architect, and change owner personas from the first slice remain in place for
the core planning modes; `security-assessment` now joins the governed mode set
as a recommendation-only operational packet for bounded existing-system threat,
risk, mitigation, and evidence-gap analysis; `supply-chain-analysis` now joins
that operational family as a recommendation-only existing-system packet for
SBOM, vulnerability, license, and legacy posture review with explicit coverage
gaps; `incident` and `migration` remain the adjacent recommendation-only
operational packets; and the existing planning, execution, and review modes
keep their prior bounded-authoring posture. Persona guidance shapes voice and
audience fit only; it never replaces missing required sections.

### 3. Start A Run

Run Canon with an explicit mode, risk class, and usage zone:

```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input canon-input/requirements.md
```

Canon returns a `run_id`. Use that id to inspect or continue the governed run.

### 4. Inspect, Approve, Resume, Publish

These are the commands you will use most often after `canon run`:

```bash
canon status --run <RUN_ID>
canon inspect invocations --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
canon approve --run <RUN_ID> --target <APPROVAL_TARGET> --decision approve --rationale "bounded approval for the packet"
canon resume --run <RUN_ID>
canon publish <RUN_ID>
```

## Common Workflow

- Use `canon init` once per repository.
- Keep authored input under `canon-input/`.
- Use `canon run` to start a governed packet.
- Use `canon inspect ...` to see what Canon actually recorded.
- Use `canon approve` and `canon resume` when a run is gated.
- Use `canon publish` when you want a completed packet copied into `docs/` or another visible path.

## Documentation

Start here if you want more than the short README flow:

- **[Getting Started](docs/guides/getting-started.md)** — Install Canon, initialize a repository, run your first packet, then inspect, approve, resume, and publish it.
- **[Governance Modes](docs/guides/modes.md)** — Choose the right mode and canonical input shape for the work you are doing.
- **[Templates and Examples](docs/templates/canon-input/requirements.md)** — Start from the canonical first-slice packet shapes and follow through to the worked examples in `docs/examples/canon-input/`.
- **[Governance Guardrails: Risk and Zone](docs/guides/risk-and-zone.md)** — Understand how risk and zone constrain autonomy and gate sensitive work.

## Roadmap

To see what is planned next for Canon beyond the current implementation, see [ROADMAP.md](ROADMAP.md).

## How To Contribute

If you want to build or develop Canon itself, see [CONTRIBUTING.md](CONTRIBUTING.md).
