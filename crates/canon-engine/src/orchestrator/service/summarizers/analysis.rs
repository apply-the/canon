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

pub(super) fn summarize_security_assessment_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "assessment-overview.md")?;
    let threat_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "threat-model.md");
    let risk_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "risk-register.md");

    let assessment_scope = extract_context_section(&primary.contents, "Assessment Scope")
        .unwrap_or_else(|| "NOT CAPTURED - Assessment scope section is missing.".to_string());
    let in_scope_assets = extract_context_section(&primary.contents, "In-Scope Assets")
        .unwrap_or_else(|| "NOT CAPTURED - In-scope assets section is missing.".to_string());
    let trust_boundaries = extract_context_section(&primary.contents, "Trust Boundaries")
        .unwrap_or_else(|| "NOT CAPTURED - Trust boundaries section is missing.".to_string());
    let threat_inventory = threat_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Threat Inventory"))
        .unwrap_or_else(|| "NOT CAPTURED - Threat inventory section is missing.".to_string());
    let risk_findings = risk_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Risk Findings"))
        .unwrap_or_else(|| "NOT CAPTURED - Risk findings section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &assessment_scope,
        &in_scope_assets,
        &trust_boundaries,
        &threat_inventory,
        &risk_findings,
    ]);
    let asset_count = count_markdown_entries(&in_scope_assets);
    let threat_count = count_markdown_entries(&threat_inventory);
    let risk_count = count_markdown_entries(&risk_findings);

    let headline = packet_output_quality_headline(
        "Security-assessment",
        missing_context_markers,
        0,
        "",
        "governed review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet bounds {asset_count} in-scope asset set(s), {threat_count} threat set(s), and {risk_count} rated risk set(s). Trust boundaries: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&trust_boundaries, 90)
        )
    } else {
        format!(
            "{} In-scope assets: {asset_count}; threats: {threat_count}; rated risks: {risk_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: Some("recommendation-only".to_string()),
        primary_artifact_title: "Assessment Overview".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&assessment_scope, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_system_assessment_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "assessment-overview.md")?;
    let coverage_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "coverage-map.md");
    let inventory_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "asset-inventory.md");
    let risk_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "risk-register.md");

    let assessment_objective = extract_context_section(&primary.contents, "Assessment Objective")
        .unwrap_or_else(|| "NOT CAPTURED - Assessment objective section is missing.".to_string());
    let assessed_views = coverage_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Assessed Views"))
        .unwrap_or_else(|| "NOT CAPTURED - Assessed views section is missing.".to_string());
    let assessed_assets = inventory_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Assessed Assets"))
        .unwrap_or_else(|| "NOT CAPTURED - Assessed assets section is missing.".to_string());
    let observed_risks = risk_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Observed Risks"))
        .unwrap_or_else(|| "NOT CAPTURED - Observed risks section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &assessment_objective,
        &assessed_views,
        &assessed_assets,
        &observed_risks,
    ]);
    let view_count = count_markdown_entries(&assessed_views);
    let asset_count = count_markdown_entries(&assessed_assets);
    let risk_count = count_markdown_entries(&observed_risks);

    let headline = packet_output_quality_headline(
        "System-assessment",
        missing_context_markers,
        0,
        "",
        "governed review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet records {view_count} assessed view set(s), {asset_count} assessed asset set(s), and {risk_count} observed risk set(s). Objective: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&assessment_objective, 90)
        )
    } else {
        format!(
            "{} Assessed views: {view_count}; assessed assets: {asset_count}; observed risks: {risk_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: Some("recommendation-only".to_string()),
        primary_artifact_title: "Assessment Overview".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&assessment_objective, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_supply_chain_analysis_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "analysis-overview.md")?;
    let vulnerability_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "vulnerability-triage.md");
    let legacy_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "legacy-posture.md");

    let declared_scope = extract_context_section(&primary.contents, "Declared Scope")
        .unwrap_or_else(|| "NOT CAPTURED - Declared scope section is missing.".to_string());
    let distribution_model = extract_context_section(&primary.contents, "Distribution Model")
        .unwrap_or_else(|| "NOT CAPTURED - Distribution model section is missing.".to_string());
    let ecosystems_in_scope = extract_context_section(&primary.contents, "Ecosystems In Scope")
        .unwrap_or_else(|| "NOT CAPTURED - Ecosystems in scope section is missing.".to_string());
    let findings_by_severity = vulnerability_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Findings By Severity"))
        .unwrap_or_else(|| "NOT CAPTURED - Findings by severity section is missing.".to_string());
    let modernization_slices = legacy_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Modernization Slices"))
        .unwrap_or_else(|| "NOT CAPTURED - Modernization slices section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &declared_scope,
        &distribution_model,
        &ecosystems_in_scope,
        &findings_by_severity,
        &modernization_slices,
    ]);
    let ecosystem_count = count_markdown_entries(&ecosystems_in_scope);
    let finding_count = count_markdown_entries(&findings_by_severity);
    let modernization_count = count_markdown_entries(&modernization_slices);

    let headline = packet_output_quality_headline(
        "Supply-chain-analysis",
        missing_context_markers,
        0,
        "",
        "governed review",
    );
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "{} Packet bounds {ecosystem_count} ecosystem set(s), {finding_count} finding set(s), and {modernization_count} modernization slice set(s). Distribution model: {}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, ""),
            truncate_context_excerpt(&distribution_model, 90)
        )
    } else {
        format!(
            "{} Ecosystems: {ecosystem_count}; findings: {finding_count}; modernization slices: {modernization_count}.",
            packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: Some("recommendation-only".to_string()),
        primary_artifact_title: "Analysis Overview".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&declared_scope, 320),
        action_chips: Vec::new(),
    })
}
