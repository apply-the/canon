use crate::domain::artifact::artifact_slug;
use crate::orchestrator::service::context_parse::truncate_context_excerpt;
use crate::review::findings::{
    ConventionalCommentScope, FindingCategory, ReviewFinding, ReviewPacket,
};
use crate::review::summary::{ReviewSummary, summary_severity_label};

use super::shared::{
    extract_authored_section_or_marker, render_authored_artifact, render_string_list,
};
use super::{AuthoredSectionSpec, render_markdown};

pub fn render_review_artifact(
    file_name: &str,
    context_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
    _validation_summary: &str,
) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = context_summary.to_lowercase();
    let review_target = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Review Target",
        &[],
        &["review target"],
    )
    .unwrap_or_else(|| "review target not yet authored".to_string());
    let evidence_basis = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Evidence Basis",
        &[],
        &["evidence basis"],
    )
    .unwrap_or_else(|| "evidence basis not yet authored".to_string());
    let summary = format!(
        "- Review target: {}\n- Evidence basis: {}",
        truncate_context_excerpt(&review_target, 120),
        truncate_context_excerpt(&evidence_basis, 120),
    );

    match file_name {
        "review-brief.md" => render_authored_artifact(
            "Review Brief",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Review Target", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Evidence Basis", aliases: &[] },
            ],
        ),
        "boundary-assessment.md" => render_authored_artifact(
            "Boundary Assessment",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Boundary Findings",
                    aliases: &["Boundary Concern"],
                },
                AuthoredSectionSpec { canonical_heading: "Ownership Notes", aliases: &[] },
            ],
        ),
        "missing-evidence.md" => render_authored_artifact(
            "Missing Evidence",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Missing Evidence",
                    aliases: &["Open Concern"],
                },
                AuthoredSectionSpec { canonical_heading: "Collection Priorities", aliases: &[] },
            ],
        ),
        "decision-impact.md" => render_authored_artifact(
            "Decision Impact",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Decision Impact",
                    aliases: &["Pending Decision"],
                },
                AuthoredSectionSpec { canonical_heading: "Reversibility Concerns", aliases: &[] },
            ],
        ),
        "review-disposition.md" => render_authored_artifact(
            "Review Disposition",
            &summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Final Disposition", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Accepted Risks", aliases: &[] },
            ],
        ),
        other => render_markdown(other, context_summary),
    }
}

/// Renders a verification mode artifact for the given filename slug.
pub fn render_verification_artifact(
    file_name: &str,
    context_summary: &str,
    _generation_summary: &str,
    _critique_summary: &str,
    _validation_summary: &str,
) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = context_summary.to_lowercase();
    let claims_under_test = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Claims Under Test",
        &[],
        &["claims under test"],
    )
    .unwrap_or_else(|| "claims under test not yet authored".to_string());
    let contract_assumptions = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Contract Assumptions",
        &["Contract Surface"],
        &["contract assumptions", "contract surface"],
    )
    .unwrap_or_else(|| "contract assumptions not yet authored".to_string());
    let overall_verdict = extract_authored_section_or_marker(
        context_summary,
        &normalized,
        "Overall Verdict",
        &[],
        &["overall verdict"],
    )
    .unwrap_or_else(|| "overall verdict not yet authored".to_string());
    let verification_summary = format!(
        "- Claims under test: {}\n- Contract assumptions: {}\n- Verdict: {}",
        truncate_context_excerpt(&claims_under_test, 120),
        truncate_context_excerpt(&contract_assumptions, 120),
        truncate_context_excerpt(&overall_verdict, 120),
    );

    match file_name {
        "invariants-checklist.md" => render_authored_artifact(
            "Invariants Checklist",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Claims Under Test", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Invariant Checks",
                    aliases: &["Risk Boundary"],
                },
            ],
        ),
        "contract-matrix.md" => render_authored_artifact(
            "Contract Matrix",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Contract Assumptions",
                    aliases: &["Contract Surface"],
                },
                AuthoredSectionSpec { canonical_heading: "Verification Outcome", aliases: &[] },
            ],
        ),
        "adversarial-review.md" => render_authored_artifact(
            "Adversarial Review",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Challenge Findings",
                    aliases: &["Challenge Focus"],
                },
                AuthoredSectionSpec { canonical_heading: "Contradictions", aliases: &[] },
            ],
        ),
        "verification-report.md" => render_authored_artifact(
            "Verification Report",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Verified Claims", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Rejected Claims", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Overall Verdict", aliases: &[] },
            ],
        ),
        "unresolved-findings.md" => render_authored_artifact(
            "Unresolved Findings",
            &verification_summary,
            context_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Open Findings", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Required Follow-Up",
                    aliases: &["Required Follow-up"],
                },
            ],
        ),
        other => render_markdown(other, context_summary),
    }
}

