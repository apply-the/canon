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

pub(super) fn summarize_requirements_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary =
        artifacts.iter().find(|artifact| artifact.record.slug() == "problem-statement.md")?;
    let constraints_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "constraints.md");
    let scope_cuts_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "scope-cuts.md");
    let decision_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "decision-checklist.md");

    let problem = extract_context_section(&primary.contents, "Problem")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Problem statement summary is missing.".to_string());
    let constraints = constraints_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Constraints"))
        .unwrap_or_else(|| "NOT CAPTURED - Constraints section is missing.".to_string());
    let scope_cuts = scope_cuts_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Scope Cuts"))
        .unwrap_or_else(|| "NOT CAPTURED - Scope cuts section is missing.".to_string());
    let open_questions = decision_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Open Questions"))
        .unwrap_or_else(|| "NOT CAPTURED - Open questions section is missing.".to_string());

    let missing_context_markers = [&problem, &constraints, &scope_cuts, &open_questions]
        .into_iter()
        .filter(|section| section.contains("NOT CAPTURED"))
        .count();
    let constraint_count = count_markdown_entries(&constraints);
    let scope_cut_count = count_markdown_entries(&scope_cuts);
    let open_question_count = count_markdown_entries(&open_questions);

    let headline = packet_output_quality_headline(
        "Requirements",
        missing_context_markers,
        open_question_count,
        "open question set(s)",
        "downstream review",
    );
    let artifact_packet_summary = format!(
        "{} Packet captures {constraint_count} constraint point(s), {scope_cut_count} scope cut(s), and {open_question_count} open question(s).",
        packet_output_quality_artifact_prefix(
            missing_context_markers,
            open_question_count,
            "open question set(s)"
        )
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Problem Statement".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&problem, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_discovery_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts.iter().find(|artifact| artifact.record.slug() == "problem-map.md")?;
    let unknowns_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "unknowns-and-assumptions.md");
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "context-boundary.md");

    let problem_domain = extract_context_section(&primary.contents, "Problem Domain")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Problem domain summary is missing.".to_string());
    let repo_signals = extract_context_section(&primary.contents, "Repo Surface")
        .unwrap_or_else(|| "NOT CAPTURED - Repository signals are missing.".to_string());
    let next_phase = extract_context_section(&primary.contents, "Downstream Handoff")
        .or_else(|| {
            boundary_artifact.and_then(|artifact| {
                extract_context_section(&artifact.contents, "Translation Trigger")
            })
        })
        .unwrap_or_else(|| "NOT CAPTURED - Next-phase handoff is missing.".to_string());
    let unknowns = unknowns_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Unknowns"))
        .unwrap_or_else(|| "NOT CAPTURED - Unknowns section is missing.".to_string());

    let missing_context_markers =
        count_missing_context_markers([&problem_domain, &repo_signals, &next_phase, &unknowns]);
    let repo_signal_count = count_markdown_entries(&repo_signals);
    let unknown_count = count_markdown_entries(&unknowns);

    let headline = packet_output_quality_headline(
        "Discovery",
        missing_context_markers,
        unknown_count,
        "unknown or assumption set(s)",
        "downstream translation",
    );
    let artifact_packet_summary = format!(
        "{} Packet maps {repo_signal_count} repository signal(s) and {unknown_count} unknown or assumption set(s). Next phase: {}.",
        packet_output_quality_artifact_prefix(
            missing_context_markers,
            unknown_count,
            "unknown or assumption set(s)"
        ),
        truncate_context_excerpt(&next_phase, 120)
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Problem Map".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&problem_domain, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_system_shaping_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts.iter().find(|artifact| artifact.record.slug() == "system-shape.md")?;
    let domain_model =
        artifacts.iter().find(|artifact| artifact.record.slug() == "domain-model.md");
    let capability_map =
        artifacts.iter().find(|artifact| artifact.record.slug() == "capability-map.md");
    let delivery_options =
        artifacts.iter().find(|artifact| artifact.record.slug() == "delivery-options.md");
    let risk_hotspots =
        artifacts.iter().find(|artifact| artifact.record.slug() == "risk-hotspots.md");

    let system_shape = extract_context_section(&primary.contents, "System Shape")
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - System shape summary is missing.".to_string());
    let boundary_decisions = extract_context_section(&primary.contents, "Boundary Decisions")
        .unwrap_or_else(|| "NOT CAPTURED - Boundary decisions are missing.".to_string());
    let capabilities = capability_map
        .and_then(|artifact| extract_context_section(&artifact.contents, "Capabilities"))
        .unwrap_or_else(|| "NOT CAPTURED - Capability map is missing.".to_string());
    let bounded_contexts = domain_model
        .and_then(|artifact| {
            extract_context_section(&artifact.contents, "Candidate Bounded Contexts")
        })
        .unwrap_or_else(|| "NOT CAPTURED - Candidate bounded contexts are missing.".to_string());
    let domain_invariants = domain_model
        .and_then(|artifact| extract_context_section(&artifact.contents, "Domain Invariants"))
        .unwrap_or_else(|| "NOT CAPTURED - Domain invariants are missing.".to_string());
    let delivery_phases = delivery_options
        .and_then(|artifact| extract_context_section(&artifact.contents, "Delivery Phases"))
        .unwrap_or_else(|| "NOT CAPTURED - Delivery phases are missing.".to_string());
    let hotspots = risk_hotspots
        .and_then(|artifact| extract_context_section(&artifact.contents, "Hotspots"))
        .unwrap_or_else(|| "NOT CAPTURED - Risk hotspots are missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &system_shape,
        &boundary_decisions,
        &capabilities,
        &bounded_contexts,
        &domain_invariants,
        &delivery_phases,
        &hotspots,
    ]);
    let capability_count = count_markdown_entries(&capabilities);
    let bounded_context_count = count_markdown_entries(&bounded_contexts);
    let domain_invariant_count = count_markdown_entries(&domain_invariants);
    let delivery_count = count_markdown_entries(&delivery_phases);
    let hotspot_count = count_markdown_entries(&hotspots);

    let headline = packet_output_quality_headline(
        "System-shaping",
        missing_context_markers,
        0,
        "",
        "downstream architecture or delivery planning",
    );
    let artifact_packet_summary = format!(
        "{} Packet names {capability_count} capability slice(s), {bounded_context_count} bounded context candidate(s), {domain_invariant_count} domain invariant set(s), {delivery_count} delivery phase set(s), and {hotspot_count} risk hotspot set(s).",
        packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "System Shape".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&system_shape, 320),
        action_chips: Vec::new(),
    })
}
