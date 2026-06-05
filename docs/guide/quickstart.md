# Quickstart

Get Canon running in your repository in 5 minutes.

This quickstart tracks `0.68.0`.

## 1. Install

- **macOS / Linux (Homebrew):**
  ```bash
  brew tap apply-the/canon && brew install canon
  ```
- **Windows (winget):**
  ```powershell
  winget install ApplyThe.Canon
  ```

## 2. Initialize

Navigate to your project root and run:
```bash
canon init
```

## 3. Create Authored Input

Canon governs knowledge based on your input. Create a simple brief:
```bash
mkdir -p canon-input
cat > canon-input/requirements.md <<EOF
# Requirements Brief
## Problem
Users need a clear "Quick Start" guide.
## Outcome
Users reach their first successful run in minutes.
EOF
```

## 4. Run Canon

Start a governed run using the `requirements` mode:
```bash
canon run \
  --mode requirements \
  --risk bounded-impact \
  --zone yellow \
  --owner product-lead \
  --input canon-input/requirements.md
```
*Save the `RUN_ID` returned by the command.*

## 5. Check Status & Publish

Check the status of your run:
```bash
# Replace <RUN_ID> with the ID from the previous step
canon status --run <RUN_ID>
```

When the run is ready, publish the artifacts to your repository:
```bash
canon publish <RUN_ID>
```

For `requirements`, the governed packet will be materialized under the default publish destination `specs/<YYYY-MM-DD>-<descriptor>/`, unless you override it with `--to`.

## Next Steps

- Explore [Getting Started](./getting-started) to understand the governance model.
- See [First Workspace](./first-workspace) to understand repository resolution and structure.
- Review [Canon Modes](./canon-modes) for advanced scenarios.
