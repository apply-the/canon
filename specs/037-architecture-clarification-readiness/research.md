# Research: Architecture Clarification, Assumptions, And Readiness Reroute

## Decision 1: Extend the existing architecture clarity surface instead of creating a new clarification mode or inspect target

- **Decision**: Keep `canon inspect clarity --mode architecture` as the
  product entry point for bounded clarification and reroute guidance.
- **Rationale**: Canon already has a file-backed clarity surface that reports
  missing context, clarification questions, reasoning signals, and output
  quality. Extending that contract keeps the behavior machine-readable without
  creating a second workflow entry point.
- **Alternatives considered**:
  - Introduce a brand new `inspect architecture-readiness` target.
  - Add a separate interactive clarification mode detached from `inspect
    clarity`.

## Decision 2: Make architecture clarification questions more structured by adding affected surface and default-if-skipped metadata

- **Decision**: Extend architecture clarification question summaries so they can
  say not only the prompt and rationale, but also which packet surface the
  answer affects, the default applied if skipped, and whether the question is
  required or optional.
- **Rationale**: The existing skill guidance already frames clarification as a
  bounded loop with explicit default behavior. The inspect contract should
  expose that structure directly so maintainers can judge whether a question is
  truly decision-changing.
- **Alternatives considered**:
  - Keep the current prompt-only contract and rely on prose rationale.
  - Add full answer-option matrices to inspect output, which would widen the
    machine contract too much for this slice.

## Decision 3: Use `readiness-assessment.md` as the durable surface for assumptions and unresolved questions

- **Decision**: Expand `readiness-assessment.md` rather than add a separate
  assumptions artifact or formalize `ai-provenance.md` as a runtime contract in
  this slice.
- **Rationale**: Architecture mode already owns a readiness artifact, and the
  feature goal is to make readiness honest and reviewable. Reusing that file
  keeps the architecture packet compact and aligned with the current artifact
  family.
- **Alternatives considered**:
  - Add a new `assumptions-and-open-questions.md` artifact.
  - Promote `ai-provenance.md` into a new runtime-required contract.

## Decision 4: Reuse the documented mode handoff rules for reroute guidance

- **Decision**: Base reroute guidance on the existing mode guide: use
  `discovery` for blurry problem spaces, `requirements` for bounded problem
  framing, and `system-shaping` for capability structure that is not yet ready
  for structural option comparison.
- **Rationale**: The mode guide already teaches these boundaries. Reusing that
  vocabulary avoids inventing a second classification policy for architecture
  clarification.
- **Alternatives considered**:
  - Keep reroute guidance as freeform prose with no consistent trigger model.
  - Route all under-bounded architecture briefs back to `requirements`, which
    would erase the distinction between exploratory and capability-shaping work.

## Decision 5: Preserve missing-authored-body precedence over generated assumptions

- **Decision**: Continue treating omitted canonical sections as explicit
  `## Missing Authored Body` failures before any generated working assumptions
  are introduced.
- **Rationale**: The product already promises that missing architecture content
  is surfaced honestly. Converting omitted sections into assumptions would
  weaken the architecture contract and hide under-bounded authored input.
- **Alternatives considered**:
  - Automatically synthesize assumptions when authored sections are missing.
  - Treat every missing authored section as a clarification question instead of
    preserving the stronger missing-body signal.