use canon_engine::{
    EngineService, RunRequest,
    domain::mode::Mode,
    domain::policy::{RiskClass, UsageZone},
};

use crate::app::OutputFormat;
use crate::commands::exit_code_for_state;
use crate::error::{CliError, CliResult};
use crate::output;

#[allow(clippy::too_many_arguments)]
pub fn execute(
    service: &EngineService,
    mode: String,
    risk: String,
    zone: String,
    owner: Option<String>,
    inputs: Vec<String>,
    excluded_paths: Vec<String>,
    policy_root: Option<String>,
    method_root: Option<String>,
    format: OutputFormat,
) -> CliResult<i32> {
    let request = RunRequest {
        mode: mode.parse::<Mode>().map_err(CliError::InvalidInput)?,
        risk: risk.parse::<RiskClass>().map_err(CliError::InvalidInput)?,
        zone: zone.parse::<UsageZone>().map_err(CliError::InvalidInput)?,
        owner: owner.unwrap_or_default(),
        inputs,
        excluded_paths,
        policy_root,
        method_root,
    };
    let summary = service.run(request)?;
    output::print_value(&summary, format)?;
    Ok(exit_code_for_state(&summary.state))
}

#[cfg(test)]
mod tests {
    use canon_engine::EngineService;

    use super::execute;
    use crate::app::OutputFormat;

    #[test]
    fn execute_rejects_unknown_mode_before_service_execution() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "not-a-mode".to_string(),
            "low-impact".to_string(),
            "green".to_string(),
            None,
            Vec::new(),
            Vec::new(),
            None,
            None,
            OutputFormat::Json,
        )
        .expect_err("invalid mode should fail");

        assert!(error.to_string().contains("unsupported mode: not-a-mode"));
    }

    #[test]
    fn execute_rejects_unknown_risk_before_service_execution() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "requirements".to_string(),
            "not-a-risk".to_string(),
            "green".to_string(),
            None,
            Vec::new(),
            Vec::new(),
            None,
            None,
            OutputFormat::Json,
        )
        .expect_err("invalid risk should fail");

        assert!(error.to_string().contains("unsupported risk class: not-a-risk"));
    }

    #[test]
    fn execute_rejects_unknown_zone_before_service_execution() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "requirements".to_string(),
            "low-impact".to_string(),
            "not-a-zone".to_string(),
            None,
            Vec::new(),
            Vec::new(),
            None,
            None,
            OutputFormat::Json,
        )
        .expect_err("invalid zone should fail");

        assert!(error.to_string().contains("unsupported usage zone: not-a-zone"));
    }
}
