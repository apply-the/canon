# Research: Backlog Mode (Delivery Decomposition)

## R-001: Backlog should be a first-class runtime mode, not a documentation-only convention

- **Decision**: Add `backlog` to Canon's mode taxonomy and runtime surfaces rather than treating it as a loose planning convention outside the existing run model.
- **Rationale**: The spec requires backlog to be inspectable, publishable, resumable, and governed through the existing Canon lifecycle. That is only credible if `backlog` is represented as a first-class mode in runtime modeling, defaults, CLI binding, and artifact contracts.
- **Alternatives considered**:
  - Keep backlog as a documentation-only planning pattern outside the runtime. Rejected because it would preserve the governance gap this feature exists to close.
  - Reuse `architecture` or `change` as a label for backlog work. Rejected because backlog has its own output contract and closure semantics.

## R-002: Canonical authored-input binding should extend the existing `canon-input/<mode>.md|/` pattern

- **Decision**: Use `canon-input/backlog.md` and `canon-input/backlog/` as the canonical authored-input locations for backlog mode, with `brief.md` authoritative in folder-backed packets and `priorities.md` plus `context-links.md` as supporting inputs.
- **Rationale**: Existing modes already rely on the `canon-input/<mode>.md|/` convention for predictable auto-binding and immutable snapshotting. Extending that pattern keeps backlog aligned with current user expectations and minimizes new CLI behavior.
- **Alternatives considered**:
  - Require explicit `--input` without a canonical location. Rejected because it would make backlog inconsistent with current governed-mode ergonomics.
  - Reuse `canon-input/change.*` or `canon-input/architecture.*`. Rejected because it would blur input intent and weaken mode boundaries.

## R-003: Closure findings should live in existing run context and summaries, not a parallel planning manifest

- **Decision**: Persist backlog planning context and closure findings through the existing run context and artifact model, and surface them through status/inspect summaries rather than inventing a separate planning manifest family.
- **Rationale**: Canon already treats `.canon/runs/<RUN_ID>/` plus emitted artifacts as the system of record. Reusing those surfaces preserves inspectability and backward compatibility while keeping closure semantics machine-readable.
- **Alternatives considered**:
  - Create a second planning-only manifest outside `.canon/runs/`. Rejected because it would split the durable record and complicate lifecycle surfaces.
  - Store closure findings only in prose artifacts. Rejected because status and inspect need structured reasons before a human opens every document.

## R-004: Backlog needs an explicit artifact contract aligned with the feature brief

- **Decision**: Define backlog as a mode with a named artifact contract covering `backlog-overview.md`, `epic-tree.md`, `capability-to-epic-map.md`, `dependency-map.md`, `delivery-slices.md`, `sequencing-plan.md`, `acceptance-anchors.md`, and `planning-risks.md`.
- **Rationale**: The runtime and publish pipeline rely on explicit artifact contracts to keep persisted outputs inspectable, testable, and mode-distinct. Backlog cannot remain a vague packet shape if it is to become a first-class mode.
- **Alternatives considered**:
  - Emit one generic `backlog.md` file with subsections. Rejected because it weakens packet structure, inspectability, and downstream traceability.
  - Derive artifact requirements only from documentation. Rejected because documentation is not the runtime contract.

## R-005: Publish should route backlog packets to `docs/planning/<RUN_ID>/`

- **Decision**: Use `docs/planning/<RUN_ID>/` as the default publish destination for backlog packets.
- **Rationale**: The feature brief in `NEXT_FEATURES.md` already names `docs/planning/<RUN_ID>/` as the natural public destination. Using that path keeps backlog outputs recognizable as planning artifacts instead of overloading architecture or change destinations.
- **Alternatives considered**:
  - Publish under `docs/backlog/<RUN_ID>/`. Rejected because the repository's own feature brief already establishes `docs/planning/` as the intended public surface.
  - Publish under `docs/changes/<RUN_ID>/`. Rejected because backlog is decomposition planning, not executed change work.

## R-006: Successful and closure-blocked runs need different packet expectations

- **Decision**: Successful backlog runs emit the full eight-artifact packet. Closure-blocked or downgraded runs emit a bounded packet centered on `backlog-overview.md` and `planning-risks.md`, with explicit closure findings surfaced in summaries and context, and must not emit a misleading full decomposition.
- **Rationale**: The feature requires honest degradation when source architecture is not sufficiently closed. Reusing the full packet shape for a blocked run would imply unjustified precision.
- **Alternatives considered**:
  - Always emit all eight artifacts and mark them partial. Rejected because it would still suggest a fuller decomposition than the evidence supports.
  - Emit no artifacts on blocked runs. Rejected because users still need a durable explanation of why decomposition failed.

## R-007: No new top-level CLI surface is required

- **Decision**: Keep `canon run --mode backlog` as the entrypoint and continue to rely on the existing `status`, `inspect`, `list`, `resume`, and `publish` commands for lifecycle visibility.
- **Rationale**: Backlog is a new mode, not a new product surface. The current lifecycle commands already cover the required visibility if backlog emits proper summaries and artifacts.
- **Alternatives considered**:
  - Add a dedicated `canon backlog` command family. Rejected because it would fracture the CLI and duplicate existing lifecycle operations.
  - Hide backlog primarily behind skills. Rejected because the runtime must stay CLI-first and skills must reflect runtime truth rather than replace it.

## R-008: Skills and docs should update only after runtime truth is designed

- **Decision**: Add a dedicated backlog skill and update docs/defaults in lockstep with runtime design so product messaging only advertises delivered mode behavior.
- **Rationale**: Canon already treats embedded and materialized skills as discoverable product surfaces. They must remain accurate and testable against the actual runtime contract.
- **Alternatives considered**:
  - Publish backlog docs/skills ahead of runtime design. Rejected because that would recreate the exact "modeled-only but described as available" drift the repo has been fixing elsewhere.

## R-009: Validation should mirror existing mode-promotion patterns

- **Decision**: Reuse the existing layered validation strategy from promoted modes: structural checks, new contract tests, new integration tests, skill validation, and an independent packet review.
- **Rationale**: The constitution requires layered verification and separation of generation from validation. Existing mode-promotion features already prove the repository can validate new modes this way.
- **Alternatives considered**:
  - Rely only on spec review and docs checks. Rejected because they do not prove runtime binding, packet emission, or closure gating.
  - Delay independent review until after implementation. Rejected because backlog's artifact credibility is a core design risk and must be challenged early.