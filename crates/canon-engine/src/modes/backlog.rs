use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

/// The method definition filename for backlog mode.
pub const MODE_FILE: &str = "backlog.toml";
/// The ordered step sequence for backlog mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "define-artifact-contract",
    "evaluate-closure",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close a backlog mode run.
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Architecture, GateKind::Risk, GateKind::ReleaseReadiness];

/// The capabilities governed during backlog mode execution.
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
    CapabilityKind::ProposeWorkspaceEdit,
];
