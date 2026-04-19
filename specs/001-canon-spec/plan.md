# Implementation Plan: Canon v0.1 Native CLI

**Branch**: `001-canon-spec` | **Date**: 2026-03-27 | **Spec**: `specs/001-canon-spec/spec.md`
**Input**: Feature specification from `specs/001-canon-spec/spec.md`

## Summary

Canon v0.1 will ship as a local-first, single-binary Rust CLI that
governs AI-assisted engineering runs through typed mode semantics, fail-closed
policy evaluation, durable artifact contracts, persisted decision memory, and
bounded execution adapters. v0.1 will deliver full operational depth for
`requirements`, `brownfield-change`, and `pr-review`, while modeling the
remaining nine modes as first-class domain concepts now so future task
generation extends implementation depth instead of reopening core architecture.

## Governance Context

**Execution Mode**: `architecture` for system-shaping product planning and control
plane design  
**Risk Classification**: `Systemic Impact` because this plan defines the core
domain, gate behavior, approval semantics, persistence model, and public CLI
surface of the governance engine itself  
**Scope In**: workspace topology, public CLI surface, domain model, run
lifecycle, policies, gates, artifacts, adapters, persistence, testing, CI, and
repository quality controls  
**Scope Out**: IDE integration, hosted control plane, plugin marketplace,
distributed execution, autonomous swarm orchestration, non-filesystem storage,
and deep workflow UX beyond the initial CLI contracts

**Invariants**:

- Mode semantics remain explicit Rust enums and code-owned profiles, not
  user-authored workflow text.
- No run advances past its first meaningful gate without mode, risk, usage
  zone, artifact contract, and ownership boundaries.
- Generation and validation are persisted as separate steps with separate
  evidence records.
- `.canon/` artifacts, decisions, and traces are the system of record rather
  than chat history.
- Adapters are governed execution surfaces and cannot bypass policy or
  persistence.

**Decision Log**: `specs/001-canon-spec/decision-log.md`
**Validation Ownership**: run orchestration and adapters generate outputs;
policy evaluation, verification layers, tests, and human review validate them
through separate artifacts and gate outcomes  
**Approval Gates**: Architecture Gate, Risk Gate, Release / Readiness Gate, and
named human approvals for `Systemic Impact` or `Red` zone work

## 1. Technical Context

**Language/Version**: Rust 1.95.0, Edition 2024  
**Primary Dependencies**: `clap` for CLI parsing; `serde`, `serde_json`,
`toml`, and `serde_yaml` for manifest and artifact serialization; `sha2` for
durable authored-input digests;
`thiserror` for typed error boundaries; `tracing` and
`tracing-subscriber` for local audit traces; `uuid` with UUIDv7 support for
sortable run identifiers; `time` for stable timestamps; test-only
dependencies `assert_cmd`, `predicates`, `tempfile`, and `insta` for CLI,
artifact, and snapshot verification  
**Storage**: local filesystem only under `.canon/`; TOML for configuration and
manifests; Markdown, JSON, and YAML for emitted artifacts; JSONL for adapter
trace streams; run-local snapshots for authored file-backed inputs under
`.canon/runs/<run-id>/inputs/`; atomic file replace semantics for every
durable write
**Testing**: `cargo test` for unit and focused integration tests, `cargo nextest
run` in CI, snapshot tests for stable artifact rendering, dedicated gate and
resume tests, and adapter isolation tests with fakes or fixture commands  
**Target Platform**: macOS, Linux, and Windows for ARM and x86_64; release
support is staged, but the domain model and CLI contracts are platform-neutral
from day one  
**Project Type**: native CLI, single binary, local-first orchestration tool  
**Existing System Touchpoints**: repository working tree, generated `.canon/`
state, external CLIs such as Copilot CLI and shell tools, optional MCP stdio
tools, test runners, linters, and repository metadata  
**Performance Goals**: under 150ms startup for `status` and `inspect`; under 1s
for classification, contract creation, and gate evaluation excluding external
tool latency; bounded resume checks proportional to referenced artifacts rather
than repository size  
**Constraints**: no async runtime in v0.1, no database, no daemon process, no
plugin DSL, no remote control plane, no IDE integration, and no policy bypass
through direct adapter invocation  
**Scale/Scope**: one local repository per invocation; dozens of artifacts and
hundreds of trace entries per run; resumable run state persisted after every
completed step and gate

