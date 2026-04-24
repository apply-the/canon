# Design Decision Log: Backlog Mode

## D-001: Introduce backlog as a first-class mode

- **Status**: Proposed
- **Context**: Canon currently has governed flows for problem understanding, shaping, decisions, execution, and review, but lacks a governed bridge from approved decisions into delivery decomposition.
- **Decision**: Add `backlog` as a first-class mode instead of treating delivery decomposition as an informal step outside the runtime.
- **Consequences**: Runtime, docs, defaults, skills, and tests must all acknowledge backlog as a real mode rather than a note in feature planning.

## D-002: Reuse the existing canonical authored-input pattern

- **Status**: Proposed
- **Context**: Existing modes use `canon-input/<mode>.md|/` for predictable authored input discovery and immutable snapshots.
- **Decision**: Use `canon-input/backlog.md` and `canon-input/backlog/` as the canonical authored-input locations for backlog mode.
- **Consequences**: CLI auto-binding remains consistent with current user expectations, and the runtime can reuse the current snapshot model.

## D-003: Keep the current backlog brief authoritative

- **Status**: Proposed
- **Context**: Backlog depends on upstream architecture and related artifacts, but it also needs a current planning brief that states delivery intent and boundaries explicitly.
- **Decision**: Treat the current backlog brief as authoritative, with upstream packets serving as provenance and evidence rather than silent overrides.
- **Consequences**: The mode remains grounded in present planning intent and avoids hidden dependency on prior packets.

## D-004: Make closure gating explicit and durable

- **Status**: Proposed
- **Context**: The feature must refuse false precision when source architecture is too vague or contradictory for credible decomposition.
- **Decision**: Represent closure assessment explicitly in the run context and user-visible summaries, with named findings that can block or downgrade decomposition.
- **Consequences**: Users receive durable reasons for planning failure, and later validation can prove Canon did not silently overreach.

## D-005: Publish backlog packets to `docs/planning/<RUN_ID>/`

- **Status**: Proposed
- **Context**: Backlog outputs are durable planning artifacts, not architecture decisions or executed changes.
- **Decision**: Use `docs/planning/<RUN_ID>/` as the default publish destination for backlog mode.
- **Consequences**: Published packets remain clearly identifiable as planning outputs and do not overload existing mode destinations.

## D-006: Differentiate successful packets from closure-limited packets

- **Status**: Proposed
- **Context**: The spec requires honest block-or-downgrade behavior when closure is insufficient.
- **Decision**: Successful runs emit the full eight-artifact backlog packet. Closure-limited runs emit a bounded risk-focused packet and must not present a fake full decomposition.
- **Consequences**: Packet shape communicates confidence level directly and reduces the risk of users acting on invented detail.

## D-007: No new top-level CLI surface is introduced

- **Status**: Proposed
- **Context**: Canon already has generic run lifecycle surfaces that work across modes.
- **Decision**: Keep `canon run --mode backlog` plus existing `status`, `inspect`, `list`, `resume`, and `publish` flows as the authoritative user surface.
- **Consequences**: Users do not need a parallel planning command family, and runtime changes stay bounded.

## D-008: Validate backlog mode using the same layered pattern as other mode promotions

- **Status**: Proposed
- **Context**: The constitution requires layered validation and explicit separation between generation and validation.
- **Decision**: Validate backlog through structural checks, dedicated contract and integration tests, skill validation, and an independent review of emitted planning packets.
- **Consequences**: Mode promotion remains auditable and consistent with recent repository governance practice.

## D-009: Implement backlog in the order mode surface -> full packet -> closure gating -> downstream handoff

- **Status**: Accepted for implementation
- **Context**: Backlog spans mode taxonomy, authored-input binding, artifact contracts, closure semantics, publish routing, and skill surfaces. Attempting all of them at once would increase blast radius and weaken validation clarity.
- **Decision**: Execute implementation in four ordered slices: shared mode/runtime scaffolding first, successful full-packet generation second, closure gating and downgrade behavior third, and downstream handoff plus publish/skill surfaces last.
- **Consequences**: Each slice gains its own validation checkpoint, the MVP remains the successful full-packet path, and closure behavior is added only after the base packet path is stable.

## D-010: Successful backlog generation uses the canonical backlog packet and stays above task-level output

- **Status**: Accepted for implementation
- **Context**: US1 requires backlog to be independently runnable from `canon-input/backlog.md|/`, to emit the full eight-artifact planning packet, and to stay reusable for downstream implementation work without collapsing into task mapping.
- **Decision**: The successful backlog path now prefers the canonical backlog authored-input packet, persists a backlog planning context in `context.toml`, records read/generate/critique/validate lineage, and emits `backlog-overview.md`, `epic-tree.md`, `capability-to-epic-map.md`, `dependency-map.md`, `delivery-slices.md`, `sequencing-plan.md`, `acceptance-anchors.md`, and `planning-risks.md` with planning-only sections.
- **Consequences**: Full-packet runs are testable through existing CLI and inspect surfaces, primary-artifact summaries stay backlog-specific, and the packet remains bounded enough for later closure-aware refinement without inventing task-level execution detail.

## D-011: Closure-limited backlog runs emit a risk-only packet, with warning findings downgrading and blocking findings stopping the run

- **Status**: Accepted for implementation
- **Context**: US2 requires backlog to stay honest when the authored brief does not credibly support full decomposition. The runtime already persisted closure findings, but it still emitted the full packet and treated warning-only findings as architecture blockers.
- **Decision**: Backlog now derives an effective artifact contract from `closure_assessment.decomposition_scope`. Both blocked and downgraded closure-limited runs emit only `backlog-overview.md` plus `planning-risks.md`; blocking findings keep the run in `Blocked`, while warning-only findings degrade the run to a completed risk-only packet without pretending the missing decomposition exists.
- **Consequences**: Packet shape now matches closure confidence, risk-only bundles remain auditable through status and inspect surfaces, and downstream readers do not see fake epics, slices, or sequencing when the brief is still materially weak.

## D-012: Downstream backlog handoff stays on existing publish and lookup surfaces, with skills advertising only delivered behavior

- **Status**: Accepted for implementation
- **Context**: US3 requires a published backlog packet to remain readable outside Canon and a downstream implementation lead to recover source links, dependencies, sequencing, and acceptance anchors without hidden runtime state. The runtime publish path already reused Canon's generic lookup and destination model, but the backlog skill and top-level docs still described backlog as planned rather than delivered.
- **Decision**: Keep backlog on the existing `canon publish`, run lookup, inspect, and next-step surfaces; treat `docs/planning/<RUN_ID>/` as the authoritative public packet location; keep published backlog artifacts explicitly planning-only; and promote the backlog skill plus shared docs/index surfaces to `available-now` only after tests prove the packet and skill materialization behave as delivered.
- **Consequences**: Downstream readers can reuse backlog packets through Canon's existing lifecycle without a parallel handoff interface, publish and `@last`/short-id lookup compatibility remain generic instead of mode-specific, and skills/docs now align with runtime truth instead of aspirational roadmap text.