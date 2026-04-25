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
3. initialize Canon in a repo for plain CLI use or for Codex, Copilot, or Claude
4. start a governed run and inspect the evidence it leaves behind

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

Inspection commands default to Markdown output for human review. Use `--output json`
when you need machine-readable output for scripts or tests.

`canon publish` promotes a completed run's emitted packet from hidden
governance storage under `.canon/` into a visible repository folder. For
example, `requirements` publishes to `specs/<RUN_ID>/` by default. Use
`--to <PATH>` to override the default destination. Publish fails if the run is
still waiting for approval or otherwise not complete.

What you get:

- a governed run under `.canon/runs/<RUN_ID>/`
- emitted artifacts under `.canon/artifacts/<RUN_ID>/requirements/`
- optional published copies under visible repo folders such as `specs/<RUN_ID>/`
- per-invocation request, policy decision, and attempt records
- an `evidence.toml` that links generation paths, validation paths, artifacts, and decisions

That is the product in one screen: Canon governs execution first, then leaves a local record you can inspect and, once complete, publish into living repository documentation.

## Two Axes In Canon

Canon is organized around two explicit axes:

- `mode`: what kind of governed work is happening
- `system_context`: whether the target system is new or existing

These axes answer different questions. A mode does not imply system state, and
system state does not imply the kind of work. When Canon needs target-state
binding, it requires `--system-context` explicitly instead of hiding that
choice in the mode name.

## Typical Flow

A common progression is:

- `discovery` when the problem space is still unclear
- `requirements` when the intent is known but still needs bounded scope and
  constraints
- `system-shaping` when the next question is how a capability should be
  structured
- `architecture` when boundaries, invariants, and tradeoffs need to be fixed
- `backlog` when bounded upstream decisions need to become governed epics,
  slices, dependencies, and sequencing before execution work starts
- `change` when the structure is known and the work is bounded modification of
  an existing system
- `implementation` when the bounded plan already exists and you need governed
  execution guidance for an existing system
- `refactor` when the goal is structural improvement with preserved behavior
  and no undeclared feature addition
- `pr-review` when there is a real diff to challenge

### Quick Decision Rule

- use `system-shaping` when the structure of a capability is not yet defined
- use `backlog` when the structure is already bounded and the next need is
  durable delivery decomposition rather than immediate execution
- use `change` when the structure is known and the task is bounded
  modification with preserved behavior
- use `implementation` when the change boundary is already fixed and the next
  need is task mapping, mutation bounds, and safety-net-backed execution guidance
- use `refactor` when the intent is structural cleanup of an existing system
  and the preserved behavior boundary is already explicit
- use `system-shaping --system-context existing` when you are working inside
  an existing system but the next need is still structural, not modification

Implemented end to end today: `requirements`, `discovery`, `system-shaping`, `architecture`, `backlog`, `change`, `implementation`, `refactor`, `review`, `verification`, and `pr-review`.

Modes that target a specific system state keep that explicit in the run
contract: use `--system-context new|existing` for `system-shaping` and
`architecture`, and use `--system-context existing` for `backlog`, `change`, `implementation`, and `refactor`.

Use `review` for bounded non-PR change packages or artifact bundles, `verification` to challenge claims and invariants directly, and `pr-review` only when the target is a real diff or `WORKTREE`.

In practice, `review` sits after authored packets such as `requirements`, `architecture`, `change`, or another non-PR proposal bundle. It is packet-backed, so do not point it at `src/` or a repository snapshot; use `pr-review` for diffs and `WORKTREE`.

## Initialize Your Repo

Run `canon init` inside the repository you want Canon to govern.

### Plain CLI Only

If you only want the Canon runtime and plan to use the CLI directly:

```bash
canon init
```

This creates `.canon/` and no editor-specific integration files.

### Codex

If you want repo-local Canon skills for Codex:

```bash
canon init --ai codex
```

