# Implementation Plan: Authoring Experience And Packet Readiness

**Branch**: `039-authoring-packet-readiness` | **Date**: 2026-05-02 | **Spec**: `specs/039-authoring-packet-readiness/spec.md`
**Input**: Feature specification from `/specs/039-authoring-packet-readiness/spec.md`

## Summary

Deliver `0.39.0` as a bounded authoring-experience slice by extending the
existing `inspect clarity` contract with an additive authoring-lifecycle
summary that explains packet shape, authoritative brief inputs, supporting or
carried-forward context, and remaining readiness delta for file-backed Canon
packets. Reuse the current canonical input binding, missing-context honesty,
and output-quality derivation, then align the shared file-backed lifecycle
guidance across the mode guide, template-facing docs, carry-forward example,
inspect-clarity skill, roadmap, changelog, and release checks.

## Governance Context

**Execution Mode**: change  
**Risk Classification**: bounded-impact; this work changes clarity-inspection
summary data, CLI rendering, shared docs, skill guidance, and release surfaces
for existing file-backed modes, but it does not introduce a new mode, new
run-state family, new persistence layout, or hidden authored-input source.  
**Scope In**: `inspect clarity` runtime summary assembly; packet-shape and
authoritative-input classification for single-file and directory-backed packets;
readiness-delta guidance; CLI clarity markdown rendering; shared file-backed
authoring docs and examples; inspect-clarity skill source and mirror; focused
contract, renderer, and docs tests; explicit `0.39.0` release alignment across
workspace manifests, runtime compatibility refs, roadmap, README, and
changelog.  
**Scope Out**: new Canon modes; new `.canon/` storage families; automatic
rewriting of `canon-input/`; hidden dereferencing of `.canon/`, published docs,
or open tabs as authored input; approval or publish semantics changes;
`pr-review` authored-input behavior changes; broad rewrite of per-mode skill
content beyond the shared lifecycle surfaces this feature needs.

**Invariants**:

- `## Missing Authored Body` remains stronger than generated filler and must
  continue to signal absent authored sections honestly.
- The current-mode brief stays authoritative for readiness; carried-forward
  inputs remain explicit provenance or narrowing support, not silent
  replacements.
- Canon must not rewrite `canon-input/` or infer authored input from `.canon/`,
  published packets, or active editor state.
- Existing canonical file bindings, directory preference, review strictness,
  and `pr-review` exclusion remain intact.

**Decision Log**: `specs/039-authoring-packet-readiness/decision-log.md`  
**Validation Ownership**: Generation updates clarity runtime data, renderer,
shared docs, skill guidance, and release surfaces; validation is performed via
focused clarity contract tests, renderer tests, docs or skill sync tests,
release-alignment checks, coverage review for touched Rust files,
`cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`,
and an independent review of authoring-lifecycle honesty.  
**Approval Gates**: No new human approval gate is introduced for this bounded-
impact slice; independent validation evidence remains mandatory.

## Technical Context

**Language/Version**: Rust 1.95.0 workspace plus Markdown documentation and embedded skill artifacts  
**Primary Dependencies**: workspace crates `canon-engine`, `canon-cli`, and `canon-adapters`; existing `serde` or `serde_json` output contracts; repo-local Speckit and embedded skill mirrors  
**Storage**: repository files plus the existing `.canon/` runtime layout only; no new persistence family  
**Testing**: focused `cargo test` targets for `inspect_clarity`, engine service helpers, CLI clarity rendering, authoring-doc sync, `cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info`, `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo nextest run`  
**Target Platform**: local CLI-driven Canon workflows and compatible Copilot or Codex skill hosts  
**Project Type**: Rust CLI workspace with shared Markdown skills, templates, examples, and release guardrails  
**Existing System Touchpoints**: `crates/canon-engine/src/orchestrator/service.rs`, `crates/canon-engine/src/orchestrator/service/inspect.rs`, `crates/canon-engine/src/orchestrator/service/clarity.rs`, `crates/canon-cli/src/output.rs`, `tests/contract/inspect_clarity.rs`, `crates/canon-engine/src/orchestrator/service/tests.rs`, `docs/guides/modes.md`, `docs/examples/canon-input/carry-forward-packets.md`, `docs/templates/canon-input/`, `defaults/embedded-skills/canon-inspect-clarity/skill-source.md`, `.agents/skills/canon-inspect-clarity/SKILL.md`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `tests/release_036_release_provenance_integrity.rs`, and `tests/integration/skills_bootstrap.rs`  
**Performance Goals**: deterministic packet-role classification, no hidden authored-input inference, and clarity output that exposes the next authoring step without extra inspection passes  
**Constraints**: keep the change additive to the current clarity contract; preserve `brief.md` authority when present; keep docs and skills honest about non-ready packets; bump and validate the workspace at `0.39.0`; cover touched Rust files before full-suite closeout  
**Scale/Scope**: one additive clarity-summary extension, one renderer update, one shared authoring-lifecycle doc pass, one inspect-clarity skill sync, and one release-alignment pass

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Declared-risk approval checkpoints are named where required by the risk classification
- [x] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/039-authoring-packet-readiness/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── authoring-lifecycle.md
│   └── clarity-packet-shape.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-cli/
│   └── src/
│       └── output.rs
└── canon-engine/
    └── src/
        └── orchestrator/
            └── service/
                ├── clarity.rs
                ├── inspect.rs
                ├── service.rs
                └── tests.rs

tests/
├── contract/
│   └── inspect_clarity.rs
├── integration/
│   └── skills_bootstrap.rs
└── release_036_release_provenance_integrity.rs

docs/
├── examples/canon-input/carry-forward-packets.md
├── guides/modes.md
└── templates/canon-input/

defaults/embedded-skills/canon-inspect-clarity/skill-source.md
.agents/skills/canon-inspect-clarity/SKILL.md

README.md
ROADMAP.md
CHANGELOG.md
Cargo.toml
Cargo.lock
```

**Structure Decision**: Keep feature `039` localized to the existing clarity
inspection path and the shared authoring guidance surfaces rather than adding a
new mode, new runtime subsystem, or per-mode bespoke lifecycle logic. The
engine remains the single place where file-backed packet facts become
readiness guidance, while docs and skills stay thin mirrors of that contract.

## Complexity Tracking

No constitution deviations are currently identified.