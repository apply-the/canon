# PR Review Markdown Templates

This package contains Markdown templates for Canon `pr-review` outputs.

## Files

- `review-summary-template.md`
  - Primary reviewer-facing summary.
  - Contains recommendation, rationale, severity summary, must-fix items, missing tests, coverage, and governance observations.

- `conventional-comments-template.md`
  - Human-readable, copy-ready Conventional Comments output.
  - File comments are grouped by severity and sorted by file.
  - Global comments appear at the end.

- `review-report-template.md`
  - Severity-oriented report with recommendation and coverage details.

## Key Rules

- `github-comments.json` and `conventional-comments.md` must be two renderings of the same canonical actionable comment set.
- Every comment must have a stable ID.
- Comment IDs must match between JSON and Markdown.
- Governance-only observations must not be emitted as actionable comments unless converted into concrete findings.
- Empty actionable comments are acceptable only when the review status explains why.
- Tests should use deterministic fixtures or stub reviewer adapters, not live LLM calls.
