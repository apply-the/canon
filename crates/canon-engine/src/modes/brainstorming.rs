/// The method definition filename for brainstorming mode.
pub const MODE_FILE: &str = "brainstorming.toml";
/// The ordered step sequence for brainstorming mode execution.
pub const STEP_SEQUENCE: &[&str] = &[
    "capture-context",
    "classify-risk",
    "govern-repository-context",
    "explore-conceptual-approaches",
    "evaluate-tradeoffs",
    "propose-spikes",
    "emit-artifacts",
    "evaluate-gates",
];
pub use super::discovery::{GOVERNED_CAPABILITIES, REQUIRED_GATES};

pub const ARTIFACT_CONTEXT_SLUG: &str = "context.md";
pub const ARTIFACT_OPTIONS_SLUG: &str = "options.md";
pub const ARTIFACT_TRADEOFFS_SLUG: &str = "tradeoffs.md";
pub const ARTIFACT_SPIKES_SLUG: &str = "spikes.md";

pub const HEADING_CONTEXT: &str = "Context";
pub const HEADING_SUMMARY: &str = "Summary";
pub const HEADING_OPTIONS: &str = "Options";
pub const HEADING_TRADEOFFS: &str = "Tradeoffs";
pub const HEADING_SPIKES: &str = "Spikes";