This creates `.canon/` and `.agents/skills/`.

### Copilot

If you want repo-local Canon skills for GitHub Copilot:

```bash
canon init --ai copilot
```

This also creates `.canon/` and `.agents/skills/`. Copilot and Codex use the same repo-local skills surface.

### Claude

If you want repo-local Canon skills for Claude:

```bash
canon init --ai claude
```

This creates `.canon/`, `.claude/skills/`, and `CLAUDE.md`.

### Refreshing Materialized Skills

If you already initialized Canon and want to refresh the repo-local skill files from the current embedded set:

```bash
canon skills update --ai codex
canon skills update --ai copilot
canon skills update --ai claude
```

Use the target that matches the surface you materialized.

## Canonical Authored Inputs

For file-backed modes, the canonical authored-input locations are under
`canon-input/`:

- `canon-input/requirements.md` or `canon-input/requirements/`
- `canon-input/discovery.md` or `canon-input/discovery/`
- `canon-input/system-shaping.md` or `canon-input/system-shaping/`
- `canon-input/architecture.md` or `canon-input/architecture/`
- `canon-input/change.md` or `canon-input/change/`
- `canon-input/implementation.md` or `canon-input/implementation/`
- `canon-input/refactor.md` or `canon-input/refactor/`
- `canon-input/review.md` or `canon-input/review/`
- `canon-input/verification.md` or `canon-input/verification/`

Repo-local skills may auto-bind only from those mode-specific canonical
locations. They must not infer `--input` from the active editor file, open
tabs, or anything under `.canon/`.

For modes that require target-state binding, Canon also expects an explicit
system context. Use `--system-context new|existing` for `system-shaping` and
`architecture`; use `--system-context existing` for `change`, `implementation`, and `refactor`.

`canon run` and `canon inspect risk-zone` also accept explicit inline authored
input through `--input-text` when you do not want to materialize a repo file
first. Inline input is snapshotted only under `.canon/runs/<RUN_ID>/inputs/`
for real runs; it does not create `canon-input/*` files for you.

Modes that expect authored input now fail fast when the supplied input is
missing, empty, whitespace-only, or structurally insufficient. An empty file,
an empty directory, or a directory that expands to only whitespace content is a
validation error, not a completed run.

When a canonical directory such as `canon-input/requirements/` exists, prefer
passing the directory itself to `canon inspect clarity` so Canon reads the full
authored input set recursively instead of a single child file. A single
`--input` group can also carry multiple explicit paths and still produce one
aggregated inspection result.

Starter templates for file-backed modes live under
[`docs/templates/canon-input/`](docs/templates/canon-input/). Realistic sample
briefs live under [`docs/examples/canon-input/`](docs/examples/canon-input/).

For `implementation` and `refactor`, use the dedicated templates and examples
there instead of starting from the `change` brief shape by hand. When those
modes need to carry forward context from earlier packets, use an explicit
folder-backed packet with `brief.md` and `source-map.md`; see
[`docs/examples/canon-input/carry-forward-packets.md`](docs/examples/canon-input/carry-forward-packets.md).

`pr-review` stays explicit and ref-based. It does not auto-bind from
`canon-input/`.

## How To Contribute

This README is written for end users first.

If you want to compile, test, or develop Canon itself, use
[`CONTRIBUTING.md`](CONTRIBUTING.md). It contains contributor rules, local
setup, build commands, validation commands, and repository development
guidance.

## Use Canon From Codex, Copilot, or Claude

Canon supports three end-user integration targets today:

- `canon init --ai codex`
- `canon init --ai copilot`
- `canon init --ai claude`

Codex and Copilot both materialize `.agents/skills/`. Claude materializes `.claude/skills/` and `CLAUDE.md`. In every case, the Canon CLI remains the engine and the repo-local files are a guided surface on top.

Those skills assume a real `canon` binary is already on PATH. If they report a
missing or incompatible Canon installation, return to the install guide above
and update the shared binary before retrying.

