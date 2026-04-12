use canon_engine::{AiTool, EngineService};

use crate::app::OutputFormat;
use crate::output;

pub fn execute(
    service: &EngineService,
    ai_tool: Option<AiTool>,
    format: OutputFormat,
) -> Result<i32, Box<dyn std::error::Error>> {
    let summary = service.init(ai_tool)?;
    output::print_value(&summary, format)?;
    Ok(0)
}
