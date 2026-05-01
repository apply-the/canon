# Quickstart: Decision Alternatives, Pattern Choices, And Framework Evaluations

## Goal

Validate that the `028` slice preserves explicit structural alternatives,
framework-evaluation reasoning, decision-evidence honesty, and `0.28.0`
release-surface alignment without changing Canon runtime semantics.

## Recommended Validation Flow

1. Confirm the updated skills, templates, and examples for `system-shaping`,
   `change`, `implementation`, and `migration` describe the new canonical
   decision-analysis or framework-evaluation sections.
2. Confirm `architecture` still presents the established option-analysis
   pattern and remains aligned with the broader decision-support vocabulary.
3. Run focused docs, renderer, contract, and run tests for the targeted modes
   to prove positive preservation, closed-decision behavior, and missing-section
   honesty behavior.
4. Run `/bin/bash scripts/validate-canon-skills.sh` to confirm embedded and
   mirrored skills remain synchronized.
5. Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, and a full `cargo nextest run --workspace --all-features` before closeout.
6. Review `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, runtime-compatibility
   references, `README.md`, `ROADMAP.md`, and `docs/guides/modes.md` to
   confirm the `0.28.0` release surface is explicit and internally consistent.

## Representative Packet Walkthroughs

- `system-shaping`: compare at least two structural patterns and verify the
  packet preserves decision drivers, tradeoffs, and why-not reasoning.
- `change`: compare at least two bounded modification paths and verify the
  packet surfaces preserved tradeoffs without fabricating missing analysis.
- `implementation`: compare at least two concrete frameworks or libraries and
  verify evidence references, ecosystem-health posture, and adoption
  implications are explicit.
- `migration`: compare coexistence and replacement paths and verify rollback or
  coexistence burden remains reviewable.
- `architecture`: verify the existing ADR plus options packet still exposes the
  winning option, rejected alternatives, and C4 context while staying aligned
  to the broader decision-support vocabulary.