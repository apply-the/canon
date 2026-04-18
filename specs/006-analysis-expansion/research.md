# Research: Analysis Mode Expansion

**Feature**: 006-analysis-expansion  
**Date**: 2026-04-13  
**Status**: Complete

## Research Tasks

### RT-001: Artifact Name Alignment

**Context**: Three sources define artifact names for discovery, system-shaping, and architecture, and they diverge.

| Source | Discovery Artifacts | System-Shaping Artifacts | Architecture Artifacts |
|--------|-------------------|---------------------|----------------------|
| `spec.md` (reviewed, authoritative) | problem-map.md, unknowns-and-assumptions.md, context-boundary.md, exploration-options.md, decision-pressure-points.md | system-shape.md, architecture-outline.md, capability-map.md, delivery-options.md, risk-hotspots.md | architecture-decisions.md, invariants.md, tradeoff-matrix.md, boundary-map.md, readiness-assessment.md |
| `defaults/methods/*.toml` (contract-only) | discovery-brief.md, assumptions-register.md, evidence-log.md, unknowns-register.md, discovery-summary.md | system-intent.md, domain-map.md, architecture-options.md, boundary-decisions.md, delivery-plan.md | invariants.md, boundary-map.md, architecture-options.md, tradeoffs.md, decision-record.md, risk-memo.md |
| `domain/mode.rs` artifact_families (display) | "discovery brief", "assumptions register", "evidence log", "unknowns register", "discovery summary" | "system intent", "domain map", "architecture options", "boundary decisions", "delivery plan" | "invariants", "boundary map", "architecture options", "tradeoffs", "decision record", "risk memo" |

**Decision**: The spec artifact names are authoritative. They were reviewed and approved. The methods TOML files and `mode.rs` artifact_families were written during the earlier contract-only modeling phase with placeholder names. Implementation must update all three sources to the spec's artifact names.

**Rationale**: The spec names are more descriptive and domain-precise (e.g., `problem-map.md` vs `discovery-brief.md`; `tradeoff-matrix.md` vs `tradeoffs.md`).

**Alternatives considered**: Using the TOML names as canonical — rejected because the spec went through adversarial review and the TOML names are explicitly `contract-only` placeholders.

---

### RT-002: Orchestration Commonality Across Analysis Modes

**Context**: The spec mandates reusing the existing runtime shape (context → generation → critique → gates → evidence → artifacts → inspection). The question is how much orchestration code can be shared.

**Findings**:

All three existing full-depth modes (`requirements`, `brownfield-change`, `pr-review`) share a common structure:
1. UUID-based run_id creation
2. Artifact contract loading + verification layer application
3. Input fingerprinting
4. Context capture invocation (ReadRepository via Filesystem adapter)
5. Policy decision evaluation
6. Generation invocation (GenerateContent via CopilotCli adapter)  
7. Critique invocation (CritiqueContent via CopilotCli adapter)
8. Generation/Validation path construction
9. Artifact rendering from evidence
10. Gate evaluation
11. State derivation from gates
12. Evidence bundle assembly
13. Persistence

Steps 1-4, 8-9, 11-13 are structurally identical across all three modes. Steps 5-7 differ only in capability, scope, and policy evaluation context. Step 10 differs per mode (different gate functions).

**Decision**: Discovery, system-shaping, and architecture should follow the same 13-step shape. Each mode needs:
- A mode-specific `run_*()` method in `service.rs` (matching existing pattern)
- Mode-specific artifact rendering functions
- Mode-specific gate evaluation functions in `gatekeeper.rs`
- Mode-specific constants (STEP_SEQUENCE, REQUIRED_GATES, GOVERNED_CAPABILITIES) in their mode files

A shared analysis orchestration helper is not warranted at this point — three modes is not enough to justify the abstraction, and the existing pattern of per-mode methods is understood and tested.

**Alternatives considered**: Extracting a generic `run_analysis_mode()` helper — rejected because the existing codebase successfully uses per-mode methods and extracting a shared helper would be a refactor that exceeds the scope of this feature.

---

