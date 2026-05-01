# Contract: Release Alignment

## Scope

This contract defines the repository-facing release surfaces that must stay
aligned when feature 033 ships as `0.33.0`.

## Required Release Surfaces

| Surface | Required Outcome | Validation Expectation |
|---------|------------------|------------------------|
| `Cargo.toml` | workspace version reports `0.33.0` | manifest review plus build and test commands see the same version |
| `Cargo.lock` | lockfile aligns with the bumped workspace version and dependency graph | no stale version references remain after the bump |
| `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml` | expected workspace version reports `0.33.0` | embedded skill validation matches the mirrored runtime compatibility contract |
| `.agents/skills/canon-shared/references/runtime-compatibility.toml` | expected workspace version reports `0.33.0` | mirrored skill validation matches the embedded runtime compatibility contract |
| `README.md`, `ROADMAP.md`, and `docs/guides/modes.md` | impacted guidance describes the delivered reasoning-evidence contract without overstating runtime behavior | focused docs review and release-surface checks pass |
| impacted templates and examples | canonical reasoning-evidence and honest-gap guidance is shown consistently whenever a template or example actually changes | targeted review confirms either that the existing H2 contracts already satisfy the release or that any required edits landed |
| `CHANGELOG.md` | the `0.33.0` entry accurately describes the shipped feature and its validation closeout | release-surface review reflects the final behavior |

## Release Validation Rules

- Versioned release surfaces must move together; partial `0.33.0` alignment is
  not acceptable.
- Release-facing docs must describe only the behavior actually implemented in
  feature 033.
- Do not reintroduce brittle repository-doc prose tests; validate release
  alignment through runtime-backed checks, shared skill validation, and focused
  surface review.
- Final closeout must include explicit coverage evidence for all modified or
  newly created Rust files plus clean `cargo fmt --check` and `cargo clippy`
  results.