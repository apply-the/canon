# Validation Report Plan

## Structural Validation
- [ ] `github-comments.json` strictly adheres to the schema (can be deserialized via `serde`).
- [ ] `review-summary.md` renders the `Decision` at the top level and includes the `Must Fix` section.
- [ ] Large diff output includes `review_coverage` metadata object.

## Logical Validation
- [ ] Unit tests in `canon-engine` for line validation against a mock patch file.
- [ ] Unit test asserting a finding without a valid line number is downgraded to a hunk-level finding.
- [ ] Engine tests asserting `Decision::Approve` is NEVER returned if `blocking: true` is present in any finding.
