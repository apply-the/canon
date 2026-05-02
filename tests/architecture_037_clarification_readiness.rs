use std::fs;

use assert_cmd::Command;
use canon_engine::EngineService;
use canon_engine::artifacts::contract::contract_for_mode;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

fn cli_command() -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "run",
        "--quiet",
        "--manifest-path",
        concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"),
        "-p",
        "canon-cli",
        "--bin",
        "canon",
        "--",
    ]);
    command
}

fn architecture_request(inputs: Vec<&str>) -> RunRequest {
    RunRequest {
        mode: Mode::Architecture,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Green,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: "principal-architect".to_string(),
        inputs: inputs.into_iter().map(ToString::to_string).collect(),
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

const AMBIGUOUS_ARCHITECTURE_BRIEF: &str = r#"# Architecture Brief

Decision focus: choose a bounded split between event intake and report
assembly.
Constraint: preserve Canon runtime contracts and keep operational complexity
low.

## Decision

Separate intake validation from report assembly.

## Constraints

- Preserve existing `.canon/` storage.
- Avoid new infrastructure tiers for the first release.

## Evaluation Criteria

- Boundary clarity
- Operational simplicity

## Decision Drivers

- Reviewers need an explicit decision surface without reconstructing chat.

## Recommendation

Start with a bounded split between intake validation and report assembly.

## Consequences

- The decision still depends on the availability target.

## Bounded Contexts

- Event Intake owns validation.
- Report Assembly owns aggregation.

## Context Relationships

- Event Intake hands validated events to Report Assembly.

## Integration Seams

- The handoff is a validated event envelope.

## Anti-Corruption Candidates

- A sink adapter should shield report assembly from external metrics labels.

## Ownership Boundaries

- Event Intake and Report Assembly are owned by the analytics team.

## Shared Invariants

- Aggregation totals remain reproducible from validated inputs.

## Open Questions

- What availability target must the reporting surface satisfy?
"#;

const UNDER_BOUNDED_ARCHITECTURE_BRIEF: &str = r#"# Architecture Brief

We should improve analytics, but the problem and downstream capability shape are
still unclear.
"#;

const READINESS_ARCHITECTURE_BRIEF: &str = r#"# Architecture Brief

Decision focus: choose a bounded split between event intake and report
assembly.
Constraint: preserve Canon runtime contracts and keep operational complexity
low.

## Decision

Separate intake validation from report assembly.

## Constraints

- Preserve existing `.canon/` storage.
- Avoid new infrastructure tiers for the first release.

## Evaluation Criteria

- Boundary clarity
- Operational simplicity

## Decision Drivers

- Reviewers need an explicit decision surface without reconstructing chat.

## Options Considered

- Keep event intake and report assembly in one context.
- Split event intake from report assembly.

## Pros

- The split keeps validation ownership explicit.

## Cons

- The split stays conditional on the availability target.

## Recommendation

Start with a bounded split between intake validation and report assembly.

## Why Not The Others

- A single context would blur validation ownership.

## Consequences

- The decision still depends on the availability target.

## Working Assumptions

- Initial traffic is low to moderate.
- Business-hours availability is acceptable for the first release.

## Unresolved Questions

- Does the reporting surface need 24x7 availability?

## Bounded Contexts

- Event Intake owns validation.
- Report Assembly owns aggregation.

## Context Relationships

- Event Intake hands validated events to Report Assembly.

## Integration Seams

- The handoff is a validated event envelope.

## Anti-Corruption Candidates

- A sink adapter should shield report assembly from external metrics labels.

## Ownership Boundaries

- Event Intake and Report Assembly are owned by the analytics team.

## Shared Invariants

- Aggregation totals remain reproducible from validated inputs.

## System Context

- System: `analytics-cli` processes bounded event batches into finance reports.
- External actors:
  - finance-analyst: reads generated reports.

## Containers

- `analytics-cli` (Rust CLI): validates events and generates reports.

## Components

- `event-loader`: reads raw events.
- `aggregator`: builds report rows.
"#;

#[test]
fn inspect_clarity_surfaces_architecture_question_metadata_and_defaults() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("architecture.md"), AMBIGUOUS_ARCHITECTURE_BRIEF)
        .expect("architecture brief");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "architecture",
            "--input",
            "architecture.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("clarity json");
    let questions =
        json["entries"][0]["clarification_questions"].as_array().expect("questions array");
    let question = questions
        .iter()
        .find(|entry| {
            entry["prompt"].as_str().is_some_and(|prompt| prompt.contains("availability target"))
        })
        .expect("availability-target question");

    assert_eq!(question["affects"].as_str(), Some("readiness-assessment.md"));
    assert!(question["default_if_skipped"].as_str().is_some_and(|value| !value.is_empty()));
    assert_eq!(question["status"].as_str(), Some("required"));
}

#[test]
fn inspect_clarity_reroutes_under_bounded_architecture_briefs() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("architecture.md"), UNDER_BOUNDED_ARCHITECTURE_BRIEF)
        .expect("architecture brief");

    let output = cli_command()
        .current_dir(workspace.path())
        .args([
            "inspect",
            "clarity",
            "--mode",
            "architecture",
            "--input",
            "architecture.md",
            "--output",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value = serde_json::from_slice(&output).expect("clarity json");
    let recommended_focus =
        json["entries"][0]["recommended_focus"].as_str().expect("recommended focus");

    assert!(
        recommended_focus.contains("discovery")
            || recommended_focus.contains("requirements")
            || recommended_focus.contains("system-shaping"),
        "expected explicit reroute guidance, got: {recommended_focus}"
    );
}

#[test]
fn architecture_contract_includes_readiness_assumption_sections() {
    let contract = contract_for_mode(Mode::Architecture);
    let readiness = contract
        .artifact_requirements
        .iter()
        .find(|requirement| requirement.file_name == "readiness-assessment.md")
        .expect("readiness requirement");

    assert_eq!(
        readiness.required_sections,
        vec![
            "Summary",
            "Readiness Status",
            "Working Assumptions",
            "Unresolved Questions",
            "Blockers",
            "Accepted Risks",
            "Recommended Next Mode",
        ]
    );
}

#[test]
fn architecture_run_materializes_readiness_assumptions_and_questions() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("architecture.md"), READINESS_ARCHITECTURE_BRIEF)
        .expect("architecture brief");

    let service = EngineService::new(workspace.path());
    let summary =
        service.run(architecture_request(vec!["architecture.md"])).expect("architecture run");

    let readiness = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("architecture")
            .join("readiness-assessment.md"),
    )
    .expect("readiness assessment");

    assert!(readiness.contains("## Working Assumptions"));
    assert!(readiness.contains("Initial traffic is low to moderate."));
    assert!(readiness.contains("## Unresolved Questions"));
    assert!(readiness.contains("Does the reporting surface need 24x7 availability?"));
    assert!(readiness.contains("## Recommended Next Mode"));
    assert!(readiness.contains("change"));
}
