# Quickstart: Decision Alternatives, Pattern Choices, And Framework Evaluations

## Goal

Validate that the `022` slice preserves explicit decision alternatives,
framework evaluation reasoning, and bounded persona guidance without changing
Canon runtime semantics.

## Recommended Validation Flow

1. Confirm the updated skills, templates, and examples for `system-shaping`,
   `architecture`, `change`, `implementation`, and `migration` describe the
   new decision or evaluation sections and their bounded personas.
2. Confirm `review`, `pr-review`, `verification`, and `incident` guidance now
   declare explicit bounded personas without implying new runtime behavior.
3. Run focused docs and renderer tests for the targeted modes to prove positive
   preservation and negative-path honesty behavior.
4. Run `scripts/validate-canon-skills.sh` to confirm embedded and mirrored
   skills remain synchronized.
5. Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and a full `cargo nextest run` before closeout.
6. Review `ROADMAP.md`, `README.md`, `CHANGELOG.md`, `docs/guides/modes.md`,
   and runtime-compatibility references to confirm the `0.22.0` release
   surface and the remaining roadmap candidates are explicit.

## Representative Packet Walkthroughs

- `system-shaping`: compare at least two structural patterns and verify the
  packet preserves options, tradeoffs, and why-not reasoning.
- `architecture`: verify the existing ADR plus options packet still exposes the
  winning option, rejected alternatives, and C4 context while staying aligned
  to the broader decision-alternatives vocabulary.
- `change`: compare at least two bounded modification paths and verify the
  packet surfaces preserved tradeoffs without fabricating missing analysis.
- `implementation`: compare at least two concrete frameworks or libraries and
  verify ecosystem-health and adoption implications are explicit.
- `migration`: compare coexistence and replacement paths and verify rollback or
  coexistence burden remains reviewable.
