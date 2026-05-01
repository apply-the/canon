# Quickstart: Scoop Distribution Follow-On

## Implementation Flow

1. Update the planning artifacts in `specs/032-scoop-distribution/` so the
   Scoop manifest contract, version bump, and validation closeout are explicit.
2. Extend `scripts/release/write-distribution-metadata.sh` and
   `scripts/release/verify-release-surface.sh` so the canonical Windows asset
   carries both `winget` and `scoop` channel expectations.
3. Add `packaging/scoop/canon.json.tpl` and implement
   `scripts/release/render-scoop-manifest.sh` to derive the manifest from
   distribution metadata.
4. Wire the Scoop artifact into `.github/workflows/release.yml` and
   `.github/release-notes-template.md` so releases publish and describe the new
   manifest artifact.
5. Add or update user-facing and maintainer-facing docs in `README.md`,
   `docs/guides/publishing-to-scoop.md`, `CHANGELOG.md`, and `ROADMAP.md`.
6. Bump the workspace version to `0.32.0` in `Cargo.toml`, update shared
   runtime-compatibility references, and align any version-sensitive tests.
7. Add `tests/release_032_scoop_distribution.rs` and use it as the focused
   executable regression check for the release, docs, and manifest surfaces.

## Focused Validation Commands

Run the slice-specific checks first:

```bash
cargo test --test release_032_scoop_distribution
cargo test --test release_032_scoop_distribution --test skills_bootstrap
/bin/bash scripts/validate-canon-skills.sh
```

Then run the packaging-surface shell checks against a synthetic release bundle:

```bash
bash scripts/release/write-distribution-metadata.sh --version 0.32.0 --dist-dir dist --output dist/canon-0.32.0-distribution-metadata.json
bash scripts/release/render-scoop-manifest.sh --metadata dist/canon-0.32.0-distribution-metadata.json --output dist/canon-0.32.0-scoop-manifest.json
bash scripts/release/verify-release-surface.sh --version 0.32.0 --dist-dir dist --release-notes dist/release-notes.md --distribution-metadata dist/canon-0.32.0-distribution-metadata.json --scoop-manifest dist/canon-0.32.0-scoop-manifest.json --winget-manifest-dir dist/winget
```

Finish with repository-wide quality gates:

```bash
cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run
```

## Evidence Closeout

- Record story-specific design choices in `decision-log.md` before closing a
  story.
- Record structural, logical, independent-review, and coverage outcomes in
  `validation-report.md`.
- Keep touched-Rust-file coverage notes aligned with `lcov.info` or, when only
  test files change, with the direct test commands that executed those files.