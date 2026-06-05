# Research: PR Review Refactor

## Decision 1: Diff Line Mapping
- **Decision**: Implement a deterministic mapping function in `canon-engine` that checks the generated `hunk_header` or `line` against the `diff.patch`. If out of bounds, downgrade the finding to `hunk` or `general`.
- **Rationale**: LLMs frequently hallucinate exact line numbers, which crashes or silences strict integrations.
- **Alternatives considered**: Fuzzy-matching text snippets within the file (too complex and slow for CLI).

## Decision 2: Output JSON Structure
- **Decision**: Define strict `serde` structs for `github-comments.json` and `review-findings.json`.
- **Rationale**: Constitution Principle VIII and language rules explicitly ban `serde_json::Map` for stable outputs.
- **Alternatives considered**: Generic maps (rejected due to constitution).