**Toolchain Baseline**:

```toml
[toolchain]
channel = "1.95.0"
profile = "minimal"
components = ["rustfmt", "clippy"]
targets = ["x86_64-unknown-linux-musl", "x86_64-unknown-linux-gnu"]
```

**Target Support Staging**:

The baseline toolchain above is acceptable for contributor setup and Linux
release work, but it is not sufficient to claim full support for macOS,
Windows, and ARM.

- **Stage 1, required in CI/release**: add build coverage for
  `x86_64-apple-darwin`, `aarch64-apple-darwin`, and
  `x86_64-pc-windows-msvc`.
- **Stage 2, required for Linux ARM parity**: add `aarch64-unknown-linux-gnu`
  to release builds and smoke tests.
- **Stage 3, only after toolchain and test stability are proven**: add
  `aarch64-pc-windows-msvc`.

The plan keeps the contributor-facing `rust-toolchain.toml` lean while using
CI and release jobs to install additional targets explicitly. This avoids
forcing every contributor to install the full cross-platform target matrix
before the release process is ready to verify it.

## 2. Constitution Check

### Pre-Design Gate

- [x] Execution mode is declared and matches the requested work
- [x] Risk classification is explicit and autonomy is appropriate for that risk
- [x] Scope boundaries and exclusions are recorded
- [x] Invariants are explicit before implementation
- [x] Required artifacts and owners are identified
- [x] Decision logging is planned and linked to a durable artifact
- [x] Validation plan separates generation from validation
- [x] High-risk approval checkpoints are named
- [x] No constitution deviations are required for this plan

### Post-Design Re-Check

- [x] All product modes exist as first-class domain concepts in the plan
- [x] MVP depth is staged without collapsing architecture into three modes
- [x] Mutating execution remains bounded by policy, zone, and approval
- [x] Filesystem persistence remains the local system of record
- [x] Review and verification stay distinct from generation
- [x] Human ownership remains mandatory for systemic or red-zone impact

**Result**: PASS. No constitution violations or justified exceptions were
required.

## 3. Architectural Decisions for v0.1

| Decision Area | v0.1 Choice | Why This Choice |
| --- | --- | --- |
| Workspace shape | Three-crate workspace: CLI, engine, adapters | Keeps domain boundaries explicit without creating crate sprawl |
| Mode modeling | All modes are explicit enums plus code-owned `ModeProfile` values | Preserves semantics and prevents a generic workflow engine from replacing the product thesis |
| Methods and policies | Hybrid: mode semantics in code, policies and selected method metadata in versioned TOML | Allows controlled configurability without inventing a DSL |
| Runtime model | Synchronous orchestration with blocking filesystem and process execution | Simpler auditability, easier cross-platform behavior, no async complexity until adapter pressure justifies it |
| Persistence | `.canon/` as append-only local memory with run manifests, artifact bundles, decisions, and JSONL traces | Makes auditability and resumability first-class instead of bolt-ons |
| Adapter boundary | Concrete adapters dispatched through typed requests and capabilities, not a public plugin framework | Keeps external tools subordinate to governance and avoids extensibility theater |
| Verification | Self-critique may be automated, but adversarial, peer, and architectural layers remain separately recorded | Prevents generated agreement from masquerading as independent validation |
| Mutating red-zone work | Recommendation-only in v0.1 for mutating adapters touching `Red` zone or `Systemic Impact` contexts | Keeps autonomy bounded by what the organization can still validate |
| Mode depth staging | `requirements`, `brownfield-change`, and `pr-review` are fully operational; all other modes have typed contracts and gated skeleton flows | Models the full product shape now without pretending all modes are equally mature |

The engine will not introduce an event bus, background worker pool, or generic
trait-driven plugin runtime in v0.1. A single orchestrator with explicit domain
types and direct module calls is the intended shape.

