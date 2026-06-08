# Quickstart: Onion-Layer PR Review

1. **Prepare the review**:
   ```bash
   canon pr-review prepare --base main --head HEAD
   ```
   This creates the run, collects the diff, and generates context indexes under `.canon/runs/<run-id>/pr-review/`.

2. **Inspect the context** (optional):
   ```bash
   canon pr-review context --run <run-id> --list
   canon pr-review context --run <run-id> --show C001
   ```

3. **Execute review layers** (LLM agent steps):
   The LLM reads each layer's `instructions.md` and `required-context.tsv`, performs semantic review, and writes findings to `output.md`. Canon records each layer as completed.

4. **Accept the final reviewer output**:
   ```bash
   canon pr-review accept --run <run-id> --reviewer-output reviewer-output.md
   ```
   Canon validates schema, paths, severities, comment IDs, and layer coverage.

5. **Finalize and render artifacts**:
   ```bash
   canon pr-review finalize --run <run-id>
   ```
   Canon generates `01-review-summary.md`, `02-conventional-comments.md`, `03-github-comments.json`, `06-review-report.md`, and other artifacts.

6. **Inspect the results**:
   Open `01-review-summary.md` for the recommendation and severity summary.
   Copy-ready comments are in `02-conventional-comments.md`.
   Machine-readable comments are in `03-github-comments.json`.
