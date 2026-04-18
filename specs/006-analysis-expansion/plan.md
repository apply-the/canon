# Implementation Plan: Analysis Mode Expansion

**Branch**: `006-analysis-expansion` | **Date**: 2026-04-13 | **Spec**: [spec.md](specs/006-analysis-expansion/spec.md)  
**Input**: Feature specification from `specs/006-analysis-expansion/spec.md`

## Summary

Promote `discovery`, `system-shaping`, and `architecture` from contract-only stubs
to full governed runtime depth. Each mode gets a concrete artifact contract,
mode-specific gate evaluation, per-mode orchestration, evidence persistence,
and inspection surface — all by reusing Canon's existing common runtime flow
and extending the same infrastructure that already powers `requirements`,
`brownfield-change`, and `pr-review`.

## 1. Governance Context

**Execution Mode Set**: discovery, system-shaping, architecture  
**Risk Classification**: bounded-impact — additive extension of existing
runtime patterns; no mutation of external systems, no protocol work, no
redesign of the governance model.

**Scope In**:
- Full governed depth for `discovery`, `system-shaping`, `architecture` modes
- Artifact contracts in `contract.rs`
- Gate evaluation functions in `gatekeeper.rs`
- Mode-specific orchestration methods in `service.rs`
- Mode constant definitions (STEP_SEQUENCE, REQUIRED_GATES, GOVERNED_CAPABILITIES)
- ModeProfile updates in `mode.rs` (`ContractOnly` → `Full`, artifact name alignment)
- Methods TOML updates (`implementation_depth`, artifact names)
- Embedded skill rewrites (modeled-only → runnable)
- Integration and contract tests for all three modes

**Scope Out**:
- Code mutation workflows (implementation, refactor)
- Review or verification modes
- Incident or migration modes
- MCP protocol or distribution work
- Codex skills taxonomy structural changes
- Shared `run_analysis_mode()` abstraction (see PD-002)
- Cross-run artifact linking

**Invariants**:
- These modes MUST remain analysis-heavy and non-mutation-first
- They MUST reuse Canon's existing governance model (no second orchestration path)
- They MUST produce artifacts grounded in bounded context, not deterministic placeholder generation
- Decisions and validation evidence MUST persist under `.canon/artifacts/` and `.canon/runs/`
- Existing `requirements`, `brownfield-change`, and `pr-review` behavior MUST NOT regress

**Decision Log**: `specs/006-analysis-expansion/decision-log.md`  
**Validation Ownership**: Implementation generates code; tests, clippy, and
manual review validate independently  
**Approval Gates**: None required for bounded-impact. All work proceeds under
standard review.

## 2. Technical Context

**Language/Version**: Rust 1.94.1, Edition 2024  
**Primary Dependencies**: clap, serde, serde_json, serde_yaml, toml, thiserror,
tracing, tracing-subscriber, uuid, time  
**Storage**: Local filesystem under `.canon/` (TOML for manifests, Markdown for
artifacts, JSONL for traces)  
**Testing**: `cargo test`, `cargo nextest run`, `cargo clippy --workspace
--all-targets --all-features -- -D warnings`, `cargo fmt --check`  
**Target Platform**: macOS, Linux (CLI binary)  
**Project Type**: CLI + engine library (workspace: canon-cli, canon-engine,
canon-adapters)

**Existing System Touchpoints**:

| File | Change Type | Impact |
|------|-------------|--------|
| `crates/canon-engine/src/artifacts/contract.rs` | Add 3 match arms | New artifact contracts |
| `crates/canon-engine/src/orchestrator/service.rs` | Add 3 run methods + 3 dispatch arms | Core orchestration |
| `crates/canon-engine/src/orchestrator/gatekeeper.rs` | Add 3 eval functions + 3 context structs | Gate evaluation |
| `crates/canon-engine/src/modes/discovery.rs` | Expand from stub | Constants |
| `crates/canon-engine/src/modes/greenfield.rs` | Expand from stub | Constants |
| `crates/canon-engine/src/modes/architecture.rs` | Expand from stub | Constants |
| `crates/canon-engine/src/domain/mode.rs` | Update 3 ModeProfiles | Depth + names |
| `defaults/methods/discovery.toml` | Update fields | Method metadata |
| `defaults/methods/system-shaping.toml` | Update fields | Method metadata |
| `defaults/methods/architecture.toml` | Update fields | Method metadata |
| `defaults/embedded-skills/canon-discovery/` | Rewrite | Skill content |
| `defaults/embedded-skills/canon-system-shaping/` | Rewrite | Skill content |
| `defaults/embedded-skills/canon-architecture/` | Rewrite | Skill content |
| `tests/` | Add integration + contract tests | Test coverage |

