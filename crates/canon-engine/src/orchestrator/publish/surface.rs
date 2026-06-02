//! Project-memory surface writing helpers for publish flows.
//!
//! Owns the filesystem mutations for managed blocks, proposal files, audit
//! indexes, and evidence bundles so the parent publish facade can stay focused
//! on orchestration and policy decisions.

use super::*;
use crate::domain::artifact::is_packet_sidecar;

pub(super) fn publish_profile_directory(
    context: &PublishProfileWriteContext<'_>,
    destination: &Path,
    published_files: &mut Vec<String>,
) -> Result<PathBuf, EngineError> {
    if destination.exists() && !destination.is_dir() {
        return Err(EngineError::Validation(format!(
            "publish destination `{}` must be a directory",
            destination.display()
        )));
    }
    fs::create_dir_all(destination)?;

    match context.strategy {
        UpdateStrategy::ManagedBlocks => {
            for artifact in context
                .artifacts
                .iter()
                .filter(|artifact| !is_packet_sidecar(&artifact.record.file_name))
            {
                let target = destination.join(&artifact.record.file_name);
                write_managed_block(&target, &context.manifest.run_id, &artifact.contents)?;
                published_files.push(display_path(context.repo_root, &target));
            }
        }
        UpdateStrategy::ProposalFiles => {
            for artifact in context
                .artifacts
                .iter()
                .filter(|artifact| !is_packet_sidecar(&artifact.record.file_name))
            {
                let proposal_path = write_proposal_file(
                    destination,
                    &artifact.record.file_name,
                    &artifact.contents,
                    context.lineage,
                )?;
                published_files.push(display_path(context.repo_root, &proposal_path));
            }
        }
        UpdateStrategy::AppendOnlyIndex => {
            let index_path = destination.join("index.md");
            let entry = format!(
                "## {}\n\n- Run: `{}`\n- Promotion: `{}`\n- Published: `{}`\n\n",
                publish_descriptor(context.manifest),
                context.manifest.run_id,
                context.promotion.as_str(),
                context.publish_timestamp,
            );
            append_index_entry(&index_path, &entry)?;
            published_files.push(display_path(context.repo_root, &index_path));
            let evidence_dir = destination.join(&context.manifest.run_id);
            for path in write_supporting_evidence_bundle(&evidence_dir, context.artifacts)? {
                published_files.push(display_path(context.repo_root, &path));
            }
        }
    }

    Ok(destination.to_path_buf())
}

pub(super) fn publish_profile_surface(
    context: &PublishProfileWriteContext<'_>,
    destination: &Path,
    published_files: &mut Vec<String>,
) -> Result<PathBuf, EngineError> {
    match context.strategy {
        UpdateStrategy::ManagedBlocks => {
            let bundle = render_project_memory_surface(context.artifacts);
            write_managed_block(destination, &context.manifest.run_id, &bundle)?;
            published_files.push(display_path(context.repo_root, destination));
            Ok(destination.to_path_buf())
        }
        UpdateStrategy::ProposalFiles => {
            let bundle = render_project_memory_surface(context.artifacts);
            let proposal_path = write_surface_proposal_file(destination, &bundle, context.lineage)?;
            published_files.push(display_path(context.repo_root, &proposal_path));
            Ok(proposal_path)
        }
        UpdateStrategy::AppendOnlyIndex => {
            let evidence_dir = resolve_profile_evidence_root(context.repo_root, context.manifest);
            let entry = render_project_memory_index_entry(
                context.manifest,
                &context.promotion,
                context.publish_timestamp,
                &display_path(context.repo_root, &evidence_dir),
            );
            append_index_entry(destination, &entry)?;
            published_files.push(display_path(context.repo_root, destination));
            for path in write_supporting_evidence_bundle(&evidence_dir, context.artifacts)? {
                published_files.push(display_path(context.repo_root, &path));
            }
            Ok(destination.to_path_buf())
        }
    }
}

pub(super) fn render_project_memory_surface(artifacts: &[PersistedArtifact]) -> String {
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
    repo_root.join("tech-docs/evidence").join(manifest.mode.as_str()).join(&manifest.run_id)
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

pub(super) fn profile_metadata_path(target: &Path) -> PathBuf {
    if target.extension().is_none() {
        return target.join(PROJECT_MEMORY_PACKET_METADATA_FILE_NAME);
    }

    let stem = target.file_stem().and_then(|value| value.to_str()).unwrap_or("packet");
    target.with_file_name(format!("{stem}{PROFILE_METADATA_FILE_SUFFIX}"))
}

pub(super) fn write_managed_block(
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
        format!("{existing}\n\n{new_block}\n")
    };

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(target, updated)?;
    Ok(())
}

pub(super) fn write_proposal_file(
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

pub(super) fn append_index_entry(target: &Path, entry: &str) -> Result<(), EngineError> {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    let existing = if target.exists() { fs::read_to_string(target)? } else { String::new() };
    fs::write(target, format!("{existing}{entry}"))?;
    Ok(())
}
