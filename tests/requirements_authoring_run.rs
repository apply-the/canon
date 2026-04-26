use std::fs;

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

fn requirements_request(input: &str) -> RunRequest {
    RunRequest {
        mode: Mode::Requirements,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: "product-lead".to_string(),
        inputs: vec![input.to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn complete_requirements_brief() -> &'static str {
    r#"# Requirements Brief

## Problem

Bound the firmware-flashing workflow to a USB-only CLI surface.

## Outcome

Operators can flash firmware safely with explicit logs and a reversible path.

## Constraints

- USB transport only
- Preserve explicit audit logs

## Non-Negotiables

- Human ownership remains explicit
- Artifacts persist under `.canon/`

## Options

1. Deliver the CLI first.
2. Defer broader orchestration.

## Recommended Path

Deliver the bounded CLI slice first.

## Tradeoffs

- Governance adds upfront structure.

## Consequences

- Reviewers can inspect the packet without chat history.

## Out of Scope

- No GUI in this slice.

## Deferred Work

- Hosted rollout stays deferred.

## Decision Checklist

- [x] Scope is explicit
- [x] Ownership is explicit

## Open Questions

- How is bootloader mode entered?
"#
}

#[test]
fn requirements_run_completes_with_authored_sections_and_no_missing_marker() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("requirements.md"), complete_requirements_brief())
        .expect("requirements brief");

    let service = EngineService::new(workspace.path());
    let summary = service.run(requirements_request("requirements.md")).expect("requirements run");

    assert_eq!(summary.state, "Completed");

    let problem_statement = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("requirements")
            .join("problem-statement.md"),
    )
    .expect("problem statement");
    assert!(
        problem_statement.contains(
            "## Problem\n\nBound the firmware-flashing workflow to a USB-only CLI surface."
        )
    );
    assert!(problem_statement.contains("## Outcome\n\nOperators can flash firmware safely with explicit logs and a reversible path."));
    assert!(!problem_statement.contains("## Missing Authored Body"));
}

#[test]
fn requirements_run_blocks_with_missing_body_marker_when_required_heading_is_absent() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("requirements.md"),
        "# Requirements Brief\n\n## Problem\n\nBound the firmware-flashing workflow to a USB-only CLI surface.\n",
    )
    .expect("requirements brief");

    let service = EngineService::new(workspace.path());
    let summary = service.run(requirements_request("requirements.md")).expect("requirements run");

    assert_eq!(summary.state, "Blocked");
    assert_eq!(summary.blocking_classification.as_deref(), Some("artifact-blocked"));

    let problem_statement = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("requirements")
            .join("problem-statement.md"),
    )
    .expect("problem statement");
    assert!(problem_statement.contains("## Missing Authored Body"));
    assert!(problem_statement.contains("`## Outcome`"));
}
