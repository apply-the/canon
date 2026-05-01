# Contract: Release Alignment

## Scope

This contract defines the repository-facing release surfaces that must stay
aligned when feature 030 ships as `0.30.0`.

## Required Release Surfaces

| Surface | Required Outcome | Validation Expectation |
|---------|------------------|------------------------|
| `Cargo.toml` | workspace version reports `0.30.0` | manifest review plus build and test commands read the same version |
| `Cargo.lock` | workspace package versions align to `0.30.0` | lockfile review shows no stale `0.29.0` workspace package entries |
| shared runtime compatibility references | expected workspace version reports `0.30.0` | embedded and mirrored references remain synchronized |
| `README.md`, `ROADMAP.md`, and `docs/guides/modes.md` | follow-on artifact-shape behavior is described accurately for the shipped slice | focused release-surface doc review passes |
| `CHANGELOG.md` | `0.30.0` entry records the follow-on artifact-shapes feature | changelog review matches implemented behavior |

## Release Validation Rules

- Versioned release surfaces must move together.
- Release-facing docs must describe only behavior implemented in the 030 slice.
- Release closeout must include coverage for every modified or new Rust file,
  `cargo clippy`, and `cargo fmt`.
- `tests/skills_bootstrap.rs` plus the current follow-on doc regression
  coverage must fail if the shipped version, mirrored runtime compatibility
  references, or delivered-slice wording drift.