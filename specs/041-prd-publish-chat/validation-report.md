# Validation Report: Requirements PRD Publishing And Chat Publish Skill

## Structural Validation

- `cargo fmt --check` passed after one `cargo fmt` normalization pass on touched files.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed.
- `/bin/bash scripts/validate-canon-skills.sh` passed.

## Logical Validation

- `cargo test --test requirements_authoring_renderer --test requirements_contract` passed.
- `cargo test -p canon-cli publish` passed.
- `cargo test --test release_036_release_provenance_integrity --test release_040_governance_runtime_framing --test skills_bootstrap` passed.
- `cargo nextest run` passed with `349` tests run and `349` tests passed.

## Independent Validation

- README readback confirms the publish boundary is explicit: artifacts land under `.canon/artifacts/<RUN_ID>/...` first, `requirements` publish now includes `prd.md`, and chat users can invoke the same step through `$canon-publish`.
- `tech-docs/guides/modes.md` readback confirms the requirements artifact list includes `prd.md` and that published requirements directories contain `prd.md`, the sectional packet files, and `packet-metadata.json`.
- The `canon-publish` skill and embedded mirror continue to describe the real `canon publish <RUN_ID> [--to <DESTINATION>]` surface without bypassing completion or approval gates.

## Evidence Log

- Focused feature behavior was validated by renderer, contract, publish-command, skill-bootstrap, and release-surface tests after the code and docs changes landed.
- The first `cargo nextest run` attempt failed with linker `errno=28` because the host had only `118 MiB` free. Removing regenerable `target/debug/incremental` and `target-agent/debug/incremental` artifacts freed space and the exact same `cargo nextest run` command then passed unchanged.
- Release surfaces now align on `0.41.0` across `Cargo.toml`, `Cargo.lock`, runtime compatibility references, `README.md`, `CHANGELOG.md`, `ROADMAP.md`, and the publishing guides.

## Invariants Check

- No publish gate bypass was introduced; chat publish remains a wrapper over the existing CLI command.
- Requirements still emit the sectional packet; `prd.md` is additive rather than a replacement.
- Publish still copies governed artifacts out of `.canon/artifacts/` without mutating the governed originals.

## Closeout

- Proposed commit message: `feat: add chat-first requirements prd publishing`