# Data Model: Agent-Governed Onion-Layer PR Review

## Core Entities

### RunState

Represents the current phase in the onion-layer review workflow.

```
prepared
awaiting_diff_review
diff_review_recorded
awaiting_whole_file_review
whole_file_review_recorded
awaiting_related_context_review
related_context_review_recorded
awaiting_stress_review
stress_review_recorded
awaiting_test_review
test_review_recorded
reviewer_output_accepted
reviewer_output_rejected
finalized
```

### LayerStatus

Each layer must end in one of:

| Value | Meaning |
|---|---|
| `completed` | Executed successfully |
| `skipped_with_reason` | Intentionally skipped with recorded reason |
| `failed` | Execution failed |

### SkipRecord

```json
{
  "layer": "logical_stress",
  "reason": "No async or concurrent code in changed files",
  "decision_source": "operator",
  "coverage_impact": "minimal",
  "downgrades_recommendation": false,
  "timestamp": "2026-06-08T22:00:00Z"
}
```

### FailureRecord

```json
{
  "layer": "whole_file",
  "failure_reason": "LLM failed to produce valid output after 3 attempts",
  "partial_output_exists": false,
  "review_can_continue": true,
  "recommendation_impact": "downgrade_to_comment",
  "timestamp": "2026-06-08T22:00:00Z"
}
```

### ContextIndexEntry

A single row in `context-index.tsv`:

| Field | Type | Description |
|---|---|---|
| `id` | String | Stable context ID (C001, C002, ...) |
| `type` | String | `diff`, `file`, `related`, `test`, `doc` |
| `path` | String | Repo-relative file path |
| `start_line` | u32? | Start of relevant range |
| `end_line` | u32? | End of relevant range |
| `reason` | String | Why this context is relevant |
| `risk` | String | `high`, `medium`, `low` |
| `layer` | String | Which layer this context is relevant to |

### ReviewerOutput

The final LLM-authored aggregate output parsed by Canon:

```json
{
  "schema_version": "1.0",
  "review_status": "actionable_review_executed",
  "coverage": {
    "files_changed": 3,
    "files_inspected_deeply": ["src/transport/http.rs"],
    "files_skipped": ["src/legacy.rs"],
    "limitations": ["src/legacy.rs not inspected due to large file size"]
  },
  "findings": [],
  "missing_tests": [],
  "recommendation": "Comment",
  "layer_coverage": {
    "diff": "completed",
    "whole_file": "completed",
    "related_context": "skipped_with_reason",
    "logical_stress": "completed",
    "tests": "completed"
  }
}
```

### ReviewFinding

A single actionable observation:

| Field | Type | Description |
|---|---|---|
| `id` | String | F001, F002, ... |
| `severity` | String | `blocking`, `major`, `minor`, `question`, `nitpick` |
| `layer` | String | `diff`, `whole_file`, `related_context`, `logical_stress`, `tests`, `global` |
| `category` | String | e.g., `timeout-handling`, `error-propagation` |
| `path` | String? | Target file path |
| `line` | u32? | Target line |
| `hunk_header` | String? | Diff hunk when line is unavailable |
| `summary` | String | One-line finding description |
| `why_it_matters` | String | Impact explanation |
| `suggested_remediation` | String | How to fix |
| `evidence` | Vec<String> | Context IDs backing this finding |
| `comment_id` | String? | Linked canonical comment ID (C001, ...) |

### GithubComment

A canonical actionable comment shared between JSON and Markdown:

| Field | Type |
|---|---|
| `id` | String (C001, C002, ...) |
| `path` | String? |
| `line` | u32? |
| `side` | String? |
| `hunk_header` | String? |
| `type` | String (issue, suggestion, question, nitpick) |
| `severity` | String |
| `blocking` | bool |
| `category` | String |
| `body` | String |
| `why_it_matters` | String |
| `suggested_remediation` | String |
| `layer` | String |

### ReviewRecommendation

```
Approve
Comment
Request changes
```
