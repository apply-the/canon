use serde::{Deserialize, Serialize};

use crate::domain::gate::GateKind;
use crate::domain::verification::VerificationLayer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactFormat {
    Markdown,
    Json,
    Yaml,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRequirement {
    pub file_name: String,
    pub format: ArtifactFormat,
    pub required_sections: Vec<String>,
    pub gates: Vec<GateKind>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactContract {
    pub version: u32,
    pub artifact_requirements: Vec<ArtifactRequirement>,
    pub required_verification_layers: Vec<VerificationLayer>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRecord {
    pub file_name: String,
    pub relative_path: String,
    pub format: ArtifactFormat,
}
