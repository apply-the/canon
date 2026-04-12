use canon_engine::{AiTool, EngineService};

use crate::app::SkillsCommand;
use crate::output;

pub fn execute(
    service: &EngineService,
    command: SkillsCommand,
) -> Result<i32, Box<dyn std::error::Error>> {
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
