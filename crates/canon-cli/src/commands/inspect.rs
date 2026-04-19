use canon_engine::{
    EngineService, InspectTarget,
    domain::mode::Mode,
    domain::policy::{RiskClass, UsageZone},
};

use crate::app::InspectCommand;
use crate::error::{CliError, CliResult};
use crate::output;

pub fn execute(service: &EngineService, command: InspectCommand) -> CliResult<i32> {
    let (target, target_name, run_id, format) = match command {
        InspectCommand::Modes { output } => (InspectTarget::Modes, "modes", None, output),
        InspectCommand::Methods { output } => (InspectTarget::Methods, "methods", None, output),
        InspectCommand::Policies { output } => (InspectTarget::Policies, "policies", None, output),
        InspectCommand::RiskZone { mode, risk, zone, inputs, inline_inputs, output } => (
            InspectTarget::RiskZone {
                mode: mode.parse::<Mode>().map_err(CliError::InvalidInput)?,
                risk: risk
                    .as_deref()
                    .map(str::parse::<RiskClass>)
                    .transpose()
                    .map_err(CliError::InvalidInput)?,
                zone: zone
                    .as_deref()
                    .map(str::parse::<UsageZone>)
                    .transpose()
                    .map_err(CliError::InvalidInput)?,
                inputs,
                inline_inputs,
            },
            "risk-zone",
            None,
            output,
        ),
        InspectCommand::Clarity { mode, inputs, output } => (
            InspectTarget::Clarity {
                mode: mode.parse::<Mode>().map_err(CliError::InvalidInput)?,
                inputs,
            },
            "clarity",
            None,
            output,
        ),
        InspectCommand::Artifacts { run, output } => {
            (InspectTarget::Artifacts { run_id: run.clone() }, "artifacts", Some(run), output)
        }
        InspectCommand::Invocations { run, output } => {
            (InspectTarget::Invocations { run_id: run.clone() }, "invocations", Some(run), output)
        }
        InspectCommand::Evidence { run, output } => {
            (InspectTarget::Evidence { run_id: run.clone() }, "evidence", Some(run), output)
        }
    };

    let response = service.inspect(target)?;
    output::print_inspect(&response, target_name, run_id.as_deref(), format)?;
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
        domain::run::ClassificationProvenance,
    };
    use tempfile::tempdir;

    use super::execute;
    use crate::app::{InspectCommand, OutputFormat};

    #[test]
    fn execute_supports_modes_inspection_without_runtime_state() {
        let service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

        let code = execute(&service, InspectCommand::Modes { output: OutputFormat::Json })
            .expect("inspect modes should succeed");

        assert_eq!(code, 0);
    }

    #[test]
    fn execute_supports_methods_and_policies_inspection() {
        let service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

        let methods = execute(&service, InspectCommand::Methods { output: OutputFormat::Json })
            .expect("inspect methods should succeed");
        let policies = execute(&service, InspectCommand::Policies { output: OutputFormat::Json })
            .expect("inspect policies should succeed");

        assert_eq!(methods, 0);
        assert_eq!(policies, 0);
    }

    #[test]
    fn execute_supports_risk_zone_inspection() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(
            workspace.path().join("discovery.md"),
            "# Discovery Brief\n\nProblem: production boundary drift.\nConstraints: preserve repo-local evidence.\n",
        )
        .expect("write discovery brief");
        let service = EngineService::new(workspace.path());

        let code = execute(
            &service,
            InspectCommand::RiskZone {
                mode: "discovery".to_string(),
                risk: None,
                zone: None,
                inputs: vec!["discovery.md".to_string()],
                inline_inputs: Vec::new(),
                output: OutputFormat::Json,
            },
        )
        .expect("inspect risk-zone should succeed");

        assert_eq!(code, 0);
    }

    #[test]
    fn execute_supports_clarity_inspection() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(
            workspace.path().join("requirements.md"),
            "# Requirements Brief\n\n## Problem\n\nBound the problem space.\n",
        )
        .expect("write requirements brief");
        let service = EngineService::new(workspace.path());

        let code = execute(
            &service,
            InspectCommand::Clarity {
                mode: "requirements".to_string(),
                inputs: vec!["requirements.md".to_string()],
                output: OutputFormat::Json,
            },
        )
        .expect("inspect clarity should succeed");

        assert_eq!(code, 0);
    }

    #[test]
    fn execute_supports_run_scoped_artifacts_invocations_and_evidence_inspection() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(workspace.path().join("idea.md"), "# Idea\n\nInspect wrapper coverage.\n")
            .expect("write idea file");
        let service = EngineService::new(workspace.path());
        let run = service
            .run(RunRequest {
                mode: Mode::Requirements,
                risk: RiskClass::LowImpact,
                zone: UsageZone::Green,
                classification: ClassificationProvenance::explicit(),
                owner: "Owner <owner@example.com>".to_string(),
                inputs: vec!["idea.md".to_string()],
                inline_inputs: Vec::new(),
                excluded_paths: Vec::new(),
                policy_root: None,
                method_root: None,
            })
            .expect("requirements run should succeed");

        let artifacts = execute(
            &service,
            InspectCommand::Artifacts { run: run.run_id.clone(), output: OutputFormat::Json },
        )
        .expect("inspect artifacts should succeed");
        let invocations = execute(
            &service,
            InspectCommand::Invocations { run: run.run_id.clone(), output: OutputFormat::Json },
        )
        .expect("inspect invocations should succeed");
        let evidence = execute(
            &service,
            InspectCommand::Evidence { run: run.run_id.clone(), output: OutputFormat::Json },
        )
        .expect("inspect evidence should succeed");

        assert_eq!(artifacts, 0);
        assert_eq!(invocations, 0);
        assert_eq!(evidence, 0);
    }
}
