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

pub(super) fn summarize_incident_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "incident-frame.md")?;
    let blast_radius_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "blast-radius-map.md");
    let containment_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "containment-plan.md");

    let incident_scope = extract_context_section(&primary.contents, "Incident Scope")
        .unwrap_or_else(|| "NOT CAPTURED - Incident scope section is missing.".to_string());
    let current_state = extract_context_section(&primary.contents, "Trigger And Current State")
        .unwrap_or_else(|| {
            "NOT CAPTURED - Trigger and current state section is missing.".to_string()
        });
    let impacted_surfaces = blast_radius_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Impacted Surfaces"))
        .unwrap_or_else(|| "NOT CAPTURED - Impacted surfaces section is missing.".to_string());
    let immediate_actions = containment_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Immediate Actions"))
        .unwrap_or_else(|| "NOT CAPTURED - Immediate actions section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &incident_scope,
        &current_state,
        &impacted_surfaces,
        &immediate_actions,
    ]);
    let impacted_surface_count = count_markdown_entries(&impacted_surfaces);
    let immediate_action_count = count_markdown_entries(&immediate_actions);

    let headline = packet_output_quality_headline(
        "Incident",
        missing_context_markers,
        0,
        "",
        "governed containment review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet bounds {impacted_surface_count} impacted surface(s) with {immediate_action_count} immediate action set(s). Current state: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&current_state, 120)
        )
    } else {
        format!(
            "{} Impacted surfaces: {impacted_surface_count}; immediate actions: {immediate_action_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: Some("recommendation-only".to_string()),
        primary_artifact_title: "Incident Frame".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&incident_scope, 320),
        action_chips: Vec::new(),
    })
}
