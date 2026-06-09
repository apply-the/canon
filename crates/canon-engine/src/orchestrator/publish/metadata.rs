use std::collections::BTreeSet;

use super::*;
use crate::domain::artifact::{
    RuntimePacketMetadata, artifact_slug, is_packet_sidecar, should_skip_repo_scan_directory,
};

const MAX_REPO_SCAN_DEPTH: usize = 4;

pub(super) fn runtime_packet_metadata(artifacts: &[PersistedArtifact]) -> RuntimePacketMetadata {
    artifacts
        .iter()
        .find(|artifact| {
            artifact_slug(&artifact.record.file_name) == PROJECT_MEMORY_PACKET_METADATA_FILE_NAME
        })
        .and_then(|artifact| serde_json::from_str::<RuntimePacketMetadata>(&artifact.contents).ok())
        .unwrap_or_else(|| infer_runtime_packet_metadata(artifacts))
}

pub(super) fn infer_runtime_packet_metadata(
    artifacts: &[PersistedArtifact],
) -> RuntimePacketMetadata {
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
        semantic_descriptor: None,
        authority_governance: None,
        adaptive_governance: None,
        workspace_identity: None,
    }
}

pub(super) fn resolve_expertise_input_metadata(
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
