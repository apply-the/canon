use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

/// The method definition filename for brainstorming mode.
pub const MODE_FILE: &str = "brainstorming.toml";
/// The ordered step sequence for brainstorming mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "explore-conceptual-approaches",
    "evaluate-tradeoffs",
    "propose-spikes",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close a brainstorming mode run.
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Risk, GateKind::ReleaseReadiness];
/// The capabilities governed during brainstorming mode execution.
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
];
