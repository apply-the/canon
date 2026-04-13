# Contributing to Canon

This document is for people developing Canon itself.

The top-level [`README.md`](README.md) is intentionally written for end users.
If you are here to compile, test, debug, or extend the repository, use this
guide instead.

## Contributor Rules

- Keep Canon CLI as the product entrypoint. Do not introduce hidden side paths that bypass the CLI contract.
- Keep changes bounded. Avoid unrelated refactors while touching a feature.
- Update user-facing docs when you change install, release, CLI, or skill behavior.
- If you touch governed workflows, keep validation evidence separate from generation behavior.
- If you change skill support-state behavior, do not fabricate runs, approvals, evidence, or CLI output.
- Run the local verification suite before asking for review.

## Prerequisites

- Rust `1.94.1` via `rustup`
- Git
-`cargo-deny`
- optional but recommended: `cargo-nextest`
- PowerShell (`pwsh`) for the PowerShell skill validator on macOS or Linux
-If you are on Windows, you can install Cygwin to test Shell (`sh`) scripts.

Recommended tool installation:

```bash
rustup toolchain install 1.94.1 --profile minimal --component rustfmt --component clippy
cargo install cargo-nextest cargo-deny
```

## First-Time Setup

```bash
git clone https://github.com/apply-the/canon.git
cd canon
cargo build
./scripts/install-hooks.sh
```

If you want a locally installed contributor binary:

```bash
cargo +1.94.1 install --path crates/canon-cli --bin canon
canon --help
```

## Repository Layout

Key locations:

- `crates/canon-cli/`: CLI surface and command handling
- `crates/canon-engine/`: governed execution model and runtime logic
- `crates/canon-adapters/`: adapter-facing types and integration surfaces
- `tests/`: contract, integration, fixture, and snapshot coverage
- `defaults/`: default methods and policy configuration shipped by Canon
- `.agents/skills/`: repo-local skills for Codex and compatible Copilot environments, plus shared runtime helpers
- `specs/`: product specs, plans, tasks, decisions, and validation artifacts

## Build Commands

Normal local build:

```bash
cargo build
```

Release CLI build:

```bash
cargo build --release -p canon-cli --bin canon
```

Install the locally built CLI into your Cargo bin directory:

```bash
cargo +1.94.1 install --path crates/canon-cli --bin canon
```

## Test and Validation Commands

Run these before opening or updating a pull request:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --locked
cargo nextest run --locked
cargo deny check licenses advisories bans sources
git diff --check
```
_Important!_ We accept zero warnings/errors policy with Clippy.

Validate Canon skill structure and shared runtime behavior:

```bash
/bin/bash scripts/validate-canon-skills.sh
pwsh -File scripts/validate-canon-skills.ps1
```

If `pwsh` is not available on your machine, document that gap in your review or
validation notes.

## Development Workflow

Use this repo workflow unless a feature calls for something stricter:

1. Create or switch to the feature branch.
2. Make the smallest coherent change that solves the problem.
3. Update the relevant docs, specs, or validation artifacts when behavior changes.
4. Run the validation commands above.
5. Review your diff for unrelated edits before asking for review.

## Working on Release or Install UX

If your change touches packaging, install guidance, or release automation,
update the full release surface together:

- `README.md`
- `.github/release-notes-template.md`
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `scripts/release/`
- `.agents/skills/canon-shared/references/runtime-compatibility.toml`
- the relevant artifacts under `specs/005-cli-release-ux/`

## Working on Skills

If your change touches `.agents/skills/`:

- keep wording accurate for both Codex and compatible Copilot skill-loading environments
- keep runnable skills honest about current support state
- keep shared Bash and PowerShell helper behavior aligned
- validate both skill structure scripts after the change
- update shared references before duplicating guidance across multiple skills

## Pull Request Checklist

- the change is scoped to a real problem and avoids unrelated cleanup
- user-facing docs are updated if behavior changed
- contributor docs are updated if the development workflow changed
- local validation commands passed, or any gap is explicitly called out
- release-surface changes keep install, packaging, and compatibility guidance aligned