use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::EngineError;
use crate::domain::mode::Mode;
use crate::domain::run::RunState;
use crate::persistence::store::WorkspaceStore;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublishSummary {
    pub run_id: String,
    pub mode: String,
    pub published_to: String,
    pub published_files: Vec<String>,
}

pub fn publish_run(
    repo_root: &Path,
    run_id: &str,
    destination_override: Option<&Path>,
) -> Result<PublishSummary, EngineError> {
    let store = WorkspaceStore::new(repo_root);
    let manifest = store.load_run_manifest(run_id)?;
    let state = store.load_run_state(run_id)?;

    if state.state != RunState::Completed {
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

    let destination = resolve_destination(repo_root, manifest.mode, run_id, destination_override);
    if destination.exists() && !destination.is_dir() {
        return Err(EngineError::Validation(format!(
            "publish destination `{}` must be a directory",
            destination.display()
        )));
    }
    fs::create_dir_all(&destination)?;

    let mut published_files = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let destination_path = destination.join(&artifact.record.file_name);
        fs::write(&destination_path, artifact.contents)?;
        published_files.push(display_path(repo_root, &destination_path));
    }

    Ok(PublishSummary {
        run_id: run_id.to_string(),
        mode: manifest.mode.as_str().to_string(),
        published_to: display_path(repo_root, &destination),
        published_files,
    })
}

fn resolve_destination(
    repo_root: &Path,
    mode: Mode,
    run_id: &str,
    destination_override: Option<&Path>,
) -> PathBuf {
    match destination_override {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => repo_root.join(path),
        None => repo_root.join(default_publish_directory(mode)).join(run_id),
    }
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
        Mode::Architecture => "docs/architecture/decisions",
        Mode::Implementation => "docs/implementation",
        Mode::Refactor => "docs/refactors",
        Mode::Verification => "docs/verification",
        Mode::Review => "docs/reviews",
        Mode::PrReview => "docs/reviews/prs",
        Mode::Incident => "docs/incidents",
        Mode::Migration => "docs/migrations",
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::{default_publish_directory, resolve_destination};
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::ClassificationProvenance;
    use crate::orchestrator::service::{EngineService, RunRequest};

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

    #[test]
    fn default_publish_directory_maps_supported_modes() {
        assert_eq!(default_publish_directory(Mode::Requirements), "specs");
        assert_eq!(default_publish_directory(Mode::Architecture), "docs/architecture/decisions");
        assert_eq!(default_publish_directory(Mode::PrReview), "docs/reviews/prs");
    }

    #[test]
    fn resolve_destination_uses_run_scoped_default_or_override() {
        let repo_root = Path::new("/repo");

        assert_eq!(
            resolve_destination(repo_root, Mode::Requirements, "R-20260422-abcd1234", None),
            Path::new("/repo/specs/R-20260422-abcd1234")
        );
        assert_eq!(
            resolve_destination(
                repo_root,
                Mode::Requirements,
                "R-20260422-abcd1234",
                Some(Path::new("docs/public/prd"))
            ),
            Path::new("/repo/docs/public/prd")
        );
    }

    #[test]
    fn publish_run_rejects_destination_that_is_an_existing_file() {
        let workspace = tempdir().expect("temp workspace");
        fs::write(workspace.path().join("idea.md"), "# Idea\n\nPublish branch coverage test.\n")
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
        fs::write(
            workspace.path().join("idea.md"),
            "# Idea\n\nAbsolute publish destination coverage test.\n",
        )
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
    }
}
