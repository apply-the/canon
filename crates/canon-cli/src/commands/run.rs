use canon_engine::{
    EngineService, RunRequest,
    domain::mode::Mode,
    domain::policy::{RiskClass, UsageZone},
    domain::run::{
        ClassificationFieldProvenance, ClassificationProvenance, ClassificationSource,
        SystemContext,
    },
};

use crate::app::OutputFormat;
use crate::commands::exit_code_for_state;
use crate::error::{CliError, CliResult};
use crate::output;

#[allow(clippy::too_many_arguments)]
pub fn execute(
    service: &EngineService,
    mode: String,
    system_context: Option<String>,
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
        system_context: system_context
            .as_deref()
            .map(str::parse::<SystemContext>)
            .transpose()
            .map_err(CliError::InvalidInput)?,
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
    use std::fs;

    use canon_engine::EngineService;
    use canon_engine::domain::run::ClassificationSource;
    use tempfile::tempdir;

    use super::{classification_field, execute};
    use crate::app::OutputFormat;

    #[test]
    fn execute_runs_requirements_brief_and_returns_state_exit_code() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(
            workspace.path().join("idea.md"),
            "# Requirements Brief\n\n## Problem\n\nLift CLI wrapper coverage.\n",
        )
        .expect("write idea file");
        let service = EngineService::new(workspace.path());

        let code = execute(
            &service,
            "requirements".to_string(),
            None,
            "low-impact".to_string(),
            "green".to_string(),
            None,
            None,
            vec!["explicit-risk".to_string()],
            None,
            None,
            vec!["explicit-zone".to_string()],
            Some("Owner <owner@example.com>".to_string()),
            vec!["idea.md".to_string()],
            Vec::new(),
            Vec::new(),
            None,
            None,
            OutputFormat::Json,
        )
        .expect("requirements run should succeed");

        assert_eq!(code, 2);
    }

    #[test]
    fn classification_field_uses_source_fallbacks_and_preserves_override_rationale() {
        let explicit = classification_field(
            ClassificationSource::Explicit,
            None,
            vec!["user-input".to_string()],
            "Risk class",
        );
        assert_eq!(explicit.source, ClassificationSource::Explicit);
        assert_eq!(explicit.rationale, "Risk class was supplied explicitly at run start.");
        assert_eq!(explicit.signals, vec!["user-input".to_string()]);

        let inferred_confirmed = classification_field(
            ClassificationSource::InferredConfirmed,
            None,
            vec!["inspect-risk-zone".to_string()],
            "Usage zone",
        );
        assert_eq!(inferred_confirmed.source, ClassificationSource::InferredConfirmed);
        assert_eq!(
            inferred_confirmed.rationale,
            "Usage zone was inferred upstream and confirmed before Canon run start."
        );

        let inferred_overridden = classification_field(
            ClassificationSource::InferredOverridden,
            None,
            Vec::new(),
            "Risk class",
        );
        assert_eq!(
            inferred_overridden.rationale,
            "Risk class was inferred upstream and then overridden before Canon run start."
        );

        let overridden_rationale = classification_field(
            ClassificationSource::Explicit,
            Some("operator override".to_string()),
            Vec::new(),
            "Usage zone",
        );
        assert_eq!(overridden_rationale.rationale, "operator override");
    }

    #[test]
    fn execute_rejects_unknown_mode_before_service_execution() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "not-a-mode".to_string(),
            None,
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
            None,
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
            None,
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
            None,
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
