//! Internal-only inspection support for legacy Canon modes.
//!
//! Stable inspection admits only frozen profiles; historical behavior remains
//! covered directly through the engine boundary.

use canon_engine::EngineService;
use canon_engine::domain::mode::Mode;
use canon_engine::orchestrator::service::{InspectResponse, InspectTarget};

/// Inspects clarity for a historical mode without public profile admission.
pub fn clarity(
    workspace: &std::path::Path,
    mode: Mode,
    inputs: Vec<String>,
) -> Result<InspectResponse, Box<dyn std::error::Error>> {
    let service = EngineService::new(workspace);
    Ok(service.inspect(InspectTarget::Clarity { mode, inputs })?)
}