High-value available-now skills:

- `$canon-init`
- `$canon-discovery`
- `$canon-requirements`
- `$canon-system-shaping`
- `$canon-architecture`
- `$canon-status`
- `$canon-inspect-invocations`
- `$canon-inspect-evidence`
- `$canon-inspect-artifacts`
- `$canon-approve`
- `$canon-resume`
- `$canon-change`
- `$canon-pr-review`

Typical skill flow:

```text
$canon-init
Initialize Canon in this repository.

$canon-requirements
Start a requirements run with owner staff-engineer, risk bounded-impact, zone yellow, input canon-input/requirements.md.

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
discoverable through `$`. Some are runnable wrappers and some remain honest
support-state wrappers. Claude materialization is separate and only happens when
you explicitly run `canon init --ai claude`.

## Example Workflows

### `requirements`

Use this when you need bounded framing before code or architecture drift starts.

```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input canon-input/requirements.md
```

Canon will:

- capture context as a governed run
- govern generation and critique requests before execution
- deny mutation in this mode
- emit a requirements artifact set backed by invocation evidence

### `change`

Use this for changes in a live codebase where preserved behavior matters.

This mode currently requires `--system-context existing`.

If you already have a change brief, use it directly. If you only have a
change intent, the skill should guide you to fill the minimum missing slots
before invoking Canon: system slice, intended change, legacy invariants,
allowed or excluded change surface, and validation strategy. Only redirect to
`requirements` when the change is still too ambiguous to bound honestly.

```bash
canon run \
  --mode change \
  --system-context existing \
  --risk bounded-impact \
  --zone yellow \
  --owner staff-engineer \
  --input canon-input/change.md
```

Canon will:

- capture repository context through governed inspection requests
- separate generation-oriented and validation-oriented work
- keep consequential mutation recommendation-only in the current release
- block readiness if invariants or independent challenge are missing

When change work is blocked or recommendation-only, the guided next step
should usually be to inspect the emitted packet under
`.canon/artifacts/<RUN_ID>/change/` before recording approval.

If a consequential request is gated, approval is explicit and local:

```bash
canon approve \
  --run <RUN_ID> \
  --target invocation:<REQUEST_ID> \
  --by principal-engineer \
  --decision approve \
  --rationale "Allow bounded systemic change generation with named ownership."

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
- emit a review packet under `.canon/artifacts/<RUN_ID>/pr-review/`, with `review-summary.md` as the primary status artifact and `conventional-comments.md` as the reviewer-facing companion

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
│       ├── inputs/
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
For authored file-backed inputs, Canon also snapshots the exact files used for
the run under `.canon/runs/<RUN_ID>/inputs/` and records digest-backed input
provenance in `context.toml`.

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
- `inspect modes|methods|policies|risk-zone|clarity|artifacts|invocations|evidence`
- `approve`
- `resume`

Implemented end to end today:

