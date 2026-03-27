use serde::{Deserialize, Serialize};

use crate::domain::artifact::ArtifactRecord;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactManifest {
    pub records: Vec<ArtifactRecord>,
}
