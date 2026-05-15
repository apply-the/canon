use crate::domain::gate::GateKind;

/// The method definition filename for refactor mode.
pub const MODE_FILE: &str = "refactor.toml";
/// The ordered step sequence for refactor mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "freeze-preserved-behavior",
    "bound-refactor-scope",
    "record-safety-net-evidence",
    "record-recommendation-only-execution",
    "emit-artifacts",
    "evaluate-gates",
];
/// The gates required to close a refactor mode run.
pub const REQUIRED_GATES: &[GateKind] = &[
    GateKind::ChangePreservation,
    GateKind::Architecture,
    GateKind::Risk,
    GateKind::ReleaseReadiness,
];
