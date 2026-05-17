use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::EngineError;
use crate::domain::artifact::{
    RuntimePacketMetadata, artifact_slug, is_packet_sidecar, should_skip_repo_scan_directory,
};
use crate::domain::mode::Mode;
use crate::domain::publish_profile::{
    ArtifactIndexingMetadata, CANON_PRODUCER, ExpertiseInputMetadata, LineageMetadata,
    ManagedBlockDescriptor, PROJECT_MEMORY_CONTRACT_VERSION,
    PROJECT_MEMORY_PACKET_METADATA_FILE_NAME, PromotionState, PublicationTargetClass,
    PublishProfile, UpdateStrategy, classify_governed_expertise_input,
};
use crate::domain::run::RunState;
use crate::persistence::manifests::RunManifest;
use crate::persistence::slug::slugify;
use crate::persistence::store::{PersistedArtifact, WorkspaceStore};

const ADR_REGISTRY_DIRECTORY: &str = "docs/adr";
const PROFILE_METADATA_FILE_SUFFIX: &str = ".packet-metadata.json";
const MAX_REPO_SCAN_DEPTH: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneratedAdr {
    destination: PathBuf,
    display_path: String,
    contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AdrDocument {
    title: String,
    context: String,
    decision: String,
    consequences: String,
    alternatives: Option<String>,
    source_packet: Option<String>,
}

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

/// Publishes the artifacts from the named run to the given destination using the default profile.
pub fn publish_run(
    repo_root: &Path,
    run_id: &str,
    destination_override: Option<&Path>,
    adr: bool,
) -> Result<PublishSummary, EngineError> {
    let store = WorkspaceStore::new(repo_root);
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
        if let Some(parent) = generated_adr.destination.parent() {
            fs::create_dir_all(parent)?;
        }
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
    run_id: &str,
    profile: PublishProfile,
    destination_override: Option<&Path>,
) -> Result<PublishSummary, EngineError> {
    let store = WorkspaceStore::new(repo_root);
    let manifest = store.load_run_manifest(run_id)?;
    let state = store.load_run_state(run_id)?;

    let promotion = evaluate_promotion_policy(manifest.mode, &state.state, &manifest);

    // Under project-memory profile, evidence-only and manual states skip
    // stable/pending surfaces entirely but still permit evidence publication.
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
        .map(|a| source_artifact_path(&manifest.run_id, manifest.mode, &a.record.file_name))
        .collect::<Vec<_>>();
    let mut packet_metadata = runtime_packet_metadata(&artifacts);
    packet_metadata.validate_artifact_indexing().map_err(EngineError::Validation)?;
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

    let published_to_path = publish_profile_contents(
        repo_root,
        &manifest,
        &destination,
        destination_override.is_some(),
        strategy,
        promotion,
        &publish_timestamp,
        &artifacts,
        &lineage,
        &mut published_files,
    )?;

    let metadata_path = profile_metadata_path(&published_to_path);
    if let Some(parent) = metadata_path.parent() {
        fs::create_dir_all(parent)?;
    }
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

fn publish_profile_contents(
    repo_root: &Path,
    manifest: &RunManifest,
    destination: &Path,
    destination_is_directory: bool,
    strategy: UpdateStrategy,
    promotion: PromotionState,
    publish_timestamp: &str,
    artifacts: &[PersistedArtifact],
    lineage: &LineageMetadata,
    published_files: &mut Vec<String>,
) -> Result<PathBuf, EngineError> {
    if destination_is_directory {
        publish_profile_directory(
            repo_root,
            manifest,
            destination,
            strategy,
            promotion,
            publish_timestamp,
            artifacts,
            lineage,
            published_files,
        )
    } else {
        publish_profile_surface(
            repo_root,
            manifest,
            destination,
            strategy,
            promotion,
            publish_timestamp,
            artifacts,
            lineage,
            published_files,
        )
    }
}

fn publish_profile_directory(
    repo_root: &Path,
    manifest: &RunManifest,
    destination: &Path,
    strategy: UpdateStrategy,
    promotion: PromotionState,
    publish_timestamp: &str,
    artifacts: &[PersistedArtifact],
    lineage: &LineageMetadata,
    published_files: &mut Vec<String>,
) -> Result<PathBuf, EngineError> {
    if destination.exists() && !destination.is_dir() {
        return Err(EngineError::Validation(format!(
            "publish destination `{}` must be a directory",
            destination.display()
        )));
    }
    fs::create_dir_all(destination)?;

    match strategy {
        UpdateStrategy::ManagedBlocks => {
            for artifact in artifacts {
                if artifact_slug(&artifact.record.file_name)
                    == PROJECT_MEMORY_PACKET_METADATA_FILE_NAME
                {
                    continue;
                }
                let target = destination.join(&artifact.record.file_name);
                write_managed_block(&target, &manifest.run_id, &artifact.contents)?;
                published_files.push(display_path(repo_root, &target));
            }
        }
        UpdateStrategy::ProposalFiles => {
            for artifact in artifacts {
                if artifact_slug(&artifact.record.file_name)
                    == PROJECT_MEMORY_PACKET_METADATA_FILE_NAME
                {
                    continue;
                }
                let proposal_path = write_proposal_file(
                    destination,
                    &artifact.record.file_name,
                    &artifact.contents,
                    lineage,
                )?;
                published_files.push(display_path(repo_root, &proposal_path));
            }
        }
        UpdateStrategy::AppendOnlyIndex => {
            let index_path = destination.join("index.md");
            let entry = format!(
                "## {}\n\n- Run: `{}`\n- Promotion: `{}`\n- Published: `{}`\n\n",
                publish_descriptor(manifest),
                manifest.run_id,
                promotion.as_str(),
                publish_timestamp,
            );
            append_index_entry(&index_path, &entry)?;
            published_files.push(display_path(repo_root, &index_path));
            let evidence_dir = destination.join(&manifest.run_id);
            for path in write_supporting_evidence_bundle(&evidence_dir, artifacts)? {
                published_files.push(display_path(repo_root, &path));
            }
        }
    }

    Ok(destination.to_path_buf())
}

fn publish_profile_surface(
    repo_root: &Path,
    manifest: &RunManifest,
    destination: &Path,
    strategy: UpdateStrategy,
    promotion: PromotionState,
    publish_timestamp: &str,
    artifacts: &[PersistedArtifact],
    lineage: &LineageMetadata,
    published_files: &mut Vec<String>,
) -> Result<PathBuf, EngineError> {
    match strategy {
        UpdateStrategy::ManagedBlocks => {
            let bundle = render_project_memory_surface(artifacts);
            write_managed_block(destination, &manifest.run_id, &bundle)?;
            published_files.push(display_path(repo_root, destination));
            Ok(destination.to_path_buf())
        }
        UpdateStrategy::ProposalFiles => {
            let bundle = render_project_memory_surface(artifacts);
            let proposal_path = write_surface_proposal_file(destination, &bundle, lineage)?;
            published_files.push(display_path(repo_root, &proposal_path));
            Ok(proposal_path)
        }
        UpdateStrategy::AppendOnlyIndex => {
            let evidence_dir = resolve_profile_evidence_root(repo_root, manifest);
            let entry = render_project_memory_index_entry(
                manifest,
                &promotion,
                publish_timestamp,
                &display_path(repo_root, &evidence_dir),
            );
            append_index_entry(destination, &entry)?;
            published_files.push(display_path(repo_root, destination));
            for path in write_supporting_evidence_bundle(&evidence_dir, artifacts)? {
                published_files.push(display_path(repo_root, &path));
            }
            Ok(destination.to_path_buf())
        }
    }
}

/// Evaluate the promotion policy for a given mode and run state.
pub fn evaluate_promotion_policy(
    mode: Mode,
    run_state: &RunState,
    _manifest: &RunManifest,
) -> PromotionState {
    match (mode, run_state) {
        // Completed runs for analysis and shaping modes auto-promote.
        (
            Mode::SystemShaping
            | Mode::Discovery
            | Mode::Requirements
            | Mode::DomainLanguage
            | Mode::DomainModel
            | Mode::Backlog,
            RunState::Completed,
        ) => PromotionState::Auto,

        // Approval-gated modes promote only when completed.
        (
            Mode::Architecture
            | Mode::Change
            | Mode::Implementation
            | Mode::Refactor
            | Mode::Migration,
            RunState::Completed,
        ) => PromotionState::AutoIfApproved,
        (
            Mode::Architecture
            | Mode::Change
            | Mode::Implementation
            | Mode::Refactor
            | Mode::Migration,
            _,
        ) => PromotionState::PendingIndex,

        // Evidence/review modes always publish evidence only.
        (
            Mode::Verification
            | Mode::Review
            | Mode::PrReview
            | Mode::SecurityAssessment
            | Mode::SupplyChainAnalysis
            | Mode::SystemAssessment,
            _,
        ) => PromotionState::EvidenceOnly,

        // Incident defaults to pending index (manual review expected).
        (Mode::Incident, RunState::Completed) => PromotionState::PendingIndex,
        (Mode::Incident, _) => PromotionState::Manual,

        // Non-completed analysis modes go to pending.
        (_, _) => PromotionState::PendingIndex,
    }
}

fn default_update_strategy_for(mode: Mode) -> UpdateStrategy {
    match mode {
        Mode::SystemShaping
        | Mode::Architecture
        | Mode::Requirements
        | Mode::Discovery
        | Mode::Change
        | Mode::Implementation
        | Mode::Refactor
        | Mode::DomainLanguage
        | Mode::DomainModel => UpdateStrategy::ManagedBlocks,
        Mode::Incident | Mode::Migration => UpdateStrategy::ProposalFiles,
        Mode::Verification
        | Mode::Review
        | Mode::PrReview
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis
        | Mode::SystemAssessment
        | Mode::Backlog => UpdateStrategy::AppendOnlyIndex,
    }
}

fn resolve_profile_destination(
    repo_root: &Path,
    manifest: &RunManifest,
    promotion: &PromotionState,
) -> PathBuf {
    repo_root.join(canonical_project_memory_surface(manifest.mode, *promotion))
}

fn canonical_project_memory_surface(mode: Mode, promotion: PromotionState) -> &'static str {
    if promotion.targets_stable_surface() {
        stable_project_memory_surface(mode)
    } else if promotion.targets_pending_surface() {
        pending_project_memory_surface(mode)
    } else {
        evidence_project_memory_surface(mode)
    }
}

fn stable_project_memory_surface(mode: Mode) -> &'static str {
    match mode {
        Mode::Discovery => "docs/project/overview.md",
        Mode::Requirements => "docs/project/product-context.md",
        Mode::SystemShaping | Mode::Architecture => "docs/project/architecture-map.md",
        Mode::Change | Mode::Migration => "docs/project/decision-index.md",
        Mode::Backlog | Mode::Implementation | Mode::Refactor => "docs/project/delivery-map.md",
        Mode::DomainLanguage => "docs/project/domain-language.md",
        Mode::DomainModel => "docs/project/domain-model.md",
        Mode::Verification
        | Mode::Review
        | Mode::PrReview
        | Mode::Incident
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis
        | Mode::SystemAssessment => "docs/project/operational-context.md",
    }
}