**Performance Goals**: N/A — CLI tool; existing performance profile is adequate  
**Constraints**: Must pass `cargo deny check licenses advisories bans sources`  
**Scale/Scope**: 3 modes, 15 artifacts, ~3 new gate evaluation functions,
~3 new orchestration methods

## 3. Constitution Check

*GATE: Passed pre-research. Re-checked post-design.*

- [x] Execution mode set is declared (`discovery`, `system-shaping`, `architecture`) and matches the work
- [x] Risk classification is explicit (`bounded-impact`) and autonomy is appropriate
- [x] Scope boundaries and exclusions are recorded (see Governance Context)
- [x] Invariants are explicit before implementation (5 invariants declared)
- [x] Required artifacts and owners are identified (see data-model.md)
- [x] Decision logging is planned and linked to `specs/006-analysis-expansion/decision-log.md`
- [x] Validation plan separates generation from validation (see validation-report.md)
- [x] Declared-risk approval checkpoints are named (none required for bounded-impact)
- [x] No constitution deviations to document

## 4. Current Baseline and Gap

### What Exists Today

| Component | Discovery | System-Shaping | Architecture |
|-----------|-----------|------------|--------------|
| Mode enum variant | ✅ | ✅ | ✅ |
| `FromStr` parsing | ✅ | ✅ | ✅ |
| ModeProfile in `all_mode_profiles()` | ✅ (ContractOnly) | ✅ (ContractOnly) | ✅ (ContractOnly) |
| Methods TOML | ✅ (contract-only) | ✅ (contract-only) | ✅ (contract-only) |
| Mode file (constants) | Stub (MODE_FILE only) | Stub (MODE_FILE only) | Stub (MODE_FILE only) |
| Artifact contract in `contract_for_mode()` | ❌ (falls through to generic) | ❌ (falls through to generic) | ❌ (falls through to generic) |
| Orchestration method in `service.rs` | ❌ (`UnsupportedMode` error) | ❌ (`UnsupportedMode` error) | ❌ (`UnsupportedMode` error) |
| Gate evaluation function | ❌ | ❌ | ❌ |
| Embedded skill | ✅ (modeled-only) | ✅ (modeled-only) | ✅ (modeled-only) |
| Integration tests | ❌ | ❌ | ❌ |

### Artifact Name Alignment Gap

The existing methods TOML files and mode.rs `artifact_families` use placeholder
names from the contract-only modeling phase. The reviewed spec defines
authoritative artifact names. See `research.md` RT-001 for the full comparison
table. All three sources must converge on the spec names.

### What Must Be Built

1. **Mode constants**: STEP_SEQUENCE, REQUIRED_GATES, GOVERNED_CAPABILITIES for each mode
2. **Artifact contracts**: 15 `ArtifactRequirement` entries across 3 modes in `contract_for_mode()`
3. **Gate evaluation**: 3 evaluation functions + 3 context structs in `gatekeeper.rs`
4. **Orchestration**: 3 `run_*()` methods in `service.rs` + dispatch arms
5. **Artifact rendering**: 3 rendering functions for evidence-backed artifact content
6. **Metadata alignment**: ModeProfile depth + artifact_families, methods TOML
7. **Skill transition**: 3 embedded skill rewrites from modeled-only to runnable
8. **Tests**: Integration runs, contract tests, gate evaluation tests

## 5. Runtime Integration Strategy

### Orchestration Shape

All three modes follow the same common runtime flow used by existing full-depth
modes (see research.md RT-002):

