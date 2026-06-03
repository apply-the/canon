use sha2::{Digest, Sha256};

use super::*;

pub(super) fn packet_readiness_for(state: &RunState) -> &'static str {
    if *state == RunState::Completed { "complete" } else { "partial" }
}

pub(super) fn content_digest_for_artifacts(artifacts: &[PersistedArtifact]) -> String {
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

pub(super) fn titleize_slug(slug: &str) -> String {
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

pub(super) fn resolve_destination(
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

pub(super) fn publish_descriptor(manifest: &RunManifest) -> String {
    manifest
        .slug
        .as_deref()
        .and_then(slugify)
        .or_else(|| manifest.title.as_deref().and_then(slugify))
        .unwrap_or_else(|| manifest.mode.as_str().to_string())
}

pub(super) fn publish_date(timestamp: OffsetDateTime) -> String {
    let date = timestamp.date();
    format!("{:04}-{:02}-{:02}", date.year(), date.month() as u8, date.day())
}

pub(super) fn short_id_fragment(manifest: &RunManifest) -> String {
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

pub(super) fn source_artifact_path(run_id: &str, mode: Mode, file_name: &str) -> String {
    format!(".canon/artifacts/{run_id}/{}/{file_name}", mode.as_str())
}

pub(super) fn display_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .map(|relative| relative.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

pub(super) fn default_publish_directory(mode: Mode) -> &'static str {
    match mode {
        Mode::Requirements => "specs",
        Mode::Discovery => "tech-docs/discovery",
        Mode::SystemShaping => "tech-docs/architecture/shaping",
        Mode::Change => "tech-docs/changes",
        Mode::Backlog => "tech-docs/planning",
        Mode::Architecture => "tech-docs/architecture/decisions",
        Mode::Implementation => "tech-docs/implementation",
        Mode::Refactor => "tech-docs/refactors",
        Mode::Verification => "tech-docs/verification",
        Mode::Review => "tech-docs/reviews",
        Mode::PrReview => "tech-docs/reviews/prs",
        Mode::Incident => "tech-docs/incidents",
        Mode::SystemAssessment => "tech-docs/architecture/assessments",
        Mode::SecurityAssessment => "tech-docs/security-assessments",
        Mode::Migration => "tech-docs/migrations",
        Mode::SupplyChainAnalysis => "tech-docs/supply-chain",
        Mode::DomainLanguage => "tech-docs/domain/language",
        Mode::DomainModel => "tech-docs/domain/model",
        Mode::Debugging => "tech-docs/debugging",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_publish_directory_covers_debugging() {
        assert_eq!(default_publish_directory(Mode::Debugging), "tech-docs/debugging");
    }
}
