use std::fs;
use std::process::Command as ProcessCommand;

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::RunRequest;
use tempfile::TempDir;

fn git(workspace: &TempDir, args: &[&str]) {
    let output = ProcessCommand::new("git")
        .args(["-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false"])
        .args(args)
        .current_dir(workspace.path())
        .output()
        .expect("git command");
    assert!(
        output.status.success(),
        "git {:?} failed: stdout=`{}` stderr=`{}`",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_change_repo(workspace: &TempDir) {
    git(workspace, &["init", "-b", "main"]);
    git(workspace, &["config", "user.name", "Canon Test"]);
    git(workspace, &["config", "user.email", "canon@example.com"]);

    fs::create_dir_all(workspace.path().join("src/auth")).expect("src dir");
    fs::create_dir_all(workspace.path().join("tests")).expect("tests dir");
    fs::write(
        workspace.path().join("src/auth/session.rs"),
        "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
    )
    .expect("source file");
    fs::write(
        workspace.path().join("tests/session.md"),
        "# Session Checks\n\n- revocation formatting remains stable\n",
    )
    .expect("test file");

    git(workspace, &["add", "."]);
    git(workspace, &["commit", "-m", "seed change repo"]);
}

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

## Domain Slice

Session lifecycle and cleanup semantics within the auth domain.

## Excluded Areas

- payment settlement
- billing reports

## Intended Change

Add bounded repository methods while preserving the public auth contract.

## Legacy Invariants

- session revocation remains eventually consistent
- audit log ordering stays stable

## Domain Invariants

- a revoked session must never become active again through cleanup retries
- audit trails must preserve causal order across repository updates

## Forbidden Normalization

- Do not collapse audit-ordering quirks that operators still rely on.

## Change Surface

- session repository
- auth service
- token cleanup job

## Ownership

- primary owner: maintainer

## Cross-Context Risks

- cleanup scheduling can leak into notification flows if repository boundaries widen

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

## Decision Drivers

- Preserve operator expectations.
- Keep the auth contract stable during the bounded repository change.

## Options Considered

- Option 1 keeps the additive repository helper inside the auth boundary.
- Option 2 normalizes scheduling and cleanup behavior in the same slice.

## Decision Evidence

- Existing operator workflows still depend on the current auth cleanup ordering.
- Contract tests already guard the preserved API surface.

## Boundary Tradeoffs

- keep cleanup logic inside the auth boundary even if that duplicates some scheduling code

## Recommendation

- Start with the additive repository helper and defer normalization to a later slice.

## Why Not The Others

- Normalizing cleanup behavior now would widen the change surface beyond the bounded auth slice.

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
    init_change_repo(&workspace);
    fs::write(workspace.path().join("change.md"), complete_change_brief()).expect("change brief");

    let service = EngineService::new(workspace.path());
    let summary = service.run(change_request("change.md")).expect("change run");

    assert_eq!(summary.state, "Draft");

    let resumed = service.resume(&summary.run_id).expect("resume change run");

    assert_eq!(resumed.state, "Completed");

    let implementation_plan = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("change")
            .join("04-implementation-plan.md"),
    )
    .expect("implementation plan");
    assert!(implementation_plan.contains("## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract."));
    assert!(!implementation_plan.contains("## Missing Authored Body"));

    let decision_record = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("change")
            .join("06-decision-record.md"),
    )
    .expect("decision record");
    assert!(decision_record.contains("## Decision Drivers"));
    assert!(decision_record.contains("## Decision Evidence"));
    assert!(decision_record.contains("## Recommendation"));
    assert!(decision_record.contains("## Why Not The Others"));
}

#[test]
fn change_run_blocks_with_missing_body_marker_when_required_heading_is_absent() {
    let workspace = TempDir::new().expect("temp dir");
    init_change_repo(&workspace);
    fs::write(
        workspace.path().join("change.md"),
        "# Change Brief\n\n## System Slice\n\nauth session boundary and persistence layer.\n\n## Excluded Areas\n\n- payment settlement\n",
    )
    .expect("change brief");

    let service = EngineService::new(workspace.path());
    let summary = service.run(change_request("change.md")).expect("change run");

    assert_eq!(summary.state, "Draft");

    let resumed = service.resume(&summary.run_id).expect("resume change run");

    assert_eq!(resumed.state, "Blocked");
    assert_eq!(resumed.blocking_classification.as_deref(), Some("artifact-blocked"));

    let change_surface = fs::read_to_string(
        workspace
            .path()
            .join(".canon")
            .join("artifacts")
            .join(&summary.run_id)
            .join("change")
            .join("03-change-surface.md"),
    )
    .expect("change surface");
    assert!(change_surface.contains("## Missing Authored Body"));
    assert!(change_surface.contains("`## Change Surface`"));
}
