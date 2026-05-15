use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

/// The method definition filename for system shaping mode.
pub const MODE_FILE: &str = "system-shaping.toml";
/// The ordered step sequence for system shaping mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "shape-system-boundaries",
    "critique-architecture-options",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close a system shaping mode run.
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Architecture, GateKind::Risk, GateKind::ReleaseReadiness];
/// The capabilities governed during system shaping mode execution.
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
];
