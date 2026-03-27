use canon_engine::{EngineService, InspectTarget};

use crate::app::InspectCommand;
use crate::output;

pub fn execute(
    service: &EngineService,
    command: InspectCommand,
) -> Result<i32, Box<dyn std::error::Error>> {
    let (target, format) = match command {
        InspectCommand::Modes { output } => (InspectTarget::Modes, output),
        InspectCommand::Methods { output } => (InspectTarget::Methods, output),
        InspectCommand::Policies { output } => (InspectTarget::Policies, output),
        InspectCommand::Artifacts { run, output } => {
            (InspectTarget::Artifacts { run_id: run }, output)
        }
    };

    let response = service.inspect(target)?;
    output::print_value(&response, format)?;
    Ok(0)
}
