# Data Model: PR Review Mode

## Entities

### `ReviewFinding` (Used in `review-findings.json`)
- `id`: String
- `kind`: String ("code", "governance", etc.)
- `path`: Option<String>
- `line`: Option<u32>
- `hunk_header`: Option<String>
- `severity`: String ("blocking", "non-blocking")
- `category`: String
- `summary`: String
- `evidence`: Vec<String>
- `recommended_action`: String
- `github_comment_id`: Option<String>

### `GithubComment` (Used in `github-comments.json`)
- `id`: String
- `path`: Option<String>
- `line`: Option<u32>
- `side`: Option<String>
- `hunk_header`: Option<String>
- `area`: String
- `type`: String (Conventional comment type)
- `blocking`: bool
- `severity`: String
- `category`: String
- `body`: String
- `why_it_matters`: String
- `suggested_remediation`: String
- `suggested_change`: Option<String>

### `MissingTest` (Used in `missing-tests.md`)
- `id`: String
- `affected_behavior`: String
- `reason`: String
- `risk`: String
- `suggested_shape`: String
- `blocking`: bool

### `ReviewCoverage`
- `changed_files_total`: u32
- `files_reviewed_deeply`: u32
- `files_sampled`: u32
- `files_not_reviewed_deeply`: u32
- `coverage_strategy`: String
- `unreviewed_risk`: String
