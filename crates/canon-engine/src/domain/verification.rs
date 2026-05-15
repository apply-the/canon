use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// Identifies the specific critique or review layer that produced a verification record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationLayer {
    /// First-pass self-critique by the generating agent.
    SelfCritique,
    /// Adversarial critique by a separate agent or prompt instance.
    AdversarialCritique,
    /// Structured human peer review of generated content.
    PeerReview,
    /// Architectural-level human or agent review of structural decisions.
    ArchitecturalReview,
}

/// A single verification record produced during a governed critique or review cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationRecord {
    /// The critique or review layer that produced this record.
    pub layer: VerificationLayer,
    /// Paths that were the target of this verification record.
    pub target_paths: Vec<String>,
    /// Human-readable disposition: `accepted`, `rejected`, or a qualified statement.
    pub disposition: String,
    /// When this verification record was created.
    pub recorded_at: OffsetDateTime,
    /// Request IDs of the invocations that contributed to this record.
    pub request_ids: Vec<String>,
    /// The validation path ID this record belongs to, if any.
    pub validation_path_id: Option<String>,
    /// An optional evidence bundle reference linking to supporting artifacts.
    pub evidence_bundle: Option<String>,
}
