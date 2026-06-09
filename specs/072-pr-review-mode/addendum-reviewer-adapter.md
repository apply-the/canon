# Addendum: AI/LLM Reviewer Adapter for pr-review

**Date**: 2026-06-08
**Parent Spec**: `specs/072-pr-review-mode/spec.md`
**Branch**: `072-pr-review-reviewer-adapter`

## Summary

The original `072-pr-review-mode` spec assumed that `pr-review` would produce
actionable review comments through an AI/LLM-backed reviewer. The initial
implementation used only deterministic file-path heuristics (`ReviewPacket::from_diff`)
and a synthetic `CopilotCliAdapter` stub that never calls an actual LLM.

This addendum defines the provider-neutral reviewer adapter contract, the
integration point in the `pr-review` pipeline, and the template-driven
artifact rendering rules.

## Architecture

```
diff collection (ShellAdapter, deterministic)
→ governance inspection (ReviewPacket::from_diff, deterministic)
→ AI/LLM reviewer adapter invocation (provider-neutral)
→ structured reviewer output validation
→ canonical actionable findings
→ canonical actionable comments
→ Markdown + JSON rendering (template-driven)
→ report recommendation
```

## Reviewer Adapter Contract

The reviewer adapter MUST:

- Be invoked through a generic `ReviewerAdapter` trait, not provider-specific logic.
- Receive: diff, changed files, relevant context, and the required output schema.
- Return: structured JSON review output with findings, comments, and coverage metadata.
- Be replaceable at configuration time (stub, CLI, local model, remote model).

When no reviewer is configured:
- `pr-review` emits `actionable_review_not_configured` or `governance_only`.
- The report explains that no AI/LLM review was executed.

When the reviewer fails:
- `pr-review` emits `actionable_review_failed`.
- Does not silently fall back to an empty successful review.

## Template-Driven Rendering

Three Markdown templates define the normative output shape:

| Artifact | Template | Role |
|---|---|---|
| `01-review-summary.md` | `review-summary-template.md` | Primary reviewer-facing summary |
| `02-conventional-comments.md` | `conventional-comments-template.md` | Copy-ready actionable comments |
| `06-review-report.md` | `review-report-template.md` | Severity-oriented report |

Key rules:
- `github-comments.json` and `conventional-comments.md` share the same comment IDs.
- Governance-only observations stay in governance sections, not in actionable comments.
- No unresolved `{{ placeholder }}` values in generated artifacts.
- Empty comment sets must explain why (actionable_review_not_configured, etc.).

## Recommendation Logic

```
Request changes → blocking findings, required artifact failure,
                  actionable review failure, or required governance gate

Comment → non-blocking findings, partial coverage, questions,
          governance-only review, or actionable review not configured

Approve → no blocking findings, sufficient coverage,
          actionable review executed, no unresolved governance gates
```

## Testing

- Use a deterministic `StubReviewerAdapter` for all tests.
- Verify the full pipeline: schema validation, rendering, sorting, severity grouping,
  recommendation logic, and failure handling.
- No live LLM calls in tests.
