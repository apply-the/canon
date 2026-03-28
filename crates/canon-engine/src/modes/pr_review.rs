use crate::domain::gate::GateKind;

pub const MODE_FILE: &str = "pr-review.toml";
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-diff-inspection",
    "map-changed-surfaces",
    "govern-review-critique",
    "assemble-evidence",
    "render-review-packet",
    "evaluate-gates",
];
pub const REQUIRED_GATES: &[GateKind] = &[
    GateKind::Risk,
    GateKind::Architecture,
    GateKind::ReviewDisposition,
    GateKind::ReleaseReadiness,
];
