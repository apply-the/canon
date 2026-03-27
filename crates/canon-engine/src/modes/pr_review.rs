use crate::domain::gate::GateKind;

pub const MODE_FILE: &str = "pr-review.toml";
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "collect-diff",
    "map-changed-surfaces",
    "render-review-packet",
    "evaluate-gates",
];
pub const REQUIRED_GATES: &[GateKind] = &[
    GateKind::Risk,
    GateKind::Architecture,
    GateKind::ReviewDisposition,
    GateKind::ReleaseReadiness,
];
