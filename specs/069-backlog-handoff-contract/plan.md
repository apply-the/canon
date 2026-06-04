# Implementation Plan: Backlog Handoff Contract

**Branch**: `069-backlog-handoff-contract` | **Date**: 2026-06-04 | **Spec**:
[spec.md](./spec.md)
**Input**: Feature specification from
`/specs/069-backlog-handoff-contract/spec.md`

**Note**: This plan extends Canon backlog mode with a governed downstream
handoff contract while preserving the existing backlog truth: the packet stays
above task-level decomposition and downstream runtimes still own execution
admission.

## Summary

Add an additive backlog execution-handoff contract rather than a second
planning subsystem. The design reuses Canon's existing backlog packet, publish,
inspect, lookup, and immutable input model while adding stable `slice_id`
identity, slice-scoped implementation artifact references, independent
verification anchors, and a dedicated `execution-handoff.md` artifact that is
emitted only when a slice is credible for downstream execution handoff. Full
planning packets remain publishable even when no slice is handoff-ready, but
they must say so explicitly. The implementation touches backlog artifact
generation, packet contracts, inspect/publish summaries, docs, skills, and
focused contract/integration coverage.

## Governance Context

**Execution Mode**: `change`
**Risk Classification**: `systemic-impact`. The work changes Canon's public
backlog packet contract and downstream interoperability posture across runtime,
published artifacts, and user-facing guidance.
**Scope In**: Add stable slice identifiers; add `execution-handoff.md`; define
handoff-available versus handoff-unavailable semantics; preserve planning-only
backlog boundaries; update docs, skills, contracts, and validation coverage.
**Scope Out**: Task-list generation, issue-tracker output, Canon-owned
execution orchestration, downstream runtime changes, staffing heuristics, and
team-capacity scheduling.

**Invariants**:

- Backlog remains a planning-mode artifact set and must not emit fine-grained
  implementation task lists.
- Every execution-readiness claim must be traceable to stable slice identity,
  implementation artifact references, dependency evidence, and independent
  verification anchors visible in the packet.
- Closure-limited or risk-only packets must never emit a misleading execution
  handoff artifact.
- Full planning-packet success and downstream execution handoff availability
  remain distinct truths and must be surfaced distinctly.
- Downstream runtimes continue to own execution admission even when Canon emits
  governed handoff signals.

**Decision Log**: `specs/069-backlog-handoff-contract/decision-log.md`
**Validation Ownership**: Generation work updates backlog runtime and packet
artifacts. Validation is owned separately by packet-contract tests, integration
tests, publish/inspect checks, skill/doc validation, and an independent review
of emitted handoff-capable and handoff-unavailable packets recorded in
`validation-report.md`.
**Approval Gates**: No new human approval gate is introduced by the runtime.
Systemic-impact controls rely on explicit invariants, contract-level
compatibility checks, layered validation, and independent packet review before
completion.

## Technical Context

