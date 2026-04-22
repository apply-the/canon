use crate::domain::gate::GateKind;

pub const MODE_FILE: &str = "change.toml";
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "map-system-slice",
    "freeze-legacy-invariants",
    "govern-change-framing",
    "bound-change-surface",
    "record-validation-evidence",
    "record-recommendation-only-mutation",
    "define-validation-strategy",
    "evaluate-gates",
];
pub const REQUIRED_GATES: &[GateKind] = &[
    GateKind::Exploration,
    GateKind::ChangePreservation,
    GateKind::Architecture,
    GateKind::Risk,
    GateKind::ReleaseReadiness,
];
