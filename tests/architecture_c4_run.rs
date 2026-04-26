use std::fs;

use canon_engine::EngineService;
use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::approval::ApprovalDecision;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

const C4_BRIEF: &str = r#"# Architecture Brief

Decision focus: shape `analytics-cli` as a bounded analytics surface with explicit
ownership and rollback-safe boundaries.
Constraint: preserve Canon runtime contracts, approvals, and evidence persistence.

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

## Components

- `event-loader` reads raw events from disk.
- `aggregator` collapses events into report rows.
- `report-writer` persists rows to `report-store`.
- `metrics-emitter` pushes counters to `metrics-sink`.
"#;

fn architecture_request(owner: &str, inputs: Vec<&str>) -> RunRequest {
    RunRequest {
        mode: Mode::Architecture,
        risk: RiskClass::SystemicImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: owner.to_string(),
        inputs: inputs.into_iter().map(ToString::to_string).collect(),
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

#[test]
fn architecture_run_persists_all_nine_artifacts_including_context_map_and_c4_views() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("architecture.md"), C4_BRIEF).expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(architecture_request("principal-architect", vec!["architecture.md"]))
        .expect("architecture run");

    // Architecture systemic-impact runs gate on Risk approval before completion.
    assert_eq!(summary.state, "AwaitingApproval");
    let approved = service
        .approve(
            &summary.run_id,
            "gate:risk",
            "principal-engineer",
            ApprovalDecision::Approve,
            "C4 architecture analysis approved.",
        )
        .expect("gate approval");
    assert_eq!(approved.state, "Completed");

    let contract = contract_for_mode(Mode::Architecture);
    assert_eq!(contract.artifact_requirements.len(), 9);

    let artifact_dir = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join(&summary.run_id)
        .join("architecture");
    for requirement in contract.artifact_requirements.iter() {
        let path = artifact_dir.join(&requirement.file_name);
        assert!(
            path.exists(),
            "expected artifact {} to be persisted at {}",
            requirement.file_name,
            path.display()
        );
    }
}

#[test]
fn architecture_run_preserves_authored_c4_bodies_in_published_artifacts() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("architecture.md"), C4_BRIEF).expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(architecture_request("principal-architect", vec!["architecture.md"]))
        .expect("architecture run");
    service
        .approve(
            &summary.run_id,
            "gate:risk",
            "principal-engineer",
            ApprovalDecision::Approve,
            "C4 architecture analysis approved.",
        )
        .expect("gate approval");

    let artifact_dir = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join(&summary.run_id)
        .join("architecture");

    let system_context =
        fs::read_to_string(artifact_dir.join("system-context.md")).expect("system-context.md");
    assert!(system_context.contains("# System Context"));
    assert!(system_context.contains("finance-analyst (reads reports)"));
    assert!(!system_context.contains("## Missing Authored Body"));

    let container_view =
        fs::read_to_string(artifact_dir.join("container-view.md")).expect("container-view.md");
    assert!(container_view.contains("# Container View"));
    assert!(container_view.contains("`analytics-cli` (single-binary Rust CLI)"));
    assert!(!container_view.contains("## Missing Authored Body"));

    let component_view =
        fs::read_to_string(artifact_dir.join("component-view.md")).expect("component-view.md");
    assert!(component_view.contains("# Component View"));
    assert!(component_view.contains("`metrics-emitter` pushes counters to `metrics-sink`."));
    assert!(!component_view.contains("## Missing Authored Body"));

    let context_map =
        fs::read_to_string(artifact_dir.join("context-map.md")).expect("context-map.md");
    assert!(context_map.contains("# Context Map"));
    assert!(context_map.contains("## Bounded Contexts"));
    assert!(context_map.contains("Metrics Telemetry"));
    assert!(!context_map.contains("## Missing Authored Body"));
}

#[test]
fn architecture_run_emits_missing_body_marker_when_brief_omits_c4_sections() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("architecture.md"),
        "# Architecture Brief\n\nDecision focus: bounded analytics CLI.\nConstraint: preserve Canon runtime contracts.\n",
    )
    .expect("brief file");

    let service = EngineService::new(workspace.path());
    let summary = service
        .run(architecture_request("principal-architect", vec!["architecture.md"]))
        .expect("architecture run");
    service
        .approve(
            &summary.run_id,
            "gate:risk",
            "principal-engineer",
            ApprovalDecision::Approve,
            "Approve to inspect missing-body emission.",
        )
        .expect("gate approval");

    let artifact_dir = workspace
        .path()
        .join(".canon")
        .join("artifacts")
        .join(&summary.run_id)
        .join("architecture");

    for file in ["system-context.md", "container-view.md", "component-view.md"] {
        let body = fs::read_to_string(artifact_dir.join(file)).expect(file);
        assert!(
            body.contains("## Missing Authored Body"),
            "{file} should emit missing-body marker when brief omits the C4 section"
        );
    }
}