```
1. Create UUID-based run_id
2. Load artifact contract + apply verification layers
3. Capture input fingerprints
4. Context capture invocation (ReadRepository → Filesystem adapter)
5. Policy decision on context
6. Read context from inputs
7. Build context attempt
8. Generation invocation (GenerateContent → CopilotCli adapter)
9. Policy decision on generation
10. Execute generation → build generation attempt
11. Critique invocation (CritiqueContent → CopilotCli adapter)  [*]
12. Build critique attempt  [*]
13. Build GenerationPath + ValidationPath
14. Render mode-specific artifacts from evidence
15. Evaluate mode-specific gates
16. Derive run state from gates
17. Create verification records
18. Build EvidenceBundle
19. Build and persist PersistedRunBundle
20. Return RunSummary
```

`[*]` Steps 11-12 are optional for discovery (see PD-003).

### Dispatch Integration

The `run()` method in `service.rs` adds three new match arms:

```rust
match request.mode {
    Mode::Requirements => self.run_requirements(&store, request, policy_set),
    Mode::BrownfieldChange => self.run_brownfield_change(&store, request, policy_set),
    Mode::PrReview => self.run_pr_review(&store, request, policy_set),
    Mode::Discovery => self.run_discovery(&store, request, policy_set),
    Mode::Greenfield => self.run_greenfield(&store, request, policy_set),
    Mode::Architecture => self.run_architecture(&store, request, policy_set),
    other => Err(EngineError::UnsupportedMode(other.as_str().to_string())),
}
```

### Adapter Usage

All three modes use the same adapter set as requirements:
- `Filesystem` adapter for `ReadRepository`
- `CopilotCli` adapter for `GenerateContent` and `CritiqueContent`
- `Shell` adapter is not needed (no mutation, no diff inspection)

## 6. Mode-by-Mode Delivery Design

### Discovery

**Purpose**: Explore unknowns without turning exploration into solution drift.

**Step Sequence**: `capture-context → classify-risk → govern-repository-context
→ explore-problem-domain → challenge-assumptions → emit-artifacts →
evaluate-gates`

**Invocations**:
1. **Context** (ReadRepository): Read raw problem statements, initial research,
   user quotes from inputs
2. **Generation** (GenerateContent): Produce exploration analysis — problem
   mapping, unknowns identification, boundary derivation
3. **Critique** (CritiqueContent): *Optional* — challenge premature commitments
   to solutions, hunt for hidden unknowns

**Artifacts** (5):

| File | Required Sections | Gate Bindings |
|------|-------------------|---------------|
| `problem-map.md` | Summary, Problem Domain, Boundaries, Unknowns | Exploration, Risk |
| `unknowns-and-assumptions.md` | Summary, Unknowns, Assumptions, Confidence Levels | Exploration, Risk |
| `context-boundary.md` | Summary, In-Scope Context, Out-of-Scope Context | Exploration, ReleaseReadiness |
| `exploration-options.md` | Summary, Options, Constraints, Recommended Direction | Exploration, Risk |
| `decision-pressure-points.md` | Summary, Pressure Points, Open Questions | Risk, ReleaseReadiness |

**Gates**: Exploration, Risk, ReleaseReadiness

**Exploration Gate Specificity**: Checks that the *problem domain* is bounded
(presence and content of `problem-map.md` and `context-boundary.md`), not that
solutions are bounded.

---

### System-Shaping

**Purpose**: Define a new capability from bounded intent through early delivery
structure.

**Step Sequence**: `capture-context → classify-risk → govern-repository-context
→ shape-system-boundaries → critique-architecture-options → emit-artifacts →
evaluate-gates`

**Invocations**:
1. **Context** (ReadRepository): Read discovery briefs, requirements
   constraints, bounded problem framing from inputs
2. **Generation** (GenerateContent): Produce system shaping analysis —
   boundaries, architecture outline, capability mapping, delivery phasing
3. **Critique** (CritiqueContent): *Mandatory* — challenge overly tight
   coupling, overly ambitious delivery slices, unmitigated risk hotspots

**Artifacts** (5):

