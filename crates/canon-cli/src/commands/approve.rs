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

    use canon_engine::EngineService;
    use canon_engine::{
        RunRequest,
        domain::mode::Mode,
        domain::policy::{RiskClass, UsageZone},
        domain::run::{ClassificationProvenance, SystemContext},
    };
    use tempfile::tempdir;

    use super::execute;

    fn complete_change_brief() -> &'static str {
        "# Change Brief\n\n## System Slice\n\nAuth session boundary.\n\n## Domain Slice\n\nSession lifecycle rules.\n\n## Excluded Areas\n\n- billing\n\n## Intended Change\n\nAdd a bounded auth persistence change.\n\n## Legacy Invariants\n\n- revocation remains eventually consistent\n\n## Domain Invariants\n\n- revoked sessions never become active again\n\n## Forbidden Normalization\n\n- do not normalize audit ordering quirks\n\n## Change Surface\n\n- auth repository\n- cleanup job\n\n## Ownership\n\n- maintainer\n\n## Cross-Context Risks\n\n- cleanup scheduling could leak into adjacent jobs\n\n## Implementation Plan\n\nAdd the bounded repository change without widening the surface.\n\n## Sequencing\n\n1. Add bounded repository methods.\n2. Switch callers behind the preserved contract.\n\n## Validation Strategy\n\n- contract tests\n\n## Independent Checks\n\n- separate rollback rehearsal\n\n## Decision Record\n\nPrefer additive change over normalization.\n\n## Decision Drivers\n\n- preserve operator expectations\n\n## Options Considered\n\n- additive bounded change\n- broader normalization\n\n## Decision Evidence\n\n- current checks already protect the invariants\n\n## Boundary Tradeoffs\n\n- keep duplicate scheduling logic inside the auth boundary\n\n## Recommendation\n\n- proceed with the additive bounded change\n\n## Why Not The Others\n\n- broader normalization widens scope too early\n\n## Consequences\n\n- preserved behavior stays reviewable\n\n## Unresolved Questions\n\n- should cleanup roll out in the same slice?\n"
    }

    fn gated_change_run(service: &EngineService, workspace: &std::path::Path) -> (String, String) {
        fs::write(workspace.join("change.md"), complete_change_brief()).expect("write brief");

        let summary = service
            .run(RunRequest {
                mode: Mode::Change,
                risk: RiskClass::SystemicImpact,
                zone: UsageZone::Yellow,
                system_context: Some(SystemContext::Existing),
                classification: ClassificationProvenance::explicit(),
                owner: "Owner <owner@example.com>".to_string(),
                inputs: vec!["change.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            })
            .expect("change run should start in approval-gated state");

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
        let (run_id, _) = gated_change_run(&service, workspace.path());

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
        let (run_id, target) = gated_change_run(&service, workspace.path());

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
