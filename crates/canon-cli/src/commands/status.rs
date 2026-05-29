use canon_engine::EngineService;
use serde_json::json;

use crate::app::OutputFormat;
use crate::error::CliResult;
use crate::output;

pub fn execute(service: &EngineService, run: &str, format: OutputFormat) -> CliResult<i32> {
    let summary = service.status(run)?;
    match format {
        OutputFormat::Text | OutputFormat::Markdown => {
            output::print_status_summary(&summary, format)?
        }
        other => output::print_value(&status_payload(&summary)?, other)?,
    }
    Ok(0)
}

fn status_payload(summary: &canon_engine::StatusSummary) -> CliResult<serde_json::Value> {
    let mut payload = serde_json::to_value(summary)?;
    if let Some(candidate) = summary
        .refinement_state
        .as_ref()
        .and_then(|refinement| refinement.suggested_candidate.as_ref())
    {
        payload["suggested_continuation"] = json!({
            "run_id": candidate.run_id,
            "mode": candidate.mode,
            "state": candidate.state,
            "match_reason": candidate.match_reason,
            "advisory": candidate.advisory,
            "mutation_allowed": false,
        });
    }

    Ok(payload)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use canon_engine::{
        EngineService, RunRequest,
        domain::mode::Mode,
        domain::policy::{RiskClass, UsageZone},
        domain::run::ClassificationProvenance,
    };
    use tempfile::tempdir;

    use super::execute;
    use crate::app::OutputFormat;

    #[test]
    fn execute_reports_status_for_completed_run() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(workspace.path().join("idea.md"), "# Idea\n\nTest status wrapper.\n")
            .expect("write idea file");
        let service = EngineService::new(workspace.path());
        let run = service
            .run(RunRequest {
                mode: Mode::Requirements,
                risk: RiskClass::LowImpact,
                zone: UsageZone::Green,
                system_context: None,
                classification: ClassificationProvenance::explicit(),
                owner: "Owner <owner@example.com>".to_string(),
                inputs: vec!["idea.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            })
            .expect("requirements run should succeed");

        let code =
            execute(&service, &run.run_id, OutputFormat::Json).expect("status should succeed");

        assert_eq!(code, 0);
    }

    #[test]
    fn execute_surfaces_advisory_continuation_for_fresh_same_mode_work() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(
            workspace.path().join("idea.md"),
            "# Requirements Brief\n\n## Problem\nKeep the same work identity visible.\n",
        )
        .expect("write idea file");
        let service = EngineService::new(workspace.path());
        let request = RunRequest {
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            inputs: vec!["idea.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        };

        let first = service.run(request.clone()).expect("first draft run");
        let second = service.run(request).expect("second draft run");

        let code =
            execute(&service, &second.run_id, OutputFormat::Json).expect("status should succeed");
        assert_eq!(code, 0);

        let status = service.status(&second.run_id).expect("status summary");
        let payload = super::status_payload(&status).expect("status json payload");

        assert_eq!(
            payload["suggested_continuation"],
            json!({
                "run_id": first.run_id,
                "mode": "requirements",
                "state": "Draft",
                "match_reason": "same authoritative input fingerprint",
                "advisory": true,
                "mutation_allowed": false,
            })
        );
    }
}