| File | Required Sections | Gate Bindings |
|------|-------------------|---------------|
| `system-shape.md` | Summary, System Shape, Boundary Decisions, Domain Responsibilities | Exploration, Architecture |
| `architecture-outline.md` | Summary, Structural Options, Selected Boundaries, Rationale | Architecture, Risk |
| `capability-map.md` | Summary, Capabilities, Dependencies, Gaps | Exploration, Architecture |
| `delivery-options.md` | Summary, Delivery Phases, Sequencing Rationale, Risk per Phase | Architecture, ReleaseReadiness |
| `risk-hotspots.md` | Summary, Hotspots, Mitigation Status, Unresolved Risks | Risk, ReleaseReadiness |

**Gates**: Exploration, Architecture, Risk, ReleaseReadiness

**Architecture Gate**: Checks `system-shape.md`, `architecture-outline.md`,
`capability-map.md` presence and section completeness.

---

### Architecture

**Purpose**: Evaluate boundaries, invariants, and structural decisions heavily.

**Step Sequence**: `capture-context → classify-risk → govern-repository-context
→ evaluate-structural-options → challenge-architectural-claims →
emit-artifacts → evaluate-gates`

**Invocations**:
1. **Context** (ReadRepository): Read current system baselines, system-shaping
   options, or specific technical dilemmas from inputs
2. **Generation** (GenerateContent): Produce structural evaluation — decision
   analysis, invariant extraction, tradeoff scoring
3. **Critique** (CritiqueContent): *Mandatory* — severe governed challenge for
   consequential structural claims and boundary crossing

**Artifacts** (5):

| File | Required Sections | Gate Bindings |
|------|-------------------|---------------|
| `architecture-decisions.md` | Summary, Decisions, Tradeoffs, Consequences, Unresolved Questions | Architecture, Risk |
| `invariants.md` | Summary, Invariants, Rationale, Verification Hooks | Architecture, ReleaseReadiness |
| `tradeoff-matrix.md` | Summary, Options, Evaluation Criteria, Scores, Selected Option | Architecture, Risk |
| `boundary-map.md` | Summary, Boundaries, Ownership, Crossing Rules | Exploration, Architecture |
| `readiness-assessment.md` | Summary, Readiness Status, Blockers, Accepted Risks | Risk, ReleaseReadiness |

**Gates**: Exploration, Architecture, Risk, ReleaseReadiness

**Architecture Gate**: Checks `architecture-decisions.md`, `invariants.md`,
`tradeoff-matrix.md` presence and section completeness. Severity is higher than
system-shaping's architecture gate — this mode's entire purpose is structural
evaluation.

**Approval**: Systemic-impact or red-zone architecture runs require explicit
approval before artifacts are considered final (inherited from existing Risk
gate policy).

## 7. Artifact Contract Strategy

### Contract Structure

Each mode's contract follows the existing `ArtifactContract` structure:

```rust
ArtifactContract {
    version: 1,
    artifact_requirements: vec![...],  // 5 per mode
    required_verification_layers: vec![...],  // from policy
}
```

Each `ArtifactRequirement` uses the `requirement()` helper:

```rust
requirement("problem-map.md", &["Summary", "Problem Domain", "Boundaries", "Unknowns"], &[Exploration, Risk])
```

### Validation

Existing `validate_artifact()` and `validate_release_bundle()` functions work
without modification. They check:
- File name presence in the bundle
- Required section headers present in content
- All artifact requirements satisfied

### Artifact Rendering

Each mode needs a rendering function that transforms evidence into artifact
content. These functions:
- Accept the artifact file name and evidence state
- Produce Markdown content with the required sections populated from evidence
- Follow the pattern of existing `render_requirements_artifact_from_evidence()`
  and `render_brownfield_artifact()`

The rendering functions must produce content *derived from supplied context and
evidence*, not generic template content (per SC-003).

## 8. Gate Reuse and Evaluation Strategy

### Gate Primitives

Existing reusable primitives:

| Primitive | Used By | Reusable? |
|-----------|---------|-----------|
| Named artifact gate (checks artifact presence + sections) | Brownfield, PR Review | ✅ Yes |
| Risk gate (owner, risk class, zone, approval state) | Requirements, Brownfield, PR Review | ✅ Yes |
| ReleaseReadiness gate (bundle validation, evidence, denials) | All modes | ✅ Yes |
| Exploration gate (boundary artifact check) | Requirements | ✅ Partially — discovery needs different target artifacts |

### New Gate Evaluation Functions

