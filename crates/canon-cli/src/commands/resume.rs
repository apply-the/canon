use canon_engine::EngineService;

use crate::commands::exit_code_for_state;

pub fn execute(service: &EngineService, run: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let summary = service.resume(run)?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(exit_code_for_state(&summary.state))
}
