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
        InspectCommand::Invocations { run, output } => {
            (InspectTarget::Invocations { run_id: run }, output)
        }
        InspectCommand::Evidence { run, output } => {
            (InspectTarget::Evidence { run_id: run }, output)
        }
    };

    let response = service.inspect(target)?;
    output::print_inspect(&response, format)?;
    Ok(0)
}
