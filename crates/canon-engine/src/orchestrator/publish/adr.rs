//! ADR export helpers for Canon publish flows.
//!
//! Keeps packet-to-ADR extraction, numbering, and rendering logic together so
//! the parent publish facade only decides when ADR export is allowed.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct GeneratedAdr {
    pub(super) destination: PathBuf,
    pub(super) display_path: String,
    pub(super) contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AdrDocument {
    pub(super) title: String,
    pub(super) context: String,
    pub(super) decision: String,
    pub(super) consequences: String,
    pub(super) alternatives: Option<String>,
    pub(super) source_packet: Option<String>,
}

pub(super) fn adr_export_enabled(mode: Mode, requested: bool) -> Result<bool, EngineError> {
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

pub(super) fn build_adr_export(
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

pub(super) fn build_architecture_adr(
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

pub(super) fn build_change_adr(
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

pub(super) fn build_migration_adr(
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

pub(super) fn preferred_section(markdown: &str, sections: &[&str]) -> Option<String> {
    sections.iter().find_map(|section| markdown_section(markdown, section))
}

pub(super) fn labeled_sections(sections: &[(&str, Option<String>)]) -> Result<String, EngineError> {
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

pub(super) fn normalize_adr_title_line(line: &str) -> String {
    line.trim()
        .trim_start_matches("- ")
        .trim_start_matches("* ")
        .trim_start_matches("1. ")
        .to_string()
}

pub(super) fn fallback_adr_title(manifest: &RunManifest) -> String {
    manifest.title.clone().unwrap_or_else(|| titleize_slug(&publish_descriptor(manifest)))
}

pub(super) fn next_adr_number(registry_root: &Path) -> Result<u32, EngineError> {
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
