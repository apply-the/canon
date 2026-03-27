use std::io::{Error, ErrorKind};

use canon_engine::{
    ApprovalSummary, EngineService, domain::approval::ApprovalDecision, domain::gate::GateKind,
};

pub fn execute(
    service: &EngineService,
    run: &str,
    gate: String,
    by: String,
    decision: String,
    rationale: String,
) -> Result<i32, Box<dyn std::error::Error>> {
    let summary: ApprovalSummary = service.approve(
        run,
        gate.parse::<GateKind>().map_err(|error| Error::new(ErrorKind::InvalidInput, error))?,
        &by,
        decision
            .parse::<ApprovalDecision>()
            .map_err(|error| Error::new(ErrorKind::InvalidInput, error))?,
        &rationale,
    )?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(0)
}