fn pending_project_memory_surface(mode: Mode) -> &'static str {
    match mode {
        Mode::Incident => "docs/project/open-risks.md",
        Mode::Verification | Mode::Review | Mode::PrReview => "docs/project/audit-log.md",
        _ => "docs/project/pending-decisions.md",
    }
}

fn evidence_project_memory_surface(mode: Mode) -> &'static str {
    match mode {
        Mode::Review | Mode::PrReview | Mode::Verification => "docs/project/audit-log.md",
        Mode::Incident
        | Mode::SecurityAssessment
        | Mode::SupplyChainAnalysis
        | Mode::SystemAssessment => "docs/project/open-risks.md",
        _ => "docs/project/audit-log.md",
    }
}

fn render_project_memory_surface(artifacts: &[PersistedArtifact]) -> String {
    let visible_artifacts = artifacts
        .iter()
        .filter(|artifact| !is_packet_sidecar(&artifact.record.file_name))
        .collect::<Vec<_>>();

    if visible_artifacts.len() == 1 {
        return visible_artifacts[0].contents.trim().to_string();
    }

    visible_artifacts
        .iter()
        .map(|artifact| {
            let stem = Path::new(&artifact.record.file_name)
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or(&artifact.record.file_name);
            format!("## {}\n\n{}", titleize_slug(stem), artifact.contents.trim())
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Resolve the [`RuntimePacketMetadata`] for a packet.
///
/// If a `packet-metadata.json` sidecar is present and parseable it is used directly;
/// otherwise the metadata is inferred from the artifact listing via
/// [`infer_runtime_packet_metadata`].
fn runtime_packet_metadata(artifacts: &[PersistedArtifact]) -> RuntimePacketMetadata {
    artifacts
        .iter()
        .find(|artifact| {
            artifact_slug(&artifact.record.file_name) == PROJECT_MEMORY_PACKET_METADATA_FILE_NAME
        })
        .and_then(|artifact| serde_json::from_str::<RuntimePacketMetadata>(&artifact.contents).ok())
        .unwrap_or_else(|| infer_runtime_packet_metadata(artifacts))
}

/// Derive [`RuntimePacketMetadata`] from the artifact listing when no sidecar is present.
///
/// Sidecars are excluded from the order; prefixed filenames are mapped to their bare
/// slug aliases so legacy consumers can still resolve unprefixed artifact names.
fn infer_runtime_packet_metadata(artifacts: &[PersistedArtifact]) -> RuntimePacketMetadata {
    let artifact_order = artifacts
        .iter()
        .filter(|artifact| !is_packet_sidecar(&artifact.record.file_name))
        .map(|artifact| artifact.record.file_name.clone())
        .collect::<Vec<_>>();
    let primary_artifact = artifact_order.first().cloned().unwrap_or_default();
    let legacy_aliases = artifacts
        .iter()
        .filter_map(|artifact| {
            let slug = artifact_slug(&artifact.record.file_name);
            (!is_packet_sidecar(&artifact.record.file_name) && slug != artifact.record.file_name)
                .then(|| (slug.to_string(), artifact.record.file_name.clone()))
        })
        .collect::<BTreeMap<_, _>>();

    RuntimePacketMetadata {
        primary_artifact,
        artifact_order,
        publish_order: None,
        legacy_aliases: (!legacy_aliases.is_empty()).then_some(legacy_aliases),
        expertise_input: None,
        publication_target_class: None,
        artifact_indexing: None,
        authority_governance: None,
        adaptive_governance: None,
    }
}

fn resolve_expertise_input_metadata(
    repo_root: &Path,
    mode: Mode,
    packet_metadata: &RuntimePacketMetadata,
) -> Option<ExpertiseInputMetadata> {
    if let Some(metadata) = packet_metadata.expertise_input.as_ref() {
        return metadata.normalized();
    }

    classify_governed_expertise_input(mode, infer_boundline_domain_families(repo_root))
}

fn infer_boundline_domain_families(repo_root: &Path) -> Vec<String> {
    let mut families = BTreeSet::new();
    let package_json = read_lowercase_file(repo_root.join("package.json"));

    add_package_json_domain_families(&mut families, package_json.as_deref());
    add_repo_language_domain_families(&mut families, repo_root);
    add_package_json_fallback_service_family(&mut families, repo_root, package_json.as_deref());

    families.into_iter().collect()
}

fn add_package_json_domain_families(families: &mut BTreeSet<String>, package_json: Option<&str>) {
    let Some(package_json) = package_json else {
        return;
    };

    if package_json.contains("\"react\"") {
        families.insert("react".to_string());
        families.insert("web_ui".to_string());
    }
    if package_json.contains("\"vue\"") {
        families.insert("vue".to_string());
        families.insert("web_ui".to_string());
    }
    if package_json.contains("\"@angular/") || package_json.contains("\"angular\"") {
        families.insert("angular".to_string());
        families.insert("web_ui".to_string());
    }
    if package_json.contains("\"express\"")
        || package_json.contains("\"nest\"")
        || package_json.contains("\"fastify\"")
        || package_json.contains("\"koa\"")
        || package_json.contains("\"hapi\"")
    {
        families.insert("node_service".to_string());
    }
}

fn add_repo_language_domain_families(families: &mut BTreeSet<String>, repo_root: &Path) {
    if repo_root.join("Cargo.toml").exists() || repo_contains_extension(repo_root, "rs", 0) {
        families.insert("systems".to_string());
    }
    if repo_root.join("pyproject.toml").exists()
        || repo_root.join("setup.py").exists()
        || repo_contains_extension(repo_root, "py", 0)
    {
        families.insert("python_service".to_string());
    }
    if repo_root.join("pom.xml").exists()
        || repo_root.join("build.gradle").exists()
        || repo_root.join("build.gradle.kts").exists()
    {
        families.insert("jvm_service".to_string());
    }
    if repo_contains_suffix(repo_root, ".csproj", 0) || repo_contains_suffix(repo_root, ".sln", 0) {
        families.insert("dotnet_service".to_string());
    }
    if repo_root.join("Gemfile").exists() {
        families.insert("ruby".to_string());
    }
    if repo_root.join("composer.json").exists() {
        families.insert("php".to_string());
    }
}

fn add_package_json_fallback_service_family(
    families: &mut BTreeSet<String>,
    repo_root: &Path,
    package_json: Option<&str>,
) {
    if !families.is_empty() || package_json.is_none() {
        return;
    }

    if repo_contains_extension(repo_root, "js", 0)
        || repo_contains_extension(repo_root, "jsx", 0)
        || repo_contains_extension(repo_root, "ts", 0)
        || repo_contains_extension(repo_root, "tsx", 0)
    {
        families.insert("node_service".to_string());
    }
}

fn read_lowercase_file(path: PathBuf) -> Option<String> {
    fs::read_to_string(path).ok().map(|contents| contents.to_ascii_lowercase())
}

fn repo_contains_extension(root: &Path, extension: &str, depth: usize) -> bool {
    if depth >= MAX_REPO_SCAN_DEPTH {
        return false;
    }

    let Ok(entries) = fs::read_dir(root) else {
        return false;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if should_skip_repo_scan_directory(&name) {
                continue;
            }
            if repo_contains_extension(&path, extension, depth + 1) {
                return true;
            }
        } else if path.extension().and_then(|value| value.to_str()) == Some(extension) {
            return true;
        }
    }

    false
}

fn repo_contains_suffix(root: &Path, suffix: &str, depth: usize) -> bool {
    if depth >= MAX_REPO_SCAN_DEPTH {
        return false;
    }

    let Ok(entries) = fs::read_dir(root) else {
        return false;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if should_skip_repo_scan_directory(&name) {
                continue;
            }
            if repo_contains_suffix(&path, suffix, depth + 1) {
                return true;
            }
        } else if path
            .file_name()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.ends_with(suffix))
        {
            return true;
        }
    }

    false
}

