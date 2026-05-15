use crate::domain::gate::GateKind;

/// The method definition filename for PR review mode.
pub const MODE_FILE: &str = "pr-review.toml";
/// The ordered step sequence for PR review mode execution.
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
/// The gates required to close a PR review mode run.
pub const REQUIRED_GATES: &[GateKind] = &[
    GateKind::Risk,
    GateKind::Architecture,
    GateKind::ReviewDisposition,
    GateKind::ReleaseReadiness,
];
