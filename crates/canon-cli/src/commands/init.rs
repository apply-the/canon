use canon_engine::{AiTool, EngineService};

use crate::app::OutputFormat;
use crate::error::CliResult;
use crate::output;

pub fn execute(
    service: &EngineService,
    ai_tool: Option<AiTool>,
    format: OutputFormat,
) -> CliResult<i32> {
    let summary = service.init(ai_tool)?;
    output::print_value(&summary, format)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use canon_engine::EngineService;
    use tempfile::tempdir;

    use super::execute;
    use crate::app::OutputFormat;

    #[test]
    fn execute_initializes_runtime_state_in_temp_workspace() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());

        let code = execute(&service, None, OutputFormat::Json).expect("init should succeed");

        assert_eq!(code, 0);
        assert!(workspace.path().join(".canon").is_dir());
    }
}
