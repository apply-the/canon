# CLI Contract: pr-review

## Inputs
`canon pr-review --patch <path-to-diff> --changed-files <path-to-files-list>`

## Outputs
- `review-summary.md`
- `github-comments.json`
- `conventional-comments.md`
- `missing-tests.md`
- `review-findings.json`
- `state.toml`, `run.toml`, etc. (Secondary)

## Exit Codes
- `0`: Success (Review generated successfully, regardless of Approve/Request Changes).
- `1`: Invalid arguments.
- `2`: Engine execution error.
