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
use crate::persistence::store::{PersistedArtifact, WorkspaceStore};

const ADR_REGISTRY_DIRECTORY: &str = "docs/adr";
const PUBLISH_METADATA_FILE_NAME: &str = "packet-metadata.json";

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

    use super::{
        PUBLISH_METADATA_FILE_NAME, PublishMetadata, default_publish_directory, resolve_destination,
    };
    use crate::domain::artifact::{ArtifactFormat, ArtifactRecord};
    use crate::domain::mode::Mode;
    use crate::domain::policy::{RiskClass, UsageZone};
    use crate::domain::run::{ClassificationProvenance, SystemContext};
    use crate::orchestrator::service::{EngineService, RunRequest};
    use crate::persistence::manifests::RunManifest;
    use crate::persistence::store::PersistedArtifact;

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

    fn architecture_request() -> RunRequest {
        RunRequest {
            mode: Mode::Architecture,
            risk: RiskClass::BoundedImpact,
            zone: UsageZone::Yellow,
            system_context: Some(SystemContext::Existing),
            classification: ClassificationProvenance::explicit(),
            owner: "Owner <owner@example.com>".to_string(),
            inputs: vec!["architecture.md".to_string()],
            inline_inputs: Vec::new(),
            excluded_paths: Vec::new(),
            policy_root: None,
            method_root: None,
        }
    }

    fn architecture_brief() -> &'static str {
        "# Architecture Brief\n\nDecision focus: map boundaries and tradeoffs for governed analysis-mode expansion.\nConstraint: preserve Canon persistence, evidence, and approval behavior.\n\n## Decision\nUse a dedicated context map to make architecture boundaries reviewable.\n\n## Options\n- Keep domain boundaries implicit in existing prose.\n- Add a dedicated `context-map.md` artifact.\n\n## Constraints\n- Preserve run identity and approval behavior.\n- Keep non-target modes unchanged.\n\n## Candidate Boundaries\n- Runtime Governance\n- Artifact Authoring\n\n## Invariants\n- Evidence remains linked to the run.\n- Risk review stays explicit.\n\n## Evaluation Criteria\n- Ownership clarity\n- Seam visibility.\n\n## Decision Drivers\n- Reviewers need the chosen direction and rationale without consulting chat history.\n- The packet must remain critique-first when authored context is weak.\n\n## Options Considered\n- Keep the current generic decision summary.\n- Preserve authored decision and option-analysis sections directly in the existing artifacts.\n\n## Pros\n- The emitted packet records the chosen option and rejected alternatives explicitly.\n- Reviewers can reuse the packet outside the originating conversation.\n\n## Cons\n- The authored brief must carry more explicit decision content.\n\n## Recommendation\nPreserve authored decision and option-analysis sections directly in the existing architecture decision artifacts.\n\n## Why Not The Others\n- The generic summary shape hides rejected alternatives.\n- A new artifact family would widen scope beyond this slice.\n\n## Consequences\n- Architecture reviewers can inspect a durable ADR without reopening the run history.\n\n## Bounded Contexts\n- Runtime Governance: owns approvals, run state, and evidence linkage.\n- Artifact Authoring: owns packet structure and authored-body fidelity.\n\n## Context Relationships\n- Artifact Authoring consumes gate and lineage outcomes from Runtime Governance.\n\n## Integration Seams\n- `mode_shaping` hands rendered artifacts to gate evaluation and summarization.\n\n## Anti-Corruption Candidates\n- Renderer helpers should remain isolated from governance-specific state semantics.\n\n## Ownership Boundaries\n- Governance code owns gate evaluation.\n- Rendering code owns authored markdown fidelity.\n\n## Shared Invariants\n- Every artifact remains bound to one run id.\n- Approval-gated architecture runs cannot skip risk review.\n\n## System Context\n- System: `canon-engine` governs analysis packets and durable evidence.\n- External actors:\n  - architect-reviewer: reads architecture packets.\n  - copilot-cli-adapter: generates and critiques packet content.\n\n## Containers\n- `canon-cli` (Rust CLI): entrypoint for run and inspect commands.\n- `canon-engine` (Rust library): orchestrates generation, critique, gates, and rendering.\n- `.canon/` (filesystem): persists run manifests, artifacts, and evidence.\n\n## Deployment\n- `canon-cli` runs on developer laptops and CI runners.\n- `canon-engine` shares the same Rust process boundary as the CLI.\n- `.canon/` remains the local runtime store on the active workspace filesystem.\n\n## Components\n- `mode_shaping`: runs architecture orchestration.\n- `gatekeeper`: validates contract and policy gates.\n- `markdown renderer`: emits reviewable architecture artifacts.\n"
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

        let error =
            super::publish_run(workspace.path(), &run.run_id, Some(&destination_file), false)
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
            super::publish_run(workspace.path(), &run.run_id, Some(&absolute_destination), false)
                .expect("publish should support absolute override");

        assert_eq!(summary.published_to, absolute_destination.display().to_string());
        assert!(absolute_destination.join("01-problem-statement.md").exists());
        assert!(summary.published_files.iter().any(|path| path
            == &absolute_destination.join("01-problem-statement.md").display().to_string()));
        assert!(absolute_destination.join(PUBLISH_METADATA_FILE_NAME).exists());
    }

    #[test]
    fn publish_run_writes_metadata_sidecar_for_default_destinations() {
        let workspace = tempdir().expect("temp workspace");
        fs::write(workspace.path().join("idea.md"), complete_requirements_brief())
            .expect("write input");

        let service = EngineService::new(workspace.path());
        let run = service.run(requirements_request()).expect("completed run");

        let summary = super::publish_run(workspace.path(), &run.run_id, None, false)
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
            metadata.source_artifacts.iter().any(|path| path.ends_with("01-problem-statement.md"))
        );
        assert!(
            summary.published_files.iter().any(|path| path.ends_with(PUBLISH_METADATA_FILE_NAME))
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
    fn publish_run_generates_and_reports_architecture_adr_in_process() {
        let workspace = tempdir().expect("temp workspace");
        fs::write(workspace.path().join("architecture.md"), architecture_brief())
            .expect("write architecture brief");

        let service = EngineService::new(workspace.path());
        let run = service.run(architecture_request()).expect("completed architecture run");

        let summary = super::publish_run(workspace.path(), &run.run_id, None, false)
            .expect("publish should succeed");

        assert!(summary.published_files.iter().any(|path| path.starts_with("docs/adr/ADR-0001-")));
        assert!(workspace.path().join("docs").join("adr").exists());
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
        assert_eq!(super::normalize_adr_title_line("- keep dual-write"), "keep dual-write");

        let registry = tempdir().expect("registry tempdir");
        assert_eq!(super::next_adr_number(registry.path()).expect("empty registry"), 1);
        fs::write(registry.path().join("ADR-0002-existing.md"), "# ADR 0002\n")
            .expect("write adr 2");
        fs::write(registry.path().join("notes.txt"), "ignore\n").expect("write other file");
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
}
