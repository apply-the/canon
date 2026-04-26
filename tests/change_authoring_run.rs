use std::fs;

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

fn change_request(input: &str) -> RunRequest {
    RunRequest {
        mode: Mode::Change,
        risk: RiskClass::BoundedImpact,
        zone: UsageZone::Yellow,
        system_context: Some(SystemContext::Existing),
        classification: ClassificationProvenance::explicit(),
        owner: "maintainer".to_string(),
        inputs: vec![input.to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    }
}

fn complete_change_brief() -> &'static str {
    r#"# Change Brief

## System Slice

auth session boundary and persistence layer.

## Excluded Areas

- payment settlement
- billing reports

## Intended Change

Add bounded repository methods while preserving the public auth contract.

## Legacy Invariants

- session revocation remains eventually consistent
- audit log ordering stays stable

## Forbidden Normalization

- Do not collapse audit-ordering quirks that operators still rely on.

## Change Surface

- session repository
- auth service
- token cleanup job

## Ownership

- primary owner: maintainer

## Implementation Plan

Add bounded repository methods and preserve the public auth contract.

## Sequencing

1. Add bounded repository methods.
2. Switch callers behind the preserved contract.

## Validation Strategy

- contract tests
- invariant checks

## Independent Checks

- rollback rehearsal by a separate operator

## Decision Record

Prefer additive change over normalization to preserve operator expectations.

## Consequences

- preserved surface remains explicit and reviewable

## Unresolved Questions

- should the cleanup job roll out in the same slice?

Owner: maintainer
Risk Level: bounded-impact
Zone: yellow
"#
}

#[test]
fn change_run_completes_with_authored_sections_and_no_missing_marker() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(workspace.path().join("change.md"), complete_change_brief()).expect("change brief");

    let service = EngineService::new(workspace.path());
    let summary = service.run(change_request("change.md")).expect("change run");

    assert_eq!(summary.state, "Completed");

    let implementation_plan = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("change")
            .join("implementation-plan.md"),
    )
    .expect("implementation plan");
    assert!(implementation_plan.contains("## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract."));
    assert!(!implementation_plan.contains("## Missing Authored Body"));
}

#[test]
fn change_run_blocks_with_missing_body_marker_when_required_heading_is_absent() {
    let workspace = TempDir::new().expect("temp dir");
    fs::write(
        workspace.path().join("change.md"),
        "# Change Brief\n\n## System Slice\n\nauth session boundary and persistence layer.\n\n## Excluded Areas\n\n- payment settlement\n",
    )
    .expect("change brief");

    let service = EngineService::new(workspace.path());
    let summary = service.run(change_request("change.md")).expect("change run");

    assert_eq!(summary.state, "Blocked");
    assert_eq!(summary.blocking_classification.as_deref(), Some("artifact-blocked"));

    let change_surface = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("change")
            .join("change-surface.md"),
    )
    .expect("change surface");
    assert!(change_surface.contains("## Missing Authored Body"));
    assert!(change_surface.contains("`## Change Surface`"));
}
