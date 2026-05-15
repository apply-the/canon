use crate::domain::artifact::artifact_slug;
use crate::orchestrator::service::context_parse::truncate_context_excerpt;

use super::shared::{
    extract_authored_h2_section, extract_authored_section_or_marker, render_authored_artifact,
    render_authored_decision_section, render_authored_section, render_missing_authored_body_block,
};
use super::{AuthoredSectionSpec, render_markdown};

pub fn render_security_assessment_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let assessment_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Assessment Scope",
        &[],
        &["assessment scope"],
    )
    .unwrap_or_else(|| "assessment scope not yet authored".to_string());
    let in_scope_assets = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "In-Scope Assets",
        &[],
        &["in-scope assets"],
    )
    .unwrap_or_else(|| "in-scope assets not yet authored".to_string());
    let summary = format!(
        "Bounded security assessment for {} covering {}.",
        truncate_context_excerpt(&assessment_scope, 80),
        truncate_context_excerpt(&in_scope_assets, 80)
    );

    match file_name {
        "assessment-overview.md" => render_authored_artifact(
            "Assessment Overview",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Assessment Scope", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "In-Scope Assets", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Trust Boundaries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Out Of Scope", aliases: &[] },
            ],
        ),
        "threat-model.md" => render_authored_artifact(
            "Threat Model",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Threat Inventory", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Attacker Goals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Boundary Threats", aliases: &[] },
            ],
        ),
        "risk-register.md" => render_authored_artifact(
            "Risk Register",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Risk Findings", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Likelihood And Impact", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Proposed Owners", aliases: &[] },
            ],
        ),
        "mitigations.md" => render_authored_artifact(
            "Mitigations",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Recommended Controls", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Tradeoffs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Sequencing Notes", aliases: &[] },
            ],
        ),
        "assumptions-and-gaps.md" => render_authored_artifact(
            "Assumptions And Gaps",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Assumptions", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Evidence Gaps", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Unobservable Surfaces", aliases: &[] },
            ],
        ),
        "compliance-anchors.md" => render_authored_artifact(
            "Compliance Anchors",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Applicable Standards", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Control Families", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Scope Limits", aliases: &[] },
            ],
        ),
        "assessment-evidence.md" => render_authored_artifact(
            "Assessment Evidence",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Source Inputs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Independent Checks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Deferred Verification", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

/// Renders a system assessment mode artifact for the given filename slug.
pub fn render_system_assessment_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let assessment_objective = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Assessment Objective",
        &[],
        &["assessment objective"],
    )
    .unwrap_or_else(|| "assessment objective not yet authored".to_string());
    let stakeholders = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Stakeholders",
        &[],
        &["stakeholders"],
    )
    .unwrap_or_else(|| "stakeholders not yet authored".to_string());
    let summary = format!(
        "Bounded system assessment for {} with reader context {}.",
        truncate_context_excerpt(&assessment_objective, 80),
        truncate_context_excerpt(&stakeholders, 80)
    );

    match file_name {
        "assessment-overview.md" => render_authored_artifact(
            "Assessment Overview",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Assessment Objective", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Stakeholders", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Primary Concerns", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Assessment Posture", aliases: &[] },
            ],
        ),
        "coverage-map.md" => render_authored_artifact(
            "Coverage Map",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Stakeholder Concerns", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Assessed Views", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Partial Or Skipped Coverage",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Confidence By Surface", aliases: &[] },
            ],
        ),
        "asset-inventory.md" => render_authored_artifact(
            "Asset Inventory",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Assessed Assets", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Critical Dependencies", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Boundary Notes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Ownership Signals", aliases: &[] },
            ],
        ),
        "functional-view.md" => render_authored_artifact(
            "Functional View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Responsibilities", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Primary Flows", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Observed Boundaries", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Confidence Notes", aliases: &[] },
            ],
        ),
        "component-view.md" => render_authored_artifact(
            "Component View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Components", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Responsibilities", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Interfaces", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Confidence Notes", aliases: &[] },
            ],
        ),
        "deployment-view.md" => render_authored_artifact(
            "Deployment View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Execution Environments", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Network And Runtime Boundaries",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Deployment Signals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Coverage Gaps", aliases: &[] },
            ],
        ),
        "technology-view.md" => render_authored_artifact(
            "Technology View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Technology Stack", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Platform Dependencies", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Version Or Lifecycle Signals",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Evidence Gaps", aliases: &[] },
            ],
        ),
        "integration-view.md" => render_authored_artifact(
            "Integration View",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Integrations", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Data Exchanges", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Trust And Failure Boundaries",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Inference Notes", aliases: &[] },
            ],
        ),
        "risk-register.md" => render_authored_artifact(
            "Risk Register",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Observed Risks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Risk Triggers", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Impact Notes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Likely Follow-On Modes", aliases: &[] },
            ],
        ),
        "assessment-evidence.md" => render_authored_artifact(
            "Assessment Evidence",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec {
                    canonical_heading: "Observed Findings",
                    aliases: &["FACT Findings"],
                },
                AuthoredSectionSpec {
                    canonical_heading: "Inferred Findings",
                    aliases: &["INFERENCE Findings"],
                },
                AuthoredSectionSpec {
                    canonical_heading: "Assessment Gaps",
                    aliases: &["GAP Findings"],
                },
                AuthoredSectionSpec { canonical_heading: "Evidence Sources", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

/// Renders a supply chain analysis mode artifact for the given filename slug.
pub fn render_supply_chain_analysis_artifact(file_name: &str, brief_summary: &str) -> String {
    let file_name = artifact_slug(file_name);
    let normalized = brief_summary.to_lowercase();
    let declared_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Declared Scope",
        &[],
        &["declared scope"],
    )
    .unwrap_or_else(|| "declared scope not yet authored".to_string());
    let ecosystems_in_scope = extract_authored_section_or_marker(
        brief_summary,
        &normalized,
        "Ecosystems In Scope",
        &[],
        &["ecosystems in scope"],
    )
    .unwrap_or_else(|| "ecosystems in scope not yet authored".to_string());
    let summary = format!(
        "Bounded supply-chain analysis for {} across {}.",
        truncate_context_excerpt(&declared_scope, 80),
        truncate_context_excerpt(&ecosystems_in_scope, 80)
    );

    match file_name {
        "analysis-overview.md" => format!(
            "# Analysis Overview\n\n## Summary\n\n{summary}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}\n",
            render_authored_section(
                brief_summary,
                &AuthoredSectionSpec { canonical_heading: "Declared Scope", aliases: &[] }
            ),
            render_authored_decision_section(
                brief_summary,
                "Licensing Posture",
                &[],
                "Record the repository licensing posture explicitly and rerun."
            ),
            render_authored_decision_section(
                brief_summary,
                "Distribution Model",
                &[],
                "Record whether the analyzed dependencies are distributed externally or internal-only and rerun."
            ),
            render_authored_decision_section(
                brief_summary,
                "Ecosystems In Scope",
                &[],
                "Record which ecosystems remain in scope for the packet and rerun."
            ),
            render_authored_decision_section(
                brief_summary,
                "Out Of Scope Components",
                &[],
                "Record the explicit out-of-scope components and rerun."
            ),
        ),
        "sbom-bundle.md" => format!(
            "# SBOM Bundle\n\n## Summary\n\n{summary}\n\n{}\n\n{}\n\n{}\n\n{}\n",
            render_authored_section(
                brief_summary,
                &AuthoredSectionSpec {
                    canonical_heading: "Scanner Selection Rationale",
                    aliases: &[],
                }
            ),
            render_authored_section(
                brief_summary,
                &AuthoredSectionSpec { canonical_heading: "SBOM Outputs", aliases: &[] }
            ),
            render_authored_decision_section(
                brief_summary,
                "Scanner Decisions",
                &[],
                "Record non-OSS tool policy and any installed, skipped, or replaced scanner decisions, then rerun."
            ),
            render_supply_chain_coverage_gaps_section(brief_summary),
        ),
        "vulnerability-triage.md" => render_authored_artifact(
            "Vulnerability Triage",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Findings By Severity", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Exploitability Notes", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Triage Decisions", aliases: &[] },
            ],
        ),
        "license-compliance.md" => render_authored_artifact(
            "License Compliance",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Compatibility Classes", aliases: &[] },
                AuthoredSectionSpec {
                    canonical_heading: "Flagged Incompatibilities",
                    aliases: &[],
                },
                AuthoredSectionSpec { canonical_heading: "Obligations", aliases: &[] },
            ],
        ),
        "legacy-posture.md" => render_authored_artifact(
            "Legacy Posture",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Outdated Dependencies", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "End Of Life Signals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Abandonment Signals", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Modernization Slices", aliases: &[] },
            ],
        ),
        "policy-decisions.md" => format!(
            "# Policy Decisions\n\n## Summary\n\n{summary}\n\n{}\n\n{}\n\n{}\n",
            render_authored_decision_section(
                brief_summary,
                "Scanner Decisions",
                &[],
                "Record non-OSS tool policy and any installed, skipped, or replaced scanner decisions, then rerun."
            ),
            render_supply_chain_coverage_gaps_section(brief_summary),
            render_authored_section(
                brief_summary,
                &AuthoredSectionSpec { canonical_heading: "Deferred Verification", aliases: &[] }
            ),
        ),
        "analysis-evidence.md" => render_authored_artifact(
            "Analysis Evidence",
            &summary,
            brief_summary,
            &[
                AuthoredSectionSpec { canonical_heading: "Source Inputs", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Independent Checks", aliases: &[] },
                AuthoredSectionSpec { canonical_heading: "Deferred Verification", aliases: &[] },
            ],
        ),
        other => render_markdown(other, brief_summary),
    }
}

fn render_supply_chain_coverage_gaps_section(authored_source: &str) -> String {
    if let Some(body) = extract_authored_h2_section(authored_source, "Coverage Gaps", &[]) {
        return format!("## Coverage Gaps\n\n{body}");
    }

    if let Some(scanner_decisions) =
        extract_authored_h2_section(authored_source, "Scanner Decisions", &[])
    {
        let normalized = scanner_decisions.to_lowercase();
        if normalized.contains("skipped") || normalized.contains("replaced") {
            return format!(
                "## Coverage Gaps\n\nCoverage gap derived from recorded scanner decisions.\n\n{}\n\n- Impacted artifacts: sbom-bundle.md, vulnerability-triage.md, license-compliance.md, legacy-posture.md, policy-decisions.md\n- Next action: install the missing scanner or document an approved replacement and rerun the packet.",
                truncate_context_excerpt(&scanner_decisions, 320)
            );
        }
    }

    render_missing_authored_body_block("Coverage Gaps")
}
