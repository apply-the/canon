<!--
Sync Impact Report
Version change: template -> 1.0.0
Modified principles:
- template principle slot 1 -> I. Method over prompting
- template principle slot 2 -> II. Artifact-first engineering
- template principle slot 3 -> III. Separation of generation and validation
- template principle slot 4 -> IV. Risk-aware execution
- template principle slot 5 -> V. Mode-driven workflows
- Added VI. Decision traceability
- Added VII. Invariants before implementation
- Added VIII. Bounded context awareness
- Added IX. Progressive autonomy
- Added X. Layered verification
Added sections:
- Non-Goals
- Definition of Done
Removed sections:
- None
Templates requiring updates:
- ✅ .specify/templates/plan-template.md
- ✅ .specify/templates/spec-template.md
- ✅ .specify/templates/tasks-template.md
- ✅ .specify/templates/checklist-template.md
- ✅ .specify/templates/agent-file-template.md
- ✅ .agents/skills/speckit-specify/SKILL.md
- ✅ .agents/skills/speckit-plan/SKILL.md
- ✅ .agents/skills/speckit-tasks/SKILL.md
- ✅ .agents/skills/speckit-implement/SKILL.md
- ✅ .agents/skills/speckit-clarify/SKILL.md
- ✅ README.md
Follow-up TODOs:
- None
-->
# Canon Constitution

Canon exists to transform AI-assisted software engineering from an
unstructured, prompt-driven activity into a governed, auditable, and
artifact-driven system.

The goal is not to maximize generation. The goal is to maximize clarity,
control, and reliability. AI is a cognitive multiplier operating under
constraints, not an autonomous decision-maker.

## Core Principles

### I. Method over prompting

All work MUST follow an explicit method before execution begins. Every method
MUST declare the operating mode, ordered steps, required artifacts, permitted
operations, and validation gates. Ad-hoc prompting is not a valid engineering
workflow.

Rationale: explicit methods make work reviewable, repeatable, and resistant to
prompt drift.

### II. Artifact-first engineering

Every engineering step MUST produce a durable, inspectable artifact in the
repository or designated project memory. Conversations MAY inform work, but
they MUST NOT be treated as the system of record. If a step produces no
persistent artifact, that step is incomplete.

Rationale: durable artifacts are the only reliable way to preserve memory,
enable review, and support downstream work.

### III. Separation of generation and validation

The same process MUST NOT both generate and validate critical outputs.
Non-trivial outputs MUST receive an independent review step, and critical
outputs MUST receive adversarial evaluation or equivalent challenge. Validation
results MUST be recorded as artifacts.

Rationale: independence reduces confirmation bias and raises the probability of
finding hidden defects.

### IV. Risk-aware execution

Every task MUST declare a risk classification before execution begins. The
classification MUST define the allowed autonomy level, required artifacts,
validation depth, and approval gates. No task may proceed without an explicit
risk record.

Rationale: risk determines the amount of control the system must apply.

### V. Mode-driven workflows

System-shaping, change, review, architecture, debugging, and operational work
MUST use mode-specific workflows. Applying the wrong workflow is a critical
process failure and MUST be corrected before implementation continues.

Rationale: different engineering activities have different failure modes and
therefore require different controls.

### VI. Decision traceability

All meaningful decisions MUST be recorded with context, alternatives
considered, rationale, and consequences. Decisions hidden inside prompts,
conversations, or code are invalid until they are written into a durable
decision log.

Rationale: untraceable decisions cannot be reviewed, audited, or safely
revisited.

### VII. Invariants before implementation

Implementation MUST NOT begin until the relevant invariants are explicit.
Invariants MUST describe system boundaries, non-negotiable truths, and the
conditions that must remain true throughout delivery. Code produced without
defined invariants is unsafe.

Rationale: invariants anchor safe change and prevent accidental architectural
drift.

### VIII. Bounded context awareness

Every operation MUST declare its scope, relevant inputs, and excluded areas
before acting. Work outside the declared boundary requires an explicit scope
update and recorded rationale. Unbounded context is a reliability risk.

Rationale: bounded context reduces hallucination, overreach, and accidental
blast radius.

### IX. Progressive autonomy

Autonomy MUST be earned through validated artifacts, stable invariants, and an
appropriate risk classification. High-risk or unstable areas MUST use tighter
controls, smaller execution steps, and more human oversight than low-risk work.

Rationale: autonomy is a consequence of evidence, not a default privilege.

### X. Layered verification

Completion claims MUST be supported by multiple validation layers appropriate
to the work, including structural validation, logical validation, consistency
checks, and adversarial review when warranted. Passing a single check never
proves correctness.

Rationale: reliability emerges from overlapping defenses, not a single gate.

## Non-Goals

This project does NOT aim to:

- maximize the speed of code generation
- replace engineering judgment
- provide generic agent orchestration detached from project governance
- act as a prompt library without artifacts, modes, or validation

## Definition of Done

A task is complete only when all of the following are true:

- required artifacts exist and are current
- execution mode and risk classification are explicit
- invariants and scope boundaries are recorded before implementation
- decisions are traceable to a durable log
- layered validation has executed and evidence is attached
- outputs remain consistent with the declared invariants

AI must not be asked "what can you generate?" It must be constrained to answer
"what is allowed to exist?"

## Governance

This constitution supersedes undocumented prompt conventions and conflicting
local habits.

Amendments MUST be proposed as artifact changes that describe the affected
principles, rationale, downstream template impacts, and any migration
expectations. Amendment approval MUST include independent review for any change
that alters execution controls or governance semantics.

Versioning policy is semantic:

- MAJOR for incompatible governance changes, principle removals, or principle
  redefinitions
- MINOR for new principles, new mandatory sections, or materially expanded
  obligations
- PATCH for clarifications that do not change operative meaning

Compliance review expectations are mandatory:

- every specification MUST declare mode, risk, scope boundaries, invariants,
  and decision traceability expectations
- every implementation plan MUST pass a constitution check before design or
  implementation proceeds
- every task list MUST include artifact creation and independent validation work
  required by the declared risk level
- no work may be marked complete until validation evidence is recorded and
  independently reviewed when required

**Version**: 1.0.0 | **Ratified**: 2026-03-26 | **Last Amended**: 2026-03-26
