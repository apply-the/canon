//! Deterministic verification of persisted Canon evidence structure.

use canon_engine::{EngineService, InspectTarget};
use serde::Serialize;

use crate::error::{CliError, CliResult};

const STRUCTURAL_STATUS: &str = "deterministic_structure_valid";
const SEMANTIC_STATUS: &str = "required_missing";
const INCOMPLETE_VERIFICATION_EXIT_CODE: i32 = 5;

#[derive(Debug, Serialize)]
struct VerifyProjection<'a> {
    run_id: &'a str,
    deterministic_status: &'static str,
    external_semantic_judgment: &'static str,
    evidence_entry_count: usize,
}

/// Verifies that a run has a readable typed evidence projection.
///
/// Structural success is projected separately from the nonzero terminal
/// status caused by missing external semantic evidence.
pub fn execute(service: &EngineService, run_id: &str) -> CliResult<i32> {
    let response = service.inspect(InspectTarget::Evidence { run_id: run_id.to_string() })?;
    if response.entries.is_empty() {
        return Err(CliError::InvalidInput(format!(
            "run `{run_id}` has no persisted evidence projection to verify"
        )));
    }

    let projection = VerifyProjection {
        run_id,
        deterministic_status: STRUCTURAL_STATUS,
        external_semantic_judgment: SEMANTIC_STATUS,
        evidence_entry_count: response.entries.len(),
    };
    println!("{}", serde_json::to_string(&projection)?);
    Ok(INCOMPLETE_VERIFICATION_EXIT_CODE)
}

#[cfg(test)]
mod tests {
    use canon_engine::{
        EngineService, RunRequest,
        domain::{
            mode::Mode,
            policy::{RiskClass, UsageZone},
            run::{ClassificationProvenance, SystemContext},
        },
    };
    use tempfile::tempdir;

    use super::execute;

    #[test]
    fn execute_rejects_unknown_run_without_claiming_verification() {
        let workspace = tempdir().expect("create temp workspace");
        let service = EngineService::new(workspace.path());
        let error = execute(&service, "missing-run")
            .expect_err("an unknown run cannot produce deterministic verification");

        assert!(!error.to_string().contains("not implemented"));
        assert!(error.to_string().contains("missing-run"));
    }

    #[test]
    fn execute_reports_incomplete_when_external_semantic_evidence_is_absent()
    -> Result<(), Box<dyn std::error::Error>> {
        let workspace = tempdir()?;
        let service = EngineService::new(workspace.path());
        let summary = service.run(RunRequest {
            mode: Mode::Verification,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "independent-reviewer".to_string(),
            inputs: Vec::new(),
            inline_inputs: vec![
                "# Verification Brief\n\n## Claims Under Test\n- rollback is bounded\n\n\
                 ## Invariant Checks\n- rollback metadata remains explicit\n\n\
                 ## Contract Assumptions\n- evidence remains external\n\n\
                 ## Verification Outcome\nStatus: supported\n\n\
                 ## Challenge Findings\n- none authored\n\n\
                 ## Contradictions\n- none authored\n\n\
                 ## Verified Claims\n- rollback is bounded\n\n\
                 ## Rejected Claims\n- none authored\n\n\
                 ## Overall Verdict\nStatus: supported\n\n\
                 ## Open Findings\nStatus: no-open-findings\n\n\
                 ## Required Follow-Up\n- obtain independent external evidence\n"
                    .to_string(),
            ],
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        })?;

        assert_eq!(execute(&service, &summary.run_id)?, 5);
        Ok(())
    }
}