pub fn render_pr_review_artifact(
    file_name: &str,
    packet: &ReviewPacket,
    summary: &ReviewSummary,
) -> String {
    let file_name = artifact_slug(file_name);
    match file_name {
        "pr-analysis.md" => format!(
            "# PR Analysis\n\n## Summary\n\nReviewing `{}` against `{}` across {} changed surface(s).\n\n## Evidence Posture\n\n- Review packet derived from governed diff inspection and critique evidence.\n- Artifact provenance remains linked from the run evidence bundle.\n\n## Scope Summary\n\n- Base ref: `{}`\n- Head ref: `{}`\n- Changed surface count: {}\n\n## Changed Modules\n\n{}\n\n## Inferred Intent\n\n{}\n\n## Surprising Surface Area\n\n{}\n",
            packet.head_ref,
            packet.base_ref,
            packet.changed_surfaces.len(),
            packet.base_ref,
            packet.head_ref,
            packet.changed_surfaces.len(),
            render_surface_list(&packet.changed_surfaces, "- No changed surfaces detected."),
            packet.inferred_intent,
            render_surface_list(
                &packet.surprising_surface_area,
                "- No surprising surface area inferred from the diff."
            ),
        ),
        "boundary-check.md" => format!(
            "# Boundary Check\n\n## Summary\n\nBoundary review for changed surfaces between `{}` and `{}`.\n\n## Boundary Findings\n\n{}\n\n## Ownership Breaks\n\n{}\n\n## Unauthorized Structural Impact\n\nStatus: {}\n\n{}\n",
            packet.base_ref,
            packet.head_ref,
            render_findings(
                &packet.findings_for(FindingCategory::BoundaryCheck),
                "- No boundary findings detected."
            ),
            ownership_breaks(packet),
            if packet.findings_for(FindingCategory::BoundaryCheck).is_empty() {
                "no-structural-impact-detected"
            } else {
                "structural-impact-reviewed"
            },
            if packet.findings_for(FindingCategory::BoundaryCheck).is_empty() {
                "- No unauthorized structural impact inferred."
            } else {
                "- Boundary-marked files changed and remain explicit in the review packet."
            },
        ),
        "conventional-comments.md" => format!(
            "# Conventional Comments\n\n## Summary\n\nReviewer-facing conventional comments derived from {} persisted finding(s) for `{}` against `{}`.\n\n## Evidence Posture\n\n- Comment kinds are deterministically mapped from persisted review findings.\n- Each entry carries an explicit scope: `pr` (whole-PR), `file` (multi-type surfaces), or `surface` (single-type surface group).\n- Scope is derived deterministically from the finding's changed surfaces and does not fabricate line-level anchors.\n- Approval posture remains anchored by `review-summary.md`.\n\n## Conventional Comments\n\n{}\n\n## Traceability\n\n- Review summary status: `{}`\n- Changed surfaces: {}\n- Source packet: `review-summary.md` and `pr-analysis.md`\n",
            packet.findings.len(),
            packet.head_ref,
            packet.base_ref,
            render_conventional_comments(packet),
            summary.disposition.as_str(),
            if packet.changed_surfaces.is_empty() {
                "none".to_string()
            } else {
                packet.changed_surfaces.join(", ")
            },
        ),
        "duplication-check.md" => format!(
            "# Duplication Check\n\n## Summary\n\nDuplication review for the bounded diff.\n\n## Duplicate Behavior\n\n{}\n\n## Canonical Owner Conflicts\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::DuplicationCheck),
                "- No duplicate behavior concerns inferred."
            ),
            if packet.findings_for(FindingCategory::DuplicationCheck).is_empty() {
                "- No canonical owner conflicts inferred."
            } else {
                "- Canonical ownership needs review where duplicate logic surfaced."
            },
        ),
        "contract-drift.md" => format!(
            "# Contract Drift\n\n## Summary\n\nContract review for the changed surfaces.\n\n## Interface Drift\n\n{}\n\n## Compatibility Concerns\n\nStatus: {}\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::ContractDrift),
                "- No contract drift inferred."
            ),
            if packet.findings_for(FindingCategory::ContractDrift).is_empty() {
                "no-contract-drift-detected"
            } else {
                "explicit-contract-drift"
            },
            if packet.findings_for(FindingCategory::ContractDrift).is_empty() {
                "- No compatibility concerns inferred from the reviewed diff."
            } else {
                "- Compatibility risk remains explicit until reviewer disposition is recorded."
            },
        ),
        "missing-tests.md" => format!(
            "# Missing Tests\n\n## Summary\n\nVerification coverage review for the diff.\n\n## Missing Invariant Checks\n\n{}\n\n## Missing Contract Checks\n\n{}\n\n## Weak or Mirrored Tests\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::MissingTests),
                "- No missing invariant checks inferred."
            ),
            if packet.findings_for(FindingCategory::MissingTests).is_empty() {
                "- No missing contract checks inferred."
            } else {
                "- Contract-facing changes should carry explicit test evidence."
            },
            if packet.findings_for(FindingCategory::MissingTests).is_empty() {
                "- Updated tests moved with the changed surface."
            } else {
                "- The current diff changes source files without parallel verification updates."
            },
        ),
        "decision-impact.md" => format!(
            "# Decision Impact\n\n## Summary\n\nDecision-impact review for the bounded diff.\n\n## Implied Decisions\n\n{}\n\n## Absent Decision Records\n\n{}\n\n## Reversibility Concerns\n\n{}\n",
            render_findings(
                &packet.findings_for(FindingCategory::DecisionImpact),
                "- No hidden decision impact inferred."
            ),
            if packet.findings_for(FindingCategory::DecisionImpact).is_empty() {
                "- No absent decision records were inferred from the changed surfaces."
            } else {
                "- Structural or contract-facing changes imply decisions that are not yet explicitly accepted."
            },
            if packet.findings_for(FindingCategory::DecisionImpact).is_empty() {
                "- Reversibility concerns remain bounded."
            } else {
                "- Reverting the current change may require downstream coordination because high-impact surfaces changed."
            },
        ),
        "review-summary.md" => format!(
            "# Review Summary\n\n## Summary\n\nStructured review completed for `{}` against `{}`.\n\n## Evidence\n\n- Governed diff inspection and critique were both recorded before disposition.\n- Review artifacts remain derived from the persisted evidence bundle.\n\n## Severity\n\n- Overall severity: {}\n- Must-fix findings: {}\n- Review notes: {}\n\n## Must-Fix Findings\n\n{}\n\n## Accepted Risks\n\n{}\n\n## Final Disposition\n\nStatus: {}\n\nRationale: {}\n",
            packet.head_ref,
            packet.base_ref,
            summary_severity_label(packet),
            summary.must_fix_findings.len(),
            packet.note_findings().len(),
            render_findings(&packet.must_fix_findings(), "- No must-fix findings remain."),
            render_string_list(&summary.accepted_risks, "- No accepted risks recorded."),
            summary.disposition.as_str(),
            summary.rationale,
        ),
        other => render_markdown(other, &packet.inferred_intent),
    }
}

