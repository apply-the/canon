use crate::orchestrator::service::ModeResultSummary;
use crate::orchestrator::service::context_parse::{
    count_markdown_entries, count_missing_context_markers, extract_context_section,
    truncate_context_excerpt,
};
use crate::persistence::store::PersistedArtifact;

use super::{
    packet_output_quality_artifact_prefix, packet_output_quality_headline,
    primary_artifact_action_for,
};

pub(super) fn summarize_domain_language_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "language-overview.md")?;
    let glossary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "domain-glossary.md");
    let conflicts_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "language-conflicts.md");

    let domain_scope = extract_context_section(&primary.contents, "Domain Scope")
        .unwrap_or_else(|| "NOT CAPTURED - Domain scope section is missing.".to_string());
    let language_maturity = extract_context_section(&primary.contents, "Language Maturity")
        .unwrap_or_else(|| "NOT CAPTURED - Language maturity section is missing.".to_string());
    let glossary_entries = glossary_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Glossary Entries"))
        .unwrap_or_else(|| "NOT CAPTURED - Glossary entries section is missing.".to_string());
    let conflict_inventory = conflicts_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Conflict Inventory"))
        .unwrap_or_else(|| "NOT CAPTURED - Conflict inventory section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &domain_scope,
        &language_maturity,
        &glossary_entries,
        &conflict_inventory,
    ]);
    let glossary_count = count_markdown_entries(&glossary_entries);
    let conflict_count = count_markdown_entries(&conflict_inventory);

    let headline = packet_output_quality_headline(
        "Domain-language",
        missing_context_markers,
        0,
        "",
        "governed review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet defines {glossary_count} glossary term(s) and {conflict_count} language conflict(s). Maturity: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&language_maturity, 90)
        )
    } else {
        format!(
            "{} Glossary terms: {glossary_count}; language conflicts: {conflict_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: Some("recommendation-only".to_string()),
        primary_artifact_title: "Language Overview".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&domain_scope, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_domain_model_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "model-overview.md")?;
    let catalog_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "concept-catalog.md");
    let relationship_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "relationship-map.md");
    let invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "domain-invariants.md");

    let domain_scope = extract_context_section(&primary.contents, "Domain Scope")
        .unwrap_or_else(|| "NOT CAPTURED - Domain scope section is missing.".to_string());
    let model_maturity = extract_context_section(&primary.contents, "Model Maturity")
        .unwrap_or_else(|| "NOT CAPTURED - Model maturity section is missing.".to_string());
    let concepts = catalog_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Concepts"))
        .unwrap_or_else(|| "NOT CAPTURED - Concepts section is missing.".to_string());
    let relationships = relationship_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Relationships"))
        .unwrap_or_else(|| "NOT CAPTURED - Relationships section is missing.".to_string());
    let invariants = invariants_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Invariants"))
        .unwrap_or_else(|| "NOT CAPTURED - Invariants section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &domain_scope,
        &model_maturity,
        &concepts,
        &relationships,
        &invariants,
    ]);
    let concept_count = count_markdown_entries(&concepts);
    let relationship_count = count_markdown_entries(&relationships);
    let invariant_count = count_markdown_entries(&invariants);

    let headline = packet_output_quality_headline(
        "Domain-model",
        missing_context_markers,
        0,
        "",
        "governed review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet defines {concept_count} concept(s), {relationship_count} relationship(s), and {invariant_count} invariant(s). Maturity: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&model_maturity, 90)
        )
    } else {
        format!(
            "{} Concepts: {concept_count}; relationships: {relationship_count}; invariants: {invariant_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: Some("recommendation-only".to_string()),
        primary_artifact_title: "Model Overview".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&domain_scope, 320),
        action_chips: Vec::new(),
    })
}
