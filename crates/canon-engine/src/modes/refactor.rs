use crate::domain::gate::GateKind;

pub const MODE_FILE: &str = "refactor.toml";
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
pub const REQUIRED_GATES: &[GateKind] = &[
    GateKind::ChangePreservation,
    GateKind::Architecture,
    GateKind::Risk,
    GateKind::ReleaseReadiness,
];
