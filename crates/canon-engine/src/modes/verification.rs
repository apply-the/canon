use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

pub const MODE_FILE: &str = "verification.toml";
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "generate-verification-frame",
    "challenge-claims",
    "validate-evidence",
    "emit-artifacts",
    "evaluate-gates",
];
pub const REQUIRED_GATES: &[GateKind] = &[GateKind::Risk, GateKind::ReleaseReadiness];
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
    CapabilityKind::ValidateWithTool,
];