### RT-003: Gate Evaluation Strategy for Analysis Modes

**Context**: The spec requires Exploration, Risk, and ReleaseReadiness gates for discovery, and adds Architecture for system-shaping and architecture modes. The existing gatekeeper has dedicated evaluation functions per mode. The question is how to structure gate evaluation for the new modes.

**Findings**:

Existing gate functions:
- `evaluate_requirements_gates()` → [Exploration, Risk, ReleaseReadiness]
- `evaluate_brownfield_gates()` → [Exploration, BrownfieldPreservation, Architecture, Risk, ReleaseReadiness]
- `evaluate_pr_review_gates()` → [Risk, Architecture, ReviewDisposition, ReleaseReadiness]

Each uses a mode-specific context struct (e.g., `BrownfieldGateContext`, `PrReviewGateContext`) and evaluates gates against mode-specific artifact expectations.

The gates themselves share evaluation primitives:
- **Exploration gate**: checks for a specific "boundary" artifact presence
- **Architecture gate**: `named_artifact_gate` checking for architecture-related artifacts
- **Risk gate**: checks owner, risk class, zone, approval state
- **ReleaseReadiness gate**: validates complete artifact bundle, evidence completeness, denied invocations

**Decision**: Create three new evaluation functions:
- `evaluate_discovery_gates()` → [Exploration, Risk, ReleaseReadiness]
- `evaluate_greenfield_gates()` → [Exploration, Architecture, Risk, ReleaseReadiness]  
- `evaluate_architecture_gates()` → [Exploration, Architecture, Risk, ReleaseReadiness]

Each with a mode-specific context struct carrying the relevant evaluation state.

**Exploration gate specificity**: Per the spec, the discovery Exploration gate must verify that the *problem domain* is bounded, not that *solutions* are bounded. This is semantically different from requirements exploration, which checks for a well-framed problem statement. Discovery's Exploration gate checks `problem-map.md` and `context-boundary.md` presence and content.

**Rationale**: This mirrors the existing pattern exactly. Shared primitives (named_artifact_gate, risk evaluation, release readiness checking) are already reusable functions.

**Alternatives considered**: A single `evaluate_analysis_gates(mode, ...)` dispatcher — rejected because it would mask mode-specific gate semantics behind a generic interface.

---

### RT-004: Evidence Contract per Mode (Resolves OQ-001)

**Context**: OQ-001 asks which evidence sources are mandatory versus optional per mode.

**Findings from existing modes**:

All three full-depth modes require:
- **Mandatory**: Context capture evidence (input fingerprints, context attempt)
- **Mandatory**: Generation evidence (generation path with request IDs and derived artifacts)
- **Mandatory**: Gate evaluation results persisted to `gates/` directory
- **Mandatory**: Artifact contract persisted to `artifact-contract.toml`
- **Mode-dependent**: Critique/challenge evidence (mandatory for requirements and pr-review, mandatory for brownfield mutation decisions)

**Decision — minimum evidence contract**:

| Evidence Source | Discovery | System-Shaping | Architecture |
|-----------------|-----------|------------|--------------|
| Context capture (ReadRepository) | Mandatory | Mandatory | Mandatory |
| Generation (GenerateContent) | Mandatory | Mandatory | Mandatory |
| Critique (CritiqueContent) | Optional | Mandatory | Mandatory |
| Gate evaluations | Mandatory | Mandatory | Mandatory |
| Artifact contract | Mandatory | Mandatory | Mandatory |
| Input fingerprints | Mandatory | Mandatory | Mandatory |

**Rationale**: 
- Discovery is exploratory by nature; critique is valuable but not structurally required for problem exploration. The generation path already produces bounded analysis.
- System-shaping and architecture make consequential structural claims (per D-002, D-003 in the spec) and therefore must include governed critique evidence.

**Alternatives considered**: Making critique mandatory everywhere — rejected because discovery's purpose is to explore unknowns, not to defend claims. Forcing critique on problem exploration adds ceremony without proportional value.

---

### RT-005: Approval Thresholds for Analysis Modes (Resolves OQ-002)

