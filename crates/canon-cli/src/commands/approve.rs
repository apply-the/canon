use canon_engine::{ApprovalSummary, EngineService, domain::approval::ApprovalDecision};

use crate::error::{CliError, CliResult};

pub fn execute(
    service: &EngineService,
    run: &str,
    target: Option<String>,
    gate: Option<String>,
    by: Option<String>,
    decision: String,
    rationale: String,
) -> CliResult<i32> {
    let target = match (target, gate) {
        (Some(target), None) => target,
        (None, Some(gate)) => format!("gate:{gate}"),
        (Some(target), Some(_)) => target,
        (None, None) => {
            return Err(CliError::InvalidInput("approval target is required".to_string()));
        }
    };
    let summary: ApprovalSummary = service.approve(
        run,
        &target,
        by.as_deref().unwrap_or_default(),
        decision.parse::<ApprovalDecision>().map_err(CliError::InvalidInput)?,
        &rationale,
    )?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(0)
}

#[cfg(test)]
mod tests {
    use canon_engine::EngineService;

    use super::execute;

    #[test]
    fn execute_requires_target_or_gate() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "run-123",
            None,
            None,
            None,
            "approve".to_string(),
            "looks good".to_string(),
        )
        .expect_err("missing target should fail");

        assert!(error.to_string().contains("approval target is required"));
    }

    #[test]
    fn execute_rejects_unknown_decision_before_service_execution() {
        let service = EngineService::new(".");

        let error = execute(
            &service,
            "run-123",
            Some("gate:risk".to_string()),
            None,
            None,
            "defer".to_string(),
            "needs discussion".to_string(),
        )
        .expect_err("invalid decision should fail");

        assert!(error.to_string().contains("unsupported approval decision: defer"));
    }
}
