# Data Model: Observability Design Mode

This document outlines the typed structures required by the `observability-design` mode within `canon-engine`. As per the constitution, stable structures must use explicitly typed `struct` and `enum` with `serde` derives.

## Domain Entities

### `TelemetryPlan`

Represents the overall mapping of system boundaries to observability signals.

- `boundaries`: `Vec<BoundarySignalMap>`
- `global_constraints`: `Vec<String>`

### `BoundarySignalMap`

Maps a specific system boundary to its corresponding logs, metrics, and traces.

- `boundary_name`: `String` (e.g., "Payment Gateway Integration")
- `signals`: `Vec<Signal>`
- `failure_domain`: `String`
- `consumer`: `String`

### `Signal`

A specific telemetry signal planned for a boundary.

- `signal_type`: `SignalType` (Enum: `Log`, `Metric`, `Trace`)
- `name`: `String`
- `description`: `String`

### `SloAlert`

Represents an actionable Service Level Indicator and its threshold.

- `sli_name`: `String`
- `threshold`: `String` (e.g., "> 200ms over 5m")
- `alert_destination`: `String`

### `RunbookStub`

An actionable playbook for first responders.

- `alert_trigger`: `String`
- `action_items`: `Vec<String>`
- `escalation_path`: `String`