**Language/Version**: Rust 1.96.0, Edition 2024
**Primary Dependencies**: `clap`, `serde`, `serde_json`, `serde_yaml`, `toml`,
`thiserror`, `tracing`, `uuid`, `time`
**Storage**: Local filesystem under `.canon/`, persisted `context.toml`,
Markdown packet artifacts, repo-local docs, and skill source documents
**Testing**: `cargo test`, `cargo nextest run`, focused contract and
integration tests under `tests/`, `cargo fmt --check`,
`cargo clippy --workspace --all-targets --all-features -- -D warnings`, and
`bash scripts/validate-canon-skills.sh`
**Target Platform**: Cross-platform local CLI workflows on macOS, Linux, and
Windows/PowerShell with local filesystem access
**Project Type**: Rust CLI plus engine workspace
**Existing System Touchpoints**:
`crates/canon-engine/src/artifacts/contract.rs`,
`crates/canon-engine/src/artifacts/markdown/delivery/backlog.rs`,
`crates/canon-engine/src/domain/run.rs`,
`crates/canon-engine/src/orchestrator/gatekeeper/context.rs`,
`crates/canon-engine/src/orchestrator/service/mode_backlog.rs`,
`crates/canon-engine/src/orchestrator/publish.rs`,
`crates/canon-cli/src/output.rs`, `tests/backlog_run.rs`,
`tests/backlog_closure_run.rs`, `tests/backlog_contract.rs`,
`README.md`, `tech-docs/`, `defaults/embedded-skills/`, and `.agents/skills/`
**Performance Goals**: Preserve current local CLI responsiveness for backlog
run, inspect, and publish flows; keep slice identity and handoff evaluation
linear in packet size; add no network dependency or background service
**Constraints**: No new top-level CLI mode; no hidden runtime metadata
required to understand published packets; no drift into task decomposition; no
downstream-runtime-specific policy encoded into Canon packet generation
**Scale/Scope**: One feature spanning backlog packet generation, contract
surfaces, docs/skills, and focused regression coverage

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] Declared-risk approval checkpoints are named where required by the risk
      classification
- [x] Any constitution deviations are documented in Complexity Tracking

## Project Structure

### Documentation (this feature)

```text
specs/069-backlog-handoff-contract/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   └── backlog-execution-handoff-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
crates/
├── canon-cli/
│   └── src/
│       ├── app.rs
│       ├── commands/
│       ├── error.rs
│       ├── main.rs
│       └── output.rs
├── canon-engine/
│   └── src/
│       ├── artifacts/
│       ├── domain/
│       ├── orchestrator/
│       └── persistence/
defaults/
├── embedded-skills/
├── methods/
└── policies/
.agents/
└── skills/
tests/
├── contract/
├── fixtures/
└── integration/
README.md
tech-docs/
scripts/
└── validate-canon-skills.sh
```

**Structure Decision**: Keep the existing Canon Rust workspace intact. Add the
feature packet under `specs/069-backlog-handoff-contract/`, implement runtime
changes mainly in `canon-engine`, synchronize CLI summaries plus docs/skills,
and validate the contract through focused contract/integration coverage.

## Complexity Tracking

No constitution deviations are planned. The design extends the current backlog
packet rather than introducing a second runtime or execution subsystem.

## Workstreams

1. **Slice Identity Contract**: Add stable `slice_id` semantics and propagate
   them consistently across backlog packet artifacts.
2. **Execution Handoff Artifact**: Define when `execution-handoff.md` exists,
   what evidence it must carry, and how handoff unavailability is surfaced.
3. **Runtime and Publish Surfaces**: Align backlog overview, inspect, and
   publish/readability behavior with the new handoff semantics.
4. **Docs and Skills**: Update README, tech docs, and Canon skills so they
   describe the new contract truthfully and preserve downstream boundary
   ownership.
5. **Verification**: Add packet-contract, integration, and independent review
   coverage for both handoff-capable and handoff-unavailable paths.

## Phase Outcomes

### Phase 0: Research

- Decide whether downstream identity should be slice-level or task-level while
  preserving Canon backlog boundaries.
- Decide whether handoff availability is modeled as a separate artifact, an
  existing-artifact enrichment, or both.
- Decide how to represent full planning success when no slice is yet handoff-ready.
- Decide what minimum evidence makes a slice credible for downstream execution
  handoff without crossing into task generation.

### Phase 1: Design

- Define data-model additions for slice identity, implementation artifact
  references, independent verification anchors, and handoff-availability state.
- Define the additive packet contract for `execution-handoff.md` and the
  existing-artifact additions required to support it.
- Define quickstart scenarios for handoff-capable, handoff-unavailable, and
  closure-limited packets.
- Record design decisions and the layered validation plan in durable artifacts.

### Phase 2: Implementation Preparation

- Leave a task-ready design for runtime, packet, docs, skills, and regression
  changes.
- Keep post-design validation explicit so implementation can preserve the
  generation-versus-validation boundary.