## 4. Proposed Project Structure

### Documentation

```text
specs/001-canon-spec/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── decision-log.md
├── validation-report.md
├── contracts/
│   ├── cli-contract.md
│   └── runtime-filesystem-contract.md
└── tasks.md
```

### Source Code

```text
Cargo.toml
rust-toolchain.toml
rustfmt.toml
deny.toml
.github/
└── workflows/
    └── ci.yml
.githooks/
└── pre-commit
scripts/
└── install-hooks.sh
defaults/
├── methods/
│   ├── requirements.toml
│   ├── discovery.toml
│   ├── system-shaping.toml
│   ├── brownfield-change.toml
│   ├── architecture.toml
│   ├── implementation.toml
│   ├── refactor.toml
│   ├── verification.toml
│   ├── review.toml
│   ├── pr-review.toml
│   ├── incident.toml
│   └── migration.toml
└── policies/
    ├── risk.toml
    ├── zones.toml
    ├── gates.toml
    ├── verification.toml
    └── adapters.toml
crates/
├── canon-cli/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── app.rs
│       ├── commands/
│       │   ├── init.rs
│       │   ├── run.rs
│       │   ├── resume.rs
│       │   ├── status.rs
│       │   ├── approve.rs
│       │   ├── verify.rs
│       │   └── inspect.rs
│       └── output.rs
├── canon-engine/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── domain/
│       │   ├── mode.rs
│       │   ├── method.rs
│       │   ├── run.rs
│       │   ├── policy.rs
│       │   ├── gate.rs
│       │   ├── artifact.rs
│       │   ├── decision.rs
│       │   ├── verification.rs
│       │   └── approval.rs
│       ├── modes/
│       │   ├── requirements.rs
│       │   ├── discovery.rs
│       │   ├── system_shaping.rs
│       │   ├── brownfield_change.rs
│       │   ├── architecture.rs
│       │   ├── implementation.rs
│       │   ├── refactor.rs
│       │   ├── verification.rs
│       │   ├── review.rs
│       │   ├── pr_review.rs
│       │   ├── incident.rs
│       │   └── migration.rs
│       ├── orchestrator/
│       │   ├── service.rs
│       │   ├── classifier.rs
│       │   ├── gatekeeper.rs
│       │   ├── resume.rs
│       │   └── verification_runner.rs
│       ├── persistence/
│       │   ├── store.rs
│       │   ├── layout.rs
│       │   ├── manifests.rs
│       │   └── atomic.rs
│       ├── artifacts/
│       │   ├── contract.rs
│       │   ├── manifest.rs
│       │   ├── markdown.rs
│       │   ├── json.rs
│       │   └── yaml.rs
│       └── review/
│           ├── critique.rs
│           ├── summary.rs
│           └── findings.rs
└── canon-adapters/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── capability.rs
        ├── dispatcher.rs
        ├── filesystem.rs
        ├── shell.rs
        ├── copilot_cli.rs
        └── mcp_stdio.rs
tests/
├── integration/
├── contract/
├── fixtures/
└── snapshots/
```

**Structure Decision**: use a three-crate workspace. `canon-engine`
owns the product semantics and orchestration. `canon-cli` owns the
user-facing command surface and output rendering. `canon-adapters`
owns concrete execution surfaces. Built-in methods and policy defaults live in
`defaults/` so they can be versioned in the repository and embedded into the
binary at build time.

## 5. Data and Artifact Model

### Primary Domain Objects

- `Mode`: the first-class operating context; all twelve modes exist in code.
- `ModeProfile`: the typed definition of purpose, artifact families, gate
  profile, verification baseline, allowed adapters, and current implementation
  depth for a mode.
- `Method`: ordered step definition for a selected mode. Steps remain typed and
  mode-aware rather than generic workflow nodes.
- `PolicySet`: versioned policy bundle loaded from TOML into typed structs for
  risk, zone, gate, verification, and adapter rules.
- `Run`: the persisted execution record tying together mode, context,
  classifications, contract, artifacts, gate results, approvals, decisions,
  verification, and traces.
