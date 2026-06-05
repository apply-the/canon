# Quickstart: Observability Design Mode

This guide walks you through using the `observability-design` mode to proactively define telemetry and alert contracts for your feature or architecture.

## 1. Prepare Your Input

Ensure you have a markdown document that describes your system or feature. This could be a `domain-model.md`, `architecture.md`, or a `spec.md`.

## 2. Run the Command

Execute the Canon CLI, passing the path to your document:

```bash
canon observability-design path/to/architecture.md
```

## 3. Interactive Disambiguation

If the provided document is too vague, the agent will prompt you interactively to clarify the system boundaries:

```text
> [Canon] The provided document lacks clear boundaries. Please list the primary failure domains or bounded contexts for this architecture:
> User: 1. Payment Gateway, 2. User Authentication, 3. Notification Service
```

## 4. Review the Artifacts

Once complete, Canon generates four files:

1. **`telemetry-plan.md`**: Outlines the specific logs, metrics, and traces for each boundary.
2. **`03-slo-alerts.md`**: Defines actionable SLI/SLO thresholds (e.g., latency > 200ms).
3. **`04-runbook.md`**: Provides a standard markdown "If-This-Then-That" playbook for on-call responders.
4. **`05-instrumentation-checklist.md`**: A checklist that your downstream implementation run MUST satisfy.

## 5. Implement

You can now pass the generated `05-instrumentation-checklist.md` into your `canon implementation` mode. Canon will verify that these signals are actually present in the final codebase diff before allowing the implementation run to close.
