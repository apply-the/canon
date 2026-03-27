use crate::domain::gate::GateKind;

pub const MODE_FILE: &str = "requirements.toml";
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "define-artifact-contract",
    "emit-artifacts",
    "evaluate-gates",
];
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Exploration, GateKind::Risk, GateKind::ReleaseReadiness];
