# Specification Quality Checklist: Controlled Execution Modes (`implementation` and `refactor`)

**Purpose**: Validate specification completeness and quality before proceeding to planning  
**Created**: 2026-04-23  
**Feature**: [spec.md](../spec.md)

## Content Quality

- [X] No implementation details (no concrete language, framework, library, file path, or API names that would lock in a specific design)
- [X] Focused on user value and business needs (the developer-as-user journey is explicit and the safety story is named)
- [X] Written for non-technical stakeholders (mode promotion is explained in plain "execute changes safely" terms, not internal type names)
- [X] All mandatory sections completed (Governance Context, User Scenarios & Testing, Requirements, Success Criteria, Validation Plan, Decision Log, Non-Goals, Assumptions)

## Governance Context Quality

- [X] Mode is declared and unambiguous (`change`, not "NEEDS CLARIFICATION")
- [X] Risk classification has explicit rationale (`bounded-impact` with reasoning)
- [X] Scope-In and Scope-Out are concrete and disjoint
- [X] Invariants cover gatekeeper bounds, refactor preservation, plan-intent mapping, immutable inputs, generation-vs-validation separation, run identity compatibility, and red-zone recommendation-only behavior
- [X] Decision Traceability names a specific destination (`specs/010-controlled-execution-modes/decision-log.md` and the change run's `.canon/runs/<…>/decisions/`)

## Requirement Completeness

- [X] No `[NEEDS CLARIFICATION]` markers remain
- [X] Functional requirements distinguish `implementation` from `refactor` rather than collapsing them
- [X] Mutation bounds are required and gate-enforced (FR-003, FR-005, FR-020)
- [X] Refactor preservation is required and gate-enforced (FR-004, FR-019, FR-021)
- [X] Authored input contracts are named for both modes (FR-006, FR-007)
- [X] Immutable input snapshot behavior is required and reuses prior runtime guarantees (FR-008, FR-028)
- [X] Safety-net requirement is present and is not satisfied solely by repo-wide coverage (FR-010, FR-011, FR-012)
- [X] Strict TDD is explicitly NOT mandated (FR-013)
- [X] Recommendation-only degradation is required for red-zone / systemic-impact and for missing safety net (FR-023, FR-024, FR-025)
- [X] Recommendation-only runs remain inspectable, listable, publishable (FR-026)
- [X] Compatibility with canonical run identity, run lookup, and publish workflow is required (FR-027, FR-029, FR-030)
- [X] Documentation, skills, and defaults updates are in scope (FR-031, FR-032, FR-033)

## Success Criteria Quality

- [X] Each SC is verifiable from artifacts on disk or from existing CLI surfaces
- [X] SCs cover both modes' completion paths (SC-001, SC-002)
- [X] SCs cover blocking semantics for missing inputs and missing safety net (SC-003, SC-005)
- [X] SCs cover recommendation-only degradation (SC-004)
- [X] SCs cover compatibility with prior runtime model (SC-006, SC-007)
- [X] SCs cover skill and documentation honesty (SC-008, SC-009)
- [X] SCs are technology-agnostic (no specific language, framework, or library lock-in)
- [X] No SC depends on metrics that cannot be observed without instrumenting beyond `.canon/`

## Mode-distinctness Quality

- [X] An `implementation` artifact bundle is distinguishable from a `refactor` artifact bundle by inspecting persisted artifacts alone (FR-016)
- [X] `implementation` failure categories differ from `refactor` failure categories in a named, specific way (FR-020 vs FR-021)
- [X] `refactor` cannot satisfy completion by claiming "no behavior change" without preservation evidence and no-feature-addition proof (FR-019)
- [X] `implementation` cannot satisfy completion via unmapped or out-of-bounds executed changes (FR-018, FR-020)

## Edge Case Coverage

- [X] Hidden feature addition disguised as refactor is treated as blocking
- [X] Mutation surface drift mid-run is treated as blocking
- [X] Authored input edited or deleted after run creation does not affect the run
- [X] Regressed safety net during a run is treated as blocking
- [X] Repo-wide coverage alone is not sufficient evidence
- [X] Legacy UUID-keyed runs remain addressable
- [X] Out-of-bounds adapter actions are denied before execution
- [X] Red-zone refactors are still executed as recommendation-only

## Compatibility & Non-Regression

- [X] Canonical run identity model is reused unchanged
- [X] Existing publish workflow is reused unchanged
- [X] No parallel CLI surface is introduced for these modes
- [X] Existing modes (`change`, `requirements`, `pr-review`, etc.) remain unchanged in behavior
- [X] Existing storage layout under `.canon/runs/<…>/` is preserved

## Non-Goals & Assumptions Quality

- [X] Non-Goals explicitly exclude strict TDD as a contributor mandate
- [X] Non-Goals explicitly exclude promoting `incident` and `migration`
- [X] Non-Goals explicitly exclude red-zone mutation maturity
- [X] Non-Goals explicitly exclude duplicating CLI surfaces for these modes
- [X] Non-Goals explicitly exclude adding new artifact storage schemes
- [X] Assumptions name the runtime baseline (run identity, immutable snapshots) as stable
- [X] Assumptions name documentation, skills, and defaults as in scope for honesty updates

## Validation Plan Quality

- [X] Structural validation lists concrete commands and inputs
- [X] Logical validation enumerates the test surfaces that MUST exist
- [X] Independent validation names a reviewer pass and what it must confirm
- [X] Evidence artifacts have a named persisted destination

## Decision Log Quality

- [X] Each decision has a rationale that ties back to scope, safety, or compatibility
- [X] Promoting both modes together is justified
- [X] Recommendation-only default for red-zone is justified
- [X] No-new-CLI-surface choice is justified
- [X] Authored input contracts are justified
- [X] Posture-via-state (rather than posture-via-different-artifact-shapes) is justified
