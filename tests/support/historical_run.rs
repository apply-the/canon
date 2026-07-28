//! Internal-only test support for legacy Canon modes.
//!
//! The stable CLI admits only the frozen profile registry. These helpers keep
//! historical runtime behavior covered without reopening public admission.

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, SystemContext};
use canon_engine::orchestrator::service::{RunRequest, RunSummary};

/// Starts a legacy-mode run through the internal engine boundary.
pub fn start(
    workspace: &std::path::Path,
    mode: Mode,
    risk: RiskClass,
    zone: UsageZone,
    system_context: SystemContext,
    owner: &str,
    input: &str,
) -> Result<RunSummary, Box<dyn std::error::Error>> {
    let service = EngineService::new(workspace);
    Ok(service.run(RunRequest {
        mode,
        risk,
        zone,
        system_context: Some(system_context),
        classification: ClassificationProvenance::explicit(),
        owner: owner.to_string(),
        inputs: vec![input.to_string()],
        inline_inputs: Vec::new(),
        excluded_paths: Vec::new(),
        policy_root: None,
        method_root: None,
    })?)
}