**Context**: OQ-002 asks whether explicit approvals apply only to systemic-impact analysis runs or also to some bounded-impact cases.

**Findings from existing policy**:

The current `risk.toml` and `gatekeeper.rs` policy is:
- `SystemicImpact` OR `Red` zone → `NeedsApproval` on Risk gate
- `BoundedImpact` in `Yellow` or `Green` → `Passed` (no approval required)
- `LowImpact` → `Passed`

The `block_mutation_for_red_or_systemic` flag in PolicySet controls mutation gating but is irrelevant for analysis-only modes.

**Decision**: Analysis modes inherit the existing approval policy without modification:
- **SystemicImpact or Red zone**: Explicit approval required before artifacts are considered final (Risk gate → NeedsApproval)
- **BoundedImpact**: No approval required for analysis-only runs
- **Architecture mode exception**: The spec says "formal sign-off required for systemic-impact or red-zone architectural runs" — this is already satisfied by the existing Risk gate policy. No additional approval gate is needed.

**Rationale**: The existing policy already correctly distinguishes risk levels. Analysis modes do not mutate code, so the mutation-blocking policy is irrelevant. Adding a separate approval gate for bounded-impact architecture runs would over-govern analysis work without clear benefit.

**Alternatives considered**: Adding a mode-specific approval gate for architecture at bounded-impact — rejected because the architecture gate already blocks on missing structural evidence, and adding approval friction to bounded-impact analysis contradicts progressive autonomy (Constitution Principle IX).

---

### RT-006: Critique/Challenge Mechanical Differences

**Context**: D-002 in the spec says architecture challenge focuses on invariants and structural boundary preservation. The question is how this differs mechanically from requirements critique.

**Findings**:

In `service.rs`, the critique invocation is structured as:
```
CritiqueContent capability → CopilotCli adapter → critique attempt → evidence
```

The critique scope and summary differ per mode (requirements critique evaluates completeness; brownfield critique evaluates preservation), but the mechanical flow is identical. The *content* of the critique prompt differs, not the *orchestration*.

**Decision**: All three analysis modes use the same `CritiqueContent` capability through the same adapter pipeline. The mode-specific differentiation is in:
1. The `requested_scope` field of the `InvocationRequest` (what to critique)
2. The `summary` field (what was critiqued)
3. The artifact rendering function (how critique evidence feeds artifacts)

No mechanical changes to the critique pipeline are needed.

**Rationale**: The existing critique pipeline is mode-agnostic by design. Mode-specific behavior is encoded in the request scope and artifact rendering, not in the orchestration.

---

### RT-007: Codex Skill Transition

**Context**: FR-006 requires transitioning embedded skills from modeled-only to runnable.

**Findings from existing skills**:

- Runnable skills (requirements, brownfield-change, pr-review) call `canon run --mode {mode}` and handle the full run/approve/resume/inspect lifecycle
- Modeled-only skills (discovery, system-shaping, architecture) explain the mode purpose and recommend using runnable alternatives. They have `visibility: discoverable-standard`.
- The `canon-shared/` embedded skill contains common helper functions used across all runnable skills

**Decision**: After runtime support is implemented, the embedded skills under `defaults/embedded-skills/{canon-discovery,canon-system-shaping,canon-architecture}/` must be rewritten from disclosure-only scripts to runnable wrappers following the pattern established by `canon-requirements`. The Codex-facing skills under `.agents/skills/` must also be updated to reflect the new `full` support state.

**Rationale**: The skill structure and conventions are already established. This is a content update, not a structural change.

---

### RT-008: Methods TOML Updates

**Context**: The `defaults/methods/*.toml` files for discovery, system-shaping, and architecture currently declare `implementation_depth = "contract-only"` and use placeholder artifact names.

**Decision**: These files must be updated to:
1. Set `implementation_depth = "full"`
2. Replace artifact names with the spec-authoritative names
3. Keep version, mode, emphasis, and gate_profile consistent with the spec

**Rationale**: These files are the declarative source of truth for method metadata. They must be accurate once the modes are runnable.
