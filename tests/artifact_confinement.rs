use std::fs;
use std::io::ErrorKind;

use canon_engine::artifacts::manifest::ArtifactManifest;
use canon_engine::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactRecord, ArtifactRequirement,
};
use canon_engine::domain::mode::Mode;
use canon_engine::domain::policy::{RiskClass, UsageZone};
use canon_engine::domain::run::{ClassificationProvenance, RunContext, RunState};
use canon_engine::persistence::manifests::{LinkManifest, RunManifest, RunStateManifest};
use canon_engine::persistence::store::{PersistedArtifact, PersistedRunBundle, WorkspaceStore};
use tempfile::TempDir;
use time::OffsetDateTime;

fn sample_bundle(repo_root: &str, relative_path: &str) -> PersistedRunBundle {
    let run_id = "run-artifact-confinement";
    let mode = Mode::Requirements;

    PersistedRunBundle {
        run: RunManifest {
            run_id: run_id.to_string(),
            uuid: None,
            short_id: None,
            slug: None,
            title: None,
            mode,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "owner@example.com".to_string(),
            created_at: OffsetDateTime::UNIX_EPOCH,
        },
        context: RunContext {
            repo_root: repo_root.to_string(),
            owner: Some("owner@example.com".to_string()),
            inputs: vec!["brief.md".to_string()],
            excluded_paths: Vec::new(),
            input_fingerprints: Vec::new(),
            system_context: None,
            upstream_context: None,
            implementation_execution: None,
            refactor_execution: None,
            backlog_planning: None,
            inline_inputs: Vec::new(),
            captured_at: OffsetDateTime::UNIX_EPOCH,
        },
        state: RunStateManifest {
            state: RunState::Completed,
            updated_at: OffsetDateTime::UNIX_EPOCH,
        },
        artifact_contract: ArtifactContract {
            version: 1,
            artifact_requirements: vec![ArtifactRequirement {
                file_name: "analysis.md".to_string(),
                format: ArtifactFormat::Markdown,
                required_sections: Vec::new(),
                gates: Vec::new(),
            }],
            required_verification_layers: Vec::new(),
        },
        artifacts: vec![PersistedArtifact {
            record: ArtifactRecord {
                file_name: "analysis.md".to_string(),
                relative_path: relative_path.to_string(),
                format: ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: "# Analysis\n".to_string(),
        }],
        links: LinkManifest {
            artifacts: Vec::new(),
            decisions: Vec::new(),
            traces: Vec::new(),
            invocations: Vec::new(),
            evidence: None,
        },
        gates: Vec::new(),
        approvals: Vec::new(),
        verification_records: Vec::new(),
        evidence: None,
        invocations: Vec::new(),
    }
}

#[test]
fn persist_run_bundle_rejects_artifact_paths_outside_canon() {
    let workspace = TempDir::new().expect("temp dir");
    let store = WorkspaceStore::new(workspace.path());
    let bundle = sample_bundle(
        workspace.path().to_str().expect("workspace path"),
        "../null_parameter_analysis.md",
    );

    let error = store.persist_run_bundle(&bundle).expect_err("invalid artifact path should fail");

    assert_eq!(error.kind(), ErrorKind::InvalidData);
    assert!(
        error
            .to_string()
            .contains("must not escape .canon/artifacts/ with traversal or root components"),
        "unexpected error: {error}"
    );
    assert!(
        !workspace.path().join(".canon").exists(),
        "invalid bundle should not materialize runtime state"
    );
    assert!(
        !workspace.path().join("null_parameter_analysis.md").exists(),
        "invalid artifact record must not create a root-level file"
    );
}

#[test]
fn inspect_artifacts_rejects_manifest_records_that_escape_run_directory() {
    let workspace = TempDir::new().expect("temp dir");
    let store = WorkspaceStore::new(workspace.path());
    store.init_runtime_state(None).expect("init runtime state");

    let artifact_dir =
        store.layout.run_artifact_dir("run-artifact-confinement", Mode::Requirements);
    fs::create_dir_all(&artifact_dir).expect("artifact dir");
    let manifest = ArtifactManifest {
        records: vec![ArtifactRecord {
            file_name: "analysis.md".to_string(),
            relative_path: "artifacts/run-artifact-confinement/requirements/../analysis.md"
                .to_string(),
            format: ArtifactFormat::Markdown,
            provenance: None,
        }],
    };
    fs::write(
        artifact_dir.join("manifest.toml"),
        toml::to_string(&manifest).expect("manifest toml"),
    )
    .expect("write manifest");

    let error = store
        .list_artifact_files("run-artifact-confinement")
        .expect_err("invalid manifest should fail");

    assert_eq!(error.kind(), ErrorKind::InvalidData);
    assert!(
        error
            .to_string()
            .contains("must not escape .canon/artifacts/ with traversal or root components"),
        "unexpected error: {error}"
    );
}
