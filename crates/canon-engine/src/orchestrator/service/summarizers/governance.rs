use crate::domain::artifact::{PR_ANALYSIS_ARTIFACT_SLUG, REVIEW_SUMMARY_ARTIFACT_SLUG};
use crate::orchestrator::service::ModeResultSummary;
use crate::orchestrator::service::context_parse::{
    count_context_items_without_placeholders, count_markdown_entries,
    count_missing_context_markers, extract_context_section, extract_labeled_context_value,
    extract_labeled_usize, truncate_context_excerpt,
};
use crate::persistence::store::PersistedArtifact;

use super::{
    packet_output_quality_artifact_prefix, packet_output_quality_headline, packet_primary_artifact,
    primary_artifact_action_for,
};

pub(super) fn summarize_architecture_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts
        .iter()
        .find(|artifact| artifact.record.slug() == "architecture-overview.md")
        .or_else(|| {
            artifacts.iter().find(|artifact| artifact.record.slug() == "architecture-decisions.md")
        })?;
    let decisions_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "architecture-decisions.md");
    let invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "invariants.md");
    let tradeoff_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "tradeoff-matrix.md");
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "boundary-map.md");
    let context_map_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "context-map.md");

    let decisions = decisions_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Decision"))
        .or_else(|| extract_context_section(&primary.contents, "Decisions"))
        .or_else(|| extract_context_section(&primary.contents, "Primary Decision"))
        .or_else(|| extract_context_section(&primary.contents, "Summary"))
        .unwrap_or_else(|| "NOT CAPTURED - Architecture decisions are missing.".to_string());
    let tradeoffs = tradeoff_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Options Considered"))
        .or_else(|| {
            tradeoff_artifact
                .and_then(|artifact| extract_context_section(&artifact.contents, "Tradeoffs"))
        })
        .or_else(|| {
            tradeoff_artifact
                .and_then(|artifact| extract_context_section(&artifact.contents, "Scores"))
        })
        .unwrap_or_else(|| "NOT CAPTURED - Architecture tradeoffs are missing.".to_string());
    let invariants = invariants_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Invariants"))
        .unwrap_or_else(|| "NOT CAPTURED - Invariants are missing.".to_string());
    let boundaries = boundary_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Boundaries"))
        .unwrap_or_else(|| "NOT CAPTURED - Boundary map is missing.".to_string());
    let bounded_contexts = context_map_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Bounded Contexts"))
        .unwrap_or_else(|| "NOT CAPTURED - Context map bounded contexts are missing.".to_string());
    let shared_invariants = context_map_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Shared Invariants"))
        .unwrap_or_else(|| "NOT CAPTURED - Context map shared invariants are missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &decisions,
        &tradeoffs,
        &invariants,
        &boundaries,
        &bounded_contexts,
        &shared_invariants,
    ]);
    let decision_count = count_markdown_entries(&decisions);
    let tradeoff_count = count_markdown_entries(&tradeoffs);
    let invariant_count = count_markdown_entries(&invariants);
    let boundary_count = count_markdown_entries(&boundaries);
    let bounded_context_count = count_markdown_entries(&bounded_contexts);
    let shared_invariant_count = count_markdown_entries(&shared_invariants);

    let headline = packet_output_quality_headline(
        "Architecture",
        missing_context_markers,
        0,
        "",
        "downstream implementation or review",
    );
    let artifact_packet_summary = format!(
        "{} Packet records {decision_count} decision set(s), {tradeoff_count} tradeoff set(s), {invariant_count} invariant set(s), {boundary_count} boundary set(s), {bounded_context_count} bounded context set(s), and {shared_invariant_count} shared invariant set(s).",
        packet_output_quality_artifact_prefix(missing_context_markers, 0, "")
    );

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: if primary.record.slug() == "architecture-overview.md" {
            "Architecture Overview".to_string()
        } else {
            "Architecture Decisions".to_string()
        },
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&decisions, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_review_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary_artifact = packet_primary_artifact(artifacts, "review-brief.md")?;
    let summary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "review-disposition.md")?;
    let boundary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "boundary-assessment.md");
    let missing_evidence_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "missing-evidence.md");

    let final_disposition =
        extract_context_section(&summary_artifact.contents, "Final Disposition")
            .unwrap_or_else(|| "NOT CAPTURED - Final disposition section is missing.".to_string());
    let accepted_risks = extract_context_section(&summary_artifact.contents, "Accepted Risks")
        .unwrap_or_else(|| "NOT CAPTURED - Accepted risks section is missing.".to_string());
    let boundary_findings = boundary_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Boundary Findings"))
        .unwrap_or_else(|| "NOT CAPTURED - Boundary findings section is missing.".to_string());
    let missing_evidence = missing_evidence_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Missing Evidence"))
        .unwrap_or_else(|| "NOT CAPTURED - Missing evidence section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &final_disposition,
        &accepted_risks,
        &boundary_findings,
        &missing_evidence,
    ]);
    let disposition_status = extract_labeled_context_value(&final_disposition, "Status")
        .unwrap_or_else(|| "unknown-disposition".to_string());
    let missing_evidence_status = extract_labeled_context_value(&missing_evidence, "Status")
        .unwrap_or_else(|| "unknown-evidence-posture".to_string());
    let rationale = extract_labeled_context_value(&final_disposition, "Rationale")
        .unwrap_or_else(|| truncate_context_excerpt(&final_disposition, 320));
    let boundary_count = count_context_items_without_placeholders(
        &boundary_findings,
        &["No boundary expansion beyond the authored review target was detected."],
    );
    let no_boundary_expansion = boundary_findings
        .contains("No boundary expansion beyond the authored review target was detected.");
    let accepted_risk_count = count_context_items_without_placeholders(
        &accepted_risks,
        &[
            "No accepted risks recorded while disposition is still pending.",
            "Residual review notes remain bounded to the current package and can be inspected through the emitted artifacts.",
        ],
    );

    let headline = if missing_context_markers == 0 {
        match disposition_status.as_str() {
            "awaiting-disposition" => {
                "Review packet requires explicit disposition before release-readiness can pass."
                    .to_string()
            }
            "accepted-with-approval" => {
                "Review packet completed with explicit approval for the remaining concerns."
                    .to_string()
            }
            _ if missing_evidence_status == "evidence-bounded" && no_boundary_expansion => {
                "Review packet is evidence-bounded with no boundary expansion beyond the authored target.".to_string()
            }
            _ => format!(
                "Review packet preserves `{disposition_status}` disposition with `{missing_evidence_status}` evidence posture."
            ),
        }
    } else {
        format!(
            "Review packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        if no_boundary_expansion {
            format!(
                "Review packet records `{disposition_status}` disposition with `{missing_evidence_status}` evidence posture, no boundary expansion beyond the authored review target, and {accepted_risk_count} accepted-risk or review-note set(s)."
            )
        } else {
            format!(
                "Review packet records `{disposition_status}` disposition with `{missing_evidence_status}` evidence posture, {boundary_count} boundary finding set(s), and {accepted_risk_count} accepted-risk or review-note set(s)."
            )
        }
    } else {
        format!(
            "Review packet is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Boundary findings: {boundary_count}; accepted risks: {accepted_risk_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Review Brief".to_string(),
        primary_artifact_path: format!(".canon/{}", primary_artifact.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary_artifact.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&rationale, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_verification_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary_artifact = packet_primary_artifact(artifacts, "invariants-checklist.md")?;
    let summary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "verification-report.md")?;
    let unresolved_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "unresolved-findings.md");
    let invariants_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "invariants-checklist.md");
    let adversarial_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == "adversarial-review.md");

    let verified_claims = extract_context_section(&summary_artifact.contents, "Verified Claims")
        .unwrap_or_else(|| "NOT CAPTURED - Verified claims section is missing.".to_string());
    let rejected_claims = extract_context_section(&summary_artifact.contents, "Rejected Claims")
        .unwrap_or_else(|| "NOT CAPTURED - Rejected claims section is missing.".to_string());
    let overall_verdict = extract_context_section(&summary_artifact.contents, "Overall Verdict")
        .unwrap_or_else(|| "NOT CAPTURED - Overall verdict section is missing.".to_string());
    let open_findings = unresolved_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Open Findings"))
        .unwrap_or_else(|| "NOT CAPTURED - Open findings section is missing.".to_string());
    let claims_under_test = invariants_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Claims Under Test"))
        .unwrap_or_else(|| "NOT CAPTURED - Claims under test section is missing.".to_string());
    let contradictions = adversarial_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Contradictions"))
        .unwrap_or_else(|| "NOT CAPTURED - Contradictions section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &verified_claims,
        &rejected_claims,
        &overall_verdict,
        &open_findings,
        &claims_under_test,
    ]);
    let verdict_status = extract_labeled_context_value(&overall_verdict, "Status")
        .unwrap_or_else(|| "unknown-verdict".to_string());
    let open_findings_status = extract_labeled_context_value(&open_findings, "Status")
        .unwrap_or_else(|| "unknown-open-findings".to_string());
    let claim_count = count_context_items_without_placeholders(
        &claims_under_test,
        &["The current invariants are bounded enough for recorded verification."],
    );
    let contradiction_count = count_context_items_without_placeholders(
        &contradictions,
        &[
            "none recorded",
            "No direct contradiction was identified against the current claim set.",
            "No direct contradiction was identified against the current verification packet.",
        ],
    );
    let no_direct_contradiction = contradictions.contains("none recorded")
        || contradictions
            .contains("No direct contradiction was identified against the current claim set.")
        || contradictions.contains(
            "No direct contradiction was identified against the current verification packet.",
        );
    let open_finding_count = count_context_items_without_placeholders(
        &open_findings,
        &[
            "No unresolved findings remain from the current verification target.",
            "No unresolved findings remain from the current verification packet.",
        ],
    );

    let headline = if missing_context_markers == 0 {
        if open_findings_status == "unresolved-findings-open" {
            format!(
                "Verification found {open_finding_count} unresolved finding(s) and blocked release readiness."
            )
        } else if no_direct_contradiction {
            format!(
                "Verification completed with `{verdict_status}` verdict, no direct contradictions, and {claim_count} claim set(s) under test."
            )
        } else {
            format!(
                "Verification packet completed with `{verdict_status}` verdict across {claim_count} claim set(s) and {contradiction_count} contradiction set(s)."
            )
        }
    } else {
        format!(
            "Verification packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        if no_direct_contradiction {
            format!(
                "Verification packet records `{verdict_status}` verdict with {claim_count} claim set(s) under test, {open_finding_count} unresolved finding set(s), and explicit no-direct-contradiction posture."
            )
        } else {
            format!(
                "Verification packet records `{verdict_status}` verdict with {claim_count} claim set(s) under test, {open_finding_count} unresolved finding set(s), and {contradiction_count} contradiction set(s)."
            )
        }
    } else {
        format!(
            "Verification packet is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Claim sets: {claim_count}; open findings: {open_finding_count}."
        )
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "Invariants Checklist".to_string(),
        primary_artifact_path: format!(".canon/{}", primary_artifact.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary_artifact.record.relative_path
        )),
        result_excerpt: truncate_context_excerpt(&overall_verdict, 320),
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_pr_review_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary_artifact = packet_primary_artifact(artifacts, PR_ANALYSIS_ARTIFACT_SLUG)?;
    let summary_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == REVIEW_SUMMARY_ARTIFACT_SLUG)?;
    let pr_analysis_artifact =
        artifacts.iter().find(|artifact| artifact.record.slug() == PR_ANALYSIS_ARTIFACT_SLUG);

    let final_disposition =
        extract_context_section(&summary_artifact.contents, "Final Disposition")
            .unwrap_or_else(|| "NOT CAPTURED - Final disposition section is missing.".to_string());
    let severity = extract_context_section(&summary_artifact.contents, "Severity")
        .unwrap_or_else(|| "NOT CAPTURED - Severity section is missing.".to_string());
    let must_fix_findings =
        extract_context_section(&summary_artifact.contents, "Must-Fix Findings")
            .unwrap_or_else(|| "NOT CAPTURED - Must-fix findings section is missing.".to_string());
    let accepted_risks = extract_context_section(&summary_artifact.contents, "Accepted Risks")
        .unwrap_or_else(|| "NOT CAPTURED - Accepted risks section is missing.".to_string());
    let changed_modules = pr_analysis_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Changed Modules"))
        .unwrap_or_else(|| "NOT CAPTURED - Changed modules section is missing.".to_string());
    let inferred_intent = pr_analysis_artifact
        .and_then(|artifact| extract_context_section(&artifact.contents, "Inferred Intent"))
        .unwrap_or_else(|| "NOT CAPTURED - Inferred intent section is missing.".to_string());

    let missing_context_markers = count_missing_context_markers([
        &final_disposition,
        &severity,
        &must_fix_findings,
        &accepted_risks,
        &changed_modules,
        &inferred_intent,
    ]);
    let disposition_status = extract_labeled_context_value(&final_disposition, "Status")
        .unwrap_or_else(|| "unknown-disposition".to_string());
    let rationale = extract_labeled_context_value(&final_disposition, "Rationale")
        .unwrap_or_else(|| truncate_context_excerpt(&final_disposition, 320));
    let overall_severity = extract_labeled_context_value(&severity, "Overall severity")
        .unwrap_or_else(|| {
            if must_fix_findings.contains("No must-fix findings remain.") {
                "review-notes".to_string()
            } else {
                "must-fix".to_string()
            }
        });
    let must_fix_count =
        extract_labeled_usize(&severity, "Must-fix findings").unwrap_or_else(|| {
            count_context_items_without_placeholders(
                &must_fix_findings,
                &["No must-fix findings remain."],
            )
        });
    let review_note_count = extract_labeled_usize(&severity, "Review notes").unwrap_or_else(|| {
        count_context_items_without_placeholders(&accepted_risks, &["No accepted risks recorded."])
    });
    let changed_surface_count = count_context_items_without_placeholders(
        &changed_modules,
        &["No changed surfaces detected."],
    );

    let headline = if missing_context_markers == 0 {
        match disposition_status.as_str() {
            "ready-with-review-notes" => format!(
                "PR review completed with {review_note_count} review note(s) and no unresolved must-fix findings."
            ),
            "awaiting-disposition" => format!(
                "PR review found {must_fix_count} must-fix finding(s) and is waiting for explicit disposition."
            ),
            "accepted-with-approval" => {
                "PR review completed with explicit approval for the remaining must-fix findings."
                    .to_string()
            }
            _ => format!("PR review completed with disposition `{disposition_status}`."),
        }
    } else {
        format!(
            "PR review packet completed with {missing_context_markers} explicit missing-context marker(s)."
        )
    };
    let artifact_packet_summary = if missing_context_markers == 0 {
        format!(
            "PR review packet records `{disposition_status}` disposition with `{overall_severity}` severity across {changed_surface_count} changed surface(s), {must_fix_count} must-fix finding(s), and {review_note_count} review note(s)."
        )
    } else {
        format!(
            "PR review packet is readable, but the packet still carries {missing_context_markers} missing-context marker(s). Changed surfaces: {changed_surface_count}; must-fix findings: {must_fix_count}; review notes: {review_note_count}."
        )
    };
    let result_excerpt = if rationale.contains("NOT CAPTURED") {
        truncate_context_excerpt(&inferred_intent, 320)
    } else {
        truncate_context_excerpt(&rationale, 320)
    };

    Some(ModeResultSummary {
        headline,
        artifact_packet_summary,
        execution_posture: None,
        primary_artifact_title: "PR Analysis".to_string(),
        primary_artifact_path: format!(".canon/{}", primary_artifact.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary_artifact.record.relative_path
        )),
        result_excerpt,
        action_chips: Vec::new(),
    })
}

