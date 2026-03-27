use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub id: String,
    pub summary: String,
    pub rationale: String,
    pub recorded_at: OffsetDateTime,
}
