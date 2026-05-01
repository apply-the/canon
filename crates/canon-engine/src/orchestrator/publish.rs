use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::EngineError;
use crate::domain::mode::Mode;
use crate::domain::run::RunState;
use crate::persistence::manifests::RunManifest;
use crate::persistence::slug::slugify;
use crate::persistence::store::WorkspaceStore;

const PUBLISH_METADATA_FILE_NAME: &str = "packet-metadata.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublishSummary {
    pub run_id: String,
    pub mode: String,
    pub published_to: String,
    pub published_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PublishMetadata {
    run_id: String,
    mode: String,
    risk: String,
    zone: String,
    publish_timestamp: String,
    descriptor: String,
    destination: String,
    source_artifacts: Vec<String>,
}

pub fn publish_run(
    repo_root: &Path,
    run_id: &str,
    destination_override: Option<&Path>,
) -> Result<PublishSummary, EngineError> {
    let store = WorkspaceStore::new(repo_root);
    let manifest = store.load_run_manifest(run_id)?;
    let state = store.load_run_state(run_id)?;

    let operational_packet_publishable = matches!(
        manifest.mode,
        Mode::Incident | Mode::Migration | Mode::SystemAssessment | Mode::SupplyChainAnalysis
    ) && matches!(
        state.state,
        RunState::AwaitingApproval | RunState::Blocked | RunState::Completed
    );

    if state.state != RunState::Completed && !operational_packet_publishable {
        return Err(EngineError::Validation(format!(
            "cannot publish run `{run_id}` while state is `{:?}`; approval and resume must complete first",
            state.state
        )));
    }

    let contract = store.load_artifact_contract(run_id).map_err(|error| {
        EngineError::Validation(format!(
            "run `{run_id}` has no publishable artifact contract: {error}"
        ))
    })?;
    let artifacts =
        store.load_persisted_artifacts(run_id, manifest.mode, &contract).map_err(|error| {
            EngineError::Validation(format!("run `{run_id}` has no publishable artifacts: {error}"))
        })?;

    if artifacts.is_empty() {
        return Err(EngineError::Validation(format!(
            "run `{run_id}` has no publishable artifacts"
        )));
    }

    let destination = resolve_destination(repo_root, &manifest, destination_override);
    if destination.exists() && !destination.is_dir() {
        return Err(EngineError::Validation(format!(
            "publish destination `{}` must be a directory",
            destination.display()
        )));
    }
    fs::create_dir_all(&destination)?;

    let source_artifacts = artifacts
        .iter()
        .map(|artifact| {
            source_artifact_path(&manifest.run_id, manifest.mode, &artifact.record.file_name)
        })
        .collect::<Vec<_>>();

    let mut published_files = Vec::with_capacity(artifacts.len() + 1);
    for artifact in artifacts {
        let destination_path = destination.join(&artifact.record.file_name);
        fs::write(&destination_path, artifact.contents)?;
        published_files.push(display_path(repo_root, &destination_path));
    }

    let metadata_path = destination.join(PUBLISH_METADATA_FILE_NAME);
    let metadata = PublishMetadata {
        run_id: manifest.run_id.clone(),
        mode: manifest.mode.as_str().to_string(),
        risk: manifest.risk.as_str().to_string(),
        zone: manifest.zone.as_str().to_string(),
        publish_timestamp: OffsetDateTime::now_utc().format(&Rfc3339).map_err(|error| {
            EngineError::Validation(format!("failed to format publish timestamp: {error}"))
        })?,
        descriptor: publish_descriptor(&manifest),
        destination: display_path(repo_root, &destination),
        source_artifacts,
    };
    let metadata_contents = serde_json::to_vec_pretty(&metadata).map_err(|error| {
        EngineError::Validation(format!("failed to serialize publish metadata: {error}"))
    })?;
    fs::write(&metadata_path, metadata_contents)?;
    published_files.push(display_path(repo_root, &metadata_path));

    Ok(PublishSummary {
        run_id: run_id.to_string(),
        mode: manifest.mode.as_str().to_string(),
        published_to: display_path(repo_root, &destination),
        published_files,
    })
}

fn resolve_destination(
    repo_root: &Path,
    manifest: &RunManifest,
    destination_override: Option<&Path>,
) -> PathBuf {
    match destination_override {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => repo_root.join(path),
        None => resolve_default_destination(repo_root, manifest),
    }
}

fn resolve_default_destination(repo_root: &Path, manifest: &RunManifest) -> PathBuf {
    let family_root = repo_root.join(default_publish_directory(manifest.mode));
    let leaf = default_destination_leaf(manifest);
    let candidate = family_root.join(&leaf);

    if !candidate.exists() || existing_destination_matches_run(&candidate, &manifest.run_id) {
        return candidate;
    }

    family_root.join(format!("{leaf}--{}", short_id_fragment(manifest)))
}

