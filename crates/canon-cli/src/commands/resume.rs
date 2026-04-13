use canon_engine::EngineService;

use crate::commands::exit_code_for_state;
use crate::error::CliResult;

pub fn execute(service: &EngineService, run: &str) -> CliResult<i32> {
    let summary = service.resume(run)?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(exit_code_for_state(&summary.state))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use canon_engine::{
        EngineService, RunRequest,
        domain::mode::Mode,
        domain::policy::{RiskClass, UsageZone},
    };
    use tempfile::tempdir;

    use super::execute;

    #[test]
    fn execute_returns_awaiting_approval_exit_code_for_gated_requirements_run() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(workspace.path().join("idea.md"), "# Idea\n\nTest resume wrapper.\n")
            .expect("write idea file");
        let service = EngineService::new(workspace.path());
        let run = service
            .run(RunRequest {
                mode: Mode::Requirements,
                risk: RiskClass::SystemicImpact,
                zone: UsageZone::Green,
                owner: "Owner <owner@example.com>".to_string(),
                inputs: vec!["idea.md".to_string()],
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            })
            .expect("requirements run should start in approval-gated state");

        let code = execute(&service, &run.run_id).expect("resume should succeed");

        assert_eq!(code, 3);
    }
}
