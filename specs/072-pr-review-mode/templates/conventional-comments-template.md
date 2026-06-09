# Conventional Comments

> Human-readable, copy-ready rendering of the canonical actionable comment set.
> Every comment ID in this file must also exist in `github-comments.json`.
> Governance-only observations must not appear here unless converted into concrete actionable findings.

## Summary

{{ total_actionable_comments }} actionable comment(s):

| Severity | Count |
|---|---:|
| Blocking | {{ blocking_count }} |
| Major | {{ major_count }} |
| Minor | {{ minor_count }} |
| Questions | {{ question_count }} |
| Nitpicks | {{ nitpick_count }} |

## Sorting Rules

File comments must be rendered before global comments.

File comments must be:

1. grouped by file path
2. sorted lexicographically by file path
3. sorted within each file by severity
4. sorted within severity by line number or hunk order

Severity order:

```text
blocking
major
minor
question
nitpick
```

Global comments must appear at the end.

---

## Blocking File Comments

{{ blocking_file_comments }}

Template for each blocking file comment:

```md
### `{{ path }}`

#### {{ comment_id }}

Severity: blocking
Target: {{ target }}

issue(blocking): {{ concise_comment }}

Why it matters:
{{ why_it_matters }}

Suggested remediation:
{{ suggested_remediation }}
```

---

## Major File Comments

{{ major_file_comments }}

Template for each major file comment:

```md
### `{{ path }}`

#### {{ comment_id }}

Severity: major
Target: {{ target }}

issue(major): {{ concise_comment }}

Why it matters:
{{ why_it_matters }}

Suggested remediation:
{{ suggested_remediation }}
```

---

## Minor File Comments

{{ minor_file_comments }}

Template for each minor file comment:

```md
### `{{ path }}`

#### {{ comment_id }}

Severity: minor
Target: {{ target }}

issue(minor): {{ concise_comment }}

Why it matters:
{{ why_it_matters }}

Suggested remediation:
{{ suggested_remediation }}
```

---

## Questions

{{ question_file_comments }}

Template for each question:

```md
### `{{ path }}`

#### {{ comment_id }}

Severity: question
Target: {{ target }}

question(non-blocking): {{ concise_question }}

Why it matters:
{{ why_it_matters }}

Suggested remediation:
{{ suggested_remediation }}
```

---

## Nitpicks

{{ nitpick_file_comments }}

Template for each nitpick:

```md
### `{{ path }}`

#### {{ comment_id }}

Severity: nitpick
Target: {{ target }}

nitpick(non-blocking): {{ concise_comment }}

Why it matters:
{{ why_it_matters }}

Suggested remediation:
{{ suggested_remediation }}
```

---

## Global Comments

> Global comments apply to the whole PR or to review coverage. They appear after file comments.

{{ global_comments }}

Template for each global comment:

```md
#### {{ comment_id }}

Severity: {{ severity }}
Target: whole PR

{{ conventional_comment_type }}({{ blocking_or_non_blocking }}): {{ concise_comment }}

Why it matters:
{{ why_it_matters }}

Suggested remediation:
{{ suggested_remediation }}
```

---

## Empty Comment Set Handling

If there are no actionable comments, do not stop at “0 comments”.

Explain why:

```md
No actionable comments were emitted.

Reason: {{ empty_comment_reason }}

Actionable review status: {{ actionable_review_status }}

Review coverage:
- Files inspected deeply: {{ files_inspected_deeply_count }}
- Files skipped: {{ files_skipped_count }}
- Limitations: {{ coverage_limitations }}
```

Allowed reasons:

```text
valid_empty_actionable_review
actionable_review_not_configured
actionable_review_failed
governance_only
partial_review_no_actionable_findings
```

---

## Consistency Checks

- [ ] Every comment has a stable ID.
- [ ] Every comment ID exists in `github-comments.json`.
- [ ] Every file comment has a path.
- [ ] Every inline comment has a line and side.
- [ ] Every hunk comment has a hunk header.
- [ ] Global comments have no path, line, side, or hunk header.
- [ ] File comments are sorted lexicographically by path.
- [ ] Global comments appear at the end.
- [ ] Governance-only observations are excluded unless converted into concrete actionable findings.
