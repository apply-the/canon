# Data Model: Analysis Mode Expansion

**Feature**: 006-analysis-expansion  
**Date**: 2026-04-13

## Entities

### 1. Existing Entities — Modifications Required

#### Mode (enum, `domain/mode.rs`)

No enum changes — `Discovery`, `Greenfield`, `Architecture` already exist internally. Public naming now exposes `system-shaping` for `Mode::Greenfield`.

**ModeProfile changes** (in `all_mode_profiles()`):

| Field | Discovery | System-Shaping | Architecture |
|-------|-----------|------------|--------------|
| `implementation_depth` | `ContractOnly` → `Full` | `ContractOnly` → `Full` | `ContractOnly` → `Full` |
| `artifact_families` | Update to spec names | Update to spec names | Update to spec names |

Discovery artifact_families:
- "problem map", "unknowns and assumptions", "context boundary", "exploration options", "decision pressure points"

System-Shaping artifact_families:
- "system shape", "architecture outline", "capability map", "delivery options", "risk hotspots"

Architecture artifact_families:
- "architecture decisions", "invariants", "tradeoff matrix", "boundary map", "readiness assessment"

#### ArtifactContract (struct, `artifacts/contract.rs`)

New match arms in `contract_for_mode()` for `Mode::Discovery`, `Mode::Greenfield`, `Mode::Architecture`.

Each returns `ArtifactContract { version: 1, artifact_requirements: [...], required_verification_layers: [...] }` with mode-specific `ArtifactRequirement` entries per the spec's Artifact Contracts tables.

No structural changes to `ArtifactContract`, `ArtifactRequirement`, or `ArtifactFormat`.

### 2. New Entities — Gate Evaluation Contexts

#### DiscoveryGateContext

```
DiscoveryGateContext {
    owner: &str,
    risk: RiskClass,
    zone: UsageZone,
    approvals: &[ApprovalRecord],
    evidence_complete: bool,
}
```

Used by `evaluate_discovery_gates()`. Simpler than brownfield context — no mutation or preservation fields.

#### GreenfieldGateContext

```
GreenfieldGateContext {
    owner: &str,
    risk: RiskClass,
    zone: UsageZone,
    approvals: &[ApprovalRecord],
    evidence_complete: bool,
}
```

Used by `evaluate_greenfield_gates()`. Same shape as discovery but evaluated against different artifact expectations (includes Architecture gate).

#### ArchitectureGateContext

```
ArchitectureGateContext {
    owner: &str,
    risk: RiskClass,
    zone: UsageZone,
    approvals: &[ApprovalRecord],
    evidence_complete: bool,
}
```

Used by `evaluate_architecture_gates()`. Same shape. Architecture gate checks architecture-decisions.md, invariants.md, tradeoff-matrix.md.

### 3. New Constants — Mode Files

#### discovery.rs

```
MODE_FILE: &str = "discovery.toml"  (already exists)
STEP_SEQUENCE: &[&str] = &[
    "capture-context", "classify-risk", "govern-repository-context",
    "explore-problem-domain", "challenge-assumptions",
    "emit-artifacts", "evaluate-gates",
]
REQUIRED_GATES: &[GateKind] = &[Exploration, Risk, ReleaseReadiness]
GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    ReadRepository, GenerateContent, CritiqueContent,
]
```

#### greenfield.rs

```
MODE_FILE: &str = "system-shaping.toml"
STEP_SEQUENCE: &[&str] = &[
    "capture-context", "classify-risk", "govern-repository-context",
    "shape-system-boundaries", "critique-architecture-options",
    "emit-artifacts", "evaluate-gates",
]
REQUIRED_GATES: &[GateKind] = &[Exploration, Architecture, Risk, ReleaseReadiness]
GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    ReadRepository, GenerateContent, CritiqueContent,
]
```

#### architecture.rs

```
MODE_FILE: &str = "architecture.toml"  (already exists)
STEP_SEQUENCE: &[&str] = &[
    "capture-context", "classify-risk", "govern-repository-context",
    "evaluate-structural-options", "challenge-architectural-claims",
    "emit-artifacts", "evaluate-gates",
]
REQUIRED_GATES: &[GateKind] = &[Exploration, Architecture, Risk, ReleaseReadiness]
GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    ReadRepository, GenerateContent, CritiqueContent,
]
```

### 4. Artifact Rendering Functions

New functions in `service.rs` or a dedicated rendering module:

- `render_discovery_artifact(file_name: &str, evidence: &DiscoveryEvidence) -> String`
- `render_greenfield_artifact(file_name: &str, evidence: &GreenfieldEvidence) -> String`
- `render_architecture_artifact(file_name: &str, evidence: &ArchitectureEvidence) -> String`

These parallel existing `render_requirements_artifact_from_evidence()` and `render_brownfield_artifact()`.

### 5. Persistence Layout (No New Types)

Follows existing `PersistedRunBundle` structure. New artifact directories:

```
.canon/artifacts/{run_id}/discovery/
    problem-map.md
    unknowns-and-assumptions.md
    context-boundary.md
    exploration-options.md
    decision-pressure-points.md

.canon/artifacts/{run_id}/system-shaping/
    system-shape.md
    architecture-outline.md
    capability-map.md
    delivery-options.md
    risk-hotspots.md

.canon/artifacts/{run_id}/architecture/
    architecture-decisions.md
    invariants.md
    tradeoff-matrix.md
    boundary-map.md
    readiness-assessment.md
```

### 6. State Transitions

No new states. All three modes use the existing `RunState` enum:
- `Draft` → `ContextCaptured` → `Classified` → `Contracted` → `Gated` → `Executing` → `Completed` | `Blocked` | `AwaitingApproval`

State derivation uses existing `run_state_from_gates()` — no changes needed.

## Relationships

```
Mode::Discovery  ──uses──▶ ArtifactContract (5 artifacts)
                 ──uses──▶ DiscoveryGateContext
                 ──evaluates──▶ [Exploration, Risk, ReleaseReadiness]

Mode::Greenfield (`system-shaping`) ──uses──▶ ArtifactContract (5 artifacts)
                 ──uses──▶ GreenfieldGateContext
                 ──evaluates──▶ [Exploration, Architecture, Risk, ReleaseReadiness]

Mode::Architecture ──uses──▶ ArtifactContract (5 artifacts)
                   ──uses──▶ ArchitectureGateContext
                   ──evaluates──▶ [Exploration, Architecture, Risk, ReleaseReadiness]
```

## Validation Rules

- All artifact file names must be kebab-case `.md`
- All required sections must be present for gate evaluation to pass
- Evidence bundle must link generation paths to derived artifact names
- Gate evaluations must persist to `gates/{gate-kind}.toml`
- No artifact may be emitted without a corresponding generation path in the evidence bundle