- `ArtifactContract`: the complete required artifact set for a run after mode,
  risk, zone, and policy are resolved.
- `ArtifactRecord`: a single emitted artifact with path, format, checksum,
  producing step, and validation status.
- `GateEvaluation`: the persisted outcome of a gate, including blockers and
  required approvals.
- `DecisionRecord`: an append-only rationale artifact for consequential choices
  and overrides.
- `VerificationRecord`: evidence from one verification layer linked to the
  artifacts or findings it challenges.
- `ApprovalRecord`: a durable human decision tied to a gate or override.
- `AdapterInvocation`: the traceable record of one external capability use.

### Artifact Contract Behavior

Artifact contracts are derived from:

1. selected `ModeProfile`
2. risk and zone classification
3. policy overlays
4. explicit run scope and ownership

The contract is persisted before the first gate can pass and includes:

- required artifact files and formats
- minimum content sections per artifact
- gate dependencies per artifact
- verification layers required for the run
- approval requirements by gate
- adapter capabilities allowed for the run

Artifact content stays human-readable. Machine metadata such as schema version,
checksums, and dependency links lives in run manifests rather than in large
front matter blocks inside the Markdown artifacts.

### Run State Machine

`RunState` will be modeled as:

- `Draft`
- `ContextCaptured`
- `Classified`
- `Contracted`
- `Gated`
- `Executing`
- `AwaitingApproval`
- `Verifying`
- `Completed`
- `Blocked`
- `Failed`
- `Aborted`
- `Superseded`

The engine persists state after every completed step and gate. `resume` starts
from the first incomplete or invalidated checkpoint, not from scratch, and it
marks the run stale if the repository head or referenced inputs changed since
the last durable checkpoint.

## 6. Mode Model and Mode Taxonomy

All modes exist now as typed `Mode` variants and typed `ModeProfile`
definitions. Implementation depth is staged, but semantic meaning is not.

### requirements

- Purpose: bound an initiative before generation expands it into platform
  sprawl.
- Expected inputs: raw idea, business goal, constraints, exclusions, referenced
  docs.
- Required artifact families: problem framing, constraints, options, tradeoffs,
  scope cuts, decision checklist.
- Gate profile: Exploration -> Risk -> Architecture when structure is touched ->
  Release / Readiness.
- Verification expectations: subtraction review, adversarial critique of scope,
  human approval of the bounded core.
- Likely adapter usage: filesystem, read-only shell, AI generation, AI
  critique.
- Weight: analysis-heavy.
- v0.1 depth: full workflow.

### discovery

- Purpose: explore unknowns and evidence without turning exploration into
  premature solution design.
- Expected inputs: hypotheses, open questions, user/problem evidence,
  repository context, domain references.
- Required artifact families: discovery brief, assumptions register, evidence
  log, unknowns register, discovery summary.
- Gate profile: Exploration -> Risk -> Release / Readiness.
- Verification expectations: challenge hidden assumptions and explicitly record
  evidence quality.
- Likely adapter usage: filesystem, shell, optional MCP research tools, AI
  synthesis.
- Weight: analysis-heavy.
- v0.1 depth: typed contract and skeleton flow, recommendation-only output.

### system-shaping

- Purpose: define a new system or capability from bounded intent through early
  delivery structure.
- Expected inputs: approved problem framing, target users, constraints, success
  criteria.
- Required artifact families: system intent, domain map, architecture options,
  boundary decisions, initial delivery plan.
- Gate profile: Exploration -> Architecture -> Risk -> Release / Readiness.
- Verification expectations: architectural coherence and scope containment.
- Likely adapter usage: filesystem, AI generation, AI critique, optional shell
  for repository scaffolding recommendations.
- Weight: analysis-heavy moving toward execution-heavy.
- v0.1 depth: typed contract and plan-first flow, execution hooks deferred.

### brownfield-change

- Purpose: constrain change in an existing system before implementation begins.
- Expected inputs: repository slice, affected modules, goal, interfaces,
  operational constraints.