- [discovery](#discovery-mode)
- [requirements](#requirements-mode)
- [system-shaping](#system-shaping-mode)
- [architecture](#architecture-mode)
- [change](#change-mode)
- [review](#review-mode)
- [verification](#verification-mode)
- [pr-review](#pr-review-mode)

Available-now repo-local skills backed by the real Canon CLI:

- `canon-init`
- `canon-discovery`
- `canon-requirements`
- `canon-system-shaping`
- `canon-architecture`
- `canon-status`
- `canon-inspect-invocations`
- `canon-inspect-evidence`
- `canon-inspect-artifacts`
- `canon-approve`
- `canon-resume`
- `canon-change`
- `canon-review`
- `canon-verification`
- `canon-pr-review`

Modeled but not fully implemented end to end yet:

- [implementation](#implementation-mode)
- [refactor](#refactor-mode)
- [incident](#incident-mode)
- [migration](#migration-mode)

## What Each Mode Does

Need detailed input templates, mode-selection guidance, and explicit
"questions answered" boundaries? See
[`MODE_GUIDE.md`](MODE_GUIDE.md).

### Discovery Mode

Use this when you do not yet have a trustworthy problem statement and need to
explore the problem space before committing to requirements, system shape, or
architecture decisions. Discovery maps the problem domain, unknowns,
assumptions, context boundaries, exploration options, and decision pressure
points from a short discovery brief. The resulting packet is grounded against
live repository surfaces, records critique plus repository validation evidence,
and names a concrete downstream handoff into the next governed mode.

### Requirements Mode

Use this when you need to bound the problem before code, design, or scope drift starts.
Canon turns raw intent into a governed requirements packet with explicit scope,
constraints, open questions, and decision checkpoints.

### System-Shaping Mode

Use this when the intent is bounded and the next question is how a new
capability should be structured. The canonical CLI name is `system-shaping`.
Start the run with an explicit `--system-context new|existing` so the target
state is recorded instead of implied.

Use `--system-context new` when you are shaping a new system from scratch. Use
`--system-context existing` when the system already exists but you first need
to shape a new capability structure inside it before planning a bounded change.

Canon shapes that capability into a first governed structure: system shape,
architecture outline, capability map, delivery options, and risk hotspots.
This mode includes mandatory critique before the artifact set is finalized.

### Architecture Mode

Use this when you are choosing boundaries, invariants, and tradeoffs for a real
design decision. Start the run with an explicit `--system-context new|existing`
so the decision packet records whether it targets a new or existing system.
Canon produces architecture decisions, invariants, a tradeoff matrix, a
boundary map, and a readiness assessment. This mode includes mandatory critique
and can stop for explicit approval when the run is systemic-impact or in a red
zone.

### Change Mode

Use this when you need to change an existing system without losing important
behavior. The canonical CLI name is `change`, and this mode currently requires
`--system-context existing` because it is defined around preserved behavior and
bounded modification of a live system.

If the real need is to shape a new capability inside an existing system before
you decide the bounded change surface, use `system-shaping --system-context
existing` first.

Canon constrains the change surface, records legacy invariants, captures a
bounded implementation plan, and makes the validation strategy part of the
governed output.

### PR-Review Mode

Use this on a real diff or worktree when you want review output backed by
governed inspection instead of a loose summary. Canon inspects the change,
runs critique as a separate governed path, and leaves behind a review packet
with explicit evidence and disposition.

The packet keeps `review-summary.md` as the primary status surface and now also
emits `conventional-comments.md` as a readable reviewer-facing companion.

Completed runs publish under `docs/reviews/prs/<RUN_ID>/`, including the
Conventional Comments artifact.

### Implementation Mode

Use this when the bounded plan already exists and you need Canon to carry it
into governed execution guidance for an existing system. `implementation`
requires `--system-context existing` plus a file-backed brief under
`canon-input/implementation.md` or `canon-input/implementation/`.

Good implementation briefs name the task mapping, mutation bounds, allowed
paths, safety-net evidence, independent checks, rollback triggers, and rollback
steps. Canon emits a distinct implementation packet with:

- `task-mapping.md`
- `mutation-bounds.md`
- `implementation-notes.md`
- `completion-evidence.md`
- `validation-hooks.md`
- `rollback-notes.md`

Run and status summaries surface `task-mapping.md` directly and make the
current `recommendation-only` execution posture explicit. In this tranche,
Canon records bounded implementation guidance and evidence but does not apply
workspace mutation on your behalf.

Completed implementation runs publish through the existing surface at
`docs/implementation/<RUN_ID>/`.

### Refactor Mode

Use this when the goal is structural improvement in an existing system without
changing externally meaningful behavior. `refactor` requires
`--system-context existing` plus a file-backed brief under
`canon-input/refactor.md` or `canon-input/refactor/`.

Good refactor briefs name the preserved behavior, approved exceptions,
refactor scope, allowed paths, structural rationale, untouched surface,
safety-net evidence, contract-drift conclusion, and no-feature-addition
decision. Canon emits a distinct refactor packet with:

- `preserved-behavior.md`
- `refactor-scope.md`
- `structural-rationale.md`
- `regression-evidence.md`
- `contract-drift-check.md`
- `no-feature-addition.md`

Run and status summaries surface `preserved-behavior.md` directly and make the
current `recommendation-only` execution posture explicit. In this tranche,
Canon records bounded structural guidance and evidence but does not apply
workspace mutation on your behalf.

Completed refactor runs publish through the existing surface at
`docs/refactors/<RUN_ID>/`.

### Verification Mode

Use this when you want to challenge claims, invariants, contracts, or evidence
directly. Canon takes a verification packet, runs critique as a separate
governed path, and leaves behind invariants, contract matrix, adversarial
review, verification report, and unresolved findings. Run and status
summaries surface `verification-report.md` directly.

### Review Mode

Use this when you need a governed review of a bounded non-PR packet (a
requirements, architecture, or change packet, or any proposal bundle). Canon
emits a boundary assessment, missing-evidence record, decision-impact note,
and explicit `review-disposition.md` that run and status summaries surface
directly.

### Incident Mode

TODO: planned mode surface, not implemented end to end yet.

### Migration Mode

TODO: planned mode surface, not implemented end to end yet.

Discoverable support-state wrappers that are still intentionally limited:

- `canon-incident`
- `canon-migration`
- `canon-incident`
- `canon-migration`

Current limitations:

- `canon verify` is present as a CLI surface but not implemented yet; use `canon run --mode verification` for the runnable verification path
- MCP runtime is modeled in policy and domain terms, but explicitly denied at runtime
- convenience channels such as Homebrew or Chocolatey are not shipped yet; use the GitHub release archives
- deeper semantic critique and broader adapter coverage are still backlog

## Command Overview

- `canon init`: materialize `.canon/` for plain CLI use
- `canon init --ai codex`: materialize `.canon/` and `.agents/skills/` for Codex
- `canon init --ai copilot`: materialize `.canon/` and `.agents/skills/` for Copilot
- `canon init --ai claude`: materialize `.canon/`, `.claude/skills/`, and `CLAUDE.md` for Claude
- `canon run`: start a governed run with explicit mode, risk, zone, owner, and inputs
- `canon status --run <RUN_ID>`: inspect run state, pending approvals, and evidence summary
- `canon inspect modes`: inspect the typed mode catalog
- `canon inspect methods`: inspect available method definitions
- `canon inspect policies`: inspect loaded policy definitions
- `canon inspect clarity --mode <MODE> --input <INPUT_PATH> [<INPUT_PATH> ...]`: derive Canon-backed missing-context findings and clarification questions from one or more authored analysis inputs; any folder input is read recursively
- `canon inspect artifacts --run <RUN_ID>`: list emitted artifact paths in Markdown by default
- `canon inspect invocations --run <RUN_ID>`: inspect request-level decisions and outcomes in Markdown by default
- `canon inspect evidence --run <RUN_ID>`: inspect readable artifact links, generation paths, validation paths, and evidence linkage in Markdown by default
- `canon approve --run <RUN_ID> --target ...`: approve a specific invocation or gate
- `canon resume --run <RUN_ID>`: continue a run after approval or after fixing a blocked condition
- `canon publish <RUN_ID> [--to <PATH>]`: copy approved artifacts from `.canon/` into a visible repo folder; defaults depend on mode, such as `specs/<RUN_ID>/` for `requirements`
- `canon verify --run <RUN_ID>`: planned surface, not implemented

## Contributor Docs

Contributor setup, build-from-source instructions, local validation, and repo
rules live in [`CONTRIBUTING.md`](CONTRIBUTING.md).