fn render_project_memory_index_entry(
    manifest: &RunManifest,
    promotion: &PromotionState,
    published_at: &str,
    evidence_path: &str,
) -> String {
    format!(
        "## {}\n\n- Run: `{}`\n- Mode: `{}`\n- Promotion: `{}`\n- Published: `{}`\n- Evidence: `{}`\n\n",
        manifest.title.as_deref().unwrap_or_else(|| manifest.mode.as_str()),
        manifest.run_id,
        manifest.mode.as_str(),
        promotion.as_str(),
        published_at,
        evidence_path,
    )
}

fn resolve_profile_evidence_root(repo_root: &Path, manifest: &RunManifest) -> PathBuf {
    repo_root.join("docs/evidence").join(manifest.mode.as_str()).join(&manifest.run_id)
}

fn write_supporting_evidence_bundle(
    destination: &Path,
    artifacts: &[PersistedArtifact],
) -> Result<Vec<PathBuf>, EngineError> {
    fs::create_dir_all(destination)?;
    let mut written = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let target = destination.join(&artifact.record.file_name);
        fs::write(&target, &artifact.contents)?;
        written.push(target);
    }
    Ok(written)
}

fn write_surface_proposal_file(
    target: &Path,
    content: &str,
    lineage: &LineageMetadata,
) -> Result<PathBuf, EngineError> {
    let stem = target.file_stem().and_then(|value| value.to_str()).unwrap_or("proposal");
    let ext = target.extension().and_then(|value| value.to_str()).unwrap_or("md");
    let proposal_path = target.with_file_name(format!("{stem}.proposal.{ext}"));

    let header = format!(
        "<!-- Canon proposal from {} ({}) -->\n<!-- promotion_state: {} | profile: {} -->\n\n",
        lineage.source_ref,
        lineage.mode.as_deref().unwrap_or("unknown-mode"),
        lineage.promotion_state,
        lineage.promotion_profile.map(PublishProfile::as_str).unwrap_or("unknown-profile"),
    );

    if let Some(parent) = proposal_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&proposal_path, format!("{header}{content}"))?;
    Ok(proposal_path)
}

fn profile_metadata_path(target: &Path) -> PathBuf {
    if target.extension().is_none() {
        return target.join(PROJECT_MEMORY_PACKET_METADATA_FILE_NAME);
    }

    let stem = target.file_stem().and_then(|value| value.to_str()).unwrap_or("packet");
    target.with_file_name(format!("{stem}{PROFILE_METADATA_FILE_SUFFIX}"))
}

/// Insert or replace a Canon-managed block inside a document. Content outside
/// the managed range is preserved.
pub fn write_managed_block(
    target: &Path,
    block_id: &str,
    content: &str,
) -> Result<(), EngineError> {
    let descriptor = ManagedBlockDescriptor::canon(block_id.to_string());
    let start_marker = descriptor.start_marker();
    let end_marker = ManagedBlockDescriptor::end_marker();
    let legacy_start_marker = format!("<!-- canon:managed-block:{block_id}:start -->");
    let legacy_end_marker = format!("<!-- canon:managed-block:{block_id}:end -->");

    let existing = if target.exists() { fs::read_to_string(target)? } else { String::new() };

    let new_block = format!("{start_marker}\n{content}\n{end_marker}");

    let updated = if let Some(start_pos) = existing.find("<!-- project-memory:managed:start") {
        if let Some(end_offset) = existing[start_pos..].find(end_marker) {
            let end_pos = start_pos + end_offset;
            let before = &existing[..start_pos];
            let after = &existing[end_pos + end_marker.len()..];
            format!("{before}{new_block}{after}")
        } else {
            // Start marker found but no end marker: replace the managed suffix.
            let before = &existing[..start_pos];
            let after = &existing[start_pos + "<!-- project-memory:managed:start".len()..];
            format!("{before}{new_block}{after}")
        }
    } else if let Some(start_pos) = existing.find(&legacy_start_marker) {
        if let Some(end_pos) = existing.find(&legacy_end_marker) {
            let before = &existing[..start_pos];
            let after = &existing[end_pos + legacy_end_marker.len()..];
            format!("{before}{new_block}{after}")
        } else {
            let before = &existing[..start_pos];
            let after = &existing[start_pos + legacy_start_marker.len()..];
            format!("{before}{new_block}{after}")
        }
    } else if existing.is_empty() {
        new_block
    } else {
        // No existing block: append.
        format!("{existing}\n\n{new_block}\n")
    };

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(target, updated)?;
    Ok(())
}

/// Emit a proposal file alongside the stable target.
pub fn write_proposal_file(
    destination: &Path,
    file_name: &str,
    content: &str,
    lineage: &LineageMetadata,
) -> Result<PathBuf, EngineError> {
    let stem = Path::new(file_name).file_stem().and_then(|s| s.to_str()).unwrap_or(file_name);
    let ext = Path::new(file_name).extension().and_then(|s| s.to_str()).unwrap_or("md");
    let proposal_name = format!("{stem}.proposal.{ext}");
    let proposal_path = destination.join(&proposal_name);

    let header = format!(
        "<!-- Canon proposal from {} ({}) -->\n<!-- promotion_state: {} | profile: {} -->\n\n",
        lineage.source_ref,
        lineage.mode.as_deref().unwrap_or("unknown-mode"),
        lineage.promotion_state,
        lineage.promotion_profile.map(PublishProfile::as_str).unwrap_or("unknown-profile"),
    );

    fs::create_dir_all(destination)?;
    fs::write(&proposal_path, format!("{header}{content}"))?;
    Ok(proposal_path)
}

/// Append an entry to an index or audit surface without rewriting existing
/// entries.
pub fn append_index_entry(target: &Path, entry: &str) -> Result<(), EngineError> {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    let existing = if target.exists() { fs::read_to_string(target)? } else { String::new() };
    fs::write(target, format!("{existing}{entry}"))?;
    Ok(())
}

fn packet_readiness_for(state: &RunState) -> &'static str {
    if *state == RunState::Completed { "complete" } else { "partial" }
}

fn content_digest_for_artifacts(artifacts: &[PersistedArtifact]) -> String {
    let mut hasher = Sha256::new();

    for artifact in artifacts {
        if artifact_slug(&artifact.record.file_name) == PROJECT_MEMORY_PACKET_METADATA_FILE_NAME {
            continue;
        }
        hasher.update(artifact.record.file_name.as_bytes());
        hasher.update([0]);
        hasher.update(artifact.contents.as_bytes());
        hasher.update([0]);
    }

    let digest = hasher.finalize();
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut hex, "{byte:02x}");
    }

    format!("sha256:{hex}")
}

fn adr_export_enabled(mode: Mode, requested: bool) -> Result<bool, EngineError> {
    match mode {
        Mode::Architecture => Ok(true),
        Mode::Change | Mode::Migration => Ok(requested),
        _ if requested => Err(EngineError::Validation(format!(
            "ADR export is not supported for mode `{}`",
            mode.as_str()
        ))),
        _ => Ok(false),
    }
}