- Required artifact families: system map or slice, legacy invariants, change
  surface, implementation plan, validation strategy, decision record.
- Gate profile: Exploration -> Brownfield Preservation -> Architecture -> Risk
  -> Release / Readiness.
- Verification expectations: preserved behavior, blast-radius clarity,
  independent validation strategy, named ownership for systemic changes.
- Likely adapter usage: filesystem, shell, repository inspection, AI analysis,
  test runner discovery.
- Weight: analysis-heavy with constrained execution planning.
- v0.1 depth: full workflow.

### architecture

- Purpose: evaluate or define boundaries, invariants, and structural decisions.
- Expected inputs: system context, current boundaries, invariants, architectural
  concerns, decision pressure.
- Required artifact families: invariants, boundary map, architecture options,
  tradeoffs, decision record, risk memo.
- Gate profile: Exploration -> Architecture -> Risk -> Release / Readiness.
- Verification expectations: architectural review is mandatory for bounded or
  systemic impact.
- Likely adapter usage: filesystem, shell, AI generation, AI critique.
- Weight: analysis-heavy.
- v0.1 depth: typed contract and reviewable design flow.

### implementation

- Purpose: turn an approved bounded plan into controlled execution.
- Expected inputs: approved implementation plan, task scope, ownership,
  validation strategy.
- Required artifact families: execution brief, task bundle, contract checklist,
  change log, verification hooks, completion record.
- Gate profile: Risk -> Implementation Readiness -> Release / Readiness.
- Verification expectations: tests and review must attach to plan claims rather
  than replace them.
- Likely adapter usage: filesystem, shell, AI generation, test runners,
  linters.
- Weight: execution-heavy.
- v0.1 depth: typed contract only; execution remains subordinate to prior
  modes.

### refactor

- Purpose: improve structure while preserving externally meaningful behavior.
- Expected inputs: current implementation slice, preservation goals, code
  smells, known invariants.
- Required artifact families: equivalence criteria, preserved surface,
  untangling plan, rollback notes, validation strategy.
- Gate profile: Exploration -> Brownfield Preservation -> Risk -> Release /
  Readiness.
- Verification expectations: prove behavior preservation, not just style
  improvement.
- Likely adapter usage: filesystem, shell, AI analysis, test runners, linters.
- Weight: execution-heavy with preservation constraints.
- v0.1 depth: typed contract and gate profile; deep automation deferred.

### verification

- Purpose: challenge claims, invariants, contracts, and evidence directly.
- Expected inputs: target artifacts, implementation claims, test results,
  decision records, invariants.
- Required artifact families: invariants checklist, contract matrix,
  adversarial review, verification report, unresolved findings.
- Gate profile: Risk -> Release / Readiness.
- Verification expectations: this mode is itself a verification surface and
  must preserve independence from the generator being challenged.
- Likely adapter usage: filesystem, shell, test runners, AI critique, human
  review.
- Weight: review-heavy.
- v0.1 depth: typed contract plus reusable verification hooks used by all runs.

### review

- Purpose: review a change package, plan, artifact bundle, or design outside of
  pull-request semantics.
- Expected inputs: artifact bundle, claimed intent, related context, ownership,
  existing verification evidence.
- Required artifact families: review brief, boundary assessment, missing
  evidence, decision impact, review disposition.
- Gate profile: Risk -> Architecture when structure is touched -> Review
  Disposition -> Release / Readiness.
- Verification expectations: explicitly identify what is being reviewed and what
  remains unvalidated.
- Likely adapter usage: filesystem, shell, AI critique.
- Weight: review-heavy.
- v0.1 depth: typed contract and reusable review engine.

### pr-review

- Purpose: produce structured review artifacts for a branch or pull request
  diff.
- Expected inputs: diff, changed files, claimed intent, linked spec or issue,
  tests, repository context.
- Required artifact families: PR analysis, boundary check, duplication check,
  contract drift, missing tests, decision impact, review summary.
- Gate profile: Risk -> Architecture when structure is touched -> Review
  Disposition -> Release / Readiness.
- Verification expectations: findings mapped to changed surfaces with explicit
  disposition and owner handling for systemic issues.
