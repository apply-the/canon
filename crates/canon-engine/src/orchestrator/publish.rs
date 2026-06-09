use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::EngineError;
use crate::domain::artifact::artifact_slug;
use crate::domain::mode::Mode;
use crate::domain::publish_profile::{
    ArtifactIndexingMetadata, CANON_PRODUCER, ExpertiseInputMetadata, LineageMetadata,
    ManagedBlockDescriptor, PROJECT_MEMORY_CONTRACT_VERSION,
    PROJECT_MEMORY_PACKET_METADATA_FILE_NAME, PromotionState, PublicationTargetClass,
    PublishProfile, SemanticArtifactDescriptor, UpdateStrategy, classify_governed_expertise_input,
};
use crate::domain::run::RunState;
use crate::persistence::manifests::RunManifest;
use crate::persistence::slug::slugify;
use crate::persistence::store::{PersistedArtifact, WorkspaceStore};

mod adr;
mod metadata;
mod policy;
mod support;
mod surface;

use adr::{adr_export_enabled, build_adr_export};
use metadata::{resolve_expertise_input_metadata, runtime_packet_metadata};
use policy::{default_update_strategy_for, evaluate_promotion_policy, resolve_profile_destination};
use support::{
    content_digest_for_artifacts, display_path, packet_readiness_for, publish_date,
    publish_descriptor, resolve_destination, source_artifact_path, titleize_slug,
};
use surface::{profile_metadata_path, publish_profile_directory, publish_profile_surface};

#[cfg(test)]
use adr::{
    build_architecture_adr, build_change_adr, build_migration_adr, fallback_adr_title,
    labeled_sections, next_adr_number, normalize_adr_title_line, preferred_section,
};

#[cfg(test)]
use metadata::infer_runtime_packet_metadata;

#[cfg(test)]
use policy::{
    evidence_project_memory_surface, pending_project_memory_surface, stable_project_memory_surface,
};

#[cfg(test)]
use support::{default_publish_directory, short_id_fragment};

#[cfg(test)]
use surface::{
    append_index_entry, render_project_memory_surface, write_managed_block, write_proposal_file,
};

const ADR_REGISTRY_DIRECTORY: &str = "tech-docs/adr";
const PROFILE_METADATA_FILE_SUFFIX: &str = ".packet-metadata.json";

/// Summary returned after publishing a Canon run's artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PublishSummary {
    /// The run ID whose artifacts were published.
    pub run_id: String,
    /// The governed mode of the published run.
    pub mode: String,
    /// The destination path the artifacts were published to.
    pub published_to: String,
    /// The list of files emitted to the destination.
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
    #[serde(default)]
    primary_artifact: String,
    #[serde(default)]
    artifact_order: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    publish_order: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    legacy_aliases: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    artifact_indexing: Option<ArtifactIndexingMetadata>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    semantic_descriptor: Option<SemanticArtifactDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    profile: Option<PublishProfile>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    promotion_state: Option<PromotionState>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    update_strategy: Option<UpdateStrategy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    publication_target_class: Option<PublicationTargetClass>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    expertise_input: Option<ExpertiseInputMetadata>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    lineage: Option<LineageMetadata>,
}

#[derive(Debug, Clone, Copy)]
struct PublishProfileWriteContext<'a> {
    repo_root: &'a Path,
    manifest: &'a RunManifest,
    strategy: UpdateStrategy,
    promotion: PromotionState,
    publish_timestamp: &'a str,
    artifacts: &'a [PersistedArtifact],
    lineage: &'a LineageMetadata,
}

fn operational_packet_state_publishable(state: &RunState) -> bool {
    matches!(state, RunState::AwaitingApproval | RunState::Blocked | RunState::Completed)
}

