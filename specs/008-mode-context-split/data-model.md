# Data Model: Mode Context Split

## Mode

- **Purpose**: Represents the governed work type Canon is performing.
- **Fields**:
  - `name`: public mode identifier such as `discovery`, `architecture`, `change`, or `review`
  - `context_requirement`: whether the mode requires explicit `system_context` or accepts omission
  - `artifact_namespace`: emitted artifact directory and summary label associated with the mode
- **Rules**:
  - Mode names describe only the work type, never the system state.
  - `brownfield-change`, `brownfield`, and `greenfield` are not valid public mode identifiers.
  - `change` replaces the previous bounded-change public mode.

## SystemContext

- **Purpose**: Makes system state explicit without overloading the mode name.
- **Values**:
  - `new`
  - `existing`
- **Rules**:
  - Required-context modes must reject missing values before run creation.
  - Optional-context modes may persist no value at all; they must not store an invented placeholder.
  - `change` accepts only `existing` in the first release of the split model.

## RunRequest

- **Purpose**: Captures user-supplied startup parameters before Canon classifies, persists, and executes a run.
- **Fields**:
  - `mode`
  - `system_context` (optional in the type system, validated per mode)
  - `risk`
  - `zone`
  - `classification`
  - `owner`
  - `inputs`
  - `inline_inputs`
  - `excluded_paths`
  - `policy_root`
  - `method_root`
- **Rules**:
  - Validation occurs before any run is created.
  - Missing required `system_context`, legacy mode names, and invalid combinations such as `change + new` are rejected before persistence.

## RunContextRecord

- **Purpose**: Durable run metadata persisted under `.canon/runs/<RUN_ID>/context.toml` and surfaced through inspect, status, evidence, and resume flows.
- **Fields**:
  - `repo_root`
  - `owner`
  - `inputs`
  - `excluded_paths`
  - `input_fingerprints`
  - `captured_at`
  - `system_context` (optional, persisted when supplied)
- **Rules**:
  - `system_context` must remain identical across context capture, manifests, and inspect summaries.
  - Omitted optional context stays absent rather than serialized as a fake sentinel value.

## ChangePacket

- **Purpose**: The bounded-change packet emitted for `change` runs in existing systems.
- **Artifacts**:
  - `system-slice.md`
  - `legacy-invariants.md`
  - `change-surface.md`
  - `implementation-plan.md`
  - `validation-strategy.md`
  - `decision-record.md`
- **Rules**:
  - The packet exists only for `change + existing`.
  - Artifact paths live under `.canon/artifacts/<RUN_ID>/change/`.
  - Preservation, readiness, and validation semantics match the previous brownfield workflow.

## ModeContextGateProfile

- **Purpose**: Encodes how gates and methods consume the split between work type and system context.
- **Fields**:
  - `mode`
  - `system_context`
  - `required_gates`
  - `validation_independence_required`
  - `evidence_complete_required`
- **Rules**:
  - `change + existing` preserves the previous bounded-change gate stack.
  - Optional-context modes may not branch on an invented context value.
  - Public gate and approval labels must use the new naming model.

## State Transitions

- `requested` -> `validated`: reject legacy names, missing required context, and invalid context combinations before persistence
- `validated` -> `context-captured`: write `context.toml` with explicit `system_context` when present
- `context-captured` -> `gated`: evaluate mode-specific gates using mode plus explicit context
- `gated` -> `completed` or `blocked`: summarize results and artifact paths using the renamed `change` namespace where applicable