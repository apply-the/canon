use std::path::{Component, Path};

use serde::{Deserialize, Serialize};

use crate::domain::execution::EvidenceDisposition;
use crate::domain::gate::GateKind;
use crate::domain::mode::Mode;
use crate::domain::publish_profile::ExpertiseInputMetadata;
use crate::domain::verification::VerificationLayer;

pub const VIEW_MANIFEST_FILE_NAME: &str = "view-manifest.json";
pub const RUNTIME_PACKET_METADATA_FILE_NAME: &str = "packet-metadata.json";
pub const PR_ANALYSIS_ARTIFACT_SLUG: &str = "pr-analysis.md";
pub const REVIEW_SUMMARY_ARTIFACT_SLUG: &str = "review-summary.md";
pub const CONVENTIONAL_COMMENTS_ARTIFACT_SLUG: &str = "conventional-comments.md";
pub const REPOSITORY_METADATA_DIRECTORY_NAME: &str = ".git";
pub const CANON_RUNTIME_DIRECTORY_NAME: &str = ".canon";
pub const BUILD_OUTPUT_DIRECTORY_NAME: &str = "target";
pub const DEPENDENCY_DIRECTORY_NAME: &str = "node_modules";

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
    #[serde(default = "default_artifact_required")]
    pub required: bool,
}

impl ArtifactRequirement {
    /// Return the bare slug (without ordinal prefix) for this artifact.
    pub fn slug(&self) -> &str {
        artifact_slug(&self.file_name)
    }
}

/// Strip the `NN-` ordinal prefix from an artifact filename, returning the bare slug.
///
/// For example, `"01-problem-statement.md"` returns `"problem-statement.md"`.
/// If no two-digit-dash prefix is present the input is returned unchanged.
pub fn artifact_slug(file_name: &str) -> &str {
    let bytes = file_name.as_bytes();
    if bytes.len() > 3 && bytes[0].is_ascii_digit() && bytes[1].is_ascii_digit() && bytes[2] == b'-'
    {
        &file_name[3..]
    } else {
        file_name
    }
}

/// Build the prefixed artifact filename from a 1-based ordinal and a bare slug.
pub fn prefixed_artifact_name(ordinal: usize, slug: &str) -> String {
    format!("{ordinal:02}-{slug}")
}

/// Return `true` if `file_name` refers to a packet sidecar rather than a body artifact.
///
/// Sidecars (`view-manifest.json` and `packet-metadata.json`) are emitted alongside
/// body artifacts but are excluded from ordering, primary-artifact resolution, and
/// consumer-facing packet listings.
pub fn is_packet_sidecar(file_name: &str) -> bool {
    matches!(artifact_slug(file_name), VIEW_MANIFEST_FILE_NAME | RUNTIME_PACKET_METADATA_FILE_NAME)
}

pub fn is_special_repository_directory(name: &str) -> bool {
    matches!(
        name,
        REPOSITORY_METADATA_DIRECTORY_NAME
            | CANON_RUNTIME_DIRECTORY_NAME
            | BUILD_OUTPUT_DIRECTORY_NAME
            | DEPENDENCY_DIRECTORY_NAME
    )
}

pub fn should_skip_repo_scan_directory(name: &str) -> bool {
    name.starts_with('.') || matches!(name, BUILD_OUTPUT_DIRECTORY_NAME | DEPENDENCY_DIRECTORY_NAME)
}

/// Metadata sidecar persisted alongside every packet as `packet-metadata.json`.
///
/// Consumers use this file to reconstruct the canonical artifact sequence without
/// inspecting raw filenames or relying on filesystem ordering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RuntimePacketMetadata {
    #[serde(default)]
    pub primary_artifact: String,
    #[serde(default)]
    pub artifact_order: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub publish_order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub legacy_aliases: Option<std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub expertise_input: Option<ExpertiseInputMetadata>,
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

const fn default_artifact_required() -> bool {
    true
}

impl ArtifactRecord {
    /// Return the bare slug (without ordinal prefix) for this artifact.
    pub fn slug(&self) -> &str {
        artifact_slug(&self.file_name)
    }

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
    use super::{
        ArtifactFormat, ArtifactRecord, ArtifactRequirement, RUNTIME_PACKET_METADATA_FILE_NAME,
        VIEW_MANIFEST_FILE_NAME, is_packet_sidecar,
    };
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

    #[test]
    fn artifact_requirement_defaults_required_to_true_when_field_absent() {
        let json =
            r#"{"file_name":"overview.md","format":"Markdown","required_sections":[],"gates":[]}"#;
        let req: ArtifactRequirement = serde_json::from_str(json).unwrap();
        assert!(req.required, "required should default to true when the field is absent");
    }

    #[test]
    fn artifact_requirement_accepts_explicit_required_false() {
        let req: ArtifactRequirement = serde_json::from_str(
            r#"{"file_name":"optional.md","format":"Markdown","required_sections":[],"gates":[],"required":false}"#,
        )
        .unwrap();
        assert!(!req.required);
    }

    #[test]
    fn is_packet_sidecar_recognizes_architecture_sidecars() {
        assert!(is_packet_sidecar(VIEW_MANIFEST_FILE_NAME));
        assert!(is_packet_sidecar(RUNTIME_PACKET_METADATA_FILE_NAME));
        assert!(is_packet_sidecar("15-packet-metadata.json"));
        assert!(!is_packet_sidecar("container-view.mmd"));
    }
}
