use canon_engine::EngineService;

use crate::app::OutputFormat;
use crate::output;

pub fn execute(
    service: &EngineService,
    format: OutputFormat,
) -> Result<i32, Box<dyn std::error::Error>> {
    let summary = service.init()?;
    output::print_value(&summary, format)?;
    Ok(0)
}
