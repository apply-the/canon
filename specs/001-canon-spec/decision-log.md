# Decision Log: Canon v0.1

## D-001: Use a three-crate workspace

**Context**: the product needs clear separation between CLI concerns, core
governance semantics, and external execution surfaces.

**Decision**: use `canon-cli`, `canon-engine`, and
`canon-adapters`.

**Alternatives considered**:

- single crate monolith
- many narrowly scoped crates for every subsystem

**Consequences**:

- domain boundaries stay clear
- crate overhead stays manageable
- adapters remain replaceable without turning the project into a plugin system

## D-002: Keep mode semantics strongly typed in code

**Context**: the product loses its thesis if modes collapse into a generic
workflow abstraction.

**Decision**: all modes exist as explicit enum variants and code-owned
`ModeProfile` values.

**Alternatives considered**:

- config-only mode definitions
- generic workflow nodes with labels

**Consequences**:

- future tasks extend specific modes instead of inventing runtime semantics
- architectural review can reason about mode behavior concretely

## D-003: Use a hybrid method and policy model

**Context**: some governance behavior must be inspectable and adjustable without
recompilation, but the product cannot become a DSL host.

**Decision**: mode semantics, step ordering, and gate wiring stay in code;
policy files and selected method metadata are versioned TOML.

**Alternatives considered**:

- all code
- all config

**Consequences**:

- repository-local overrides remain possible
- unknown policy structure can be rejected safely at load time

## D-004: Keep orchestration synchronous in v0.1

**Context**: the product is a local control plane with limited parallelism
pressure.

**Decision**: use blocking filesystem and process execution in v0.1.

**Alternatives considered**:

- async runtime with concurrent adapter calls
- worker queue or daemon architecture

**Consequences**:

- simpler tracing and resume semantics
- less runtime complexity and lower test overhead

## D-005: Persist all run memory under .canon

**Context**: auditability and resumability are product requirements, not
optional diagnostics.

**Decision**: use `.canon/` as the local system of record with TOML manifests,
artifact bundles, decision records, and JSONL traces.

**Alternatives considered**:

- embedded database
- ephemeral-only CLI output

**Consequences**:

- run state is inspectable with normal repository tools
- future runs can link to prior artifacts and decisions directly

## D-006: Red-zone mutating execution is recommendation-only in v0.1

**Context**: v0.1 must keep autonomy bounded by what humans can still validate.

**Decision**: block mutating adapters for `Red` zone or `Systemic Impact` runs
and emit recommendations plus required artifacts instead.

**Alternatives considered**:

- allow mutation with warnings
- allow mutation after automated self-critique only

**Consequences**:

- the product remains governable during early releases
- incident, migration, and brownfield flows retain strong human ownership

## D-007: Model all modes now, stage workflow depth

**Context**: the product architecture must cover the full lifecycle, but v0.1
cannot implement every mode equally deeply.

**Decision**: implement full workflow depth for `requirements`,
`brownfield-change`, and `pr-review`; model the remaining modes with typed
profiles, contracts, and gate skeletons now.

**Alternatives considered**:

- only model the three MVP modes
- promise equal implementation depth across all modes

**Consequences**:

- the next task phase can expand modes without reworking the domain model
- v0.1 remains deliverable

## D-008: Treat the CLI and runtime filesystem as the public contracts

**Context**: users and automated tooling need stable surfaces to rely on.

**Decision**: document and preserve the CLI commands, exit codes, and runtime
filesystem layout as v0.1 contracts.

**Alternatives considered**:

- undocumented internal behavior
- library-first API surface

**Consequences**:

- contract tests can guard compatibility
- internal modules remain refactorable behind stable external behavior

## D-009: Stage platform support through CI and releases

**Context**: contributor ergonomics and release correctness have different
needs.

**Decision**: keep `rust-toolchain.toml` lean, then add Apple, Windows, and ARM
targets explicitly in CI and release jobs.

**Alternatives considered**:

- force every target into local developer toolchains immediately
- claim narrower platform support

**Consequences**:

- the contributor setup stays manageable
- platform claims remain tied to verified build automation

## D-010: Start implementation with governance artifacts and bootstrap only

**Context**: the implementation phase begins in an empty repository, so setup
must happen before behavior work, but behavior changes still need TDD.

**Decision**: treat governance updates, workspace bootstrap, toolchain files,
ignore hygiene, hook scripts, and default method or policy files as phase-zero
and setup work. Begin test-first execution when domain behavior starts.

**Alternatives considered**:

- hand-wave setup and start directly in engine code
- treat all repository bootstrap as exempt from governance tracking

**Consequences**:

- the implementation starts within the declared invariants
- TDD remains preserved for run behavior, gates, persistence, and CLI flows

## D-011: Keep runtime output under .canon and out of version control

**Context**: this repository will generate `.canon/` state when the CLI is run
locally or in tests.

**Decision**: ignore `.canon/` and other transient Rust outputs in `.gitignore`
while keeping the runtime filesystem contract documented in planning artifacts.

**Alternatives considered**:

- commit generated runtime state into the product repository
- leave runtime output unignored and accept noisy working trees

**Consequences**:

- local runs stay inspectable without polluting git state
- the runtime contract remains a public interface without becoming checked-in
  build output

## D-012: Ship requirements mode as the first executable governed slice

**Context**: the implementation plan calls for one complete end-to-end mode
before deeper brownfield and review behaviors are attempted.

**Decision**: complete the `requirements` path first with typed classification,
persisted run manifests, artifact-contract snapshots, persisted gate outcomes,
and a runnable CLI flow that emits the six required artifacts.

**Alternatives considered**:

- continue broad foundational work before proving one vertical slice
- start with `brownfield-change` as the first executable mode
- postpone snapshots until later mode work

