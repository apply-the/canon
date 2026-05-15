use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

/// The method definition filename for verification mode.
pub const MODE_FILE: &str = "verification.toml";
/// The ordered step sequence for verification mode execution.
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
/// The gates required to close a verification mode run.
pub const REQUIRED_GATES: &[GateKind] = &[GateKind::Risk, GateKind::ReleaseReadiness];
/// The capabilities governed during verification mode execution.
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
    CapabilityKind::ValidateWithTool,
];
