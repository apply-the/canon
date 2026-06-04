# Feature Specification: Policy Shaping Mode

**Feature Branch**: `070-policy-shaping-mode`

**Created**: 2026-06-04

**Status**: Draft

**Input**: User description: "Canon 05 (Policy Shaping): Introduzione di una nuova modalità (policy-shaping o simile) per gestire l'evoluzione e l'analisi di impatto retroattiva per i cambiamenti alle regole o alla costituzione. Follow docs in the roadmap, make a copy (withut number) in the spec folder. Use English"

## Clarifications
### Session 2026-06-04
- Q: Triggering mechanism (Interaction & UX Flow) → A: Canon CLI command (e.g., canon policy-shaping <draft-file>)
- Q: Structured format for automation parsing (Domain & Data Model) → A: YAML frontmatter within the Markdown documents
- Q: Validation Runtime (Integration & External Dependencies) → A: hybrid CLI+skills (Canon CLI owns deterministic contracts; skills provide semantic evaluation normalized into CLI evidence)
- Q: Edge Cases & Failure Handling (Impact Radius Limits) → A: Group violations by directory/module. Do not fail on large radius; instead require a broad-impact approval gate.

## Governance Context *(mandatory)*
- **Mode**: `policy-shaping`
- **Risk**: Systemic Impact (Red) - A policy change alters the behavioral constraints for every future autonomous run. Miscalibrated rules can silently break compliant work or create unbounded migration debt. Requires human Systemic Impact sign-off before the new rule is enforceable.
- **Scope-In**: 
  - Defining proposed changes to repository-wide rules, guidelines (e.g. `.agents/skills`), or core constitution.
  - Generating and running exploratory validation passes to assess the retroactive impact of new policies on the existing codebase.
  - Planning conformance migration strategies (waivers, staging, tech debt scheduling).
- **Scope-Out**: 
  - Direct, immediate application of the policy diff without verification and migration planning (downstream `change` packet handles the actual application).
  - Implicit changes to vocabulary or structures (handled by `domain-language` and `domain-model`).
- **Invariants**:
  - A proposed rule must be specific enough to be directly enforceable or audited; vague guidance is rejected.
  - The impact radius of the policy must be quantified prior to rule acceptance.
  - Legacy exceptions must be explicitly named and time-bounded, not left as silent waivers.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Proposing a new constitution rule (Priority: P1)

As a maintainer, I want to propose a new, strict rule for logging behavior, so that all future autonomous agent runs adhere to the new standard without breaking the existing repository abruptly.

**Why this priority**: Core value of the `policy-shaping` mode is establishing safe, enforceable rules while quantifying their impact.

**Independent Test**: Can be fully tested by creating a `draft-policy.md`, executing the `policy-shaping` mode, and observing the generated impact report.

**Acceptance Scenarios**:

1. **Given** an existing codebase with legacy logging practices and a `draft-policy.md` introducing the new logging rule, **When** the `policy-shaping` workflow runs an impact assessment, **Then** a `conformance-impact-report.md` is generated detailing exactly how many files currently violate the new rule.
2. **Given** the impact report shows widespread violations, **When** the `policy-shaping` workflow progresses, **Then** a `04-migration.md` plan is created to schedule tech debt resolution and apply time-bounded waivers to legacy modules.

### Edge Cases

- What happens when a draft policy is too vague to evaluate? (Should fail validation and request refinement).
- How does system handle policies that conflict with existing, approved rules? (Should flag contradictions during the drafting phase).
- What happens if the policy introduces a massive blast radius (thousands of files)? (Do not fail; instead group violations by module, paginate the report, move file-level details to an appendix, and trigger an explicit broad-impact approval gate).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST expose a Canon CLI command (e.g., `canon policy-shaping <draft-policy.md>`) as the normative execution surface, accepting the draft policy as input and identifying the protected surface.
- **FR-002**: System MUST refine the policy language to be directly enforceable by downstream rules (e.g., turning guidelines into assertions).
- **FR-003**: System MUST execute or simulate an exploratory validation pass against the existing codebase to assess policy compliance.
- **FR-004**: System MUST output a `conformance-impact-report.md` quantifying existing violations, edge cases, and ambiguities.
- **FR-005**: System MUST generate a `04-migration.md` containing a migration strategy, waiver policy, and debt rollout phases if violations are found.
- **FR-006**: System MUST generate a `policy-diff.md` showing semantic changes to the existing constitution.
- **FR-007**: System MUST block finalization of the policy packet until explicit human Systemic Impact sign-off is recorded.
- **FR-008**: System MUST structure large impact reports by grouping violations by directory/module, quantifying total impact/severity in the summary, and moving file-level details to a machine-readable appendix.
- **FR-009**: System MUST enforce an explicit broad-impact approval gate when the affected scope exceeds a configured threshold.

### Key Entities

- **Draft Policy**: The proposed governance rule change. (Must contain YAML frontmatter for machine parsing of scope and invariants).
- **Impact Report**: Evidence describing current violations against the draft policy. (Must contain YAML frontmatter detailing violation counts).
- **Migration Plan**: Strategy for transitioning legacy areas to compliance. (Must contain YAML frontmatter detailing waivers and staging timelines).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of accepted policies have explicit, quantified impact reports detailing existing codebase violations before finalization.
- **SC-002**: 100% of accepted policies are enforceable without interpretation guesswork by downstream automations.
- **SC-003**: All identified legacy exceptions during policy creation are explicitly named and bounded (e.g., recorded in migration plans).

## Validation Plan *(mandatory)*
- **Structural Validation**: Ensure the generated packet contains `01-policy-context.md`, `02-proposed-rule.md`, `03-conformance-impact.md`, `04-migration.md`, and `05-approval.md`.
- **Logical Validation**: Verify that the impact report directly corresponds to the constraints defined in the draft policy.
- **Independent Validation**: Review the migration plan to ensure all reported violations are addressed by either a fix plan or an explicit waiver.

## Decision Log *(mandatory)*
- **Decision 1**: Retroactive sanity checks are enforced *before* policy acceptance.
  - *Rationale*: Updating policies alters behavioral constraints for all future autonomous modes. Changing rules without structured checks is dangerous and can break compliant work.

## Non-Goals *(mandatory)*
- Establishing vocabulary definitions (handled by `domain-language`).
- Applying the actual repository changes derived from the policy (handled by `change` packet downstream).

## Assumptions *(mandatory)*
- Validation follows a hybrid CLI+skills architecture: the Canon CLI owns the deterministic contract, execution envelope, and fail-closed behavior, while LLM-backed `.agents/skills` provide semantic evaluation that is normalized into CLI-validated evidence.
- The repository already has a concept of a "constitution" or existing enforced rules.
