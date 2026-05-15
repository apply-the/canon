use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

/// The method definition filename for requirements mode.
pub const MODE_FILE: &str = "requirements.toml";
/// The ordered step sequence for requirements mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "define-artifact-contract",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close a requirements mode run.
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Risk, GateKind::ReleaseReadiness];

/// The capabilities governed during requirements mode execution.
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
    CapabilityKind::ProposeWorkspaceEdit,
];
