use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

/// The method definition filename for discovery mode.
pub const MODE_FILE: &str = "discovery.toml";
/// The ordered step sequence for discovery mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "explore-problem-domain",
    "challenge-assumptions",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close a discovery mode run.
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Risk, GateKind::ReleaseReadiness];
/// The capabilities governed during discovery mode execution.
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
];
