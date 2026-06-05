# Quickstart

1. Generate a patch: `git diff main > diff.patch`
2. Run canon review: `canon pr-review --patch diff.patch`
3. Inspect `review-summary.md` to see the decision (Approve/Comment/Request Changes).
4. Inspect `github-comments.json` for precise line-level feedback ready for a GitHub Action integration.
