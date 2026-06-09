# PR Review Report

> Severity-oriented report used to understand the final recommendation.

## Recommendation

**{{ recommendation }}**

Allowed values:

```text
Approve
Comment
Request changes
```

## Recommendation Rationale

{{ recommendation_rationale }}

## Severity Summary

| Severity | Count |
|---|---:|
| Blocking | {{ blocking_count }} |
| Major | {{ major_count }} |
| Minor | {{ minor_count }} |
| Questions | {{ question_count }} |
| Nitpicks | {{ nitpick_count }} |

## Blocking Issues

{{ blocking_issues }}

## Major Issues

{{ major_issues }}

## Minor Issues

{{ minor_issues }}

## Questions

{{ questions }}

## Nitpicks

{{ nitpicks }}

## Review Coverage

| Field | Value |
|---|---|
| Actionable review status | {{ actionable_review_status }} |
| Reviewer adapter | {{ reviewer_adapter }} |
| Files changed | {{ changed_files_count }} |
| Files inspected deeply | {{ files_inspected_deeply_count }} |
| Files skipped | {{ files_skipped_count }} |
| Review depth | {{ review_depth }} |
| Exhaustive review | {{ exhaustive_review }} |

### Files Inspected Deeply

{{ files_inspected_deeply }}

### Files Skipped

{{ files_skipped }}

### Limitations

{{ coverage_limitations }}

## Governance Observations

{{ governance_observations }}

## Decision Rules Applied

{{ decision_rules_applied }}

## Final Recommendation

**{{ recommendation }}**

{{ final_recommendation_next_step }}