fn create_parent_directory(path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Publishes the artifacts from the named run to the given destination using the default profile.
pub fn publish_run(
    repo_root: &Path,
    canon_workspace_root: &Path,
    run_id: &str,
    destination_override: Option<&Path>,
    adr: bool,
) -> Result<PublishSummary, EngineError> {
    let store = WorkspaceStore::from_roots(repo_root, canon_workspace_root);
    let manifest = store.load_run_manifest(run_id)?;
    let state = store.load_run_state(run_id)?;

    let operational_packet_publishable = matches!(
        manifest.mode,
        Mode::Incident
            | Mode::SecurityAssessment
            | Mode::Migration
            | Mode::SystemAssessment
            | Mode::SupplyChainAnalysis
            | Mode::DomainLanguage
            | Mode::DomainModel
    ) && operational_packet_state_publishable(&state.state);

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

    let export_adr = adr_export_enabled(manifest.mode, adr)?;

    let destination = resolve_destination(repo_root, &manifest, destination_override);
    if destination.exists() && !destination.is_dir() {
        return Err(EngineError::Validation(format!(
            "publish destination `{}` must be a directory",
            destination.display()
        )));
    }
    fs::create_dir_all(&destination)?;

    let generated_adr = if export_adr {
        Some(build_adr_export(repo_root, &manifest, &artifacts, &destination)?)
    } else {
        None
    };

    let source_artifacts = artifacts
        .iter()
        .map(|artifact| {
            source_artifact_path(&manifest.run_id, manifest.mode, &artifact.record.file_name)
        })
        .collect::<Vec<_>>();
    let packet_metadata = runtime_packet_metadata(&artifacts);
    packet_metadata.validate_artifact_indexing().map_err(EngineError::Validation)?;
    packet_metadata.validate_semantic_descriptor().map_err(EngineError::Validation)?;

    let mut published_files = Vec::with_capacity(artifacts.len() + 1);
    for artifact in &artifacts {
        if artifact_slug(&artifact.record.file_name) == PROJECT_MEMORY_PACKET_METADATA_FILE_NAME {
            continue;
        }
        let destination_path = destination.join(&artifact.record.file_name);
        fs::write(&destination_path, &artifact.contents)?;
        published_files.push(display_path(repo_root, &destination_path));
    }

    let metadata_path = destination.join(PROJECT_MEMORY_PACKET_METADATA_FILE_NAME);
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
        primary_artifact: packet_metadata.primary_artifact,
        artifact_order: packet_metadata.artifact_order,
        publish_order: packet_metadata.publish_order,
        legacy_aliases: packet_metadata.legacy_aliases,
        artifact_indexing: packet_metadata.artifact_indexing,
        semantic_descriptor: packet_metadata.semantic_descriptor,
        profile: None,
        promotion_state: None,
        update_strategy: None,
        publication_target_class: packet_metadata.publication_target_class,
        expertise_input: None,
        lineage: None,
    };
    let metadata_contents = serde_json::to_vec_pretty(&metadata).map_err(|error| {
        EngineError::Validation(format!("failed to serialize publish metadata: {error}"))
    })?;
    fs::write(&metadata_path, metadata_contents)?;
    published_files.push(display_path(repo_root, &metadata_path));

    if let Some(generated_adr) = generated_adr {
        create_parent_directory(&generated_adr.destination)?;
        fs::write(&generated_adr.destination, generated_adr.contents)?;
        published_files.push(generated_adr.display_path);
    }

    Ok(PublishSummary {
        run_id: run_id.to_string(),
        mode: manifest.mode.as_str().to_string(),
        published_to: display_path(repo_root, &destination),
        published_files,
    })
}