- Likely adapter usage: filesystem, shell, repository diff inspection, AI
  critique.
- Weight: review-heavy.
- v0.1 depth: full workflow.

### incident

- Purpose: bound investigation and containment work during failures without
  letting urgency erase governance.
- Expected inputs: incident report, blast radius, observed symptoms, recent
  changes, operational constraints.
- Required artifact families: incident frame, hypothesis log, blast-radius map,
  containment plan, incident decision record, follow-up verification notes.
- Gate profile: Risk -> Incident Containment -> Architecture when system
  boundaries are affected -> Release / Readiness.
- Verification expectations: containment actions need explicit ownership and
  post-incident verification.
- Likely adapter usage: filesystem, shell, logs through shell or MCP, AI
  analysis.
- Weight: analysis-heavy under time pressure.
- v0.1 depth: typed contract and containment-oriented stop conditions.

### migration

- Purpose: manage movement between systems, schemas, contracts, or boundaries
  with explicit compatibility control.
- Expected inputs: source and target context, migration goal, compatibility
  constraints, fallback path, data or contract boundaries.
- Required artifact families: source-target map, compatibility matrix,
  sequencing plan, fallback plan, migration verification report, decision
  record.
- Gate profile: Exploration -> Architecture -> Migration Safety -> Risk ->
  Release / Readiness.
- Verification expectations: compatibility and reversibility evidence are
  mandatory.
- Likely adapter usage: filesystem, shell, AI analysis, test runners.
- Weight: analysis-heavy and execution-heavy.
- v0.1 depth: typed contract and safety gate skeleton.

## 7. Policy and Gate Strategy

### Policy Resolution

The engine resolves policy in this order:

1. load built-in typed defaults from embedded method and policy files
2. materialize or refresh `.canon/methods/` and `.canon/policies/`
3. merge repository-local overrides that match the known schemas
4. construct an effective `PolicySet` for the run

Policies remain schema-bound and typed. v0.1 does not permit arbitrary script
hooks or free-form policy expressions. If a policy cannot be parsed into a
known typed structure, the engine fails closed.

### Gate Model

`GateKind` will include:

- `Exploration`
- `BrownfieldPreservation`
- `Architecture`
- `Risk`
- `ReviewDisposition`
- `ReleaseReadiness`
- `ImplementationReadiness`
- `IncidentContainment`
- `MigrationSafety`

`GateStatus` will include:

- `Pending`
- `Passed`
- `Blocked`
- `NeedsApproval`
- `Overridden`

Every gate evaluation persists:

- the question answered
- inputs used
- artifacts inspected
- blockers found
- approval requirement, if any
- evaluator identity
- timestamp

### Gate Enforcement Rules

- Gates are fail-closed by default.
- Missing artifacts block the gate before any content judgment is attempted.
- Artifact structural checks are automated in v0.1 through file existence,
  schema version, minimum sections, and contract linkage.
- Logical sufficiency remains mode-aware and risk-aware; higher-risk runs
  require human review artifacts in addition to structural validation.
- Any override requires both an `ApprovalRecord` and a linked `DecisionRecord`.

### Classification and Approval Rules

- Risk and usage zone are resolved before artifact generation or external
  mutation.
- The stricter of risk and zone always wins.
- `Systemic Impact` or `Red` zone work always requires named `HumanOwnership`.
- Mutating adapters are blocked in `Red` zone or `Systemic Impact` contexts in
  v0.1; the engine emits recommendations and artifacts instead.

## 8. Adapter Strategy

### Adapter Philosophy

Adapters are execution surfaces. They are not the product identity and they do
not own policy. The engine will use concrete adapters behind a typed dispatcher
rather than a public trait-based extension API.

### Initial Adapter Set

- `FilesystemAdapter`: native Rust read and write operations constrained to the
  repository or declared output roots.
- `ShellAdapter`: blocking process execution for repository inspection, tests,
  linters, and other declared local commands.
- `CopilotCliAdapter`: wrapper around Copilot CLI invocations with explicit
  purpose tags such as generation, critique, or synthesis.
