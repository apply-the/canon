# Decision Log: Analysis Mode Expansion

**Feature**: 006-analysis-expansion  
**Date**: 2026-04-13

## Planning-Phase Decisions

### PD-001: Artifact Names Follow Spec, Not Methods TOML

**Context**: Three sources define artifact names for these modes: the reviewed spec, the contract-only methods TOML files, and the mode.rs artifact_families. They diverge.

**Decision**: The spec artifact names are canonical. Methods TOML and mode.rs must be updated to match.

**Alternatives**:
- Use TOML names — rejected; they are placeholders from the contract-only phase
- Merge naming conventions — rejected; creates confusion about which source is authoritative

**Consequences**: Methods TOML files and mode.rs artifact_families require coordinated updates during implementation.

---

### PD-002: Per-Mode Orchestration Methods Over Shared Helper

**Context**: All three new modes share the same 13-step orchestration shape as requirements. A shared `run_analysis_mode()` helper could reduce duplication.

**Decision**: Use per-mode `run_discovery()`, `run_system_shaping()`, `run_architecture()` methods in service.rs, matching the existing pattern.

**Alternatives**:
- Shared `run_analysis_mode(mode, config)` — rejected; only 3 modes is not enough to justify the abstraction, and the existing pattern is well-tested
- Trait-based dispatch — rejected; over-engineering for the current scope

**Consequences**: Some code duplication across the three run methods. Acceptable at this scale and consistent with existing codebase conventions.

---

### PD-003: Critique Optional for Discovery, Mandatory for System-Shaping and Architecture

**Context**: OQ-001 asks which evidence sources are mandatory per mode.

**Decision**: Critique/challenge is optional for discovery, mandatory for system-shaping and architecture.

**Alternatives**:
- Mandatory everywhere — rejected; discovery explores unknowns, forcing critique adds ceremony without proportional value
- Optional everywhere — rejected; system-shaping and architecture make consequential structural claims that must be challenged

**Consequences**: Discovery's run_discovery() may skip the critique invocation if the problem domain is sufficiently bounded. System-shaping and architecture always execute critique.

---

### PD-004: Analysis Modes Inherit Existing Approval Policy

**Context**: OQ-002 asks whether bounded-impact architecture runs need additional approval.

**Decision**: All three modes use the existing Risk gate approval policy: SystemicImpact or Red zone → NeedsApproval. No mode-specific approval overrides.

**Alternatives**:
- Architecture-specific approval gate at bounded-impact — rejected; contradicts progressive autonomy (Constitution IX) and over-governs analysis work
- No approval for any analysis mode — rejected; systemic-impact work must always require approval regardless of mode

**Consequences**: Bounded-impact analysis runs complete without approval gates. Systemic-impact analysis runs require explicit approval before artifacts are final.

---

### PD-005: Per-Mode Gate Evaluation Functions

**Context**: Gate evaluation needs mode-specific logic. Options are per-mode functions or a generic dispatcher.

**Decision**: Create `evaluate_discovery_gates()`, `evaluate_system_shaping_gates()`, `evaluate_architecture_gates()` in gatekeeper.rs, each with a mode-specific context struct.

**Alternatives**:
- Single `evaluate_analysis_gates(mode, ...)` — rejected; masks mode-specific gate semantics
- Reuse `evaluate_requirements_gates()` — rejected; requirements has different artifact expectations

**Consequences**: Three new public functions and three new context structs in gatekeeper.rs.

---

### PD-006: Discovery Exploration Gate Checks Problem Domain Boundedness

**Context**: The spec says "the discovery Exploration gate must verify that the problem domain is bounded, not that solutions are bounded."

**Decision**: Discovery's Exploration gate checks for `problem-map.md` and `context-boundary.md` presence, verifying that the problem domain has defined boundaries. This differs from requirements' Exploration gate which checks for `problem-statement.md` as a well-framed problem.

**Alternatives**:
- Reuse requirements' Exploration gate logic — rejected; discovery has different exploration semantics
- No Exploration gate for discovery — rejected; unbounded exploration violates the spec's D-001 decision

**Consequences**: The Exploration gate primitive needs to accept mode-specific artifact names as the "boundary" check target.
