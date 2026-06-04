# Research: Backlog Handoff Contract

## Decision 1: Use stable slice identifiers, not task identifiers

**Decision**: Canon will introduce stable `slice_id` values for delivery
slices rather than generating task-level identifiers.

**Rationale**: The existing backlog contract explicitly stays above
implementation-task detail. Stable slice identity gives downstream runtimes a
durable handle they can validate without forcing Canon to become a task
generator.

**Alternatives considered**:

- Generate task-level IDs in backlog mode: rejected because it breaks the
  existing backlog granularity invariant.
- Keep prose-only slice names: rejected because downstream runtimes cannot
  validate cross-artifact consistency reliably from unstable prose labels.
- Reuse epic names as identity: rejected because multiple slices may exist
  under one epic and need independent handoff truth.

## Decision 2: Add a dedicated `execution-handoff.md` artifact

**Decision**: Emit a separate `execution-handoff.md` artifact when at least one
slice is credible for downstream execution handoff.

**Rationale**: A dedicated additive artifact keeps the existing backlog packet
readable while making handoff availability explicit and auditable. It also
lets Canon say "planning packet is complete, handoff is unavailable" without
polluting every other artifact with downstream-only semantics.

**Alternatives considered**:

- Encode handoff only inside `backlog-overview.md`: rejected because downstream
  consumers would still need to scrape prose for critical evidence.
- Expand `delivery-slices.md` into a handoff manifest: rejected because that
  document should stay slice-oriented, not become the packet's downstream
  control file.
- Always emit `execution-handoff.md` even when unavailable: rejected because an
  absent artifact is a cleaner signal for risk-only and no-handoff cases, as
  long as overview and inspect surfaces explain why.

## Decision 3: Full planning success and handoff availability are distinct truths

**Decision**: A successful full planning packet may still be handoff-unavailable
when no slice has sufficient implementation refs or independent verification
anchors.

**Rationale**: Planning completeness answers a different question from
execution-readiness. Conflating them would either under-report valuable
planning packets or overstate downstream safety.

**Alternatives considered**:

- Treat missing handoff evidence as a blocked backlog run: rejected because the
  planning packet may still be credible and useful.
- Treat every full packet as handoff-ready: rejected because it would recreate
  the exact producer-gap problem found by Boundline.

## Decision 4: Handoff evidence must be slice-scoped and independently checkable

**Decision**: A handoff-capable slice must declare implementation artifact
references, dependency prerequisites, and independent verification anchors that
can be reviewed without trusting the original planner's confidence alone.

**Rationale**: Downstream execution gates need bounded evidence, not just
English confidence language. Slice-scoped evidence preserves traceability and
lets later validators challenge the handoff independently.

**Alternatives considered**:

- Use acceptance anchors alone as handoff evidence: rejected because planning
  completion signals are not necessarily independent proof targets.
- Permit vague implementation areas like "backend work" or "frontend cleanup":
  rejected because downstream readers cannot act on or validate such labels
  consistently.

## Decision 5: Preserve downstream ownership boundaries

**Decision**: Canon emits governed handoff signals, but downstream runtimes
such as Boundline continue to own execution admission decisions.

**Rationale**: Canon's job is governed packet generation. Execution admission
is runtime-owned and may require additional local policy, environment, or
verification gates beyond what Canon can know.

**Alternatives considered**:

- Let Canon declare execution approval directly: rejected because it would
  expand Canon into runtime orchestration rather than artifact governance.
- Special-case Boundline in Canon packet format: rejected because the packet
  contract must remain consumer-agnostic.