fn default_destination_leaf(manifest: &RunManifest) -> String {
    format!("{}-{}", publish_date(manifest.created_at), publish_descriptor(manifest))
}

fn publish_descriptor(manifest: &RunManifest) -> String {
    manifest
        .slug
        .as_deref()
        .and_then(slugify)
        .or_else(|| manifest.title.as_deref().and_then(slugify))
        .unwrap_or_else(|| manifest.mode.as_str().to_string())
}

fn publish_date(timestamp: OffsetDateTime) -> String {
    let date = timestamp.date();
    format!("{:04}-{:02}-{:02}", date.year(), date.month() as u8, date.day())
}

fn short_id_fragment(manifest: &RunManifest) -> String {
    manifest.short_id.clone().unwrap_or_else(|| {
        manifest.run_id.chars().rev().take(8).collect::<Vec<_>>().into_iter().rev().collect()
    })
}

fn existing_destination_matches_run(destination: &Path, run_id: &str) -> bool {
    let metadata_path = destination.join(PUBLISH_METADATA_FILE_NAME);
    fs::read_to_string(metadata_path)
        .ok()
        .and_then(|raw| serde_json::from_str::<PublishMetadata>(&raw).ok())
        .is_some_and(|metadata| metadata.run_id == run_id)
}

fn source_artifact_path(run_id: &str, mode: Mode, file_name: &str) -> String {
    format!(".canon/artifacts/{run_id}/{}/{file_name}", mode.as_str())
}

