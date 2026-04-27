# Getting Started with Canon

This guide is the practical version of the README: what Canon does, how to install it, and how to use it in a repository.

## What Canon Is

Canon is a local CLI for governed AI-assisted engineering work.

You use Canon to:

- start a run with an explicit mode, risk, zone, owner, and authored input
- keep artifacts, approvals, evidence, and invocation history under `.canon/`
- inspect what happened before trusting the result
- publish completed packets into visible repository paths such as `docs/` or `specs/`

The shipped CLI binary is `canon`.

## Install Canon

### Prebuilt Binary

Download the latest binary from [GitHub Releases](https://github.com/apply-the/canon/releases).

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

If you already have Rust `1.95.0` installed:

```bash
cargo +1.95.0 install --path crates/canon-cli --bin canon
canon --help
```

## First Run in a Repository

### 1. Initialize Canon

Inside the repository you want to govern:

```bash
canon init
```

If you want repo-local AI skills as well:

```bash
canon init --ai codex
canon init --ai copilot
canon init --ai claude
```

### 2. Write Authored Input

Canon expects authored input in canonical `canon-input/` locations. For a requirements run, use `canon-input/requirements.md`.

```bash
mkdir -p canon-input
cat > canon-input/requirements.md <<'EOF'
# Idea

Define requirements for a bounded internal CLI without letting scope drift.
EOF
```

### 3. Start the Run

```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input canon-input/requirements.md
```

Canon returns a `run_id`. Keep that id.

### 4. Inspect What Canon Recorded

```bash
canon status --run <RUN_ID>
canon inspect invocations --run <RUN_ID>
canon inspect evidence --run <RUN_ID>
canon inspect artifacts --run <RUN_ID>
```

These commands tell you:

- what Canon attempted
- what was blocked or gated
- which artifacts were emitted
- what evidence supports the packet

### 5. Approve or Resume When Needed

Some runs stop in `AwaitingApproval` or require an explicit follow-up step.

```bash
canon approve --run <RUN_ID> --target <APPROVAL_TARGET> --decision approve --rationale "bounded approval for the packet"
canon resume --run <RUN_ID>
```

### 6. Publish the Packet

When the run is ready to leave `.canon/` and become visible repository documentation:

```bash
canon publish <RUN_ID>
canon publish <RUN_ID> --to docs/custom/path
```

## The Core Commands

| Command | What it is for |
| --- | --- |
| `canon init` | Create `.canon/` and optional repo-local AI skills |
| `canon run` | Start a governed run |
| `canon status` | Check the current run state |
| `canon inspect ...` | Read invocations, evidence, artifacts, and other runtime detail |
| `canon approve` | Record an approval for a gated run |
| `canon resume` | Continue a run after approval or follow-up work |
| `canon publish` | Copy emitted artifacts into a visible repository path |

## Choosing the Right Inputs

Canon is strict about authored input shape.

- Most modes bind from `canon-input/<mode>.md` or `canon-input/<mode>/`
- `review` expects a review packet under `canon-input/review.md` or `canon-input/review/`
- `pr-review` does not bind from `canon-input/`; it works from explicit refs or `WORKTREE`

Use the mode guide when you are not sure which packet shape to author.

## Next Reading

- [Mode Guide](modes.md)
- [Risk and Zone](risk-and-zone.md)
- [README](../../README.md)