**`evaluate_discovery_gates(contract, artifacts, context: DiscoveryGateContext) -> Vec<GateEvaluation>`**
- Exploration: check `problem-map.md` + `context-boundary.md` for problem domain boundedness
- Risk: reuse existing risk gate logic (owner, risk class, zone, approvals)
- ReleaseReadiness: reuse existing release readiness logic

**`evaluate_greenfield_gates(contract, artifacts, context: GreenfieldGateContext) -> Vec<GateEvaluation>`**
- Exploration: check `system-shape.md` + `capability-map.md` for bounded intent
- Architecture: named artifact gate on `system-shape.md`, `architecture-outline.md`, `capability-map.md`
- Risk: reuse existing risk gate logic
- ReleaseReadiness: reuse existing release readiness logic

**`evaluate_architecture_gates(contract, artifacts, context: ArchitectureGateContext) -> Vec<GateEvaluation>`**
- Exploration: check `boundary-map.md` for bounded context
- Architecture: named artifact gate on `architecture-decisions.md`, `invariants.md`, `tradeoff-matrix.md`
- Risk: reuse existing risk gate logic
- ReleaseReadiness: reuse existing release readiness logic

### Gate Context Structs

All three context structs share the same shape (see data-model.md). This is
intentional — analysis modes have no mutation or preservation state to track.

## 9. Critique/Challenge Strategy

### Mechanical Pattern

All three modes use the same `CritiqueContent` capability through the `CopilotCli`
adapter. Mode differentiation is in:

1. **`requested_scope`** on the `InvocationRequest` — what to critique
2. **`summary`** — what was critiqued
3. **Artifact rendering** — how critique evidence feeds the final artifacts

No changes to the critique pipeline are needed (research.md RT-006).

### Mode-Specific Challenge Focus

| Mode | Critique Focus | Intensity |
|------|---------------|-----------|
| Discovery | Premature commitments to solutions; hidden unknowns | Moderate — exploratory |
| System-Shaping | Overly tight coupling; ambitious delivery slices; unmitigated risk | High — structural claims |
| Architecture | Invariant violations; boundary crossings; consequential structural claims | Severe — the mode's purpose |

### Critique Optionality

- **Discovery**: Critique is optional. The `run_discovery()` method may skip
  steps 11-12 if the generation already produces bounded exploration. If
  critique is skipped, the evidence bundle omits the validation path for
  critique but still records the generation path.
- **System-Shaping**: Critique is mandatory. The `run_greenfield()` method always
  executes the critique invocation.
- **Architecture**: Critique is mandatory. The `run_architecture()` method
  always executes the critique invocation with the highest challenge intensity.

## 10. Evidence and Persistence Strategy

### Evidence Bundle Structure

Each mode's evidence bundle follows the existing `EvidenceBundle` structure:

```
EvidenceBundle {
    run_id,
    generation_paths: [GenerationPath { request_ids, lineage: [AiVendorFamily], derived_artifacts }],
    validation_paths: [ValidationPath { request_ids, lineage, verification_refs, independence }],
    denied_invocations: [...],
    trace_refs: ["{run_id}.jsonl"],
    artifact_refs: ["artifacts/{run_id}/{mode}/{filename}", ...],
    decision_refs: ["runs/{run_id}/invocations/{request_id}/decision.toml", ...],
    approval_refs: [...],
}
```

### Minimum Evidence Contract (Resolves OQ-001)

| Evidence Source | Discovery | System-Shaping | Architecture |
|-----------------|-----------|------------|--------------|
| Context capture (ReadRepository) | Mandatory | Mandatory | Mandatory |
| Generation (GenerateContent) | Mandatory | Mandatory | Mandatory |
| Critique (CritiqueContent) | Optional | Mandatory | Mandatory |
| Gate evaluations | Mandatory | Mandatory | Mandatory |
| Artifact contract | Mandatory | Mandatory | Mandatory |
| Input fingerprints | Mandatory | Mandatory | Mandatory |

**Rationale**: Discovery explores unknowns — critique adds value but is not
structurally required. System-shaping and architecture make consequential structural
claims that must be challenged (spec D-002, D-003).

### Persistence Layout

