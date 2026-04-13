use canon_engine::EngineService;

use crate::app::OutputFormat;
use crate::error::CliResult;
use crate::output;

pub fn execute(service: &EngineService, run: &str, format: OutputFormat) -> CliResult<i32> {
    let summary = service.status(run)?;
    output::print_value(&summary, format)?;
    Ok(0)
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
                owner: "Owner <owner@example.com>".to_string(),
                inputs: vec!["idea.md".to_string()],
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            })
            .expect("requirements run should succeed");

        let code =
            execute(&service, &run.run_id, OutputFormat::Json).expect("status should succeed");

        assert_eq!(code, 0);
    }
}
