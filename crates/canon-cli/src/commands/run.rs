use canon_engine::{
    EngineService, RunRequest,
    domain::mode::Mode,
    domain::policy::{RiskClass, UsageZone},
    domain::run::{ClassificationFieldProvenance, ClassificationProvenance, ClassificationSource},
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
    risk_source: Option<String>,
    risk_rationale: Option<String>,
    risk_signals: Vec<String>,
    zone_source: Option<String>,
    zone_rationale: Option<String>,
    zone_signals: Vec<String>,
    owner: Option<String>,
    inputs: Vec<String>,
    inline_inputs: Vec<String>,
    excluded_paths: Vec<String>,
    policy_root: Option<String>,
    method_root: Option<String>,
    format: OutputFormat,
) -> CliResult<i32> {
    let risk_source = risk_source
        .as_deref()
        .unwrap_or("explicit")
        .parse::<ClassificationSource>()
        .map_err(CliError::InvalidInput)?;
    let zone_source = zone_source
        .as_deref()
        .unwrap_or("explicit")
        .parse::<ClassificationSource>()
        .map_err(CliError::InvalidInput)?;
    let request = RunRequest {
        mode: mode.parse::<Mode>().map_err(CliError::InvalidInput)?,
        risk: risk.parse::<RiskClass>().map_err(CliError::InvalidInput)?,
        zone: zone.parse::<UsageZone>().map_err(CliError::InvalidInput)?,
        classification: ClassificationProvenance {
            risk: classification_field(risk_source, risk_rationale, risk_signals, "Risk class"),
            zone: classification_field(zone_source, zone_rationale, zone_signals, "Usage zone"),
        },
        owner: owner.unwrap_or_default(),
        inputs,
        inline_inputs,
        excluded_paths,
        policy_root,
        method_root,
    };
    let summary = service.run(request)?;
    output::print_run_summary(&summary, format)?;
    Ok(exit_code_for_state(&summary.state))
}

fn classification_field(
    source: ClassificationSource,
    rationale: Option<String>,
    signals: Vec<String>,
    label: &str,
) -> ClassificationFieldProvenance {
    let fallback = match source {
        ClassificationSource::Explicit => {
            format!("{label} was supplied explicitly at run start.")
        }
        ClassificationSource::InferredConfirmed => {
            format!("{label} was inferred upstream and confirmed before Canon run start.")
        }
        ClassificationSource::InferredOverridden => {
            format!("{label} was inferred upstream and then overridden before Canon run start.")
        }
    };

    ClassificationFieldProvenance::new(source, rationale.unwrap_or(fallback), signals)
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
            None,
            Vec::new(),
            None,
            None,
            Vec::new(),
            None,
            Vec::new(),
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
            None,
            Vec::new(),
            None,
            None,
            Vec::new(),
            None,
            Vec::new(),
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
            None,
            Vec::new(),
            None,
            None,
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            OutputFormat::Json,
        )
        .expect_err("invalid zone should fail");

        assert!(error.to_string().contains("unsupported usage zone: not-a-zone"));
    }

    #[test]
    fn execute_rejects_unknown_classification_source_before_service_execution() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "requirements".to_string(),
            "low-impact".to_string(),
            "green".to_string(),
            Some("not-a-source".to_string()),
            None,
            Vec::new(),
            None,
            None,
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            OutputFormat::Json,
        )
        .expect_err("invalid classification source should fail");

        assert!(error.to_string().contains("unsupported classification source: not-a-source"));
    }
}
