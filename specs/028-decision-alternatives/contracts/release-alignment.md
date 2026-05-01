# Contract: Release Alignment

## Scope

This contract defines the repository-facing release surfaces that must stay
aligned when the 028 decision-support slice ships as `0.28.0`.

## Required Release Surfaces

| Surface | Required Outcome | Validation Expectation |
|---------|------------------|------------------------|
| `Cargo.toml` | workspace version reports `0.28.0` | manifest review plus build/test commands read the same version |
| `Cargo.lock` | lockfile aligns with the bumped workspace version and dependency graph | lockfile diff is present when needed and no stale version references remain |
| `.agents/skills/canon-shared/references/runtime-compatibility.toml` | expected workspace version reports `0.28.0` | targeted runtime-compatibility review and skills bootstrap validation remain green |
| `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml` | expected workspace version reports `0.28.0` | embedded skill validation matches the mirrored runtime compatibility contract |
| `CHANGELOG.md` | `0.28.0` entry accurately describes the shipped slice | release-surface doc review reflects the final behavior |
| `README.md`, `ROADMAP.md`, and `docs/guides/modes.md` | impacted guidance describes the targeted decision-support expansion without overstating runtime behavior | focused doc review and release-surface regression checks pass |
| impacted templates and examples | canonical headings for decision analysis and framework evaluation are shown consistently | focused docs/examples tests pass |

## Release Validation Rules

- Versioned release surfaces must move together; partial `0.28.0` alignment is
  not acceptable.
- Release-facing docs must describe only the behavior actually implemented in
  this slice.
- Validation closeout must include `cargo fmt --check`, `cargo clippy`,
  focused tests for touched runtime surfaces, and coverage evidence for all
  modified or newly created Rust files.