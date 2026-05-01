# Quickstart: Remaining Industry-Standard Artifact Shapes

## Implementation Flow

1. Update the remaining-rollout planning artifacts in
   `specs/031-remaining-artifact-shapes/`.
2. Update the three source-of-truth skill files under
   `defaults/embedded-skills/`:
   - `canon-implementation/skill-source.md`
   - `canon-refactor/skill-source.md`
   - `canon-verification/skill-source.md`
3. Mirror the same guidance changes into `.agents/skills/` for the shipped
   Codex/Copilot-facing surfaces.
4. Update shared renderer or preservation logic in
   `crates/canon-engine/src/artifacts/markdown.rs` and
   `crates/canon-engine/src/artifacts/contract.rs` only where the remaining
   shape contract actually needs code-level changes.
5. Update input templates, worked examples, and operator-facing docs for the
   three targeted modes.
6. Update release-facing surfaces for `0.31.0` in `Cargo.toml`, `Cargo.lock`,
   shared runtime-compatibility references, and impacted docs plus changelog.

## Focused Validation Commands

Run the slice-specific checks first:

```bash
cargo test --test implementation_authoring_docs --test implementation_authoring_renderer --test implementation_contract --test implementation_run
cargo test --test refactor_authoring_docs --test refactor_authoring_renderer --test refactor_contract --test refactor_run --test refactor_preservation_run
cargo test --test verification_authoring_docs --test verification_authoring_renderer --test verification_contract --test verification_run --test skills_bootstrap
scripts/validate-canon-skills.sh
```

Then run the final repository quality gates:

```bash
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace --all-features
```

## Evidence Closeout

- Record story-specific design choices in `decision-log.md` before closing a
  story.
- Record structural, logical, and independent validation outcomes in
  `validation-report.md`.
- Keep touched-Rust-file coverage notes aligned with `lcov.info`.