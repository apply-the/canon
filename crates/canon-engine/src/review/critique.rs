use serde::{Deserialize, Serialize};

use crate::domain::verification::VerificationLayer;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CritiqueNote {
    pub layer: VerificationLayer,
    pub summary: String,
}
