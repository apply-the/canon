# Validation Report: Assistant Plugin Packages

## Planned Evidence

| Validation Area | Command Or Review | Evidence Status |
|-----------------|-------------------|-----------------|
| Speckit spec quality | `specs/044-assistant-plugin-packages/checklists/requirements.md` | Complete before planning |
| Cross-artifact coherence | Manual review of spec, plan, research, data model, contract, quickstart, tasks | Pending |
| Plugin package validation | `bash scripts/validate-assistant-plugins.sh` | Pending implementation |
| Focused Rust validation | `cargo test --test assistant_plugin_packages` | Pending implementation |
| Formatting | `cargo fmt --check` | Pending closeout |
| Lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Pending closeout |
| Full tests | `cargo test` | Pending closeout |
| Coverage | touched-file `cargo llvm-cov` evidence for new or modified Rust files | Pending closeout |

## Coherence Review Findings

- 2026-05-10 review before task generation: spec, plan, research, data model, contract, and quickstart are aligned on bounded-impact packaging work, `0.44.0` version target, root host package folders, shared `assistant/` metadata, Copilot command/prompt pack boundary, and validation ownership.
- Placeholder scan found no unresolved `NEEDS CLARIFICATION`, `ACTION REQUIRED`, or `TODO` markers in generated design artifacts. Checklist bracket syntax is intentional markdown.
- Scope review confirmed the design does not change Canon runtime behavior, `.canon/` persistence, approval semantics, evidence/provenance semantics, or governance adapter authority.
- Acceptance criteria map to planned artifacts: package folders and install docs cover user discovery, manifests and shared command definitions cover host-native capability discovery, and the Rust validation test plus shell wrapper cover drift prevention.

## Implementation Evidence

- 2026-05-10 RED validation: `cargo test --test assistant_plugin_packages` exited 101 after compiling. Expected failures: missing `.claude-plugin`, missing `assistant/plugin-metadata.json`, and dependent metadata/version checks. Negative validation helper test passed, proving invalid JSON and drift checks are active before package files exist.
- 2026-05-10 US1 validation: `cargo test --test assistant_plugin_packages package_folders_and_docs_are_present` passed with 1 test passing and 3 filtered out.
- 2026-05-10 US2 validation: `cargo test --test assistant_plugin_packages manifests_expose_required_governed_methods` passed with 1 test passing and 3 filtered out.
- 2026-05-10 focused validation: `cargo test --test assistant_plugin_packages` passed with 4 tests passing.
- 2026-05-10 validation wrapper: `bash scripts/validate-assistant-plugins.sh` passed with 4 tests passing and printed `PASS: Canon assistant plugin packages are valid.`
- 2026-05-10 manual readback: `.claude-plugin/manifest.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/manifest.json`, `assistant/plugin-metadata.json`, `assistant/commands/governed-methods.json`, `docs/guides/assistant-plugin-packages.md`, and `README.md` align with the contract. The package folders reference shared Canon skills, methods, commands, prompts, and assets; Copilot is documented as a command/prompt pack; Canon CLI and the governance adapter remain authoritative.
- 2026-05-10 positioning scan: `rg -n "0\\.43\\.0|agent framework|orchestrator|coding agent|workspace mutation engine" .claude-plugin .codex-plugin .cursor-plugin assistant docs/guides/assistant-plugin-packages.md README.md specs/044-assistant-plugin-packages` found no invalid host package positioning. Hits are intentional historical version-bump text, existing README negative-boundary language, and prohibited-term lists in spec/contract/shared metadata.

## Closeout Evidence

- 2026-05-10 root cause follow-up: the first full `cargo test` run failed in `skills_bootstrap` because runtime compatibility references still expected `0.43.0`. Updated `defaults/embedded-skills/canon-shared/references/runtime-compatibility.toml`, `.agents/skills/canon-shared/references/runtime-compatibility.toml`, and the matching assertion in `tests/integration/skills_bootstrap.rs` to `0.44.0`. Focused rerun `cargo test --test skills_bootstrap` passed with 15 tests passing.
- 2026-05-10 formatting: `cargo fmt --check` passed with exit 0.
- 2026-05-10 coverage: `cargo llvm-cov --test assistant_plugin_packages --summary-only --fail-under-lines 95` passed. `src/assistant_plugin_validation.rs` reported 96/96 lines covered, 20/20 functions covered, and 149/149 regions covered, for 100.00% line coverage.
- 2026-05-10 lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed with exit 0.
- 2026-05-10 full tests: `cargo test` passed with exit 0 across the workspace.
- 2026-05-10 final package wrapper rerun: `bash scripts/validate-assistant-plugins.sh` passed with 5 tests passing and printed `PASS: Canon assistant plugin packages are valid.`
- 2026-05-10 invariant confirmation: Canon CLI and governance adapter authority were not changed; host folders contain package metadata/glue and reference shared Canon-owned skills, methods, commands, prompts, and assets; plugin package validation fails on version drift, missing fields, missing paths, missing required method surfaces, invalid JSON, and prohibited positioning.