fn build_adr_export(
    repo_root: &Path,
    manifest: &RunManifest,
    artifacts: &[PersistedArtifact],
    packet_destination: &Path,
) -> Result<GeneratedAdr, EngineError> {
    let packet_display_path = display_path(repo_root, packet_destination);
    let adr = match manifest.mode {
        Mode::Architecture => build_architecture_adr(manifest, artifacts, &packet_display_path)?,
        Mode::Change => build_change_adr(manifest, artifacts, &packet_display_path)?,
        Mode::Migration => build_migration_adr(manifest, artifacts, &packet_display_path)?,
        _ => {
            return Err(EngineError::Validation(format!(
                "ADR export is not supported for mode `{}`",
                manifest.mode.as_str()
            )));
        }
    };

    let registry_root = repo_root.join(ADR_REGISTRY_DIRECTORY);
    let number = next_adr_number(&registry_root)?;
    let slug = slugify(&adr.title).unwrap_or_else(|| publish_descriptor(manifest));
    let destination = registry_root.join(format!("ADR-{number:04}-{slug}.md"));
    let display_path = display_path(repo_root, &destination);
    let contents = render_adr_document(number, manifest, &adr, &packet_display_path);

    Ok(GeneratedAdr { destination, display_path, contents })
}

fn build_architecture_adr(
    manifest: &RunManifest,
    artifacts: &[PersistedArtifact],
    packet_display_path: &str,
) -> Result<AdrDocument, EngineError> {
    let overview = artifact_contents(artifacts, "architecture-overview.md")?;
    let decisions = artifact_contents(artifacts, "architecture-decisions.md")?;
    let tradeoff_matrix = artifact_contents(artifacts, "tradeoff-matrix.md")?;

    let context = markdown_section(overview, "Summary").ok_or_else(|| {
        EngineError::Validation(
            "published architecture overview is missing the `## Summary` section required for ADR context"
                .to_string(),
        )
    })?;
    let decision = markdown_section(decisions, "Decision").ok_or_else(|| {
        EngineError::Validation(
            "published architecture decisions are missing the `## Decision` section required for ADR export"
                .to_string(),
        )
    })?;

    Ok(AdrDocument {
        title: adr_title_from_decision(&decision, manifest),
        context,
        decision,
        consequences: architecture_consequences(decisions, tradeoff_matrix)?,
        alternatives: markdown_section(tradeoff_matrix, "Options Considered"),
        source_packet: Some(packet_display_path.to_string()),
    })
}

fn build_change_adr(
    manifest: &RunManifest,
    artifacts: &[PersistedArtifact],
    packet_display_path: &str,
) -> Result<AdrDocument, EngineError> {
    let change_surface = artifact_contents(artifacts, "change-surface.md")?;
    let implementation_plan = artifact_contents(artifacts, "implementation-plan.md")?;
    let decision_record = artifact_contents(artifacts, "decision-record.md")?;

    let context = labeled_sections(&[
        ("Implementation Plan", markdown_section(implementation_plan, "Implementation Plan")),
        ("Change Surface", markdown_section(change_surface, "Change Surface")),
        ("Cross-Context Risks", markdown_section(change_surface, "Cross-Context Risks")),
    ])?;
    let decision = preferred_section(decision_record, &["Decision Record", "Recommendation"])
        .ok_or_else(|| {
            EngineError::Validation(
                "published change packet is missing a decision section required for ADR export"
                    .to_string(),
            )
        })?;
    let consequences = labeled_sections(&[
        ("Consequences", markdown_section(decision_record, "Consequences")),
        ("Boundary Tradeoffs", markdown_section(decision_record, "Boundary Tradeoffs")),
    ])?;

    Ok(AdrDocument {
        title: adr_title_from_decision(&decision, manifest),
        context,
        decision,
        consequences,
        alternatives: markdown_section(decision_record, "Options Considered"),
        source_packet: Some(packet_display_path.to_string()),
    })
}

fn build_migration_adr(
    manifest: &RunManifest,
    artifacts: &[PersistedArtifact],
    packet_display_path: &str,
) -> Result<AdrDocument, EngineError> {
    let source_target_map = artifact_contents(artifacts, "source-target-map.md")?;
    let compatibility_matrix = artifact_contents(artifacts, "compatibility-matrix.md")?;
    let decision_record = artifact_contents(artifacts, "decision-record.md")?;

    let context = labeled_sections(&[
        ("Current State", markdown_section(source_target_map, "Current State")),
        ("Target State", markdown_section(source_target_map, "Target State")),
        ("Transition Boundaries", markdown_section(source_target_map, "Transition Boundaries")),
    ])?;
    let decision = preferred_section(decision_record, &["Migration Decisions", "Recommendation"])
        .ok_or_else(|| {
        EngineError::Validation(
            "published migration packet is missing a decision section required for ADR export"
                .to_string(),
        )
    })?;
    let consequences = labeled_sections(&[
        ("Tradeoff Analysis", markdown_section(decision_record, "Tradeoff Analysis")),
        ("Ecosystem Health", markdown_section(decision_record, "Ecosystem Health")),
        ("Residual Risks", markdown_section(decision_record, "Residual Risks")),
    ])?;

    Ok(AdrDocument {
        title: adr_title_from_decision(&decision, manifest),
        context,
        decision,
        consequences,
        alternatives: markdown_section(compatibility_matrix, "Options Matrix"),
        source_packet: Some(packet_display_path.to_string()),
    })
}

fn architecture_consequences(
    decisions: &str,
    tradeoff_matrix: &str,
) -> Result<String, EngineError> {
    let mut sections = Vec::new();

    if let Some(value) = markdown_section(decisions, "Consequences") {
        sections.push(value);
    }
    if let Some(value) = markdown_section(tradeoff_matrix, "Pros") {
        sections.push(format!("### Pros\n\n{value}"));
    }
    if let Some(value) = markdown_section(tradeoff_matrix, "Cons") {
        sections.push(format!("### Cons\n\n{value}"));
    }

    if sections.is_empty() {
        return Err(EngineError::Validation(
            "published architecture packet is missing consequence material required for ADR export"
                .to_string(),
        ));
    }

    Ok(sections.join("\n\n"))
}

fn artifact_contents<'a>(
    artifacts: &'a [PersistedArtifact],
    file_name: &str,
) -> Result<&'a str, EngineError> {
    artifacts
        .iter()
        .find(|artifact| artifact.record.slug() == file_name)
        .map(|artifact| artifact.contents.as_str())
        .ok_or_else(|| {
            EngineError::Validation(format!(
                "published packet is missing `{file_name}`, which is required for ADR export"
            ))
        })
}

fn markdown_section(markdown: &str, section: &str) -> Option<String> {
    let mut collecting = false;
    let mut lines = Vec::new();

    for line in markdown.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if collecting {
                break;
            }
            collecting = heading.trim() == section;
            continue;
        }

        if collecting {
            lines.push(line);
        }
    }

    let body = lines.join("\n").trim().to_string();
    if body.is_empty() { None } else { Some(body) }
}

fn preferred_section(markdown: &str, sections: &[&str]) -> Option<String> {
    sections.iter().find_map(|section| markdown_section(markdown, section))
}

fn labeled_sections(sections: &[(&str, Option<String>)]) -> Result<String, EngineError> {
    let rendered = sections
        .iter()
        .filter_map(|(title, content)| {
            content.as_ref().map(|content| format!("### {title}\n\n{content}"))
        })
        .collect::<Vec<_>>();

    if rendered.is_empty() {
        return Err(EngineError::Validation(
            "published packet is missing the authored sections required for ADR export".to_string(),
        ));
    }

    Ok(rendered.join("\n\n"))
}

fn adr_title_from_decision(decision: &str, manifest: &RunManifest) -> String {
    decision
        .split('\n')
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(normalize_adr_title_line)
        .unwrap_or_else(|| fallback_adr_title(manifest))
}

fn normalize_adr_title_line(line: &str) -> String {
    line.trim()
        .trim_start_matches("- ")
        .trim_start_matches("* ")
        .trim_start_matches("1. ")
        .to_string()
}

fn fallback_adr_title(manifest: &RunManifest) -> String {
    manifest.title.clone().unwrap_or_else(|| titleize_slug(&publish_descriptor(manifest)))
}

