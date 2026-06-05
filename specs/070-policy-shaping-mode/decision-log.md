# Decision Log: Policy Shaping Mode

## Decision 1: Hybrid CLI + LLM validation
- **Context**: Evaluating policy compliance across an entire codebase is too complex for static AST analysis, but pure LLM prompts violate Canon's deterministic workflow constraints.
- **Decision**: The Canon CLI orchestrates the validation boundary and required artifacts, while LLM-backed `.agents/skills` provide semantic evaluation of the codebase.
- **Rationale**: Best of both worlds: deterministic governance and semantic flexibility.

## Decision 2: Impact Report Pagination
- **Context**: A massive policy change could affect thousands of files, breaking context windows.
- **Decision**: Violations are grouped by directory/module. The main report is a quantified summary, and individual files go to an appendix.
- **Rationale**: Prevents LLM context overflow and keeps human-readable documents focused.

## Decision 3: No automatic failures on broad impact
- **Context**: If a policy breaks 1,000 files, should it instantly fail?
- **Decision**: No, it requires an explicit broad-impact approval gate rather than automatically failing.
- **Rationale**: Some policies (like repo-wide formatting or strict logging) are meant to be broad. Explicit gating is safer than arbitrary hard limits.
