use serde::{Deserialize, Serialize};

use crate::domain::verification::VerificationLayer;

/// A single critique note produced during governed diff inspection.
///
/// Critique notes are collected from the verification layer and attached to a
/// [`ReviewPacket`](crate::review::findings::ReviewPacket) before rendering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CritiqueNote {
    /// The verification layer this note was produced by.
    pub layer: VerificationLayer,
    /// Human-readable summary of the critique observation.
    pub summary: String,
}
