# Validation Report Plan: Backlog Mode

## Summary

Validation for backlog mode must prove that Canon can turn bounded upstream decisions into governed delivery decomposition without inventing task-level detail and without pretending vague architecture is decomposable. Validation remains layered and explicitly separate from generation.

## Validation Ownership

- **Generation owners**: runtime changes in `canon-cli`, `canon-engine`, defaults, docs, and skills
- **Structural validators**: formatter, linter, config validation, and skill validation scripts
- **Logical validators**: contract tests, integration tests, inspect/status/publish checks, and authored-input binding checks
- **Independent validators**: reviewer-mode or equivalent separate-reader pass against emitted backlog packets and closure-blocked outputs

## Structural Validation

Run these after implementation changes land:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test
cargo nextest run
bash scripts/validate-canon-skills.sh
```

Structural validation must also confirm:

- new backlog mode defaults and artifact lists are loadable through the embedded method store
- backlog-related embedded skills and materialized `.agents/skills/` copies stay in sync
- no unresolved placeholders remain in `spec.md`, `plan.md`, `research.md`, `data-model.md`, contracts, `quickstart.md`, `decision-log.md`, or this report

## Logical Validation

### Contract Tests

Add dedicated coverage for:

- backlog artifact contract completeness and missing-artifact failures
- canonical authored-input binding for `canon-input/backlog.md` and `canon-input/backlog/`
- folder-backed packet precedence with `brief.md` authoritative
- persisted run context contents for backlog planning context and closure findings

### Integration Tests

Add dedicated coverage for:

- successful bounded backlog run that emits the full eight-artifact packet
- closure-blocked backlog run that exposes explicit findings instead of a full packet
- publish compatibility for backlog through the existing publish flow
- status and inspect compatibility for display id, UUID, short id, slug, and `@last`
- traceability preservation from source references into emitted epics and slices
- granularity discipline that rejects task-level decomposition requests

### Non-Regression Checks

- existing modes continue to auto-bind their canonical `canon-input/<mode>.md|/` inputs
- current publish routing for already delivered modes remains unchanged
- existing inspect and status surfaces remain backward-compatible
- no existing skill validation or mode summary contracts regress when backlog is added

## Independent Validation

An independent reviewer pass must verify:

- backlog packets are credible as standalone planning documents
- closure-limited packets make the lack of decomposition credibility obvious
- published backlog packets remain above task level and do not drift into tool-specific ticket output
- source traceability is visible enough that a reader can understand why an epic or slice exists

## Evidence Artifacts

Record evidence in the delivering Canon run under `.canon/runs/<RUN_ID>/evidence/` and cross-link results from `specs/012-backlog-mode/decision-log.md` and any later `tasks.md` entries.

## Implementation Checkpoints

- **Governance checkpoint**: Before runtime changes begin, confirm `spec.md`, `plan.md`, `research.md`, contracts, and `decision-log.md` still describe the same scope and invariants.
- **Closure checkpoint**: Before enabling a full backlog packet, confirm closure-failure behavior is explicit and durable.
- **Packet checkpoint**: Before marking backlog mode complete, confirm the emitted packet is distinguishable, traceable, and publishable without new runtime surfaces.
- **Final review checkpoint**: Before feature completion, perform independent review of both successful packets and closure-limited packets.

## Implementation Kickoff Status

- Governance artifacts were re-confirmed on 2026-04-23 against `spec.md`, `plan.md`, contracts, and `quickstart.md` before runtime edits began.
- The requirements checklist passed with 20 completed items and 0 incomplete items.
- Existing project ignore coverage was verified through `.gitignore`; no additional `.dockerignore`, `.eslintignore`, `.prettierignore`, `.npmignore`, `.terraformignore`, or `.helmignore` files were required for the current Rust-only workspace.
- Evidence ownership remains split between runtime generation changes and separate structural, logical, and independent validation steps recorded in this report.

## Current Evidence

- `cargo test --workspace --no-run` passed after wiring backlog into the mode taxonomy, run context, publish routing, contracts, renderers, and orchestrator flow.
- `cargo test --test inspect_modes --test mode_profiles --test runtime_filesystem --test cli_contract` passed after updating taxonomy expectations, materialized defaults, and canonical backlog input auto-binding coverage.
- `cargo test --test backlog_contract --test backlog_run` passed with full-packet assertions for the eight backlog artifacts, four persisted invocation capabilities, and markdown output that does not expose task-mapping sections.
- `cargo test --test direct_runtime_coverage --test invocation_cli_contract` passed with service-level backlog coverage for persisted `backlog_planning` context and CLI-level coverage for completed backlog invocation/evidence inspection.
- `cargo test --test backlog_contract --test backlog_closure_run --test runtime_evidence_contract --test policy_and_traces` passed after implementing closure-aware contract selection, blocked-versus-downgraded severity handling, risk-only packet emission, structured closure fields in `status` and `inspect evidence`, and trace/evidence assertions for downgraded backlog runs.
- `cargo test --test backlog_run --test run_lookup --test render_next_steps --test skills_bootstrap` passed for US3 with explicit assertions that published backlog packets land under `docs/planning/<RUN_ID>/`, stay readable without hidden runtime state, preserve source links/dependencies/sequencing/acceptance-anchor context for downstream handoff, remain resolvable through `@last` and short-id lookup, keep completed next-step rendering backlog-safe, and materialize `canon-backlog` as an `available-now` skill.
- `/bin/bash scripts/validate-canon-skills.sh` passed after promoting `canon-backlog` to `available-now` and syncing the embedded skill source, materialized skill copy, and shared skill index.
- Successful backlog runs now complete through the existing CLI surface with `--mode backlog --system-context existing`, emit `.canon/artifacts/<RUN_ID>/backlog/backlog-overview.md` as the primary artifact, and surface the backlog-specific mode result summary through existing output rendering.
- Closure-limited backlog runs now either block with explicit blocking findings or complete in downgraded mode with the risk-only packet. In both cases the runtime persists only `backlog-overview.md` plus `planning-risks.md`, exposes closure status and findings in `canon status --output json` and `canon inspect evidence --output json`, and keeps the reduced packet visible in the evidence bundle and trace-linked artifact refs.
- README, MODE_GUIDE, NEXT_FEATURES, AGENTS skill listings, the shared skill index, and both backlog skill surfaces now describe backlog as a delivered governed mode instead of a planned one.

## Open Validation Gaps

- Structural closure for `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test`, and `cargo nextest run` remains in the final verification tranche.

## Exit Criteria

- backlog contract and integration tests exist and pass
- backlog mode participates in CLI parsing, inspect, status, publish, and mode summaries without regressions
- backlog docs and skills describe real runtime behavior instead of aspirational behavior
- backlog packets remain above task level and traceable to bounded source inputs
- closure-blocked runs are explicit, durable, and independently reviewed