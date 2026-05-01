# Quickstart: Industry-Standard Artifact Shapes Follow-On

## Implementation Flow

1. Update the follow-on planning artifacts in `specs/030-artifact-shapes-follow-on/`.
2. Update the three source-of-truth skill files under `defaults/embedded-skills/`:
   - `canon-discovery/skill-source.md`
   - `canon-system-shaping/skill-source.md`
   - `canon-review/skill-source.md`
3. Mirror the same guidance changes into `.agents/skills/` for the shipped
   Codex/Copilot-facing surfaces.
4. Update shared renderer or preservation logic in
   `crates/canon-engine/src/artifacts/markdown.rs` only where the follow-on
   contract actually needs code-level changes.
5. Update release-facing surfaces for `0.30.0` in `Cargo.toml`, `Cargo.lock`,
   shared runtime-compatibility references, and impacted docs plus changelog.

## Focused Validation Commands

Run the slice-specific checks first:

```bash
cargo test --test discovery_authoring_docs --test discovery_authoring_renderer --test discovery_authoring_run
cargo test --test system_shaping_domain_modeling_docs --test system_shaping_authoring_renderer --test system_shaping_run
cargo test --test review_authoring_docs --test review_authoring_renderer --test review_run --test skills_bootstrap
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