**Consequences**:

- the product now has one auditable run path that proves the filesystem model,
  gate persistence, and CLI contract together

## D-013: Use typed diff heuristics for the first PR review slice

**Context**: `pr-review` needed real end-to-end behavior in v0.1, but the
engine still needs deterministic, local-first semantics rather than an open
ended review agent.

**Decision**: derive the first review packet from git diff inputs plus typed
surface heuristics for contract files, boundary-marked files, source files, and
test surfaces. Persist the resulting review packet as artifacts and gates
rather than relying on unstructured model output.

**Alternatives considered**:

- require Copilot CLI or another model for every review run
- delay `pr-review` until a richer semantic analysis engine exists
- treat review as a generic prompt workflow

**Consequences**:

- `pr-review` now works locally with no external AI dependency
- review findings remain bounded and auditable
- future semantic review depth can be added without replacing the gate or
  artifact contract model

## D-014: Make unresolved must-fix review findings approval-driven

**Context**: the review slice needs a clear distinction between structural
blockers and findings that can only be accepted by an accountable human owner.

**Decision**: unresolved must-fix PR review findings move the run into
`AwaitingApproval` through the `ReviewDisposition` gate. A persisted
`ApprovalRecord` for `review-disposition` can then override the gate and allow
the run to complete without mutating the generated review packet in place.

**Alternatives considered**:

- mark all must-fix findings as hard blocks with no approval path
- auto-complete the review and leave disposition implicit in chat
- rewrite artifacts in place after approval

**Consequences**:

- Canon preserves explicit human accountability for risky review outcomes
- the first review slice stays resumable and auditable
- disposition handling stays separate from artifact generation, matching the
  constitution’s generation-versus-validation split

## D-015: Keep the local toolchain lean and stage platform support in CI

**Context**: Canon claims support across macOS, Linux, and Windows, but forcing
every contributor to install every target locally would add setup cost without
improving day-to-day governability.

**Decision**: keep `rust-toolchain.toml` lean for contributors and stage the
additional Apple and Windows targets in `.github/workflows/ci.yml` as smoke
builds. Keep Linux ARM and Windows ARM explicitly out of scope for the current
workflow until they have dedicated verification time.

**Alternatives considered**:

- add every promised target to the local default toolchain now
- claim only the host platform until a release pipeline exists
- imply full target coverage in documentation without CI evidence

**Consequences**:

- local contributor setup stays small and deterministic
- platform claims are backed by committed CI behavior instead of aspiration
- unverified ARM targets remain visible future work rather than accidental
  marketing drift

## D-016: Close v0.1 around three deep modes and explicit verification debt

**Context**: after `requirements`, `brownfield-change`, and `pr-review` were
implemented, the remaining choice was whether to keep expanding runtime depth
or close v0.1 around a smaller but governable surface.

**Decision**: close the v0.1 milestone around the three deep modes, typed
coverage for the remaining nine modes, committed CI and repository gates, and
honest documentation that `verify` remains unimplemented.

**Alternatives considered**:

- continue expanding runtime depth before tightening CI and public guidance
- ship with ambiguous docs that imply the whole mode surface is executable
- delay milestone closeout until every planned command exists

**Consequences**:

- Canon now has a coherent, auditable v0.1 boundary
- users can adopt the working deep modes without being misled about the
  remaining surface area
- the next implementation cycle can focus on new capability rather than
  re-stabilizing the foundation
- brownfield and review work can build on a real run bundle instead of more
  scaffolding
- verification-layer persistence remains an explicit follow-up, not an implied
  completion claim

## D-013: Brownfield approval and resume happen before mutating adapter work

**Context**: the next executable slice after `requirements` needed to prove
that Canon can block unsafe brownfield work, require named approval for
systemic risk, and preserve run provenance across blocked or resumed states.
The adapter layer still has open foundational tasks around trace emission and
shell capability separation.

**Decision**: implement brownfield artifact generation, preservation gates,
approval persistence, semantic exit codes, and stale-context `resume`
re-evaluation before finishing recommendation-only mutating adapter
enforcement.

**Alternatives considered**:

- finish adapter tracing and mutating enforcement before any brownfield user
  story work
- postpone approval and resume until after PR review mode
- treat blocked brownfield runs as generic failures with no semantic exit codes

**Consequences**:

- Canon now has a real brownfield control path with distinct `Blocked`,
  `AwaitingApproval`, and `Completed` outcomes
- approval records are durable run artifacts instead of conversational state
- `resume` now protects against stale input context and can re-evaluate edited
  artifact bundles without inventing a new run
- the recommendation-only adapter groundwork can now be finished on top of this
  brownfield control path instead of inventing separate execution semantics

## D-014: Local policy overrides stay typed and filesystem traces become evidence

**Context**: the remaining foundational gaps after the brownfield slice were
policy override resolution, filesystem trace persistence, and a concrete shell
adapter boundary between read-only and mutating execution.

**Decision**: merge repository-local policy overrides only through known typed
schemas, reject unknown override fields fail-closed, persist adapter
invocations as JSONL trace streams under `.canon/traces/`, and classify denied
mutating shell work as `RecommendationOnly` at the dispatcher level.

**Alternatives considered**:

- keep policy loading fixed to materialized defaults only
- record no adapter traces until CI or PR review work needs them
- let shell execution remain a generic untyped wrapper and infer mutability from
  command strings later

**Consequences**:

- `policy_root` can now adjust risk, zone, gate, and adapter rules without
  inventing a runtime DSL
- override files with unknown fields now fail closed instead of silently
  drifting governance
- run traces become durable evidence linked from `links.toml`, not hidden debug
  output
- the adapter layer now has enough structure to support `T051`, so the next
  pending dependency-ordered work moves to the PR review story
