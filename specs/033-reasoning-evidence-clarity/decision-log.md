# Decision Log: Cross-Mode Reasoning Evidence And Clarity Expansion

## D-001: Deliver feature 033 as one end-to-end change

- **Decision**: Keep runtime behavior, mirrored skills, templates, examples,
  docs, version bump, changelog, coverage, `cargo clippy`, and `cargo fmt`
  inside one feature delivery.
- **Rationale**: The user explicitly requested that 033 not be split into
  slices, and the runtime contract would be incomplete if the authoring
  surfaces drifted.

## D-002: Reuse the existing clarity contract as the main expansion point

- **Decision**: Extend `inspect_clarity` and `reasoning_signals` from the
  existing requirements, discovery, and supply-chain implementations instead of
  inventing a parallel reasoning-evaluation service.
- **Rationale**: The clarity contract is already inspectable, serialized, and
  wired into the CLI. Extending it is lower-risk than creating a second path.

## D-003: Keep `pr-review` diff-backed while aligning its honesty posture

- **Decision**: Do not force `pr-review` into the file-backed clarity intake;
  instead, align its review packet and summary posture with the broader
  reasoning-evidence contract.
- **Rationale**: `pr-review` is fundamentally diff-driven, but it still has the
  same product risk of sounding authoritative without surfacing real evidence
  or explicit absence of findings.

## D-004: Tighten placeholder-heavy fallback surfaces instead of soft-scoring them

- **Decision**: Replace or tighten generic renderer fallback prose so missing
  reasoning reads as an honest gap or closure finding, not as synthetic user
  content.
- **Rationale**: The strongest “template compiler” failure mode in the current
  codebase is visible placeholder prose, especially in planning artifacts.

## D-005: Treat release alignment and touched-Rust-file coverage as contractual

- **Decision**: Make `0.33.0` version alignment, impacted docs plus changelog,
  focused Rust coverage, `cargo clippy`, and `cargo fmt` required closeout work
  inside the implementation plan and task graph.
- **Rationale**: Release drift and missing validation coverage are already
  known failure modes in this repository and must not be deferred.

## User Story 1 Decisions

- **Decision**: Route every supported file-backed governed mode through the
  shared authored-mode clarity family instead of adding mode-local clarity
  forks.
- **Rationale**: The runtime already had one inspectable clarity contract, so a
  shared authored-mode family keeps reasoning signals uniform and keeps
  `pr-review` explicitly excluded as a diff-backed surface.

- **Decision**: Treat materially closed choices as a first-class clarity
  outcome rather than a weaker form of missing context.
- **Rationale**: Canon must not invent balance or rejected alternatives when
  the authored packet already bounds the decision to one viable path.

## User Story 2 Decisions

- **Decision**: Backlog fallback artifacts must state explicitly that Canon did
  not synthesize approved decomposition when authored epic or slice content is
  missing.
- **Rationale**: Planning artifacts were the strongest template-compiler risk;
  generic epics, slices, or sequencing prose would overstate approval.

- **Decision**: Review and verification summaries should lift explicit
  `evidence-bounded` and `no-direct-contradiction` posture from the emitted
  artifacts.
- **Rationale**: The honesty markers already existed in the packet content; the
  summarizer layer was flattening them into weaker generic success language.

## User Story 3 Decisions

- **Decision**: Synchronize the delivered reasoning contract through
  `canon-inspect-clarity` and the shared output-shape references instead of
  rewriting every mode-specific skill surface.
- **Rationale**: The shared references are the authoritative assistant-facing
  contract for the new clarity and reasoning posture, so updating them avoids
  unnecessary drift-heavy churn.

- **Decision**: Keep existing templates and worked examples unchanged when they
  already encode evidence, tradeoff, contradiction, or unresolved-findings H2
  contracts for the targeted mode families.
- **Rationale**: The existing authoring surfaces already require explicit
  evidence-bearing sections; rewriting them would add churn without increasing
  contract coverage.

- **Decision**: Do not add repository-doc prose regression tests for
  `README.md`, `ROADMAP.md`, `docs/guides/`, or `CHANGELOG.md`.
- **Rationale**: The user explicitly asked to remove brittle doc-content tests,
  so release alignment is validated through runtime-backed checks, skill
  validation, and focused release-surface review instead.