/// Publish a run using a named profile. The profile determines the promotion
/// state, update strategy, and lineage metadata for project-memory promotion.
pub fn publish_run_with_profile(
    repo_root: &Path,
    canon_workspace_root: &Path,
    run_id: &str,
    profile: PublishProfile,
    destination_override: Option<&Path>,
) -> Result<PublishSummary, EngineError> {
    let store = WorkspaceStore::from_roots(repo_root, canon_workspace_root);
    let manifest = store.load_run_manifest(run_id)?;
    let state = store.load_run_state(run_id)?;

    let promotion = evaluate_promotion_policy(manifest.mode, &state.state, &manifest);

    if promotion == PromotionState::Manual {
        return Err(EngineError::Validation(format!(
            "run `{run_id}` requires manual promotion; publish with `canon publish` instead of profile `{}`",
            profile.as_str()
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

    let strategy = default_update_strategy_for(manifest.mode);

    let destination = match destination_override {
        Some(path) if path.is_absolute() => path.to_path_buf(),
        Some(path) => repo_root.join(path),
        None => resolve_profile_destination(repo_root, &manifest, &promotion),
    };

    let source_artifacts = artifacts
        .iter()
        .map(|artifact| {
            source_artifact_path(&manifest.run_id, manifest.mode, &artifact.record.file_name)
        })
        .collect::<Vec<_>>();
    let mut packet_metadata = runtime_packet_metadata(&artifacts);
    packet_metadata.validate_artifact_indexing().map_err(EngineError::Validation)?;
    packet_metadata.validate_semantic_descriptor().map_err(EngineError::Validation)?;
    let expertise_input =
        resolve_expertise_input_metadata(repo_root, manifest.mode, &packet_metadata);
    let publication_target_class = PublicationTargetClass::for_publication(promotion, strategy);
    let artifact_indexing = ArtifactIndexingMetadata::for_publication(
        publication_target_class,
        strategy,
    )
    .map_err(|error| {
        EngineError::Validation(format!("unsupported artifact indexing projection: {error}"))
    })?;
    packet_metadata.publication_target_class = Some(publication_target_class);
    packet_metadata.artifact_indexing = Some(artifact_indexing.clone());

    let publish_timestamp = OffsetDateTime::now_utc().format(&Rfc3339).map_err(|error| {
        EngineError::Validation(format!("failed to format publish timestamp: {error}"))
    })?;

    let lineage = LineageMetadata {
        contract_version: PROJECT_MEMORY_CONTRACT_VERSION.to_string(),
        producer: CANON_PRODUCER.to_string(),
        source_ref: format!("canon-run:{}", manifest.run_id),
        source_artifacts: source_artifacts.clone(),
        promotion_state: promotion,
        promoted_at: publish_timestamp.clone(),
        content_digest: content_digest_for_artifacts(&artifacts),
        mode: Some(manifest.mode.as_str().to_string()),
        stage: None,
        owner: Some(manifest.owner.clone()),
        risk: Some(manifest.risk.as_str().to_string()),
        zone: Some(manifest.zone.as_str().to_string()),
        approval_state: Some(format!("{:?}", state.state)),
        packet_readiness: Some(packet_readiness_for(&state.state).to_string()),
        promotion_profile: Some(profile),
    };

    let mut published_files = Vec::with_capacity(artifacts.len() + 2);
    let publish_context = PublishProfileWriteContext {
        repo_root,
        manifest: &manifest,
        strategy,
        promotion,
        publish_timestamp: &publish_timestamp,
        artifacts: &artifacts,
        lineage: &lineage,
    };

    let published_to_path = if destination_override.is_some() {
        publish_profile_directory(&publish_context, &destination, &mut published_files)?
    } else {
        publish_profile_surface(&publish_context, &destination, &mut published_files)?
    };

    let metadata_path = profile_metadata_path(&published_to_path);
    create_parent_directory(&metadata_path)?;
    let metadata = PublishMetadata {
        run_id: manifest.run_id.clone(),
        mode: manifest.mode.as_str().to_string(),
        risk: manifest.risk.as_str().to_string(),
        zone: manifest.zone.as_str().to_string(),
        publish_timestamp,
        descriptor: publish_descriptor(&manifest),
        destination: display_path(repo_root, &published_to_path),
        source_artifacts,
        primary_artifact: packet_metadata.primary_artifact,
        artifact_order: packet_metadata.artifact_order,
        publish_order: packet_metadata.publish_order,
        legacy_aliases: packet_metadata.legacy_aliases,
        artifact_indexing: Some(artifact_indexing),
        semantic_descriptor: packet_metadata.semantic_descriptor,
        profile: Some(profile),
        promotion_state: Some(promotion),
        update_strategy: Some(strategy),
        publication_target_class: Some(publication_target_class),
        expertise_input,
        lineage: Some(lineage),
    };
    let metadata_contents = serde_json::to_vec_pretty(&metadata).map_err(|error| {
        EngineError::Validation(format!("failed to serialize publish metadata: {error}"))
    })?;
    fs::write(&metadata_path, metadata_contents)?;
    published_files.push(display_path(repo_root, &metadata_path));

    Ok(PublishSummary {
        run_id: run_id.to_string(),
        mode: manifest.mode.as_str().to_string(),
        published_to: display_path(repo_root, &published_to_path),
        published_files,
    })
}

#[cfg(test)]
mod tests;
