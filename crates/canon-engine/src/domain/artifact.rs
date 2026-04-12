use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::domain::execution::EvidenceDisposition;
use crate::domain::gate::GateKind;
use crate::domain::mode::Mode;
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
    pub provenance: Option<ArtifactProvenance>,
}

impl ArtifactRecord {
    pub fn validate_relative_path(&self, run_id: &str, mode: Mode) -> Result<(), String> {
        if self.file_name.trim().is_empty() {
            return Err("artifact file_name must not be empty".to_string());
        }

        if self.relative_path.trim().is_empty() {
            return Err(format!(
                "artifact `{}` must declare a non-empty relative_path",
                self.file_name
            ));
        }

        let path = Path::new(&self.relative_path);
        if path.is_absolute() {
            return Err(format!(
                "artifact `{}` must remain under .canon/artifacts/ and cannot use an absolute path",
                self.file_name
            ));
        }

        let components = path
            .components()
            .map(|component| match component {
                Component::Normal(value) => value.to_str().ok_or_else(|| {
                    format!(
                        "artifact `{}` relative_path must contain valid UTF-8 path segments",
                        self.file_name
                    )
                }),
                Component::CurDir
                | Component::ParentDir
                | Component::RootDir
                | Component::Prefix(_) => Err(format!(
                    "artifact `{}` must not escape .canon/artifacts/ with traversal or root components",
                    self.file_name
                )),
            })
            .collect::<Result<Vec<_>, _>>()?;

        let expected = ["artifacts", run_id, mode.as_str(), self.file_name.as_str()];
        if components.as_slice() != expected {
            return Err(format!(
                "artifact `{}` must use relative_path `artifacts/{}/{}/{}`; found `{}`",
                self.file_name,
                run_id,
                mode.as_str(),
                self.file_name,
                self.relative_path
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactProvenance {
    pub request_ids: Vec<String>,
    pub evidence_bundle: Option<String>,
    pub disposition: EvidenceDisposition,
}

#[cfg(test)]
mod tests {
    use super::{ArtifactFormat, ArtifactRecord};
    use crate::domain::mode::Mode;

    fn sample_record(relative_path: &str) -> ArtifactRecord {
        ArtifactRecord {
            file_name: "analysis.md".to_string(),
            relative_path: relative_path.to_string(),
            format: ArtifactFormat::Markdown,
            provenance: None,
        }
    }

    #[test]
    fn validate_relative_path_accepts_expected_run_scoped_artifact_path() {
        let record = sample_record("artifacts/run-123/requirements/analysis.md");

        let result = record.validate_relative_path("run-123", Mode::Requirements);

        assert!(result.is_ok());
    }

    #[test]
    fn validate_relative_path_rejects_absolute_paths() {
        let record = sample_record("/tmp/analysis.md");

        let error = record
            .validate_relative_path("run-123", Mode::Requirements)
            .expect_err("absolute path should fail");

        assert!(error.contains("cannot use an absolute path"));
    }

    #[test]
    fn validate_relative_path_rejects_parent_traversal() {
        let record = sample_record("artifacts/run-123/requirements/../analysis.md");

        let error = record
            .validate_relative_path("run-123", Mode::Requirements)
            .expect_err("parent traversal should fail");

        assert!(
            error.contains("must not escape .canon/artifacts/ with traversal or root components")
        );
    }
}