fn titleize_slug(slug: &str) -> String {
    slug.split('-')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => {
                    let mut titled = first.to_uppercase().collect::<String>();
                    titled.push_str(chars.as_str());
                    titled
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn next_adr_number(registry_root: &Path) -> Result<u32, EngineError> {
    if !registry_root.exists() {
        return Ok(1);
    }

    let mut max_number = 0;
    for entry in fs::read_dir(registry_root)? {
        let entry = entry?;
        let Some(file_name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        let Some(rest) = file_name.strip_prefix("ADR-") else {
            continue;
        };
        let Some((digits, _)) = rest.split_once('-') else {
            continue;
        };
        let Ok(number) = digits.parse::<u32>() else {
            continue;
        };
        max_number = max_number.max(number);
    }

    Ok(max_number + 1)
}

fn render_adr_document(
    number: u32,
    manifest: &RunManifest,
    adr: &AdrDocument,
    packet_display_path: &str,
) -> String {
    let mut rendered = String::new();
    rendered.push_str(&format!("# ADR {number:04}: {}\n\n", adr.title));
    rendered.push_str(&format!("**Date:** {}\n", publish_date(manifest.created_at)));
    rendered.push_str("**Status:** Accepted\n\n");
    rendered.push_str("## Context\n\n");
    rendered.push_str(&adr.context);
    rendered.push_str("\n\n## Decision\n\n");
    rendered.push_str(&adr.decision);
    rendered.push_str("\n\n## Consequences\n\n");
    rendered.push_str(&adr.consequences);

    if let Some(alternatives) = &adr.alternatives {
        rendered.push_str("\n\n## Alternatives Considered\n\n");
        rendered.push_str(alternatives);
    }

    rendered.push_str("\n\n## Source Packet\n\n");
    rendered.push_str(&format!("- Run: {}\n", manifest.run_id));
    rendered.push_str(&format!(
        "- Packet: {}\n",
        adr.source_packet.as_deref().unwrap_or(packet_display_path)
    ));
    rendered
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
    let metadata_path = destination.join(PROJECT_MEMORY_PACKET_METADATA_FILE_NAME);
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
        Mode::DomainLanguage => "docs/domain/language",
        Mode::DomainModel => "docs/domain/model",
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;
    use time::OffsetDateTime;
    use time::format_description::well_known::Rfc3339;

    use super::{PublishMetadata, default_publish_directory, resolve_destination};
    use crate::domain::artifact::{ArtifactFormat, ArtifactRecord};
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::publish_profile::{
        PROJECT_MEMORY_PACKET_METADATA_FILE_NAME, PromotionState, PublishProfile, UpdateStrategy,
    };
    use crate::domain::run::{ClassificationProvenance, RunState, SystemContext};
    use crate::persistence::manifests::RunManifest;
    use crate::persistence::store::PersistedArtifact;

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

    fn manifest_for(mode: Mode, run_id: &str) -> RunManifest {
        let mut manifest = sample_manifest(run_id);
        manifest.mode = mode;
        manifest.system_context = Some(SystemContext::Existing);
        manifest.title = None;
        manifest.slug = None;
        manifest
    }

    fn markdown_artifact(
        run_id: &str,
        mode: Mode,
        file_name: &str,
        contents: &str,
    ) -> PersistedArtifact {
        PersistedArtifact {
            record: ArtifactRecord {
                file_name: file_name.to_string(),
                relative_path: format!(".canon/artifacts/{run_id}/{}/{file_name}", mode.as_str()),
                format: ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: contents.to_string(),
        }
    }

    fn json_artifact(
        run_id: &str,
        mode: Mode,
        file_name: &str,
        contents: &str,
    ) -> PersistedArtifact {
        PersistedArtifact {
            record: ArtifactRecord {
                file_name: file_name.to_string(),
                relative_path: format!(".canon/artifacts/{run_id}/{}/{file_name}", mode.as_str()),
                format: ArtifactFormat::Json,
                provenance: None,
            },
            contents: contents.to_string(),
        }
    }

    #[test]
    fn render_project_memory_surface_returns_trimmed_single_artifact() {
        let artifact = markdown_artifact(
            "R-test",
            Mode::Requirements,
            "product-context.md",
            "\n# Product Context\n\nBounded summary.\n\n",
        );

        let rendered = super::render_project_memory_surface(&[artifact]);

        assert_eq!(rendered, "# Product Context\n\nBounded summary.");
        assert!(!rendered.contains("## Product Context"));
    }

    #[test]
    fn infer_runtime_packet_metadata_ignores_sidecars_and_maps_legacy_aliases() {
        let artifacts = vec![
            markdown_artifact(
                "R-test",
                Mode::Requirements,
                "01-problem-statement.md",
                "# Problem Statement\n\nBounded summary.",
            ),
            markdown_artifact(
                "R-test",
                Mode::Requirements,
                "02-scope-cuts.md",
                "# Scope Cuts\n\nBounded summary.",
            ),
            json_artifact("R-test", Mode::Requirements, "view-manifest.json", "{\"views\":[]}"),
        ];

        let metadata = super::infer_runtime_packet_metadata(&artifacts);

        assert_eq!(metadata.primary_artifact, "01-problem-statement.md");
        assert_eq!(
            metadata.artifact_order,
            vec!["01-problem-statement.md".to_string(), "02-scope-cuts.md".to_string(),]
        );
        assert_eq!(
            metadata
                .legacy_aliases
                .as_ref()
                .and_then(|aliases| aliases.get("problem-statement.md")),
            Some(&"01-problem-statement.md".to_string())
        );
    }

    #[test]
    fn runtime_packet_metadata_falls_back_to_inferred_order_when_sidecar_is_unparseable() {
        let artifacts = vec![
            markdown_artifact(
                "R-test",
                Mode::Requirements,
                "01-problem-statement.md",
                "# Problem Statement\n\nBounded summary.",
            ),
            markdown_artifact(
                "R-test",
                Mode::Requirements,
                "02-scope-cuts.md",
                "# Scope Cuts\n\nBounded summary.",
            ),
            json_artifact(
                "R-test",
                Mode::Requirements,
                PROJECT_MEMORY_PACKET_METADATA_FILE_NAME,
                "{not-json}",
            ),
        ];

        let metadata = super::runtime_packet_metadata(&artifacts);

        assert_eq!(metadata.primary_artifact, "01-problem-statement.md");
        assert_eq!(metadata.artifact_order.len(), 2);
        assert!(metadata.legacy_aliases.is_some());
    }

    #[test]
    fn profile_metadata_path_uses_directory_and_file_conventions() {
        assert_eq!(
            super::profile_metadata_path(Path::new("docs/project/custom-dest")),
            Path::new("docs/project/custom-dest/packet-metadata.json")
        );
        assert_eq!(
            super::profile_metadata_path(Path::new("docs/project/open-risks.proposal.md")),
            Path::new("docs/project/open-risks.proposal.packet-metadata.json")
        );
    }

    #[test]
    fn default_publish_directory_maps_supported_modes() {
        assert_eq!(default_publish_directory(Mode::Requirements), "specs");
        assert_eq!(default_publish_directory(Mode::Discovery), "docs/discovery");
        assert_eq!(default_publish_directory(Mode::SystemShaping), "docs/architecture/shaping");
        assert_eq!(default_publish_directory(Mode::Change), "docs/changes");
        assert_eq!(default_publish_directory(Mode::Backlog), "docs/planning");
        assert_eq!(default_publish_directory(Mode::Architecture), "docs/architecture/decisions");
        assert_eq!(default_publish_directory(Mode::Implementation), "docs/implementation");
        assert_eq!(default_publish_directory(Mode::Refactor), "docs/refactors");
        assert_eq!(default_publish_directory(Mode::Verification), "docs/verification");
        assert_eq!(default_publish_directory(Mode::Review), "docs/reviews");
        assert_eq!(default_publish_directory(Mode::PrReview), "docs/reviews/prs");
        assert_eq!(default_publish_directory(Mode::Incident), "docs/incidents");
        assert_eq!(
            default_publish_directory(Mode::SystemAssessment),
            "docs/architecture/assessments"
        );
        assert_eq!(
            default_publish_directory(Mode::SecurityAssessment),
            "docs/security-assessments"
        );
        assert_eq!(default_publish_directory(Mode::Migration), "docs/migrations");
        assert_eq!(default_publish_directory(Mode::SupplyChainAnalysis), "docs/supply-chain");
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
            collision_path.join(PROJECT_MEMORY_PACKET_METADATA_FILE_NAME),
            serde_json::to_vec_pretty(&PublishMetadata {
                run_id: "R-20260422-deadbeef".to_string(),
                mode: "requirements".to_string(),
                risk: "low-impact".to_string(),
                zone: "green".to_string(),
                publish_timestamp: "2026-04-22T08:00:00Z".to_string(),
                descriptor: "publish-scope".to_string(),
                destination: "specs/2026-04-22-publish-scope".to_string(),
                source_artifacts: vec![
                    ".canon/artifacts/R-20260422-deadbeef/requirements/01-problem-statement.md"
                        .to_string(),
                ],
                primary_artifact: "01-problem-statement.md".to_string(),
                artifact_order: vec!["01-problem-statement.md".to_string()],
                publish_order: None,
                legacy_aliases: None,
                artifact_indexing: None,
                profile: None,
                promotion_state: None,
                update_strategy: None,
                publication_target_class: None,
                expertise_input: None,
                lineage: None,
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
    fn adr_export_policy_distinguishes_default_opt_in_and_unsupported_modes() {
        assert!(super::adr_export_enabled(Mode::Architecture, false).expect("architecture policy"));
        assert!(!super::adr_export_enabled(Mode::Change, false).expect("change default policy"));
        assert!(super::adr_export_enabled(Mode::Migration, true).expect("migration opt-in policy"));
        assert!(
            !super::adr_export_enabled(Mode::Requirements, false)
                .expect("requirements default policy")
        );
        assert!(super::adr_export_enabled(Mode::Incident, true).is_err());
    }

    #[test]
    fn build_adr_export_rejects_unsupported_modes_when_called_directly() {
        let workspace = tempdir().expect("temp workspace");
        let manifest = sample_manifest("R-20260422-abcd1234");

        let error = super::build_adr_export(
            workspace.path(),
            &manifest,
            &[],
            Path::new("specs/2026-04-22-publish-scope"),
        )
        .expect_err("unsupported mode should be rejected");

        assert!(error.to_string().contains("ADR export is not supported"));
    }

    #[test]
    fn build_change_adr_maps_change_packet_sections() {
        let manifest = manifest_for(Mode::Change, "R-20260422-change01");
        let artifacts = vec![
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "change-surface.md",
                "# Change Surface\n\n## Change Surface\n\n- session repository\n\n## Cross-Context Risks\n\n- cleanup scheduling can leak into notification flows\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "implementation-plan.md",
                "# Implementation Plan\n\n## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract.\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "decision-record.md",
                "# Decision Record\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Options Considered\n\n- Option 1 keeps the additive repository helper inside the auth boundary.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n\n## Boundary Tradeoffs\n\n- keep cleanup logic inside the auth boundary\n",
            ),
        ];

        let adr = super::build_change_adr(&manifest, &artifacts, "docs/changes/2026-04-22-change")
            .expect("change adr");

        assert_eq!(
            adr.title,
            "Prefer additive change over normalization to preserve operator expectations."
        );
        assert!(adr.context.contains("### Implementation Plan"));
        assert!(adr.context.contains("### Change Surface"));
        assert!(adr.decision.contains("Prefer additive change over normalization"));
        assert!(adr.consequences.contains("### Boundary Tradeoffs"));
        assert!(
            adr.alternatives.as_deref().is_some_and(
                |value| value.contains("Option 1 keeps the additive repository helper")
            )
        );
    }

    #[test]
    fn build_change_adr_reports_missing_decision_section() {
        let manifest = manifest_for(Mode::Change, "R-20260422-change-missing");
        let artifacts = vec![
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "change-surface.md",
                "# Change Surface\n\n## Change Surface\n\n- session repository\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "implementation-plan.md",
                "# Implementation Plan\n\n## Implementation Plan\n\nAdd bounded repository methods.\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "decision-record.md",
                "# Decision Record\n\n## Consequences\n\n- preserved surface remains explicit\n",
            ),
        ];

        let error = super::build_change_adr(
            &manifest,
            &artifacts,
            "docs/changes/2026-04-22-change-missing",
        )
        .expect_err("missing decision section should fail");

        assert!(error.to_string().contains("missing a decision section"));
    }

    #[test]
    fn build_migration_adr_maps_migration_packet_sections() {
        let manifest = manifest_for(Mode::Migration, "R-20260422-migrate1");
        let artifacts = vec![
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "source-target-map.md",
                "# Source-Target Map\n\n## Current State\n\n- auth-v1 serves login and token refresh traffic.\n\n## Target State\n\n- auth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\n- login and token refresh only.\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "compatibility-matrix.md",
                "# Compatibility Matrix\n\n## Options Matrix\n\n- Option 1 keeps dual-write through the cutover window.\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "decision-record.md",
                "# Decision Record\n\n## Migration Decisions\n\n- retain dual-write during the bounded cutover\n\n## Tradeoff Analysis\n\n- dual-write raises temporary complexity but keeps rollback safer while the bounded surface proves stable\n\n## Ecosystem Health\n\n- auth-v2 dependencies are healthy enough for bounded cutover\n\n## Residual Risks\n\n- admin reporting remains temporarily inconsistent\n",
            ),
        ];

        let adr = super::build_migration_adr(
            &manifest,
            &artifacts,
            "docs/migrations/2026-04-22-migration",
        )
        .expect("migration adr");

        assert_eq!(adr.title, "retain dual-write during the bounded cutover");
        assert!(adr.context.contains("### Current State"));
        assert!(adr.context.contains("### Target State"));
        assert!(adr.decision.contains("retain dual-write during the bounded cutover"));
        assert!(adr.consequences.contains("### Tradeoff Analysis"));
        assert!(adr.alternatives.as_deref().is_some_and(|value| {
            value.contains("Option 1 keeps dual-write through the cutover window.")
        }));
    }

    #[test]
    fn build_migration_adr_reports_missing_decision_section() {
        let manifest = manifest_for(Mode::Migration, "R-20260422-migrate-missing");
        let artifacts = vec![
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "source-target-map.md",
                "# Source-Target Map\n\n## Current State\n\n- auth-v1\n\n## Target State\n\n- auth-v2\n\n## Transition Boundaries\n\n- login only\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "compatibility-matrix.md",
                "# Compatibility Matrix\n\n## Options Matrix\n\n- dual-write\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "decision-record.md",
                "# Decision Record\n\n## Tradeoff Analysis\n\n- bounded dual-write\n",
            ),
        ];

        let error = super::build_migration_adr(
            &manifest,
            &artifacts,
            "docs/migrations/2026-04-22-migration-missing",
        )
        .expect_err("missing migration decision section should fail");

        assert!(error.to_string().contains("missing a decision section"));
    }

    #[test]
    fn build_architecture_adr_reports_missing_required_sections() {
        let manifest = manifest_for(Mode::Architecture, "R-20260422-arch0001");
        let tradeoffs = markdown_artifact(
            &manifest.run_id,
            manifest.mode,
            "tradeoff-matrix.md",
            "# Tradeoff Matrix\n\n## Options Considered\n\n- keep the current generic decision summary\n\n## Pros\n\n- reviewers can reuse the packet outside the originating conversation\n",
        );

        let missing_summary = vec![
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "architecture-overview.md",
                "# Architecture Overview\n\n## Included Views\n\n- context map\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "architecture-decisions.md",
                "# Architecture Decisions\n\n## Decision\n\nUse a dedicated context map to make architecture boundaries reviewable.\n",
            ),
            tradeoffs.clone(),
        ];
        let missing_summary_error = super::build_architecture_adr(
            &manifest,
            &missing_summary,
            "docs/architecture/decisions/2026-04-22-architecture",
        )
        .expect_err("missing summary should fail");
        assert!(missing_summary_error.to_string().contains("`## Summary`"));

        let missing_decision = vec![
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "architecture-overview.md",
                "# Architecture Overview\n\n## Summary\n\nDecision focus: map boundaries and tradeoffs.\n",
            ),
            markdown_artifact(
                &manifest.run_id,
                manifest.mode,
                "architecture-decisions.md",
                "# Architecture Decisions\n\n## Decision Drivers\n\n- reviewers need the chosen direction\n",
            ),
            tradeoffs,
        ];
        let missing_decision_error = super::build_architecture_adr(
            &manifest,
            &missing_decision,
            "docs/architecture/decisions/2026-04-22-architecture",
        )
        .expect_err("missing decision should fail");
        assert!(missing_decision_error.to_string().contains("`## Decision`"));
    }

    #[test]
    fn helper_functions_cover_section_selection_title_fallback_and_registry_numbering() {
        assert_eq!(
            super::preferred_section(
                "## Recommendation\n\n- use the fallback\n",
                &["Decision", "Recommendation"]
            ),
            Some("- use the fallback".to_string())
        );
        assert!(super::labeled_sections(&[("Only", None)]).is_err());

        let with_title = sample_manifest("R-20260422-title001");
        assert_eq!(super::fallback_adr_title(&with_title), "Publish Scope");

        let mut slug_only = sample_manifest("R-20260422-title002");
        slug_only.title = None;
        slug_only.slug = Some("durable-decision".to_string());
        assert_eq!(super::fallback_adr_title(&slug_only), "Durable Decision");
        assert_eq!(super::titleize_slug(""), "");
        assert_eq!(super::normalize_adr_title_line("- keep dual-write"), "keep dual-write");

        let mut no_short_id = sample_manifest("run-12345678");
        no_short_id.short_id = None;
        assert_eq!(super::short_id_fragment(&no_short_id), "12345678");

        let registry = tempdir().expect("registry tempdir");
        assert_eq!(super::next_adr_number(registry.path()).expect("empty registry"), 1);
        fs::write(registry.path().join("ADR-0002-existing.md"), "# ADR 0002\n")
            .expect("write adr 2");
        fs::write(registry.path().join("notes.txt"), "ignore\n").expect("write other file");
        fs::write(registry.path().join("ADR-0005"), "missing separator\n")
            .expect("write malformed adr without suffix");
        fs::write(registry.path().join("ADR-00x4-bad.md"), "bad digits\n")
            .expect("write malformed adr digits");
        assert_eq!(super::next_adr_number(registry.path()).expect("numbered registry"), 3);
    }

    #[test]
    fn build_adr_export_assigns_numbered_paths_for_supported_opt_in_modes() {
        let workspace = tempdir().expect("temp workspace");
        let registry_root = workspace.path().join("docs").join("adr");
        fs::create_dir_all(&registry_root).expect("registry root");
        fs::write(registry_root.join("ADR-0002-existing.md"), "# ADR 0002\n")
            .expect("write existing adr");

        let change_manifest = manifest_for(Mode::Change, "R-20260422-change02");
        let change_artifacts = vec![
            markdown_artifact(
                &change_manifest.run_id,
                change_manifest.mode,
                "change-surface.md",
                "# Change Surface\n\n## Change Surface\n\n- session repository\n",
            ),
            markdown_artifact(
                &change_manifest.run_id,
                change_manifest.mode,
                "implementation-plan.md",
                "# Implementation Plan\n\n## Implementation Plan\n\nAdd bounded repository methods and preserve the public auth contract.\n",
            ),
            markdown_artifact(
                &change_manifest.run_id,
                change_manifest.mode,
                "decision-record.md",
                "# Decision Record\n\n## Decision Record\n\nPrefer additive change over normalization to preserve operator expectations.\n\n## Consequences\n\n- preserved surface remains explicit and reviewable\n",
            ),
        ];
        let change_adr = super::build_adr_export(
            workspace.path(),
            &change_manifest,
            &change_artifacts,
            Path::new("docs/changes/2026-04-22-change"),
        )
        .expect("change adr export");
        assert!(change_adr.display_path.starts_with("docs/adr/ADR-0003-"));

        fs::write(&change_adr.destination, &change_adr.contents).expect("persist change adr");

        let migration_manifest = manifest_for(Mode::Migration, "R-20260422-migrate2");
        let migration_artifacts = vec![
            markdown_artifact(
                &migration_manifest.run_id,
                migration_manifest.mode,
                "source-target-map.md",
                "# Source-Target Map\n\n## Current State\n\n- auth-v1 serves login and token refresh traffic.\n\n## Target State\n\n- auth-v2 serves the same bounded traffic surface.\n\n## Transition Boundaries\n\n- login and token refresh only.\n",
            ),
            markdown_artifact(
                &migration_manifest.run_id,
                migration_manifest.mode,
                "compatibility-matrix.md",
                "# Compatibility Matrix\n\n## Options Matrix\n\n- Option 1 keeps dual-write through the cutover window.\n",
            ),
            markdown_artifact(
                &migration_manifest.run_id,
                migration_manifest.mode,
                "decision-record.md",
                "# Decision Record\n\n## Migration Decisions\n\n- retain dual-write during the bounded cutover\n\n## Tradeoff Analysis\n\n- dual-write raises temporary complexity but keeps rollback safer while the bounded surface proves stable\n",
            ),
        ];
        let migration_adr = super::build_adr_export(
            workspace.path(),
            &migration_manifest,
            &migration_artifacts,
            Path::new("docs/migrations/2026-04-22-migration"),
        )
        .expect("migration adr export");
        assert!(migration_adr.display_path.starts_with("docs/adr/ADR-0004-"));
    }

    #[test]
    fn evaluate_promotion_policy_maps_completed_analysis_to_auto() {
        let manifest = sample_manifest("R-test");
        for mode in [
            Mode::SystemShaping,
            Mode::Discovery,
            Mode::Requirements,
            Mode::DomainLanguage,
            Mode::DomainModel,
            Mode::Backlog,
        ] {
            let mut m = manifest.clone();
            m.mode = mode;
            assert_eq!(
                super::evaluate_promotion_policy(mode, &RunState::Completed, &m),
                PromotionState::Auto,
                "expected Auto for completed {mode:?}"
            );
        }
    }

    #[test]
    fn evaluate_promotion_policy_maps_completed_gated_to_auto_if_approved() {
        let manifest = sample_manifest("R-test");
        for mode in [
            Mode::Architecture,
            Mode::Change,
            Mode::Implementation,
            Mode::Refactor,
            Mode::Migration,
        ] {
            let mut m = manifest.clone();
            m.mode = mode;
            assert_eq!(
                super::evaluate_promotion_policy(mode, &RunState::Completed, &m),
                PromotionState::AutoIfApproved,
                "expected AutoIfApproved for completed {mode:?}"
            );
        }
    }

    #[test]
    fn evaluate_promotion_policy_maps_non_completed_gated_to_pending() {
        let manifest = sample_manifest("R-test");
        for mode in [Mode::Architecture, Mode::Change] {
            let mut m = manifest.clone();
            m.mode = mode;
            assert_eq!(
                super::evaluate_promotion_policy(mode, &RunState::AwaitingApproval, &m),
                PromotionState::PendingIndex,
            );
        }
    }

    #[test]
    fn evaluate_promotion_policy_maps_evidence_modes_to_evidence_only() {
        let manifest = sample_manifest("R-test");
        for mode in [
            Mode::Verification,
            Mode::Review,
            Mode::PrReview,
            Mode::SecurityAssessment,
            Mode::SupplyChainAnalysis,
            Mode::SystemAssessment,
        ] {
            let mut m = manifest.clone();
            m.mode = mode;
            assert_eq!(
                super::evaluate_promotion_policy(mode, &RunState::Completed, &m),
                PromotionState::EvidenceOnly,
            );
        }
    }

    #[test]
    fn evaluate_promotion_policy_maps_incident_states() {
        let manifest = sample_manifest("R-test");
        let mut m = manifest.clone();
        m.mode = Mode::Incident;
        assert_eq!(
            super::evaluate_promotion_policy(Mode::Incident, &RunState::Completed, &m),
            PromotionState::PendingIndex,
        );
        assert_eq!(
            super::evaluate_promotion_policy(Mode::Incident, &RunState::AwaitingApproval, &m),
            PromotionState::Manual,
        );
    }

    #[test]
    fn default_update_strategy_for_modes() {
        assert_eq!(
            super::default_update_strategy_for(Mode::Architecture),
            UpdateStrategy::ManagedBlocks
        );
        assert_eq!(
            super::default_update_strategy_for(Mode::Migration),
            UpdateStrategy::ProposalFiles
        );
        assert_eq!(
            super::default_update_strategy_for(Mode::Incident),
            UpdateStrategy::ProposalFiles
        );
        assert_eq!(
            super::default_update_strategy_for(Mode::Review),
            UpdateStrategy::AppendOnlyIndex
        );
    }

    #[test]
    fn write_managed_block_inserts_and_replaces() {
        let workspace = tempdir().expect("temp workspace");
        let target = workspace.path().join("doc.md");

        super::write_managed_block(&target, "test-block", "initial content").expect("first write");
        let first = fs::read_to_string(&target).expect("read first");
        assert!(first.contains(
            "<!-- project-memory:managed:start producer=\"canon\" source_ref=\"test-block\" contract_version=\"v1\" -->"
        ));
        assert!(first.contains("initial content"));
        assert!(first.contains("<!-- project-memory:managed:end -->"));

        super::write_managed_block(&target, "test-block", "updated content").expect("second write");
        let second = fs::read_to_string(&target).expect("read second");
        assert!(second.contains("updated content"));
        assert!(!second.contains("initial content"));
    }

    #[test]
    fn write_managed_block_preserves_curated_content() {
        let workspace = tempdir().expect("temp workspace");
        let target = workspace.path().join("curated.md");
        let curated = "# My Document\n\nHuman-authored context.\n\n<!-- project-memory:managed:start producer=\"canon\" source_ref=\"test\" contract_version=\"v1\" -->\nold Canon data\n<!-- project-memory:managed:end -->\n\nMore human content.\n";
        fs::write(&target, curated).expect("write curated");

        super::write_managed_block(&target, "test", "new Canon data").expect("update block");
        let result = fs::read_to_string(&target).expect("read result");
        assert!(result.contains("Human-authored context."));
        assert!(result.contains("new Canon data"));
        assert!(result.contains("More human content."));
        assert!(!result.contains("old Canon data"));
    }

    #[test]
    fn write_managed_block_replaces_legacy_canon_marker() {
        let workspace = tempdir().expect("temp workspace");
        let target = workspace.path().join("legacy.md");
        let legacy = "<!-- canon:managed-block:test:start -->\nold Canon data\n<!-- canon:managed-block:test:end -->\n";
        fs::write(&target, legacy).expect("write legacy");

        super::write_managed_block(&target, "test", "new Canon data").expect("migrate block");
        let result = fs::read_to_string(&target).expect("read migrated");
        assert!(result.contains("project-memory:managed:start"));
        assert!(!result.contains("canon:managed-block"));
        assert!(result.contains("new Canon data"));
    }

    #[test]
    fn write_managed_block_replaces_legacy_marker_without_end_marker() {
        let workspace = tempdir().expect("temp workspace");
        let target = workspace.path().join("legacy-open.md");
        let legacy = "preface\n<!-- canon:managed-block:test:start -->\nold Canon data\n";
        fs::write(&target, legacy).expect("write legacy");

        super::write_managed_block(&target, "test", "new Canon data").expect("migrate block");
        let result = fs::read_to_string(&target).expect("read migrated");
        assert!(result.contains("preface"));
        assert!(result.contains("project-memory:managed:start"));
        assert!(result.contains("<!-- project-memory:managed:end -->"));
        assert!(result.contains("new Canon data"));
    }

    #[test]
    fn write_proposal_file_emits_sidecar() {
        let workspace = tempdir().expect("temp workspace");
        let dest = workspace.path().join("proposals");
        let lineage = super::LineageMetadata {
            contract_version: "v1".into(),
            producer: "canon".into(),
            source_ref: "canon-run:R-test".into(),
            source_artifacts: vec!["artifact.md".into()],
            promotion_state: PromotionState::PendingIndex,
            promoted_at: "2026-05-13T00:00:00Z".into(),
            content_digest: "sha256:abc123".into(),
            mode: Some("architecture".into()),
            stage: None,
            owner: None,
            risk: None,
            zone: None,
            approval_state: Some("AwaitingApproval".into()),
            packet_readiness: Some("partial".into()),
            promotion_profile: Some(PublishProfile::ProjectMemory),
        };

        let path = super::write_proposal_file(
            &dest,
            "decision-record.md",
            "# Proposal\nContent",
            &lineage,
        )
        .expect("write proposal");
        assert!(path.ends_with("decision-record.proposal.md"));
        let content = fs::read_to_string(&path).expect("read proposal");
        assert!(content.contains("Canon proposal from canon-run:R-test"));
        assert!(content.contains("# Proposal\nContent"));
    }

    #[test]
    fn write_proposal_file_uses_unknown_placeholders_for_missing_optional_lineage_fields() {
        let workspace = tempdir().expect("temp workspace");
        let dest = workspace.path().join("proposals");
        let lineage = super::LineageMetadata {
            contract_version: "v1".into(),
            producer: "canon".into(),
            source_ref: "canon-run:R-test".into(),
            source_artifacts: vec!["artifact.md".into()],
            promotion_state: PromotionState::PendingIndex,
            promoted_at: "2026-05-13T00:00:00Z".into(),
            content_digest: "sha256:abc123".into(),
            mode: None,
            stage: None,
            owner: None,
            risk: None,
            zone: None,
            approval_state: None,
            packet_readiness: None,
            promotion_profile: None,
        };

        let path = super::write_proposal_file(&dest, "decision-record.md", "# Proposal", &lineage)
            .expect("write proposal");
        let content = fs::read_to_string(&path).expect("read proposal");
        assert!(content.contains("Canon proposal from canon-run:R-test (unknown-mode)"));
        assert!(content.contains("promotion_state: pending-index | profile: unknown-profile"));
    }

    #[test]
    fn content_digest_for_artifacts_is_stable_for_same_inputs() {
        let artifacts = vec![
            markdown_artifact(
                "R-test",
                Mode::Requirements,
                "product-context.md",
                "# Product Context\n\nBounded summary.\n",
            ),
            markdown_artifact(
                "R-test",
                Mode::Requirements,
                "delivery-map.md",
                "# Delivery Map\n\nBounded sequence.\n",
            ),
        ];

        let first = super::content_digest_for_artifacts(&artifacts);
        let second = super::content_digest_for_artifacts(&artifacts);

        assert_eq!(first, second);
        assert!(first.starts_with("sha256:"));
    }

    #[test]
    fn content_digest_for_artifacts_ignores_publish_metadata_sidecar() {
        let artifacts = vec![
            markdown_artifact(
                "R-test",
                Mode::Requirements,
                "product-context.md",
                "# Product Context\n\nBounded summary.\n",
            ),
            markdown_artifact(
                "R-test",
                Mode::Requirements,
                PROJECT_MEMORY_PACKET_METADATA_FILE_NAME,
                "{\"ignored\":true}\n",
            ),
        ];
        let without_sidecar = vec![markdown_artifact(
            "R-test",
            Mode::Requirements,
            "product-context.md",
            "# Product Context\n\nBounded summary.\n",
        )];

        assert_eq!(
            super::content_digest_for_artifacts(&artifacts),
            super::content_digest_for_artifacts(&without_sidecar)
        );
    }

    #[test]
    fn append_index_entry_creates_and_appends() {
        let workspace = tempdir().expect("temp workspace");
        let target = workspace.path().join("index.md");

        super::append_index_entry(&target, "- Entry 1\n").expect("first append");
        let first = fs::read_to_string(&target).expect("read first");
        assert!(first.contains("- Entry 1"));

        super::append_index_entry(&target, "- Entry 2\n").expect("second append");
        let second = fs::read_to_string(&target).expect("read second");
        assert!(second.contains("- Entry 1"));
        assert!(second.contains("- Entry 2"));
    }

    #[test]
    fn resolve_profile_destination_routes_by_promotion_state() {
        let repo_root = Path::new("/repo");
        let stable_manifest = sample_manifest("R-20260422-abcd1234");
        let pending_manifest = manifest_for(Mode::Architecture, "R-20260422-bcde2345");
        let evidence_manifest = manifest_for(Mode::Review, "R-20260422-cdef3456");

        let stable =
            super::resolve_profile_destination(repo_root, &stable_manifest, &PromotionState::Auto);
        assert_eq!(stable, Path::new("/repo/docs/project/product-context.md"));

        let pending = super::resolve_profile_destination(
            repo_root,
            &pending_manifest,
            &PromotionState::PendingIndex,
        );
        assert_eq!(pending, Path::new("/repo/docs/project/pending-decisions.md"));

        let evidence = super::resolve_profile_destination(
            repo_root,
            &evidence_manifest,
            &PromotionState::EvidenceOnly,
        );
        assert_eq!(evidence, Path::new("/repo/docs/project/audit-log.md"));
    }

    #[test]
    fn canonical_project_memory_surface_map_covers_all_modes() {
        let expected = [
            (
                Mode::Discovery,
                "docs/project/overview.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::Requirements,
                "docs/project/product-context.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::SystemShaping,
                "docs/project/architecture-map.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::Architecture,
                "docs/project/architecture-map.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::SystemAssessment,
                "docs/project/operational-context.md",
                "docs/project/pending-decisions.md",
                "docs/project/open-risks.md",
            ),
            (
                Mode::Change,
                "docs/project/decision-index.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::Backlog,
                "docs/project/delivery-map.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::PrReview,
                "docs/project/operational-context.md",
                "docs/project/audit-log.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::Implementation,
                "docs/project/delivery-map.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::Refactor,
                "docs/project/delivery-map.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::Verification,
                "docs/project/operational-context.md",
                "docs/project/audit-log.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::Review,
                "docs/project/operational-context.md",
                "docs/project/audit-log.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::Incident,
                "docs/project/operational-context.md",
                "docs/project/open-risks.md",
                "docs/project/open-risks.md",
            ),
            (
                Mode::SecurityAssessment,
                "docs/project/operational-context.md",
                "docs/project/pending-decisions.md",
                "docs/project/open-risks.md",
            ),
            (
                Mode::Migration,
                "docs/project/decision-index.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::SupplyChainAnalysis,
                "docs/project/operational-context.md",
                "docs/project/pending-decisions.md",
                "docs/project/open-risks.md",
            ),
            (
                Mode::DomainLanguage,
                "docs/project/domain-language.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
            (
                Mode::DomainModel,
                "docs/project/domain-model.md",
                "docs/project/pending-decisions.md",
                "docs/project/audit-log.md",
            ),
        ];

        assert_eq!(expected.len(), Mode::all().len());

        for (mode, stable, pending, evidence) in expected {
            assert_eq!(
                super::stable_project_memory_surface(mode),
                stable,
                "stable surface mismatch for {}",
                mode.as_str()
            );
            assert_eq!(
                super::pending_project_memory_surface(mode),
                pending,
                "pending surface mismatch for {}",
                mode.as_str()
            );
            assert_eq!(
                super::evidence_project_memory_surface(mode),
                evidence,
                "evidence surface mismatch for {}",
                mode.as_str()
            );
        }
    }

    #[test]
    fn publish_metadata_backward_compatible_deserialization() {
        let legacy_json = r#"{
            "run_id": "R-test",
            "mode": "requirements",
            "risk": "low-impact",
            "zone": "green",
            "publish_timestamp": "2026-05-13T00:00:00Z",
            "descriptor": "test",
            "destination": "specs/test",
            "source_artifacts": ["artifact.md"]
        }"#;
        let metadata: PublishMetadata =
            serde_json::from_str(legacy_json).expect("legacy metadata should parse");
        assert!(metadata.primary_artifact.is_empty());
        assert!(metadata.artifact_order.is_empty());
        assert!(metadata.profile.is_none());
        assert!(metadata.promotion_state.is_none());
        assert!(metadata.publication_target_class.is_none());
        assert!(metadata.expertise_input.is_none());
        assert!(metadata.lineage.is_none());
    }
}
