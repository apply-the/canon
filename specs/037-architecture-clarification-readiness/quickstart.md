# Quickstart: Architecture Clarification, Assumptions, And Readiness Reroute

## Goal

Validate that architecture mode asks only decision-changing clarification
questions, records explicit defaults, materializes assumptions and unresolved
questions in readiness output, and recommends reroute when the brief is not yet
architecture-ready.

## Prerequisites

- Repository checkout on branch `037-architecture-clarification-readiness`
- Rust toolchain available
- A writable temporary workspace for focused architecture briefs

## Walkthrough

1. Create an architecture brief that includes the canonical architecture
   headings but leaves one or two decision-changing assumptions unresolved.
2. Run `canon inspect clarity --mode architecture --input <brief> --output
   json` and confirm:
   - the question set is bounded
   - each question includes affected surface and default-if-skipped metadata
   - materially closed briefs do not get synthetic clarification churn
3. Create a second under-bounded brief that is really a discovery,
   requirements, or system-shaping problem and confirm architecture clarity now
   recommends reroute with explicit rationale.
4. Start an architecture run against a bounded brief and inspect the generated
   `readiness-assessment.md`.
5. Confirm the readiness artifact now records:
   - readiness status
   - working assumptions
   - unresolved questions
   - blockers
   - accepted risks
   - recommended next mode
6. Review the architecture template, example input, skill guidance, roadmap,
   changelog, and version surfaces for `0.37.0` alignment.

## Expected Result

- Architecture clarity becomes a bounded, decision-changing clarification
  surface instead of a generic interview.
- Under-bounded architecture briefs are rerouted honestly to existing earlier
  modes.
- Architecture readiness output makes the limiting assumptions and unresolved
  questions durable without inventing certainty.