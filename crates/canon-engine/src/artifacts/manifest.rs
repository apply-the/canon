use serde::{Deserialize, Serialize};

use crate::domain::artifact::ArtifactRecord;

/// An ordered list of artifact records emitted during a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactManifest {
    /// The artifact records, in emission order.
    pub records: Vec<ArtifactRecord>,
}
