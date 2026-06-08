# CLI Contract: pr-review (Onion-Layer)

## Sub-commands

### `canon pr-review prepare`

```
canon pr-review prepare --base <base> --head <head> [--risk <risk>] [--zone <zone>] [--owner <owner>]
```

**Inputs**: `--base` (required), `--head` (required)

**Outputs** (under `.canon/runs/<run-id>/pr-review/`):
- `run-state.json` (state: `awaiting_diff_review`)
- `review-brief.md`
- `review-plan.md`
- `context-index.tsv`
- `context-index.json`
- `changed-files.tsv`
- `high-risk-files.tsv`
- `relation-hints.tsv`
- `diff.patch`
- `reviewer-output.schema.json`

**Exit Codes**:
- `0`: Prepare succeeded, run state set to `awaiting_diff_review`
- `1`: Invalid arguments
- `2`: Diff collection failure

---

### `canon pr-review accept`

```
canon pr-review accept --run <run-id> --reviewer-output <path>
```

**Inputs**: `--run` (required), `--reviewer-output` (required, path to `reviewer-output.md`)

**Validation**:
- JSON/Markdown syntax
- Schema version
- Comment ID uniqueness
- Severity vocabulary (`blocking`, `major`, `minor`, `question`, `nitpick`)
- Recommendation vocabulary (`Approve`, `Comment`, `Request changes`)
- Path validation (exists in changed or related files)
- Line/hunk applicability (downgrade invalid lines to hunk/global)
- Layer coverage (all layers must have terminal state)

**Outputs**:
- Updates `run-state.json` to `reviewer_output_accepted` or `reviewer_output_rejected`
- Persists validated `reviewer-output.md` under run folder
- Generates `canonical-review-output.json`

**Exit Codes**:
- `0`: Accepted
- `1`: Invalid arguments
- `3`: Rejected (validation failure); actionable_review_failed recorded

---

### `canon pr-review finalize`

```
canon pr-review finalize --run <run-id>
```

**Precondition**: Run state must be `reviewer_output_accepted` or `reviewer_output_rejected` with explicit skip/failure records for all layers.

**Outputs**:
- `01-review-summary.md`
- `02-conventional-comments.md`
- `03-github-comments.json`
- `04-review-findings.json`
- `05-missing-tests.md`
- `06-review-report.md`
- `07-governance-notes.md`
- `manifest.toml`
- `packet-metadata.json`

**Exit Codes**:
- `0`: Finalized
- `1`: Invalid arguments
- `3`: Blocked (layer coverage incomplete)

---

### `canon pr-review context` (optional helper)

```
canon pr-review context --run <id> --list
canon pr-review context --run <id> --show <context-id>
canon pr-review context --run <id> --show <context-id> --range <start>..<end>
canon pr-review context --run <id> --related <context-id>
canon pr-review context --run <id> --tests <context-id>
canon pr-review context --run <id> --explain <context-id>
```

Read-only. Reads local repository files and Canon run artifacts.
