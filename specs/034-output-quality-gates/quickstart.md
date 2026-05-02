# Quickstart: Output Quality Gates

## Goal

Validate that feature 034 teaches Canon to distinguish structurally complete
packets from materially useful and publishable packets, keeps downgrade reasons
explicit when quality is weak, and ships `0.34.0` with synchronized skills,
docs, roadmap cleanup, coverage, lint, and formatting closeout.

## Recommended Validation Flow

1. Confirm that inspect-facing outputs expose an explicit output-quality
   posture plus evidence or downgrade reasons on representative authored inputs.
2. Run focused engine and contract tests for the shared quality assessment,
   inspect surfaces, and summary posture changes.
3. Run focused renderer tests for targeted fallback-heavy packet families to
   confirm weak or missing support stays explicit instead of synthetic.
4. Run skill-sync and version-anchor checks so runtime-compatibility references,
   shared output-shape guidance, and mirrored skill docs stay aligned.
5. Run `/bin/bash scripts/validate-canon-skills.sh`.
6. Run `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and the focused coverage command for touched Rust files.
7. Review `Cargo.toml`, `Cargo.lock`, runtime-compatibility references,
   `README.md`, `ROADMAP.md`, `docs/guides/modes.md`, impacted publication
   guides, and `CHANGELOG.md` to confirm a coherent `0.34.0` release story.

## Representative Walkthroughs

- Use a weak but structurally complete authored brief and confirm the inspect
  response labels it `structurally-complete` with downgrade reasons.
- Use a strong authored brief with explicit support and closure evidence and
  confirm the inspect or summary response can elevate it to
  `materially-useful` or `publishable`.
- Render one targeted fallback-heavy packet surface with missing authored
  content and verify the artifact preserves explicit missing-body or downgrade
  language instead of filler.
- Review one runtime summary from a targeted mode family and verify the new
  posture language is visible without reading the whole artifact.