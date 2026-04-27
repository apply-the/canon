# Validation Report: Mode Authoring Specialization Completion

## Planning Status

- This artifact records the planned validation layers for feature `020-authoring-specialization-completion` during the planning phase.
- Implementation-phase results will be appended as tasks complete.

## Checklist Status

| Checklist | Total | Completed | Incomplete | Status |
|-----------|-------|-----------|------------|--------|
| `requirements.md` | 20 | 20 | 0 | PASS |

## Governance And Setup Confirmation

- Confirmed `spec.md`, `plan.md`, `contracts/mode-authored-body-contracts.md`, `decision-log.md`, and this validation report carry the declared mode, risk, scope, invariants, and validation split for feature 020.
- Confirmed the repository is on branch `020-authoring-specialization-completion` with a clean starting worktree before implementation.
- Confirmed the current authored-guidance surfaces for `review`, `verification`, `incident`, and `migration` are the expected docs-sync targets for this slice.
- Confirmed the current runtime touchpoints in `markdown.rs` and the relevant orchestrator services are the expected code anchors for implementation.

## Planned Baseline Validation

- Capture a focused pre-implementation baseline for the four targeted modes before authored-body changes land.
- Planned baseline suite:
  - `cargo test --test review_run --test verification_run --test incident_run --test migration_run`
  - `cargo test --test review_contract --test verification_contract --test incident_contract --test migration_contract`

## Planned Structural Validation

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `/bin/bash scripts/validate-canon-skills.sh`

## Planned Logical Validation

- Focused contract tests for `review`, `verification`, `incident`, and `migration` artifact contracts.
- Focused renderer tests proving verbatim authored-body preservation for each targeted mode.
- Focused negative renderer or run tests proving `## Missing Authored Body` for absent or blank required sections.
- Focused near-match heading tests proving canonical-heading enforcement.
- Focused run tests proving review/verification disposition semantics and incident/migration recommendation-only posture remain unchanged.
- Docs-sync tests covering embedded skills, mirrored `.agents` skills, templates, examples, mode-guide wording, roadmap state, and release/version references.

## Planned Independent Validation

- Review `spec.md`, `plan.md`, and `tasks.md` before implementation begins.
- Walk one realistic positive brief through each targeted mode and compare the authored source with the emitted packet artifacts.
- Walk one negative brief per targeted mode with a required heading removed and verify the emitted missing-body marker names the canonical heading.
- Perform an independent artifact review confirming the roadmap and release/docs surfaces describe the rollout completion and `0.20.0` consistently.

## Evidence To Capture

- Structural validation command output
- Focused targeted test command output
- Skill validation output
- Notes from positive and negative authored-brief walkthroughs
- Any deviations, compatibility findings, or follow-on work captured in `decision-log.md`

## Validation Ownership Split

- Generation changes: artifact contracts, renderer logic, orchestrator handoff, skill sources, mirrored skills, templates, examples, guide text, roadmap/changelog/version references, and focused tests.
- Validation owners: structural commands, focused logical test runs, skill validator, and independent artifact review recorded separately from generation edits.

## Baseline Risks To Watch

- `incident` and `migration` currently render from marker extraction over an evidence-mixed summary, so authored H2 preservation will not work reliably until the renderer and handoff are aligned.
- `review` and `verification` currently synthesize several gate-critical sections from summary text, so specialization must preserve posture and verdict/disposition semantics while making authored bodies explicit.
- Docs drift across embedded skills, mirrored skills, templates, examples, guide text, and release/version references is a direct feature failure for this slice.