```
.canon/
├── runs/{run_id}/
│   ├── run.toml
│   ├── context.toml
│   ├── state.toml
│   ├── artifact-contract.toml
│   ├── links.toml
│   ├── evidence.toml
│   ├── gates/
│   │   ├── exploration.toml
│   │   ├── architecture.toml       (system-shaping, architecture only)
│   │   ├── risk.toml
│   │   └── release-readiness.toml
│   ├── verification/
│   │   └── verification-00.toml
│   ├── invocations/{request_id}/
│   │   ├── decision.toml
│   │   └── attempt-01.toml
│   └── approvals/                   (if systemic-impact)
├── artifacts/{run_id}/{mode}/
│   └── {5 mode-specific .md files}
└── traces/{run_id}.jsonl
```

No new persistence types are needed. All existing `PersistedRunBundle`,
`RunManifest`, `RunStateManifest`, `LinkManifest` structures work as-is.

## 11. Approval Policy for Analysis Modes (Resolves OQ-002)

**Decision**: Analysis modes inherit the existing approval policy without
modification.

| Risk Classification | Zone | Approval Required? |
|---------------------|------|--------------------|
| LowImpact | Any | No |
| BoundedImpact | Green/Yellow | No |
| BoundedImpact | Red | Yes (Risk gate → NeedsApproval) |
| SystemicImpact | Any | Yes (Risk gate → NeedsApproval) |

**Architecture mode**: The spec says "formal sign-off required for
systemic-impact or red-zone architectural runs." This is already satisfied by
the existing Risk gate policy — no additional approval gate is needed.

**Rationale**: Analysis modes do not mutate code. The existing policy already
correctly distinguishes risk levels. Adding a separate approval gate for
bounded-impact architecture would over-govern analysis and contradict
progressive autonomy (Constitution Principle IX).

## 12. Codex Skill Transition Plan

### Current State

Three embedded skills under `defaults/embedded-skills/` are modeled-only:
- `canon-discovery/skill-source.md` — discloses mode, recommends `canon-requirements`
- `canon-system-shaping/skill-source.md` — discloses mode, recommends `canon-requirements`
- `canon-architecture/skill-source.md` — discloses mode, recommends `canon-requirements`/`canon-brownfield`

Three Codex-facing skills under `.agents/skills/` mirror this modeled-only state.

### Target State

After runtime implementation:
1. **Embedded skills**: Rewrite `skill-source.md` for each mode to:
   - Remove modeled-only disclaimers
   - Add `canon run --mode {mode}` invocation
   - Follow the pattern of `canon-requirements/skill-source.md` (run → inspect → approve → resume lifecycle)
   - Set visibility to `supported-standard`

2. **Codex skills**: Update `.agents/skills/canon-{discovery,system-shaping,architecture}/SKILL.md` to:
   - Remove "not runnable end to end" language
   - Update description to reflect full support state
   - Add runnable workflow instructions

### Sequencing

Skill updates happen *after* runtime implementation is verified. An incorrect
skill claiming runnability before the runtime supports it violates the
"discoverable support-state skills MUST NOT fabricate Canon runs" invariant.

## 13. Validation and Test Strategy

### Test Files to Create

| Test File | Mode | Type | Validates |
|-----------|------|------|-----------|
| `tests/discovery_run.rs` | Discovery | Integration | End-to-end run emits 5 artifacts, gates persist |
| `tests/discovery_contract.rs` | Discovery | Contract | Artifact contracts match spec, gate evaluation |
| `tests/greenfield_run.rs` | System-Shaping | Integration | End-to-end run emits 5 artifacts, critique mandatory |
| `tests/greenfield_contract.rs` | System-Shaping | Contract | Artifact contracts, Architecture gate blocking |
| `tests/architecture_run.rs` | Architecture | Integration | End-to-end run emits 5 artifacts, challenge evidence |
| `tests/architecture_contract.rs` | Architecture | Contract | Artifact contracts, approval for systemic-impact |

### Test Pattern (following existing convention)

```
1. Initialize temp workspace (tempfile::tempdir)
2. Create input files (idea.md, constraints.md, etc.)
3. Run CLI command: canon run --mode {mode} --risk {risk} --zone {zone} --owner {owner} --input {input}
4. Assert exit code (0 for completed, 2 for blocked, 3 for awaiting-approval)
5. Assert run.toml, state.toml, artifact-contract.toml exist
6. Assert mode-specific gates persisted with expected status
7. Assert 5 artifacts exist with correct section headers
8. Call canon status --run {run_id} to verify state
```

