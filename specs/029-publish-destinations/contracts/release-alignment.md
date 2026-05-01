# Contract: Release Alignment

## Scope

This contract defines the repository-facing release surfaces that must stay
aligned when feature 029 ships as `0.29.0`.

## Required Release Surfaces

| Surface | Required Outcome | Validation Expectation |
|---------|------------------|------------------------|
| `Cargo.toml` | workspace version reports `0.29.0` | manifest review plus build and test commands read the same version |
| `Cargo.lock` | workspace package versions align to `0.29.0` | lockfile review shows no stale `0.28.0` workspace package entries |
| shared runtime compatibility references | expected workspace version reports `0.29.0` | mirrored references remain synchronized |
| `README.md`, `ROADMAP.md`, and `docs/guides/modes.md` | publish behavior is described accurately for the shipped slice | focused release-surface doc review passes |
| `CHANGELOG.md` | `0.29.0` entry records the structured publish destination feature | changelog review matches implemented behavior |

## Release Validation Rules

- Versioned release surfaces must move together.
- Release-facing docs must describe only behavior implemented in the 029 slice.
- `tests/release_029_publish.rs` and `tests/skills_bootstrap.rs` must fail if
  the shipped version or mirrored runtime compatibility references drift.
- Validation closeout must include coverage for every modified or new Rust
  file, `cargo clippy`, and `cargo fmt`.