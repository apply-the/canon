# PR Review Summary

> Primary reviewer-facing entry point for a `pr-review` packet.
> This file summarizes the outcome. Detailed copy-ready comments live in `conventional-comments.md`.

## Recommendation

<!-- One of: Approve / Comment / Request changes -->

**Recommendation**: {{ recommendation }}

## Recommendation Rationale

{{ recommendation_rationale }}

## Review Status

| Field | Value |
|---|---|
| Actionable review status | {{ actionable_review_status }} |
| Reviewer adapter | {{ reviewer_adapter }} |
| Governance inspection status | {{ governance_inspection_status }} |
| Review coverage | {{ review_coverage_level }} |
| Generated at | {{ generated_at }} |
| Base ref | `{{ base_ref }}` |
| Head ref | `{{ head_ref }}` |

Allowed actionable review statuses:

```text
actionable_review_executed
actionable_review_failed
actionable_review_not_configured
governance_only
```

## Severity Summary

| Severity | Count |
|---|---:|
| Blocking | {{ blocking_count }} |
| Major | {{ major_count }} |
| Minor | {{ minor_count }} |
| Questions | {{ question_count }} |
| Nitpicks | {{ nitpick_count }} |

## Must Fix

<!-- Required when recommendation is Request changes. -->

{{ must_fix_summary }}

Example:

```md
- C001 / F001 — `src/client.rs:123`
  - issue(blocking): Timeout only wraps request sending, not response body reading.
  - Required remediation: Wrap the full HTTP operation in the timeout window.
```

## Should Fix

{{ should_fix_summary }}

## Questions

{{ questions_summary }}

## Missing Tests

{{ missing_tests_summary }}

Each missing test should include:

```md
- MT001 — {{ affected_behavior }}
  - Why it matters: {{ reason }}
  - Suggested test: {{ suggested_test_shape }}
  - Severity: {{ severity }}
```

## GitHub-Ready Comments Index

The following actionable comments are rendered in:

```text
conventional-comments.md
github-comments.json
```

| ID | Severity | Scope | Target | Summary |
|---|---|---|---|---|
{{ comments_index_rows }}

## Review Coverage

| Metric | Value |
|---|---:|
| Changed files | {{ changed_files_count }} |
| Files inspected deeply | {{ files_inspected_deeply_count }} |
| Files skipped | {{ files_skipped_count }} |
| Global findings | {{ global_findings_count }} |

### Files Inspected Deeply

{{ files_inspected_deeply }}

### Files Skipped

{{ files_skipped }}

### Coverage Limitations

{{ coverage_limitations }}

## Governance Observations

> Governance observations may influence the recommendation, but they are not GitHub-ready comments unless converted into concrete actionable findings.

{{ governance_observations }}

Example:

```md
- Boundary-marked surfaces changed.
  - Impact: explicit reviewer disposition required.
  - Not emitted as GitHub-ready comment because it is not a concrete code finding.
```

## Consistency Checks

- [ ] If recommendation is `Request changes`, at least one blocking actionable finding, failed required review step, or explicit governance disposition gate is listed.
- [ ] If actionable comments are empty, the review status explains why.
- [ ] `conventional-comments.md` and `github-comments.json` use the same comment IDs.
- [ ] Governance-only findings are not emitted as actionable comments.
- [ ] Missing tests are concrete, or the report explains why no missing tests were identified.

## Final Recommendation

**{{ recommendation }}**

{{ final_recommendation_next_step }}
