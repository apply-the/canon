# Canon

[![CI](https://github.com/apply-the/canon/actions/workflows/ci.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/ci.yml)
[![Lint](https://github.com/apply-the/canon/actions/workflows/lint.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/lint.yml)
[![Vulnerabilities](https://github.com/apply-the/canon/actions/workflows/vulnerabilities.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/vulnerabilities.yml)
[![Coverage](https://codecov.io/gh/apply-the/canon/graph/badge.svg?token=JZ4IPF51DH)](https://codecov.io/gh/apply-the/canon)

**Canon is a CLI you run inside a repository to govern engineering work with AI and external tools, then leave durable evidence under `.canon/`.**

## Description

Canon is not a generic agent framework. It is not a prompt library. It is not a Copilot/Claude replacement. It is a disciplined terminal tool for governed execution.

- A local CLI for governed engineering runs.
- A runtime that sits above shell actions, repository inspection, and AI-assisted generation or critique.
- A tool with explicit modes, risk classes, usage zones, invocation policy, gates, approvals, and decision memory.
- A system where artifacts are evidence of governed work, not the whole product.

You give Canon a mode, risk class, usage zone, owner, and inputs. Canon decides what is allowed to run before anything runs, records what was attempted, what was denied, what needed approval, and what evidence supports the result. It is local-first, inspectable, and built for people who would rather read files on disk than trust an opaque agent loop.

## Quickstart

### Install

Canon ships as a single prebuilt binary for macOS, Linux, and Windows. Check [Releases](https://github.com/apply-the/canon/releases) for the latest tag.

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

### Try it out

Try Canon in a throwaway repo:

```bash
mkdir -p ~/tmp/canon-demo
cd ~/tmp/canon-demo
git init

mkdir -p canon-input
cat > canon-input/requirements.md <<'EOF'
# Idea

Define requirements for a bounded internal CLI without letting scope drift.
EOF

canon init

canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input canon-input/requirements.md
```

Take the `run_id` from the output, then inspect what Canon actually did:

```bash
canon status --run <RUN_ID>
canon inspect invocations --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
canon publish <RUN_ID>
```

### Initialize Your Repo

To use Canon's built-in skills with an AI coding assistant, run one of the following inside your repository:

- `canon init --ai codex`
- `canon init --ai copilot`
- `canon init --ai claude`

This materializes the repo-local AI skills required for governed conversational workflows.

## Documentation

Canon uses hierarchical documentation to isolate concepts and ensure clear understanding of how the system works.

- **[Governance Modes](docs/guides/modes.md)** — Explains the specific execution modes (e.g. `requirements`, `architecture`, `incident`), what they are for, and what inputs they expect.
- **[Governance Guardrails: Risk and Zone](docs/guides/risk-and-zone.md)** — Details how risk profiles and execution zones constrain AI autonomy and require explicit approvals.

## Roadmap

To see what is planned next for Canon beyond the current implementation, see the **[ROADMAP.md](ROADMAP.md)** document.

## How To Contribute

If you want to compile, test, or develop Canon itself, see **[CONTRIBUTING.md](CONTRIBUTING.md)**. It contains contributor rules, local setup, build commands, validation commands, and repository development guidance.
