use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

pub const MODE_FILE: &str = "discovery.toml";
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "explore-problem-domain",
    "challenge-assumptions",
    "emit-artifacts",
    "evaluate-gates",
];
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Risk, GateKind::ReleaseReadiness];
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
];