- `McpStdioAdapter`: optional adapter for local MCP-compatible stdio tools,
  disabled unless configured.

### Capability Model

Each adapter invocation declares:

- adapter kind
- requested capability
- invocation purpose
- side-effect class
- allowed output channels
- trace sink

Initial capabilities include:

- read repository context
- write governed artifact
- execute read-only command
- execute mutating command
- invoke AI generation
- invoke AI critique
- invoke external structured tool

### Isolation and Failure Handling

- Adapter failures are persisted as trace and gate evidence.
- Missing adapters degrade the run to recommendation-only mode when safe.
- Unsafe missing adapters block the run instead of silently succeeding.
- Validation layers cannot silently mutate the primary artifact set.

## 9. Persistence Strategy

### Runtime Filesystem Layout

```text
.canon/
├── sessions/
├── artifacts/
├── decisions/
├── traces/
├── methods/
├── policies/
└── runs/
```

### Directory Responsibilities

- `sessions/`: current CLI session metadata and lightweight session-to-run
  pointers.
- `artifacts/`: per-run artifact bundles, organized by run id and mode.
- `decisions/`: append-only decision records and explicit approval overrides.
- `traces/`: JSONL streams of adapter invocations, capability use, and evidence
  references.
- `methods/`: materialized method definitions generated from built-in defaults
  and allowed local overrides.
- `policies/`: effective policy files for classification, gates, verification,
  and adapters.
- `runs/`: run manifests, state transitions, gate outcomes, approval records,
  verification summaries, and links to artifacts and traces.

### Run Persistence Model

Each run will persist a dedicated directory:

```text
.canon/runs/<run-id>/
├── run.toml
├── context.toml
├── artifact-contract.toml
├── state.toml
├── gates/
├── inputs/
├── approvals/
├── verification/
└── links.toml
```

Key persistence rules:

- `run.toml` is the stable manifest header and includes mode, risk, zone,
  policy version, method version, owner, and parent run pointer.
- `artifact-contract.toml` is written before gate evaluation and never replaced
  in place; revisions create a new contract version file and update links.
- `inputs/` stores snapshots of the authored file-backed inputs that were
  actually used for the run.
- `links.toml` binds the run to artifacts, decisions, and traces.
- trace files live in `.canon/traces/<run-id>.jsonl` and are referenced rather
  than embedded.
- artifact bundles live in `.canon/artifacts/<run-id>/<mode>/`.

### Resumability

Resume is stateful rather than heuristic:

- every step completion updates `state.toml`
- every gate outcome persists independently
- input file fingerprints and repository head are stored in `context.toml`
- `resume` compares the current environment to stored fingerprints
- if the environment changed, the run becomes `Blocked` until the user chooses
  reuse, refresh, or fork
- reruns default to `fork` semantics with `parent_run_id` linkage instead of
  overwriting prior provenance

## 10. Testing Strategy

### Unit Tests

- domain invariants for `Mode`, `RiskClass`, `UsageZone`, `GateKind`,
  `RunState`, and `ArtifactContract`
- policy loading and merge rules
- gate evaluation logic and blocker detection
- run state transitions and stale-context detection
- manifest serialization and atomic persistence helpers

### Integration Tests

- end-to-end `requirements` run producing the full artifact bundle
- end-to-end `brownfield-change` run blocking when legacy invariants are absent
- end-to-end `pr-review` run producing the full review packet
- resume behavior after interruption and after repository drift
- approval workflow for blocked or systemic runs

### Contract and Adapter Tests

- CLI contract snapshots for command outputs and exit codes
- artifact rendering snapshots for Markdown, JSON, and YAML outputs
- adapter isolation tests using fixture executables or faked command runners
- failure-path tests for unavailable Copilot CLI or MCP adapters
- gate enforcement tests proving mutating adapters are blocked in `Red` or
  `Systemic Impact` contexts

### Coverage Strategy

The test plan favors confidence on domain rules and run flow over a massive
combinatorial matrix. Full end-to-end runs are required for the three deep MVP
modes. The remaining modes receive typed profile tests proving that contracts,
gate profiles, and verification baselines are wired correctly.

