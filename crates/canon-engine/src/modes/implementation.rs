use crate::domain::gate::GateKind;

pub const MODE_FILE: &str = "implementation.toml";
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
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::ImplementationReadiness, GateKind::Risk, GateKind::ReleaseReadiness];
