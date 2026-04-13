# Canon

[![CI](https://github.com/apply-the/canon/actions/workflows/ci.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/ci.yml)
[![Lint](https://github.com/apply-the/canon/actions/workflows/lint.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/lint.yml)
[![Vulnerabilities](https://github.com/apply-the/canon/actions/workflows/vulnerabilities.yml/badge.svg)](https://github.com/apply-the/canon/actions/workflows/vulnerabilities.yml)
[![Coverage](https://codecov.io/gh/apply-the/canon/graph/badge.svg?token=JZ4IPF51DH)](https://codecov.io/gh/apply-the/canon)

**Canon is a CLI you run inside a repository to govern engineering work with AI and external tools, then leave durable evidence under `.canon/`.**

You give Canon a mode, risk class, usage zone, owner, and inputs. Canon decides what is allowed to run before anything runs, records what was attempted, what was denied, what needed approval, and what evidence supports the result. It is local-first, inspectable, and built for people who would rather read files on disk than trust an opaque agent loop.

Canon is not a generic agent framework. It is not a prompt library. It is not a Copilot/Claude replacement. It is a disciplined terminal tool for governed execution.

If you are here to use Canon rather than build it, the path is simple:

1. install the right release archive for your machine
2. verify `canon --version` and the resolved PATH location
3. initialize Canon in a repo and start a governed run

## What Canon Is

- A local CLI for governed engineering runs.
- A runtime that sits above shell actions, repository inspection, and AI-assisted generation or critique.
- A tool with explicit modes, risk classes, usage zones, invocation policy, gates, approvals, and decision memory.
- A system where artifacts are evidence of governed work, not the whole product.

## Why You Would Use It

Because the hard part is not getting output. The hard part is knowing:

- what was allowed to run
- what was denied
- what required approval
- what challenged generated output
- what evidence exists after the run is over

If you are skeptical of AI wrappers, that is the point of Canon. It turns a run into something you can inspect, audit, resume, and review instead of something you are asked to trust.

## Install

Canon ships as a single prebuilt binary for macOS, Linux, and Windows.
Check [Releases](https://github.com/apply-the/canon/releases) for the latest tag.

### Option 1: Persistent Installation (Recommended)

Install once, use everywhere. Pin a specific release tag for stability.

**macOS / Linux**

```bash
# Detect architecture
ARCH=$(uname -m)   # arm64 or x86_64
OS=$(uname -s | tr '[:upper:]' '[:lower:]')   # darwin or linux

# Replace vX.Y.Z with the latest release tag
VERSION=vX.Y.Z
ARCHIVE="canon-${VERSION}-${OS}-${ARCH}.tar.gz"

curl -LO "https://github.com/apply-the/canon/releases/download/${VERSION}/${ARCHIVE}"
tar -xzf "${ARCHIVE}"
install -m 0755 canon "$HOME/.local/bin/canon"
```

Then verify:

```bash
canon --version
command -v canon
```

**Windows (PowerShell)**

```powershell
# Replace vX.Y.Z with the latest release tag
$Version = 'vX.Y.Z'
$Archive = "canon-$Version-windows-x86_64.zip"

Invoke-WebRequest -Uri "https://github.com/apply-the/canon/releases/download/$Version/$Archive" -OutFile $Archive
Expand-Archive -Path $Archive -DestinationPath "$env:USERPROFILE\bin" -Force
```

Then verify:

```powershell
canon --version
Get-Command canon
```

To upgrade, repeat the install steps with the new tag. If the resolved path
still points to an older binary, move the install directory earlier on PATH
and run the verification commands again.

### Option 2: One-time Usage

Run Canon from the current directory without adding it to PATH.

**macOS / Linux**

```bash
VERSION=vX.Y.Z
ARCHIVE="canon-${VERSION}-darwin-arm64.tar.gz"   # adjust OS/arch as needed

curl -LO "https://github.com/apply-the/canon/releases/download/${VERSION}/${ARCHIVE}"
tar -xzf "${ARCHIVE}"
./canon --version
```

**Windows (PowerShell)**

```powershell
$Version = 'vX.Y.Z'
$Archive = "canon-$Version-windows-x86_64.zip"

Invoke-WebRequest -Uri "https://github.com/apply-the/canon/releases/download/$Version/$Archive" -OutFile $Archive
Expand-Archive -Path $Archive -DestinationPath ".\canon-$Version" -Force
.\canon-$Version\canon.exe --version
```

### Option 3: Enterprise / Air-Gapped Installation

If your environment blocks access to GitHub, download the release archive on a
connected machine and transfer it to the target host via your internal artifact
registry or secure file transfer. Then follow the Option 1 steps using the
local archive path instead of the `curl` step.

## Quickstart

Try Canon in a throwaway repo:

```bash
mkdir -p ~/tmp/canon-demo
cd ~/tmp/canon-demo
git init

cat > idea.md <<'EOF'
# Idea

Define requirements for a bounded internal CLI without letting scope drift.
EOF

canon init

canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input idea.md
```

Take the `run_id` from the output, then inspect what Canon actually did:

```bash
canon status --run <RUN_ID>
canon inspect invocations --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
```

Inspection commands default to Markdown output for human review. Use `--output json`
when you need machine-readable output for scripts or tests.

What you get:

- a governed run under `.canon/runs/<RUN_ID>/`
- emitted artifacts under `.canon/artifacts/<RUN_ID>/requirements/`
- per-invocation request, policy decision, and attempt records
- an `evidence.toml` that links generation paths, validation paths, artifacts, and decisions

That is the product in one screen: Canon governs execution first, then leaves a local record you can inspect.

## How To Contribute

This README is written for end users first.

If you want to compile, test, or develop Canon itself, use
[`CONTRIBUTING.md`](CONTRIBUTING.md). It contains contributor rules, local
setup, build commands, validation commands, and repository development
guidance.

## Use Canon From Codex or Copilot

To materialize repo-local skills for Codex or compatible Copilot environments,
run:

```bash
canon init --ai codex
```

Use `--ai copilot` if you want the explicit Copilot label instead. Both targets
materialize the same `.agents/skills/` surface. The CLI is still the engine.
The skills are just a sharper way to invoke it.

Those skills assume a real `canon` binary is already on PATH. If they report a
missing or incompatible Canon installation, return to the install guide above
and update the shared binary before retrying.

High-value available-now skills:

- `$canon-init`
- `$canon-requirements`
- `$canon-status`
- `$canon-inspect-invocations`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
- `$canon-brownfield`
- `$canon-pr-review`

Typical skill flow:

```text
$canon-init
Initialize Canon in this repository.

$canon-requirements
Start a requirements run with owner staff-engineer, risk bounded-impact, zone yellow, input idea.md.

$canon-status
Show status for run <RUN_ID>.

$canon-inspect-evidence
Inspect evidence for run <RUN_ID>.
```

Canon skills are intended to behave as guided handoffs, not as a flat menu.
When a run is blocked, the next response should say what is wrong, point to the
readable `.canon/artifacts/...` or `.canon/runs/...` files that explain it,
list the valid actions, and recommend one next step without auto-executing it.

In environments that support repo-local skill discovery, Canon skills are
discoverable through `$`. Not all are runnable yet. The non-runnable ones are
still visible, but they must say so explicitly and must not fabricate runs,
run ids, approvals, or evidence. Claude materialization is separate and only
happens when you explicitly run `canon init --ai claude`.

## Example Workflows

### `requirements`

Use this when you need bounded framing before code or architecture drift starts.

```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input idea.md
```

Canon will:

- capture context as a governed run
- govern generation and critique requests before execution
- deny mutation in this mode
- emit a requirements artifact set backed by invocation evidence

### `brownfield-change`

Use this for changes in a live codebase where preserved behavior matters.

If you already have a brownfield brief, use it directly. If you only have a
change intent, the skill should guide you to fill the minimum missing slots
before invoking Canon: system slice, intended change, legacy invariants,
allowed or excluded change surface, and validation strategy. Only redirect to
`requirements` when the change is still too ambiguous to bound honestly.

```bash
canon run \
  --mode brownfield-change \
  --risk bounded-impact \
  --zone yellow \
  --owner staff-engineer \
  --input brownfield.md
```

Canon will:

- capture repository context through governed inspection requests
- separate generation-oriented and validation-oriented work
- keep consequential mutation recommendation-only in the current release
- block readiness if invariants or independent challenge are missing

When brownfield work is blocked or recommendation-only, the guided next step
should usually be to inspect the emitted packet under
`.canon/artifacts/<RUN_ID>/brownfield-change/` before recording approval.

If a consequential request is gated, approval is explicit and local:

```bash
canon approve \
  --run <RUN_ID> \
  --target invocation:<REQUEST_ID> \
  --by principal-engineer \
  --decision approve \
  --rationale "Allow bounded systemic brownfield generation with named ownership."

canon resume --run <RUN_ID>
```

### `pr-review`

Use this on a real diff when you want review output backed by governed inspection rather than a loose generated summary.

```bash
canon run \
  --mode pr-review \
  --risk bounded-impact \
  --zone yellow \
  --owner reviewer \
  --input refs/heads/main \
  --input HEAD
```

To review uncommitted changes in the working tree instead of a committed ref:

```bash
canon run \
  --mode pr-review \
  --risk low-impact \
  --zone green \
  --owner reviewer \
  --input refs/heads/main \
  --input WORKTREE
```

Canon will:

- inspect the diff through governed shell-based requests
- retain bounded payload refs when policy allows it
- run critique as a separate governed path
- emit a review packet under `.canon/artifacts/<RUN_ID>/pr-review/`

If the review leaves must-fix findings unresolved, that disposition stays explicit:

```bash
canon approve \
  --run <RUN_ID> \
  --target gate:review-disposition \
  --by principal-engineer \
  --decision approve \
  --rationale "Accept the remaining review risk with explicit ownership."
```

## What Canon Persists Locally

Canon writes local runtime state under `.canon/` in the current repo:

```text
.canon/
├── artifacts/
├── decisions/
├── methods/
├── policies/
├── runs/
│   └── <run-id>/
│       ├── approvals/
│       ├── context.toml
│       ├── evidence.toml
│       ├── gates/
│       ├── invocations/
│       │   └── <request-id>/
│       │       ├── attempt-01.toml
│       │       ├── decision.toml
│       │       ├── payload/
│       │       └── request.toml
│       ├── links.toml
│       ├── run.toml
│       ├── state.toml
│       └── verification/
├── sessions/
└── traces/
```

Why that matters:

- you can inspect what Canon allowed, constrained, denied, or escalated
- you can see evidence without replaying the run
- you can audit approvals and decisions as files, not hidden application state
- you can resume a run against durable local context
- run-scoped analysis and review artifacts belong under `.canon/`, not as ad-hoc files in the repository root

Canon is not trying to preserve every prompt transcript. It preserves the durable parts of consequential work: requests, policy outcomes, attempts, traces, evidence bundles, artifacts, and decisions.

## Why This Is Different

### Not a prompt runner

A prompt runner sends text to a model and returns output. Canon resolves mode, risk, zone, policy, and ownership requirements before invocation, and persists the request even when it is denied or gated.

### Not an agent framework

Agent frameworks optimize for orchestration, extensibility, and generic tool graphs. Canon keeps typed mode semantics in the core and optimizes for bounded behavior inside a repo.

### Not a plain Copilot wrapper

A wrapper forwards work to a tool. Canon decides whether that capability is allowed at all, under which constraints, and what evidence must exist afterward. External tools are execution surfaces, not the product identity.

## Why This Exists

Engineering runs go bad long before anyone notices the final output is weak. Scope drifts. Generated text validates generated text. Decisions disappear into chat logs. Reviews happen after too much has already happened.

Canon exists to move governance forward in time. It governs the invocation itself, not just the final markdown someone saved afterward.

## Current Status

Available now:

- `init`
- `run`
- `status`
- `inspect modes|methods|policies|artifacts|invocations|evidence`
- `approve`
- `resume`

Implemented end to end today:

- `requirements`
- `brownfield-change`
- `pr-review`

Available-now Codex skills backed by the real Canon CLI:

- `canon-init`
- `canon-requirements`
- `canon-status`
- `canon-inspect-invocations`
- `canon-inspect-evidence`
- `canon-inspect-artifacts`
- `canon-approve`
- `canon-resume`
- `canon-brownfield`
- `canon-pr-review`

Modeled but not fully implemented end to end yet:

- `discovery`
- `greenfield`
- `architecture`
- `implementation`
- `refactor`
- `verification`
- `review`
- `incident`
- `migration`

Discoverable Codex skills that are honest support-state wrappers:

- `canon-discovery`
- `canon-greenfield`
- `canon-architecture`
- `canon-implementation`
- `canon-refactor`
- `canon-review`
- `canon-incident`
- `canon-migration`
- `canon-verification` as `intentionally-limited`

Current limitations:

- `verify` is present as a CLI surface but not implemented yet
- MCP runtime is modeled in policy and domain terms, but explicitly denied at runtime
- convenience channels such as Homebrew or Chocolatey are not shipped yet; use the GitHub release archives
- deeper semantic critique and broader adapter coverage are still backlog

## Command Overview

- `canon init [--ai codex|copilot|claude]`: materialize `.canon/` and optionally the requested AI-facing repo-local surface
- `canon run`: start a governed run with explicit mode, risk, zone, owner, and inputs
- `canon status --run <RUN_ID>`: inspect run state, pending approvals, and evidence summary
- `canon inspect modes`: inspect the typed mode catalog
- `canon inspect methods`: inspect available method definitions
- `canon inspect policies`: inspect loaded policy definitions
- `canon inspect artifacts --run <RUN_ID>`: list emitted artifact paths in Markdown by default
- `canon inspect invocations --run <RUN_ID>`: inspect request-level decisions and outcomes in Markdown by default
- `canon inspect evidence --run <RUN_ID>`: inspect readable artifact links, generation paths, validation paths, and evidence linkage in Markdown by default
- `canon approve --run <RUN_ID> --target ...`: approve a specific invocation or gate
- `canon resume --run <RUN_ID>`: continue a run after approval or after fixing a blocked condition
- `canon verify --run <RUN_ID>`: planned surface, not implemented

## Contributor Docs

Contributor setup, build-from-source instructions, local validation, and repo
rules live in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Deeper Docs

- Contributor guide: [`CONTRIBUTING.md`](CONTRIBUTING.md)
- Constitution: [`.specify/memory/constitution.md`](.specify/memory/constitution.md)
- Core product spec: [`specs/001-canon-spec/spec.md`](specs/001-canon-spec/spec.md)
- Core implementation plan: [`specs/001-canon-spec/plan.md`](specs/001-canon-spec/plan.md)
- Governed execution spec: [`specs/002-governed-execution-adapters/spec.md`](specs/002-governed-execution-adapters/spec.md)
- Governed execution plan: [`specs/002-governed-execution-adapters/plan.md`](specs/002-governed-execution-adapters/plan.md)
- Governed execution validation: [`specs/002-governed-execution-adapters/validation-report.md`](specs/002-governed-execution-adapters/validation-report.md)
- Codex skills frontend spec: [`specs/003-codex-skills-frontend/spec.md`](specs/003-codex-skills-frontend/spec.md)
- Codex skills frontend plan: [`specs/003-codex-skills-frontend/plan.md`](specs/003-codex-skills-frontend/plan.md)
- Codex skills frontend validation: [`specs/003-codex-skills-frontend/validation-report.md`](specs/003-codex-skills-frontend/validation-report.md)
