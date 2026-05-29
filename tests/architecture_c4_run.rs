use canon_engine::artifacts::contract::{architecture_contract_for_context, contract_for_mode};
use canon_engine::artifacts::markdown::{
    MISSING_AUTHORED_BODY_MARKER, render_architecture_artifact,
};
use canon_engine::domain::mode::Mode;

const C4_BRIEF: &str = r#"# Architecture Brief

Decision focus: shape `analytics-cli` as a bounded analytics surface with explicit
ownership and rollback-safe boundaries.
Constraint: preserve Canon runtime contracts, approvals, and evidence persistence.

## Decision

Use a dedicated context map and ADR-like decision packet to make architecture boundaries reviewable.

## Constraints

- Preserve Canon runtime contracts, approvals, and evidence persistence.
- Keep the C4 artifact family additive rather than replacing it.

## Evaluation Criteria

- Boundary clarity.
- Rollback-safe seams.
- Reviewability without chat history.

## Decision Drivers

- Reviewers need both the structural boundary story and the selected option in one packet.
- The architecture run must preserve existing C4 artifacts unchanged.

## Options Considered

- Keep boundary and C4 details implicit in summary prose.
- Add a dedicated context map and ADR-like decision shape inside the existing packet.

## Pros

- Reviewers can inspect decisions, options, and C4 views without reconstructing the conversation.
- The packet remains publishable without inventing a new artifact family.

## Cons

- The authored brief must carry more explicit structure up front.

## Recommendation

Add a dedicated context map and ADR-like decision shape inside the existing architecture packet.

## Why Not The Others

- Summary-only output hides the rejected alternative.
- A brand new packet family would widen scope and churn.

## Consequences

- Architecture authors must keep decision rationale and option analysis synchronized with the C4 views.
- Reviewers gain a stronger packet, but the authored input surface becomes more explicit.

## Bounded Contexts

- Event Intake: owns ingestion of raw event files and input validation.
- Report Assembly: owns aggregation logic and report generation semantics.
- Metrics Telemetry: owns operational counters and sink-facing telemetry translation.

## Context Relationships

- Event Intake feeds validated events into Report Assembly.
- Report Assembly emits operational signals to Metrics Telemetry without owning sink-specific models.

## Integration Seams

- The handoff from Event Intake to Report Assembly is a validated in-memory event envelope.
- The handoff from Report Assembly to Metrics Telemetry is a domain-neutral metrics event.

## Anti-Corruption Candidates

- Metrics Telemetry should shield Report Assembly from the managed sink's labeling model.

## Ownership Boundaries

- Event Intake and Report Assembly are owned by the analytics domain team.
- Metrics Telemetry is jointly reviewed, but sink translation stays owned by the platform observability team.

## Shared Invariants

- Aggregated report totals must remain reproducible from the validated input set.
- Metrics emission must never mutate reporting outcomes.

## System Context

The bounded `analytics-cli` consumes raw event files and produces aggregated
reports for downstream finance teams. External actors:

- finance-analyst (reads reports)
- ops-pipeline (writes raw event files)

## Containers

- `analytics-cli` (single-binary Rust CLI)
- `report-store` (object storage bucket)
- `metrics-sink` (managed time-series store)

## Deployment

- `analytics-cli` runs on a scheduled reporting worker.
- `report-store` lives in the finance data account.
- `metrics-sink` remains a managed shared observability service.

## Components

- `event-loader` reads raw events from disk.
- `aggregator` collapses events into report rows.
- `report-writer` persists rows to `report-store`.
- `metrics-emitter` pushes counters to `metrics-sink`.

## Dynamic View

- `event-loader` validates the raw file.
- `aggregator` derives report rows.
- `report-writer` persists the report.
- `metrics-emitter` publishes telemetry.
"#;

const MINIMAL_C4_BRIEF: &str = r#"# Architecture Brief

Decision focus: bounded analytics CLI.
Constraint: preserve Canon runtime contracts.
"#;

