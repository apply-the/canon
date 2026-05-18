# Release Checklist: Version Bump

This document lists the repository files that must be reviewed or updated when
advancing the Canon crate version and GitHub release tag.

Canon release tags use the raw semantic version `X.Y.Z`. There is no `v`
prefix.

## Files To Update

### Version source of truth

- **`Cargo.toml`** — update `[workspace.package] version = "X.Y.Z"`.

### Cargo-managed lockfile

- **`Cargo.lock`** — refresh the workspace package entries for
  `canon-workspace`, `canon-cli`, `canon-engine`, and `canon-adapters` through
  the normal Cargo workflow after the version bump.

### Repository docs

- **`README.md`** — update the current delivery-line sentence.
- **`CHANGELOG.md`** — add `## [X.Y.Z] - YYYY-MM-DD` as the first release entry
  with a concise summary of the delivered slice.

### Assistant package surface

- **`assistant/plugin-metadata.json`** — update `"version": "X.Y.Z"`.
- **`.claude-plugin/manifest.json`** — update `"version": "X.Y.Z"`.
- **`.codex-plugin/plugin.json`** — update `"version": "X.Y.Z"`.
- **`.cursor-plugin/manifest.json`** — update `"version": "X.Y.Z"`.
- **`.copilot-prompts/pack.json`** — update `"version": "X.Y.Z"`.

### Skill runtime compatibility surface

- **`.agents/skills/canon-shared/references/runtime-compatibility.toml`** —
  update `[canon].expected_workspace_version = "X.Y.Z"` in the repo-local
  skill source copy.
- **`defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`** —
  update the same field in the shipped embedded copy.

Keep the repo-local `.agents` copy and the embedded `defaults/embedded-skills`
copy aligned. The embedded copy is what `canon skills install` materializes,
but the source copy should not drift from it.

## Validation

Run focused validation after every version bump:

```bash
cargo test --test assistant_plugin_packages metadata_paths_and_versions_are_aligned -- --exact
cargo test --test skills_bootstrap skills_install_for_codex_carries_current_runtime_compatibility_reference -- --exact
```

Then run the broader repository suite appropriate for the release-ready change:

```bash
cargo nextest run --workspace --all-features
```

`README.md` and `CHANGELOG.md` are still manual readback surfaces; there is no
dedicated contract test that updates those entries for you.