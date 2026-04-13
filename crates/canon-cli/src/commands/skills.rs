use canon_engine::{AiTool, EngineService};

use crate::app::SkillsCommand;
use crate::error::CliResult;
use crate::output;

pub fn execute(service: &EngineService, command: SkillsCommand) -> CliResult<i32> {
    match command {
        SkillsCommand::Install { ai, output } => {
            let summary = service.skills_install(AiTool::from(ai))?;
            output::print_value(&summary, output)?;
            Ok(0)
        }
        SkillsCommand::Update { ai, output } => {
            let summary = service.skills_update(AiTool::from(ai))?;
            output::print_value(&summary, output)?;
            Ok(0)
        }
        SkillsCommand::List { output } => {
            let entries = service.skills_list();
            output::print_value(&entries, output)?;
            Ok(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use canon_engine::EngineService;
    use tempfile::tempdir;

    use super::execute;
    use crate::app::{AiTarget, OutputFormat, SkillsCommand};

    #[test]
    fn execute_lists_embedded_skills_without_mutating_runtime_state() {
        let service = EngineService::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."));

        let code = execute(&service, SkillsCommand::List { output: OutputFormat::Json })
            .expect("skills list should succeed");

        assert_eq!(code, 0);
    }

    #[test]
    fn execute_install_and_update_materialize_skills_in_temp_workspace() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());

        let install = execute(
            &service,
            SkillsCommand::Install { ai: AiTarget::Codex, output: OutputFormat::Json },
        )
        .expect("skills install should succeed");
        let update = execute(
            &service,
            SkillsCommand::Update { ai: AiTarget::Codex, output: OutputFormat::Json },
        )
        .expect("skills update should succeed");

        assert_eq!(install, 0);
        assert_eq!(update, 0);
        assert!(workspace.path().join(".agents").join("skills").is_dir());
    }
}
