use canon_engine::{ApprovalSummary, EngineService, domain::approval::ApprovalDecision};

use crate::error::{CliError, CliResult};

pub fn execute(
    service: &EngineService,
    run: &str,
    target: Option<String>,
    gate: Option<String>,
    by: Option<String>,
    decision: String,
    rationale: String,
) -> CliResult<i32> {
    let target = match (target, gate) {
        (Some(target), None) => target,
        (None, Some(gate)) => format!("gate:{gate}"),
        (Some(target), Some(_)) => target,
        (None, None) => {
            return Err(CliError::InvalidInput("approval target is required".to_string()));
        }
    };
    let summary: ApprovalSummary = service.approve(
        run,
        &target,
        by.as_deref().unwrap_or_default(),
        decision.parse::<ApprovalDecision>().map_err(CliError::InvalidInput)?,
        &rationale,
    )?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(0)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    use canon_engine::EngineService;
    use canon_engine::{
        RunRequest,
        domain::mode::Mode,
        domain::policy::{RiskClass, UsageZone},
        domain::run::{ClassificationProvenance, SystemContext},
    };
    use tempfile::tempdir;

    use super::execute;

    fn git(workspace: &std::path::Path, args: &[&str]) {
        let output = Command::new("git")
            .current_dir(workspace)
            .args(args)
            .output()
            .expect("run git command");

        assert!(
            output.status.success(),
            "git {:?} failed: stdout=`{}` stderr=`{}`",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn init_change_repo(workspace: &std::path::Path) {
        git(workspace, &["init", "-b", "main"]);
        git(workspace, &["config", "user.name", "Canon Test"]);
        git(workspace, &["config", "user.email", "canon@example.com"]);

        fs::create_dir_all(workspace.join("src/auth")).expect("src dir");
        fs::create_dir_all(workspace.join("tests")).expect("tests dir");
        fs::write(
            workspace.join("src/auth/session.rs"),
            "pub fn revoke_session(id: &str) -> String {\n    format!(\"revoked:{id}\")\n}\n",
        )
        .expect("source file");
        fs::write(
            workspace.join("tests/session.md"),
            "# Session Checks\n\n- revocation formatting remains stable\n",
        )
        .expect("test file");

        git(workspace, &["add", "."]);
        git(workspace, &["commit", "-m", "seed change repo"]);
    }

    fn complete_implementation_brief() -> &'static str {
        r#"# Implementation Brief

Feature Slice: Auth session revocation repository wiring inside the existing login subsystem.
Primary Upstream Mode: change

## Task Mapping
1. Add bounded auth session repository helpers.
2. Thread the new helper through the revocation service without expanding the public API.
3. Record implementation notes for operator review and rollback.

## Bounded Changes
- Auth session repository helper wiring.
- Revocation service internal composition.

## Mutation Bounds
src/auth/session.rs and src/auth/repository.rs only.

## Allowed Paths
- src/auth/session.rs
- src/auth/repository.rs

## Executed Changes
- Add the bounded repository helper and thread it through the revocation service without widening the public API.

## Candidate Frameworks
- Candidate 1 keeps the helper local to the auth-session slice.
- Candidate 2 introduces a shared auth abstraction before the bounded slice is proven.

## Options Matrix
- Option 1 keeps the helper inside the auth-session slice.
- Option 2 introduces a shared auth abstraction before the bounded slice is proven.

## Decision Evidence
- Existing auth-session rollback posture already aligns with the local helper approach.
- Focused implementation suites already guard the bounded packet contract.

## Recommendation
- Start with the local helper and defer broader abstraction until a later change proves it necessary.

## Task Linkage
- Step 1 adds the helper.
- Step 2 rewires the service behind the existing external contract.
- Step 3 records the resulting packet and rollback posture.

## Completion Evidence
- The emitted implementation packet and focused tests confirm the bounded slice is ready for operator review.

## Adoption Implications
- Operators can adopt the helper in the auth-session slice without widening the pattern across the rest of auth.

## Remaining Risks
- Repository wiring could still drift into adjacent auth modules if the bounded paths expand during review.

## Ecosystem Health
- The surrounding auth subsystem is stable enough for a local helper, but shared abstraction pressure is still low.

## Safety-Net Evidence
Contract coverage protects revocation formatting and audit ordering before mutation.

## Independent Checks
- cargo test --test session_contract
- cargo test --test auth_audit_ordering

## Rollback Triggers
Revocation output drifts, audit ordering becomes unstable, or repository wiring expands beyond the declared auth-session slice.

## Rollback Steps
1. Revert the bounded auth-session patch.
2. Redeploy the previous build.
3. Restore the last known-good audit ordering snapshot.
"#
    }

    fn gated_implementation_run(
        service: &EngineService,
        workspace: &std::path::Path,
    ) -> (String, String) {
        init_change_repo(workspace);
        fs::write(workspace.join("implementation.md"), complete_implementation_brief())
            .expect("write brief");

        let summary = service
            .run(RunRequest {
                mode: Mode::Implementation,
                risk: RiskClass::BoundedImpact,
                zone: UsageZone::Yellow,
                system_context: Some(SystemContext::Existing),
                classification: ClassificationProvenance::explicit(),
                owner: "Owner <owner@example.com>".to_string(),
                inputs: vec!["implementation.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            })
            .expect("implementation run should start in approval-gated state");

        let target = summary
            .approval_targets
            .first()
            .cloned()
            .expect("approval-gated run should expose a target");

        (summary.run_id, target)
    }

    #[test]
    fn execute_requires_target_or_gate() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "run-123",
            None,
            None,
            None,
            "approve".to_string(),
            "looks good".to_string(),
        )
        .expect_err("missing target should fail");

        assert!(error.to_string().contains("approval target is required"));
    }

    #[test]
    fn execute_rejects_unknown_decision_before_service_execution() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "run-123",
            Some("gate:risk".to_string()),
            None,
            None,
            "defer".to_string(),
            "needs discussion".to_string(),
        )
        .expect_err("invalid decision should fail");

        assert!(error.to_string().contains("unsupported approval decision: defer"));
    }

    #[test]
    fn execute_builds_gate_target_when_only_gate_is_provided() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());
        let (run_id, _) = gated_implementation_run(&service, workspace.path());

        let code = execute(
            &service,
            &run_id,
            None,
            Some("risk".to_string()),
            Some("approver@example.com".to_string()),
            "approve".to_string(),
            "Looks good".to_string(),
        )
        .expect("approval should succeed when gate is provided");

        assert_eq!(code, 0);
    }

    #[test]
    fn execute_prefers_explicit_target_when_target_and_gate_are_both_present() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());
        let (run_id, target) = gated_implementation_run(&service, workspace.path());

        let code = execute(
            &service,
            &run_id,
            Some(target),
            Some("zone".to_string()),
            Some("approver@example.com".to_string()),
            "approve".to_string(),
            "Target should win".to_string(),
        )
        .expect("approval should use the explicit target");

        assert_eq!(code, 0);
    }
}
