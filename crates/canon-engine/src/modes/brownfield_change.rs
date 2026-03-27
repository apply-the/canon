use crate::domain::gate::GateKind;

pub const MODE_FILE: &str = "brownfield-change.toml";
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "map-system-slice",
    "freeze-legacy-invariants",
    "bound-change-surface",
    "define-validation-strategy",
    "evaluate-gates",
];
pub const REQUIRED_GATES: &[GateKind] = &[
    GateKind::Exploration,
    GateKind::BrownfieldPreservation,
    GateKind::Architecture,
    GateKind::Risk,
    GateKind::ReleaseReadiness,
];
