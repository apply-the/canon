use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

pub const MODE_FILE: &str = "backlog.toml";
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "define-artifact-contract",
    "evaluate-closure",
    "emit-artifacts",
    "evaluate-gates",
];
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Architecture, GateKind::Risk, GateKind::ReleaseReadiness];

pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
    CapabilityKind::ProposeWorkspaceEdit,
];
