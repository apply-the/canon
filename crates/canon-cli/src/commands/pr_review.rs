//! CLI handlers for onion-layer PR review: prepare, accept, finalize.
//!
//! Delegates to the EngineService orchestrators for each phase.

use canon_engine::EngineService;

use crate::app::PrReviewCommand;
use crate::error::{CliError, CliResult};

/// Dispatches a [`PrReviewCommand`] to the appropriate engine orchestrator.
pub fn execute(service: &EngineService, command: PrReviewCommand) -> CliResult<i32> {
    match command {
        PrReviewCommand::Prepare { run, base, head, skip_early_signal, skip_reason, output } => {
            // Validate skip-reason requirement (FR-015, T021)
            if skip_early_signal && skip_reason.as_deref().is_none_or(|r| r.trim().is_empty()) {
                return Err(CliError::InvalidInput(
                    "--skip-early-signal requires a non-empty --skip-reason".to_string(),
                ));
            }
            let run_id = run.unwrap_or_else(generate_run_id);
            let skip_reason_ref = skip_reason.as_deref();
            service
                .run_pr_review_prepare(&run_id, &base, &head, skip_early_signal, skip_reason_ref)
                .map_err(CliError::InvalidInput)?;
            if output == crate::app::OutputFormat::Json {
                println!(r#"{{"event":"prepare.completed","run_id":"{run_id}","status":"ready"}}"#);
            } else {
                println!("Prepare complete. Run ID: {run_id}");
                println!(
                    "Next: submit reviewer-output.md and run `canon pr-review accept --run {run_id}`"
                );
            }
            Ok(0)
        }
        PrReviewCommand::Accept { run } => {
            service.run_pr_review_accept(&run).map_err(CliError::InvalidInput)?;
            println!("Accept complete. Run ID: {run}");
            println!("Next: run `canon pr-review finalize --run {run}`");
            Ok(0)
        }
        PrReviewCommand::Finalize { run } => {
            service.run_pr_review_finalize(&run).map_err(CliError::InvalidInput)?;
            println!("Finalize complete. Run ID: {run}");
            println!("Artifacts written to .canon/runs/{run}/pr-review/");
            Ok(0)
        }
    }
}

/// Generates a short run identifier from the current timestamp.
fn generate_run_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    format!("pr-review-{ts}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_run_id_produces_non_empty_string() {
        let id = generate_run_id();
        assert!(id.starts_with("pr-review-"));
        assert!(id.len() > "pr-review-".len());
    }

    #[test]
    fn test_generate_run_id_is_deterministic_within_same_second() {
        let id1 = generate_run_id();
        let id2 = generate_run_id();
        assert_eq!(id1, id2);
    }
}
