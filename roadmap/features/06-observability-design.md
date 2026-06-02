# 06 - Observability Design

## Problem
Canon supports reactive operations planning via the `incident` mode. However, it lacks a dedicated proactive operations mode meant to anticipate logging and observability before deployments happen.

## Proposal
Introduce an `observability-design` (or `operations`) mode specifically tailored to plan the system's runtime footprint.
- **Log Contracts**: Define what needs tracing and at what level.
- **SLI/SLO**: Establish Service Level Indicators and Objectives for feature slices.
- **Runbooks**: Define first-responder actions, alert triggers, and telemetry expectations before the code goes into production.

## Risk Profile

**Governance Zone**: Green (advisory design artifacts, no runtime mutation).
The mode produces telemetry contracts and runbook stubs; it does not instrument
code directly. Risk emerges downstream if the implementation run fails to honour
the designed signals, but the observability packet itself is read-only.

*(Note: Canon governs the static design artifacts like telemetry plans, SLOs, and runbooks. Runtime event schemas, JSONL trace export, eval telemetry, and execution-time instrumentation checks remain owned by Boundline.)*

## Why Existing Modes Are Not Enough
- `incident` and postmortem work are reactive; they begin after the system has
  already failed or degraded.
- `architecture` may mention non-functional concerns, but it does not force a
  concrete telemetry contract, alert model, or operator handoff.

## Dependencies

- **None upstream**: can start in parallel with any other feature.
- **Pairs with 05 (Policy Shaping)**: observability SLOs are themselves
  enforceable policies; aligning the two modes avoids duplicate governance.
- **Downstream consumer**: linked `implementation` runs check the
  instrumentation checklist before closing.

## Related Modes

| Existing Mode | Relationship |
|---|---|
| `architecture` | Upstream: architecture defines the boundaries; observability designs signals for them. |
| `incident` | Complementary: observability designs proactively what incident learns reactively. |
| `implementation` | Downstream: the instrumentation checklist becomes an implementation gate. |
| `verification` | Consumed: signal presence is verifiable via code search or trace inspection. |

## Entry Gates
- The packet must start from an accepted feature, architecture, or domain slice;
  observability design without a bounded target surface becomes generic advice.
- The critical user journeys, failure domains, and owning operator surface must
  be named before metrics are proposed.
- If a feature is too early to name signals, the correct next step is upstream
  clarification rather than speculative SLOs.

## Operational Mechanics
- **Inputs**: A system `architecture.md`, `domain-model.md`, or a specific `feature-spec.md`.
- **Workflow Steps**:
  1. **Telemetry Mapping**: The agent identifies critical system boundaries, external API calls, and failure domains within the input doc.
  2. **Instrumentation Design**: It drafts the exact structured log events (e.g., `tracing::info!` spans in Rust) or metrics (counters, histograms) required to observe those specific boundaries.
  3. **Alerting Thresholds**: It proposes concrete thresholds (e.g., "P99 latency > 200ms over a 5-minute rolling window") for the system's SLIs.
  4. **Runbook Drafting**: It creates "If-This-Then-That" playbooks for on-call responders dictating what to do when those exact alerts trigger.
- **Required Artifacts**: `telemetry-plan.md` (detailing metrics and log boundaries) and a `runbook-stub.md`. These artifacts act as execution gates for consuming runtimes: linked implementation flows should not be marked complete unless the planned instrumentation is verifiably present in the code.

## Exit Gates
- Each critical boundary must map to at least one concrete signal, one consumer,
  and one failure interpretation.
- Alert thresholds must include enough context to act on them, not just a raw
  threshold number.
- The resulting packet must be specific enough that an implementation run can
  check whether the planned instrumentation was actually added.

## Packet Shape
- `01-context.md`: service slice, critical journeys, and operator owner.
- `02-signal-map.md`: logs, metrics, traces, and boundary coverage.
- `03-slo-alerts.md`: proposed SLIs, thresholds, and alert destinations.
- `04-runbook.md`: first-response workflow and escalation expectations.
- `05-instrumentation-checklist.md`: implementation-facing checklist tied to the
  planned signals.

## Success Criteria

- Every critical boundary named in the architecture has at least one signal, one
  consumer, and one actionable failure interpretation documented before code
  ships.
- Linked implementation runs cannot close unless the instrumentation checklist
  items are verifiably present in the diff.
- On-call responders can find a runbook entry within 60 seconds for any alert
  that fires on a surface covered by this packet.