pub(super) fn summarize_policy_shaping_mode_result(
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    let primary = artifacts
        .iter()
        .find(|artifact| artifact.record.slug() == "conformance-impact-report.md")?;

    Some(ModeResultSummary {
        headline: "Policy Shaping impact report completed.".to_string(),
        artifact_packet_summary: "Policy Shaping run generated an impact report.".to_string(),
        execution_posture: None,
        primary_artifact_title: "Conformance Impact Report".to_string(),
        primary_artifact_path: format!(".canon/{}", primary.record.relative_path),
        primary_artifact_action: primary_artifact_action_for(&format!(
            ".canon/{}",
            primary.record.relative_path
        )),
        result_excerpt: "Impact report ready for review.".to_string(),
        action_chips: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::artifact::{ArtifactFormat, ArtifactRecord};
    use crate::persistence::store::PersistedArtifact;

    #[test]
    fn test_summarize_policy_shaping_mode_result() {
        let artifacts = vec![PersistedArtifact {
            record: ArtifactRecord {
                file_name: "conformance-impact-report.md".to_string(),
                relative_path: "artifacts/test/conformance-impact-report.md".to_string(),
                format: ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: "".to_string(),
        }];

        let summary = summarize_policy_shaping_mode_result(&artifacts).unwrap();
        assert_eq!(summary.headline, "Policy Shaping impact report completed.");
        assert_eq!(summary.primary_artifact_title, "Conformance Impact Report");
        assert_eq!(
            summary.primary_artifact_path,
            ".canon/artifacts/test/conformance-impact-report.md"
        );
    }

    #[test]
    fn test_summarize_policy_shaping_mode_result_missing_artifact() {
        let artifacts = vec![];
        let summary = summarize_policy_shaping_mode_result(&artifacts);
        assert!(summary.is_none());
    }
}
