use std::io::{Error, ErrorKind};

use canon_engine::{ApprovalSummary, EngineService, domain::approval::ApprovalDecision};

pub fn execute(
    service: &EngineService,
    run: &str,
    target: Option<String>,
    gate: Option<String>,
    by: String,
    decision: String,
    rationale: String,
) -> Result<i32, Box<dyn std::error::Error>> {
    let target = match (target, gate) {
        (Some(target), None) => target,
        (None, Some(gate)) => format!("gate:{gate}"),
        (Some(target), Some(_)) => target,
        (None, None) => {
            return Err(Box::new(Error::new(
                ErrorKind::InvalidInput,
                "approval target is required",
            )));
        }
    };
    let summary: ApprovalSummary = service.approve(
        run,
        &target,
        &by,
        decision
            .parse::<ApprovalDecision>()
            .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?,
        &rationale,
    )?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(0)
}
