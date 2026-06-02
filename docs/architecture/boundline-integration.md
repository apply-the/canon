# Boundline Integration

Canon and Boundline are designed to cooperate without collapsing into one system.

## What Boundline Consumes From Canon

Boundline can consume Canon-governed knowledge such as:

- accepted domain language
- domain model concepts and invariants
- architecture decisions
- governed standards
- evidence summaries
- approval state
- packet readiness
- lineage and provenance
- project memory
- authority metadata

This lets delivery execution use knowledge that has a source, evidence, and governance posture.

## Governed Standards

Canon can produce standards that Boundline may activate as guidance or checks.

Examples:

- accepted vocabulary
- deprecated term warnings
- architecture constraints
- required verification posture
- security findings
- migration compatibility rules

Canon owns the meaning and evidence. Boundline owns runtime activation and enforcement behavior.

## Project Memory

Project memory gives Boundline a durable knowledge surface that outlives individual AI sessions.

Boundline should prefer accepted, source-linked memory over raw generated notes. It should preserve packet refs so knowledge can be refreshed, challenged, or superseded later.

## Evidence

Evidence helps Boundline distinguish strong guidance from weak context.

For example:

- an accepted invariant with source evidence can become a strong review signal
- a pending finding can become advisory context
- a rejected packet should not drive delivery mutation

Evidence and readiness should influence how strongly downstream tools rely on Canon knowledge.

## Approvals

Approval state is a boundary signal.

Boundline should stop or require human review when Canon indicates that approval is required, rejected, expired, or unavailable for a critical authority boundary.

Canon does not decide Boundline's runtime behavior. It provides governed state that Boundline can respect.

## Lineage

Lineage lets Boundline explain why it activated a guidance rule or delivery constraint.

Useful lineage includes:

- packet ref
- source document ref
- promotion event
- evidence summary
- approval state
- authority contract

This keeps delivery behavior auditable.

## What Canon Does Not Execute

Canon does not:

- execute coding tasks
- choose models
- run agent councils
- manage runtime stages
- mutate production systems
- replace the delivery orchestrator

Canon is the semantic governance runtime. Boundline is a runtime delivery system. They should stay cleanly separated.

## Canon-Aware But Not Boundline-Dependent

Canon should remain useful without Boundline. Other consumers can use the same governed packets, project memory, adapter fields, and authority metadata.

For machine-facing integration, use the canonical source guide: [Governance Adapter Integration](https://github.com/apply-the/canon/blob/main/tech-docs/integration/governance-adapter.md).
