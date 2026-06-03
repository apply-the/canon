use crate::domain::gate::GateKind;

/// The method definition filename for debugging mode.
pub const MODE_FILE: &str = "debugging.toml";

/// The ordered step sequence for debugging mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "generate-hypothesis",
    "record-red-state",
    "isolate-root-cause",
    "apply-fix",
    "record-green-state",
    "evaluate-gates",
];

/// The gates required to close a debugging mode run.
pub const REQUIRED_GATES: &[GateKind] =
    &[GateKind::Reproduction, GateKind::TestDrivenDevelopment, GateKind::RootCause];
