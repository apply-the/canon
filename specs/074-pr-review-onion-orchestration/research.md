# Research: Agent-Governed Onion-Layer PR Review

## Decision 1: Onion-Layer Orchestration Model

- **Decision**: Five sequential layers (diff → whole-file → related-context → logical-stress → tests), each with its own run state, context packet, and instructions. Canon orchestrates each layer transition.
- **Rationale**: The user explicitly rejected a single `prepare → LLM review → accept → finalize` pipeline. A multi-step orchestrated workflow forces the LLM to inspect code in layers, preventing shallow diff-only review.
- **Alternatives considered**: Single-step LLM review (rejected — produces shallow reviews). Parallel layers (rejected — loss of progressive context building).

## Decision 2: File-Based Handoff with Optional Stdio

- **Decision**: File-based handoff is normative. Canon writes context packets as files; the LLM reads from disk. Stdio is an optional transport optimization for automated pipelines.
- **Rationale**: File-based handoff is inspectable, deterministic, testable, and compatible with IDE-based agents. All inputs and outputs must be persisted on disk before validation.
- **Alternatives considered**: Stdio-only (rejected — not inspectable, no traceability). Network API (rejected — complicates testing, introduces service dependencies).

## Decision 3: Format Strategy (Markdown/TSV/JSON Split)

- **Decision**: Markdown for instructions and LLM-authored outputs. TSV for compact LLM-facing indexes (context index, changed files, high-risk files, relation hints). JSON for validated schemas, canonical outputs, and automation artifacts.
- **Rationale**: TSV avoids repeated field names in large flat lists, making it token-efficient for LLM scanning. Markdown is natural for LLM instruction comprehension. JSON is appropriate for validated machine contracts.
- **Alternatives considered**: JSON for everything (rejected — wastes LLM context tokens on field names). Markdown for everything (rejected — harder to parse machine contracts).

## Decision 4: LLM-Authored Markdown vs Canon-Generated JSON

- **Decision**: LLM writes layer outputs and final reviewer output in Markdown with structured sections. Canon parses, validates, and compiles into `canonical-review-output.json` and final artifacts.
- **Rationale**: Markdown is a natural authoring format for LLMs. Canon remains the single source of validated truth by compiling Markdown into validated JSON.
- **Alternatives considered**: LLM writes JSON directly (rejected — error-prone, LLM hallucinates JSON fields). Canon auto-generates everything (rejected — removes LLM semantic value).

## Decision 5: Compact Context Indexes Over Full File Embedding

- **Decision**: Canon provides compact TSV context indexes with file paths, line ranges, risk hints, and layer relevance. LLM inspects full files on demand through helper commands or IDE access.
- **Rationale**: Embedding full file content in every layer's context packet would exceed LLM context windows and duplicate payloads. Progressive discovery is token-efficient.
- **Alternatives considered**: Embed all files in every layer (rejected — wastes context window). No context at all (rejected — LLM needs guidance).

## Decision 6: Layer Completion Rules (Completed/Skipped/Failed)

- **Decision**: Every layer must end in a terminal state: `completed`, `skipped_with_reason`, or `failed`. `finalize` blocks when any layer is missing. Skip/failure records must include reason, impact, and timestamp.
- **Rationale**: Prevents incomplete reviews from being presented as complete. Makes skipped layers explicit rather than silently absent.
- **Alternatives considered**: Allow finalize with any layer state (rejected — silent incompleteness). Require all layers always (rejected — some repos have no tests, no related files, etc.).