#[test]
fn architecture_run_persists_overview_visual_sidecars_and_optional_deeper_views() {
    let contract =
        architecture_contract_for_context(&contract_for_mode(Mode::Architecture), C4_BRIEF);
    assert_eq!(contract.artifact_requirements.len(), 19);
    let artifact_names = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    for file_name in [
        "01-architecture-overview.md",
        "08-system-context.md",
        "09-system-context.mmd",
        "10-container-view.md",
        "11-container-view.mmd",
        "12-deployment-view.md",
        "13-deployment-view.mmd",
        "14-component-view.md",
        "15-component-view.mmd",
        "16-dynamic-view.md",
        "17-dynamic-view.mmd",
        "view-manifest.json",
        "packet-metadata.json",
    ] {
        assert!(
            artifact_names.contains(&file_name),
            "{file_name} should be selected for the C4-rich brief"
        );
    }

    let overview = render_architecture_artifact("architecture-overview.md", C4_BRIEF, "", "");
    assert!(overview.starts_with("# Architecture Overview"));
    assert!(overview.contains("## Included Views"));
    assert!(overview.contains("```mermaid"));
    assert!(overview.contains("Dynamic View"));
}

#[test]
fn architecture_run_preserves_authored_c4_bodies_in_published_artifacts() {
    let overview = render_architecture_artifact("architecture-overview.md", C4_BRIEF, "", "");
    assert!(overview.starts_with("# Architecture Overview"));
    assert!(overview.contains("## Included Views"));
    assert!(overview.contains("```mermaid"));

    let system_context = render_architecture_artifact("system-context.md", C4_BRIEF, "", "");
    assert!(system_context.contains("# System Context"));
    assert!(system_context.contains("finance-analyst (reads reports)"));
    assert!(!system_context.contains("## Missing Authored Body"));
    assert!(!system_context.contains("## Decision Drivers"));

    let container_view = render_architecture_artifact("container-view.md", C4_BRIEF, "", "");
    assert!(container_view.contains("# Container View"));
    assert!(container_view.contains("`analytics-cli` (single-binary Rust CLI)"));
    assert!(!container_view.contains("## Missing Authored Body"));
    assert!(!container_view.contains("## Options Considered"));

    let component_view = render_architecture_artifact("component-view.md", C4_BRIEF, "", "");
    assert!(component_view.contains("# Component View"));
    assert!(component_view.contains("`metrics-emitter` pushes counters to `metrics-sink`."));
    assert!(!component_view.contains("## Missing Authored Body"));
    assert!(!component_view.contains("## Recommendation"));

    let deployment_view = render_architecture_artifact("deployment-view.md", C4_BRIEF, "", "");
    assert!(deployment_view.contains("# Deployment View"));
    assert!(deployment_view.contains("scheduled reporting worker"));
    assert!(!deployment_view.contains("## Missing Authored Body"));

    let dynamic_view = render_architecture_artifact("dynamic-view.md", C4_BRIEF, "", "");
    assert!(dynamic_view.contains("# Dynamic View"));
    assert!(dynamic_view.contains("publishes telemetry"));

    let context_map = render_architecture_artifact("context-map.md", C4_BRIEF, "", "");
    assert!(context_map.contains("# Context Map"));
    assert!(context_map.contains("## Bounded Contexts"));
    assert!(context_map.contains("Metrics Telemetry"));
    assert!(!context_map.contains("## Missing Authored Body"));
}

#[test]
fn architecture_run_emits_missing_body_marker_when_brief_omits_c4_sections() {
    let contract =
        architecture_contract_for_context(&contract_for_mode(Mode::Architecture), MINIMAL_C4_BRIEF);
    let artifact_names = contract
        .artifact_requirements
        .iter()
        .map(|requirement| requirement.file_name.as_str())
        .collect::<Vec<_>>();

    for file in ["system-context.md", "container-view.md", "deployment-view.md"] {
        let body = render_architecture_artifact(file, MINIMAL_C4_BRIEF, "", "");
        assert!(
            body.contains(MISSING_AUTHORED_BODY_MARKER),
            "{file} should emit missing-body marker when brief omits the C4 section"
        );
    }

    assert!(artifact_names.contains(&"view-manifest.json"));
    assert!(artifact_names.contains(&"packet-metadata.json"));
    assert!(!artifact_names.contains(&"component-view.md"));
    assert!(!artifact_names.contains(&"component-view.mmd"));
    assert!(!artifact_names.contains(&"dynamic-view.md"));
    assert!(!artifact_names.contains(&"dynamic-view.mmd"));
}
