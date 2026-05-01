# Contract: Release Alignment

## Scope

This contract defines the repository-facing release surfaces that must stay
aligned when feature 031 ships as `0.31.0`.

## Required Release Surfaces

| Surface | Required Outcome | Validation Expectation |
|---------|------------------|------------------------|
| `Cargo.toml` | workspace version reports `0.31.0` | manifest review plus build and test commands read the same version |
| `Cargo.lock` | workspace package versions align to `0.31.0` | lockfile review shows no stale `0.30.0` workspace package entries |
| shared runtime compatibility references | expected workspace version reports `0.31.0` | embedded and mirrored references remain synchronized |
| `README.md`, `ROADMAP.md`, and `docs/guides/modes.md` | remaining artifact-shape behavior is described accurately for the shipped slice | focused release-surface doc review passes |
| `CHANGELOG.md` | `0.31.0` entry records the remaining artifact-shapes feature | changelog review matches implemented behavior |

## Release Validation Rules

- Versioned release surfaces must move together.
- Release-facing docs must describe only behavior implemented in the 031 slice.
- Release closeout must include coverage for every modified or new Rust file,
  `cargo clippy`, and `cargo fmt`.
- `tests/skills_bootstrap.rs` plus the targeted mode doc and run regression
  coverage must fail if the shipped version, mirrored runtime compatibility
  references, or delivered-slice wording drift.