## 11. CI and Repository Quality Strategy

### Required Repository Files

- `rust-toolchain.toml`
- `rustfmt.toml`
- `deny.toml`
- `.github/workflows/ci.yml`
- `.githooks/pre-commit`
- `scripts/install-hooks.sh`

### rustfmt Configuration

```toml
edition = "2024"
hard_tabs = false
max_width = 100
tab_spaces = 4
use_small_heuristics = "Max"
newline_style = "Unix"
reorder_imports = true
```

### CI Pipeline

The initial GitHub Actions pipeline will contain these jobs:

1. `fmt-clippy`
   - `cargo fmt --check`
   - `cargo clippy --all-targets --all-features -- -D warnings`
2. `test`
   - `cargo test`
   - `cargo nextest run`
3. `msrv`
   - prefer `cargo msrv verify`
   - fallback strategy if the tool lags Edition 2024 support:
     pinned `cargo +1.95.0 test --all-targets --all-features`
4. `deny`
   - `cargo deny check licenses advisories bans sources`
5. `cross-platform-build`
   - native smoke builds on Linux, macOS, and Windows
   - staged target installs for Apple and Windows triples
   - later addition of Linux ARM and Windows ARM builds

### Pre-Commit

The repository pre-commit hook must fail fast and run:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

### Dependency and License Policy

`deny.toml` will:

- allow only permissive licenses such as Apache-2.0, MIT, BSD-2-Clause,
  BSD-3-Clause, ISC, Unicode-DFS-2016, and Zlib
- deny copyleft or source-available licenses
- enable RustSec advisories checking
- ban duplicate or unsafe dependency patterns where practical
- restrict unknown registries and unknown git sources

## 12. Risks and Complexity Tracking

| Risk | Why It Matters | Mitigation |
| --- | --- | --- |
| Full mode taxonomy with staged depth | Modeling all modes now can create surface area pressure | Keep semantics in typed profiles, but only implement deep workflows for the three MVP modes |
| Cross-platform adapter behavior | Shell and external CLI behavior diverge across macOS, Linux, and Windows | Normalize invocation through adapter requests, keep shell usage narrow, and add contract tests for quoting and exit handling |
| Overly weak artifact validation | File existence alone can become documentation theater | Validate minimum sections structurally and require higher-risk human review artifacts before readiness |
| Resume on stale context | Reusing artifacts after repository changes can create false confidence | Persist fingerprints and block resume until reuse, refresh, or fork is explicitly chosen |
| Policy drift through local overrides | Flexible config can erode governance guarantees | Use schema-bound TOML only, fail closed on unknown fields, and persist effective policy versions in every run |
| Target matrix inflation | Full platform support can slow delivery if pursued all at once | Stage release coverage while keeping contributor defaults lean and testing the core logic on every platform first |

No constitution deviations are required.

## 13. Planning Decisions Resolving Open Technical Questions

| Open Question | Planning Decision |
| --- | --- |
| Should methods be code-defined, config-defined, or hybrid? | Hybrid. Mode semantics, step ordering, and gate wiring stay in code. Method metadata and selected defaults are versioned TOML. |
| How strict should gates be at runtime? | Fail-closed by default. Any bypass requires both an approval record and a decision record. |
| How should adapters expose capabilities safely? | Through typed capability enums and dispatcher requests, not a general plugin API or policy DSL. |
| How should human approvals be represented? | As persisted `ApprovalRecord` files under the run directory, linked to gates and decision records. |
| How should artifact schemas be versioned? | Through schema version fields in run manifests and artifact records, not hidden Markdown metadata conventions. |
| How should reruns relate to prior decision memory? | Fork by default with `parent_run_id` links; never overwrite prior run history. |
| How much trace detail is enough? | JSONL summaries with capability, intent, timestamps, status, and evidence links; do not persist raw prompt transcripts by default. |
| Should red-zone execution ever mutate state in v0.1? | No. `Red` zone or `Systemic Impact` runs are recommendation-only for mutating adapters in v0.1. |
