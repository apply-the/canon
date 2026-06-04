# Research & Architecture Decisions: Policy Shaping Mode

## Decision 1: Validation Execution Engine
- **Decision**: Hybrid CLI + Skills architecture. The `canon` CLI will orchestrate the validation pass and own the fail-closed envelope, while invoking `.agents/skills` to perform semantic LLM-driven evaluation on the codebase.
- **Rationale**: Policy evaluation often involves semantic meaning (e.g., "does this code use magic strings") that static analyzers cannot easily catch without massive configuration overhead. Leveraging the LLM for the heavy lifting while CLI guarantees the contract is the most robust approach.
- **Alternatives considered**: Pure Rust AST parsing (too complex for general policies); Pure LLM chat (violates constitution's artifact-first and deterministic constraints).

## Decision 2: Impact Report Pagination
- **Decision**: The impact report (`03-conformance-impact.md`) will group violations by module/directory and offload file-level granularity to a machine-readable appendix or secondary file.
- **Rationale**: A massive blast radius could exceed LLM context windows or human readability during migration planning.
- **Alternatives considered**: Hard limit failure (too brittle); dumping all violations into one file (context overflow risk).

## Decision 3: Structured Data Format
- **Decision**: YAML frontmatter inside Markdown files.
- **Rationale**: Required by Canon's documentation-driven workflow to balance human readability with strict machine-parseable data required by the CLI.
- **Alternatives considered**: Separate JSON sidecar files (adds clutter and sync issues).
