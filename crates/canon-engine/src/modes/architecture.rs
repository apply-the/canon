use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

/// The method definition filename for architecture mode.
pub const MODE_FILE: &str = "architecture.toml";
/// The ordered step sequence for architecture mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "evaluate-structural-options",
    "challenge-architectural-claims",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close an architecture mode run.
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Architecture, GateKind::Risk, GateKind::ReleaseReadiness];
/// The capabilities governed during architecture mode execution.
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
];