fn render_surface_list(values: &[String], empty_message: &str) -> String {
    render_string_list(values, empty_message)
}

fn render_findings(findings: &[&ReviewFinding], empty_message: &str) -> String {
    if findings.is_empty() {
        return empty_message.to_string();
    }

    findings
        .iter()
        .map(|finding| {
            format!(
                "- [{}] {}: {}\n  - Surfaces: {}",
                finding.severity.as_str(),
                finding.title,
                finding.details,
                if finding.changed_surfaces.is_empty() {
                    "none".to_string()
                } else {
                    finding.changed_surfaces.join(", ")
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_conventional_comments(packet: &ReviewPacket) -> String {
    if packet.findings.is_empty() {
        return "- thought(scope:pr): No findings were recorded for this review packet."
            .to_string();
    }

    packet.findings.iter().map(render_conventional_comment_entry).collect::<Vec<_>>().join("\n")
}

fn render_conventional_comment_entry(finding: &ReviewFinding) -> String {
    let scope = finding.scope.as_str();
    let surfaces = if finding.changed_surfaces.is_empty() {
        "none".to_string()
    } else {
        finding.changed_surfaces.join(", ")
    };
    let scope_detail = match finding.scope {
        ConventionalCommentScope::Pr => String::new(),
        ConventionalCommentScope::File | ConventionalCommentScope::Surface => {
            format!("\n  - Scope surfaces: {surfaces}")
        }
    };
    format!(
        "- {}(scope:{}): {}\n  - Why: {}\n  - Surfaces: {}{}",
        finding.conventional_comment_kind(),
        scope,
        finding.title,
        finding.details,
        surfaces,
        scope_detail,
    )
}

fn ownership_breaks(packet: &ReviewPacket) -> String {
    let boundary_findings = packet.findings_for(FindingCategory::BoundaryCheck);
    if boundary_findings.is_empty() {
        "- No ownership breaks inferred from changed surfaces.".to_string()
    } else {
        "- Reviewer ownership is required before boundary-marked changes can be treated as acceptable output.".to_string()
    }
}
