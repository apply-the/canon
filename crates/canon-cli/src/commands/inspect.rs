use canon_engine::{EngineService, InspectTarget};

use crate::app::InspectCommand;
use crate::output;

pub fn execute(
    service: &EngineService,
    command: InspectCommand,
) -> Result<i32, Box<dyn std::error::Error>> {
    let (target, target_name, run_id, format) = match command {
        InspectCommand::Modes { output } => (InspectTarget::Modes, "modes", None, output),
        InspectCommand::Methods { output } => (InspectTarget::Methods, "methods", None, output),
        InspectCommand::Policies { output } => (InspectTarget::Policies, "policies", None, output),
        InspectCommand::Artifacts { run, output } => {
            (InspectTarget::Artifacts { run_id: run.clone() }, "artifacts", Some(run), output)
        }
        InspectCommand::Invocations { run, output } => {
            (InspectTarget::Invocations { run_id: run.clone() }, "invocations", Some(run), output)
        }
        InspectCommand::Evidence { run, output } => {
            (InspectTarget::Evidence { run_id: run.clone() }, "evidence", Some(run), output)
        }
    };

    let response = service.inspect(target)?;
    output::print_inspect(&response, target_name, run_id.as_deref(), format)?;
    Ok(0)
}
