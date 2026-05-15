use crate::domain::gate::GateKind;
use canon_adapters::CapabilityKind;

/// The method definition filename for review mode.
pub const MODE_FILE: &str = "review.toml";
/// The ordered step sequence for review mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "generate-review-brief",
    "challenge-review-claims",
    "validate-review-evidence",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close a review mode run.
pub const REQUIRED_GATES: &[GateKind] = &[
    GateKind::Risk,
    GateKind::Architecture,
    GateKind::ReviewDisposition,
    GateKind::ReleaseReadiness,
];
/// The capabilities governed during review mode execution.
pub const GOVERNED_CAPABILITIES: &[CapabilityKind] = &[
    CapabilityKind::ReadRepository,
    CapabilityKind::GenerateContent,
    CapabilityKind::CritiqueContent,
    CapabilityKind::ValidateWithTool,
];
