use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationLayer {
    SelfCritique,
    AdversarialCritique,
    PeerReview,
    ArchitecturalReview,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationRecord {
    pub layer: VerificationLayer,
    pub target_paths: Vec<String>,
    pub disposition: String,
    pub recorded_at: OffsetDateTime,
}
