use canon_engine::EngineService;

use crate::app::OutputFormat;
use crate::output;

pub fn execute(
    service: &EngineService,
    run: &str,
    format: OutputFormat,
) -> Result<i32, Box<dyn std::error::Error>> {
    let summary = service.status(run)?;
    output::print_value(&summary, format)?;
    Ok(0)
}
