use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// A persisted decision record produced during a governed run.
///
/// Decision records capture choices made during execution that affect future
/// runs, architecture, or risk posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionRecord {
    /// Unique identifier for this decision (e.g. a short slug or UUID).
    pub id: String,
    /// One-line summary of the decision made.
    pub summary: String,
    /// Human-readable explanation of why this decision was made.
    pub rationale: String,
    /// When this decision was recorded.
    pub recorded_at: OffsetDateTime,
}
