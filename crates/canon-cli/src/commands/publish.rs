use canon_engine::EngineService;

use crate::app::PublishCommand;
use crate::error::CliResult;

pub fn execute(service: &EngineService, command: PublishCommand) -> CliResult<i32> {
    let summary = service.publish(&command.run_id, command.to)?;

    println!("Published run {}", summary.run_id);
    println!("Mode: {}", summary.mode);
    println!("Destination: {}", summary.published_to);
    if !summary.published_files.is_empty() {
        println!("Files:");
        for path in summary.published_files {
            println!("- {path}");
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::tempdir;

    use super::execute;
    use crate::app::PublishCommand;
    use canon_engine::domain::mode::Mode;
    use canon_engine::domain::policy::{RiskClass, UsageZone};
    use canon_engine::domain::run::ClassificationProvenance;
    use canon_engine::{EngineService, RunRequest};

    fn default_publish_leaf(run_id: &str, descriptor: &str) -> String {
        format!("{}-{}-{}-{descriptor}", &run_id[2..6], &run_id[6..8], &run_id[8..10])
    }

    fn requirements_request(risk: RiskClass, zone: UsageZone) -> RunRequest {
        RunRequest {
            mode: Mode::Requirements,
            risk,
            zone,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            inputs: vec!["idea.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        }
    }

    fn complete_requirements_brief() -> &'static str {
        "# Requirements Brief\n\n## Problem\n\nPublish completed requirements artifacts.\n\n## Outcome\n\nArtifacts are persisted and publishable.\n\n## Constraints\n\n- Keep output local-first.\n\n## Non-Negotiables\n\n- Artifacts must persist under .canon/.\n\n## Options\n\n1. Publish to default path.\n\n## Recommended Path\n\nPublish to the default mode directory.\n\n## Tradeoffs\n\n- Simpler path at cost of flexibility.\n\n## Consequences\n\n- Reviewers can inspect the packet.\n\n## Out of Scope\n\n- No hosted publishing.\n\n## Deferred Work\n\n- Remote destinations deferred.\n\n## Decision Checklist\n\n- [x] Scope is explicit.\n\n## Open Questions\n\n- None at this time.\n"
    }

    #[test]
    fn execute_publishes_completed_run_to_default_mode_directory() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
            .expect("write idea file");

        let service = EngineService::new(workspace.path());
        let run = service
            .run(requirements_request(RiskClass::LowImpact, UsageZone::Green))
            .expect("requirements run should succeed");

        let code = execute(&service, PublishCommand { run_id: run.run_id.clone(), to: None })
            .expect("publish should succeed");
        let published_dir =
            workspace.path().join("specs").join(default_publish_leaf(&run.run_id, "requirements"));

        assert_eq!(code, 0);
        assert!(published_dir.join("problem-statement.md").exists());
        assert!(published_dir.join("constraints.md").exists());
        assert!(published_dir.join("packet-metadata.json").exists());
    }

    #[test]
    fn execute_rejects_publish_when_run_is_still_awaiting_approval() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(
            workspace.path().join("idea.md"),
            "# Idea\n\nSystemic requirements framing needs approval.\n",
        )
        .expect("write idea file");

        let service = EngineService::new(workspace.path());
        let run = service
            .run(requirements_request(RiskClass::SystemicImpact, UsageZone::Yellow))
            .expect("requirements run should start");

        let error = execute(&service, PublishCommand { run_id: run.run_id, to: None })
            .expect_err("publish should fail for approval-gated run");

        assert!(error.to_string().contains("cannot publish run"));
    }

    #[test]
    fn execute_publishes_to_explicit_override_path() {
        let workspace = tempdir().expect("create temp workspace");
        fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
            .expect("write idea file");

        let service = EngineService::new(workspace.path());
        let run = service
            .run(requirements_request(RiskClass::LowImpact, UsageZone::Green))
            .expect("requirements run should succeed");

        let override_path = PathBuf::from("docs/public/prd");
        let code = execute(
            &service,
            PublishCommand { run_id: run.run_id.clone(), to: Some(override_path.clone()) },
        )
        .expect("publish should succeed");

        assert_eq!(code, 0);
        assert!(workspace.path().join(&override_path).join("problem-statement.md").exists());
        assert!(!workspace.path().join("specs").join(run.run_id).exists());
        assert!(workspace.path().join(&override_path).join("packet-metadata.json").exists());
    }
}
