# Quickstart: Cross-Mode Reasoning Evidence And Clarity Expansion

## Goal

Validate that feature 033 expands clarity inspection and reasoning-evidence
posture across the governed mode surface, tightens placeholder-heavy packet
fallbacks into honest gap behavior, and ships `0.33.0` with synchronized
skills, templates, docs, coverage, lint, and formatting closeout.

## Recommended Validation Flow

1. Confirm the updated skills, templates, examples, and docs describe the same
   reasoning-evidence and honest-gap contract as the runtime.
2. Run focused clarity-inspection tests for newly supported file-backed modes
   and confirm `reasoning_signals` are present and meaningful.
3. Run focused renderer and packet-summary tests for affected modes,
   especially fallback-heavy planning surfaces, to prove that missing authored
   reasoning now reads as an honest gap rather than generic filler.
4. Run review-family regressions for `review`, `verification`, and `pr-review`
   to confirm contradictions, missing evidence, unresolved findings, and
   explicit no-finding posture remain honest.
5. Run `/bin/bash scripts/validate-canon-skills.sh` to confirm embedded and
   mirrored skills remain synchronized.
6. Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, a focused coverage command for touched Rust files, and the final workspace regression command used for closeout.
7. Review `Cargo.toml`, `Cargo.lock`, runtime-compatibility references,
   `README.md`, `ROADMAP.md`, `docs/guides/modes.md`, impacted templates or
   examples, and `CHANGELOG.md` to confirm the `0.33.0` release surface is
   explicit and internally consistent.

## Representative Walkthroughs

- Run one newly supported clarity target with a weak authored brief and verify
  the response surfaces missing context or shallow reasoning signals rather
  than implying readiness.
- Run one targeted packet surface that previously emitted placeholder-heavy
  fallback prose and verify the output now presents explicit gap or closure
  language.
- Run one verification or review-family example with no real contradiction and
  verify the packet says so explicitly instead of inventing a finding.