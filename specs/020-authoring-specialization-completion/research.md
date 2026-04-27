# Research: Mode Authoring Specialization Completion

## Decision 1: Finish the remaining four modes in one bounded slice

- Decision: Complete `review`, `verification`, `incident`, and `migration` together and pair the rollout closeout with the `0.20.0` version/docs update.
- Rationale: Splitting the last four modes would keep the roadmap and release surfaces in a misleading half-complete state while reusing the same renderer/doc/test pattern twice.
- Alternatives considered:
  - Deliver `review` and `verification` first, then do the operational modes later. Rejected because it leaves the operational half of the specialization visibly unfinished.
  - Deliver only `incident` and `migration`. Rejected because the overall specialization would still be incomplete and the release/docs sync would remain ambiguous.

## Decision 2: Reuse shared authored-section extraction and missing-body helpers

- Decision: Reuse `render_authored_artifact()`, `extract_authored_h2_section()`, and `## Missing Authored Body` behavior as the common foundation for the remaining modes.
- Rationale: The helper path already anchors the specialization behavior in previously delivered modes and reduces the chance of drifting into four separate honesty rules.
- Alternatives considered:
  - Build a new mode-specific preservation helper for each remaining mode. Rejected because it duplicates proven behavior and increases regression surface.
  - Keep the current summary-driven renderers and only update docs. Rejected because the feature goal is runtime honesty, not documentation alone.

## Decision 3: Keep canonical headings strict and explicit

- Decision: Treat the documented canonical H2 headings as the contract; a missing, blank, or near-match heading produces `## Missing Authored Body` unless an alias is explicitly documented.
- Rationale: The specialization is meant to make authoring contracts reviewable, and permissive matching would hide drift.
- Alternatives considered:
  - Fuzzy-match similar headings automatically. Rejected because it makes contract enforcement opaque.
  - Accept any heading with a similar noun phrase. Rejected because the runtime and docs would no longer have one auditable contract.

## Decision 4: Preserve each mode's current governance posture

- Decision: Improve authored-body fidelity without changing `review` disposition semantics, `verification` blocked-readiness semantics, or `incident`/`migration` recommendation-only posture.
- Rationale: The slice is about packet honesty and discoverability, not governance policy churn.
- Alternatives considered:
  - Simplify gate outcomes while changing the renderers. Rejected because that widens the blast radius from representation into policy.
  - Treat missing authored bodies as soft warnings only. Rejected because the spec requires explicit, reviewer-visible incompleteness and honest blocked outcomes.

## Decision 5: Treat release/docs surfaces as first-class contract surfaces

- Decision: Update roadmap, changelog, compatibility references, guide text, and versioned manifests as part of the feature rather than as postscript cleanup.
- Rationale: Once the remaining four modes land, the repo should describe specialization as complete and report `0.20.0` consistently.
- Alternatives considered:
  - Land code/tests first and defer version/docs sync. Rejected because the repo uses those surfaces to describe delivered capability.
  - Update only `Cargo.toml` and changelog. Rejected because roadmap, guide, and compatibility references would still contradict the shipped behavior.

## Decision 6: Implement the shared runtime change from the lowest-coupling path first

- Decision: Use `incident` and `migration` as the first runtime conversion path because they currently depend on direct marker extraction from an evidence-mixed summary and can move cleanly to authored-body preservation with minimal cross-mode coupling.
- Rationale: This yields the smallest falsifiable implementation slice and validates the authored-source handoff pattern before refactoring `review` and `verification`, which have more mode-specific summary and verdict logic.
- Alternatives considered:
  - Start with `review`. Rejected because its disposition semantics are the most coupled to packet text and gate outcomes.
  - Start with `verification`. Rejected because its renderer currently synthesizes several sections from multiple summaries and has more paths to preserve.