### Validation Layers

- **Structural**: `cargo check`, `cargo clippy`, `cargo fmt --check`
- **Logical**: Integration tests (6 files), contract tests, existing test
  regression
- **Independent**: Manual review of generated artifacts against spec contracts
- **Consistency**: Artifact names match across contract.rs, mode.rs, methods
  TOML, and spec
- **Adversarial**: Gate blocking tests — verify modes block when evidence is
  insufficient

See `validation-report.md` for the complete validation matrix.

## 14. Risks and Complexity Tracking

### Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Artifact rendering produces generic content despite bounded evidence | Medium | High (violates SC-003) | Test against concrete problem inputs, not generic prompts |
| Gate evaluation shares too much code, masking mode-specific semantics | Low | Medium | Per-mode functions maintain mode clarity (PD-005) |
| Methods TOML / mode.rs / contract.rs artifact name drift | Low | Medium | Consistency check tests (CC-01, CC-02, CC-04) |
| Orchestration methods become very long | Medium | Low | Acceptable per PD-002; refactor if a 4th mode is added |
| Discovery critique skip path untested | Low | Medium | Dedicated test (LV-10) for critique-optional path |

### Complexity Tracking

No constitution violations to track. All gates pass.

## 15. Open Technical Questions (Resolved)

### OQ-001: Mandatory vs Optional Evidence per Mode — RESOLVED

See §10 Evidence and Persistence Strategy. Critique is optional for discovery,
mandatory for system-shaping and architecture. Decision recorded as PD-003 in the
decision log.

### OQ-002: Approval Thresholds for Analysis Modes — RESOLVED

See §11 Approval Policy for Analysis Modes. Analysis modes inherit the existing
Risk gate policy without modification. SystemicImpact or Red zone requires
approval; BoundedImpact does not. Decision recorded as PD-004 in the decision
log.

## Project Structure

### Documentation (this feature)

```text
specs/006-analysis-expansion/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── decision-log.md
├── validation-report.md
└── tasks.md                (to be generated)
```

### Source Code (repository root)

```text
crates/
├── canon-engine/
│   └── src/
│       ├── artifacts/
│       │   └── contract.rs          # +3 match arms (discovery, system-shaping, architecture)
│       ├── domain/
│       │   └── mode.rs              # Update 3 ModeProfiles
│       ├── modes/
│       │   ├── discovery.rs         # Expand: STEP_SEQUENCE, REQUIRED_GATES, GOVERNED_CAPABILITIES
│       │   ├── greenfield.rs        # Expand: STEP_SEQUENCE, REQUIRED_GATES, GOVERNED_CAPABILITIES
│       │   └── architecture.rs      # Expand: STEP_SEQUENCE, REQUIRED_GATES, GOVERNED_CAPABILITIES
│       └── orchestrator/
│           ├── service.rs           # +3 run methods + dispatch arms + rendering functions
│           └── gatekeeper.rs        # +3 eval functions + 3 context structs
└── canon-cli/
    └── src/                         # No changes needed — dispatch is mode-agnostic

defaults/
├── methods/
│   ├── discovery.toml               # Update: depth=full, artifact names
│   ├── system-shaping.toml          # Update: depth=full, artifact names
│   └── architecture.toml            # Update: depth=full, artifact names
└── embedded-skills/
    ├── canon-discovery/             # Rewrite: modeled-only → runnable
   ├── canon-system-shaping/        # Rewrite: modeled-only → runnable
    └── canon-architecture/          # Rewrite: modeled-only → runnable

tests/
├── discovery_run.rs                 # New: integration
├── discovery_contract.rs            # New: contract
├── greenfield_run.rs                # New: integration
├── greenfield_contract.rs           # New: contract
├── architecture_run.rs              # New: integration
└── architecture_contract.rs         # New: contract
```

**Structure Decision**: Existing workspace crate structure. No new crates, no
new modules, no new binary targets. All changes are additive within existing
files or new test files.