fn display_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .map(|relative| relative.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

fn default_publish_directory(mode: Mode) -> &'static str {
    match mode {
        Mode::Requirements => "specs",
        Mode::Discovery => "docs/discovery",
        Mode::SystemShaping => "docs/architecture/shaping",
        Mode::Change => "docs/changes",
        Mode::Backlog => "docs/planning",
        Mode::Architecture => "docs/architecture/decisions",
        Mode::Implementation => "docs/implementation",
        Mode::Refactor => "docs/refactors",
        Mode::Verification => "docs/verification",
        Mode::Review => "docs/reviews",
        Mode::PrReview => "docs/reviews/prs",
        Mode::Incident => "docs/incidents",
        Mode::SystemAssessment => "docs/architecture/assessments",
        Mode::SecurityAssessment => "docs/security-assessments",
        Mode::Migration => "docs/migrations",
        Mode::SupplyChainAnalysis => "docs/supply-chain",
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;
    use time::OffsetDateTime;
    use time::format_description::well_known::Rfc3339;

    use super::{
        PUBLISH_METADATA_FILE_NAME, PublishMetadata, default_publish_directory, resolve_destination,
    };
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::ClassificationProvenance;
    use crate::orchestrator::service::{EngineService, RunRequest};
    use crate::persistence::manifests::RunManifest;

    fn complete_requirements_brief() -> &'static str {
        "# Requirements Brief\n\n## Problem\n\nPublish engine unit test coverage.\n\n## Outcome\n\nPublish functions are exercised under full artifact contracts.\n\n## Constraints\n\n- Keep output local-first.\n\n## Non-Negotiables\n\n- Artifacts must persist under .canon/.\n\n## Options\n\n1. Publish to default path.\n\n## Recommended Path\n\nPublish to the default mode directory.\n\n## Tradeoffs\n\n- Simpler path at cost of flexibility.\n\n## Consequences\n\n- Reviewers can inspect the packet.\n\n## Out of Scope\n\n- No hosted publishing.\n\n## Deferred Work\n\n- Remote destinations deferred.\n\n## Decision Checklist\n\n- [x] Scope is explicit.\n\n## Open Questions\n\n- None at this time.\n"
    }

    fn requirements_request() -> RunRequest {
        RunRequest {
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            inputs: vec!["idea.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        }
    }

    fn sample_manifest(run_id: &str) -> RunManifest {
        RunManifest {
            run_id: run_id.to_string(),
            uuid: Some("12345678123456781234567812345678".to_string()),
            short_id: Some("abcd1234".to_string()),
            slug: Some("publish-scope".to_string()),
            title: Some("Publish Scope".to_string()),
            mode: Mode::Requirements,
            risk: RiskClass::LowImpact,
            zone: UsageZone::Green,
            system_context: None,
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            created_at: OffsetDateTime::parse("2026-04-22T08:00:00Z", &Rfc3339)
                .expect("parse timestamp"),
        }
    }

    #[test]
    fn default_publish_directory_maps_supported_modes() {
        assert_eq!(default_publish_directory(Mode::Requirements), "specs");
        assert_eq!(default_publish_directory(Mode::Backlog), "docs/planning");
        assert_eq!(default_publish_directory(Mode::Architecture), "docs/architecture/decisions");
        assert_eq!(default_publish_directory(Mode::PrReview), "docs/reviews/prs");
    }

    #[test]
    fn resolve_destination_uses_structured_default_or_override() {
        let repo_root = Path::new("/repo");
        let manifest = sample_manifest("R-20260422-abcd1234");

        assert_eq!(
            resolve_destination(repo_root, &manifest, None),
            Path::new("/repo/specs/2026-04-22-publish-scope")
        );
        assert_eq!(
            resolve_destination(repo_root, &manifest, Some(Path::new("docs/public/prd"))),
            Path::new("/repo/docs/public/prd")
        );
    }

    #[test]
    fn resolve_destination_suffixes_collisions_for_other_runs() {
        let workspace = tempdir().expect("temp workspace");
        let manifest = sample_manifest("R-20260422-abcd1234");
        let collision_path = workspace.path().join("specs").join("2026-04-22-publish-scope");
        fs::create_dir_all(&collision_path).expect("collision dir");
        fs::write(
            collision_path.join(PUBLISH_METADATA_FILE_NAME),
            serde_json::to_vec_pretty(&PublishMetadata {
                run_id: "R-20260422-deadbeef".to_string(),
                mode: "requirements".to_string(),
                risk: "low-impact".to_string(),
                zone: "green".to_string(),
                publish_timestamp: "2026-04-22T08:00:00Z".to_string(),
                descriptor: "publish-scope".to_string(),
                destination: "specs/2026-04-22-publish-scope".to_string(),
                source_artifacts: vec![
                    ".canon/artifacts/R-20260422-deadbeef/requirements/problem-statement.md"
                        .to_string(),
                ],
            })
            .expect("metadata json"),
        )
        .expect("write collision metadata");

        assert_eq!(
            resolve_destination(workspace.path(), &manifest, None),
            workspace.path().join("specs").join("2026-04-22-publish-scope--abcd1234")
        );
    }

    #[test]
    fn publish_run_rejects_destination_that_is_an_existing_file() {
        let workspace = tempdir().expect("temp workspace");
        fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
            .expect("write input");

        let service = EngineService::new(workspace.path());
        let run = service.run(requirements_request()).expect("completed run");
        let destination_file = workspace.path().join("published.txt");
        fs::write(&destination_file, "not a directory").expect("write file destination");

        let error = super::publish_run(workspace.path(), &run.run_id, Some(&destination_file))
            .expect_err("publish should reject file destination");

        assert!(error.to_string().contains("must be a directory"));
    }

    #[test]
    fn publish_run_supports_absolute_override_outside_repo_root() {
        let workspace = tempdir().expect("temp workspace");
        fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
            .expect("write input");

        let external = tempdir().expect("external destination");
        let absolute_destination = external.path().join("published-packet");

        let service = EngineService::new(workspace.path());
        let run = service.run(requirements_request()).expect("completed run");

        let summary =
            super::publish_run(workspace.path(), &run.run_id, Some(&absolute_destination))
                .expect("publish should support absolute override");

        assert_eq!(summary.published_to, absolute_destination.display().to_string());
        assert!(absolute_destination.join("problem-statement.md").exists());
        assert!(
            summary.published_files.iter().any(|path| path
                == &absolute_destination.join("problem-statement.md").display().to_string())
        );
        assert!(absolute_destination.join(PUBLISH_METADATA_FILE_NAME).exists());
    }

    #[test]
    fn publish_run_writes_metadata_sidecar_for_default_destinations() {
        let workspace = tempdir().expect("temp workspace");
        fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
            .expect("write input");

        let service = EngineService::new(workspace.path());
        let run = service.run(requirements_request()).expect("completed run");

        let summary = super::publish_run(workspace.path(), &run.run_id, None)
            .expect("publish should succeed");
        let metadata_path =
            workspace.path().join(summary.published_to.clone()).join(PUBLISH_METADATA_FILE_NAME);
        let metadata: PublishMetadata =
            serde_json::from_slice(&fs::read(&metadata_path).expect("read metadata sidecar"))
                .expect("metadata json");

        assert_eq!(metadata.run_id, run.run_id);
        assert_eq!(metadata.mode, "requirements");
        assert_eq!(metadata.risk, "low-impact");
        assert_eq!(metadata.zone, "green");
        assert!(metadata.destination.starts_with("specs/"));
        assert!(
            metadata.source_artifacts.iter().any(|path| path.ends_with("problem-statement.md"))
        );
        assert!(
            summary.published_files.iter().any(|path| path.ends_with(PUBLISH_METADATA_FILE_NAME))
        );
    }
}
