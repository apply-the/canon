use crate::domain::gate::GateKind;

/// The method definition filename for implementation mode.
pub const MODE_FILE: &str = "implementation.toml";
/// The ordered step sequence for implementation mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "map-task-bounds",
    "record-safety-net-evidence",
    "record-recommendation-only-execution",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close an implementation mode run.
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::ImplementationReadiness, GateKind::Risk, GateKind::ReleaseReadiness];
