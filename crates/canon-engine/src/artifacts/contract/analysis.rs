/// Artifact requirements for analysis-class modes: SecurityAssessment, SystemAssessment, SupplyChainAnalysis.
use crate::domain::artifact::ArtifactRequirement;
use crate::domain::gate::GateKind;

use super::requirement;
use super::sections::*;

// ── Artifact file-name constants ──────────────────────────────────────────────

// SecurityAssessment
const ASSESSMENT_OVERVIEW_MD: &str = "assessment-overview.md";
const THREAT_MODEL_MD: &str = "threat-model.md";
const RISK_REGISTER_MD: &str = "risk-register.md";
const MITIGATIONS_MD: &str = "mitigations.md";
const ASSUMPTIONS_AND_GAPS_MD: &str = "assumptions-and-gaps.md";
const COMPLIANCE_ANCHORS_MD: &str = "compliance-anchors.md";
const ASSESSMENT_EVIDENCE_MD: &str = "assessment-evidence.md";

// SystemAssessment
const SYS_COVERAGE_MAP_MD: &str = "coverage-map.md";
const SYS_ASSET_INVENTORY_MD: &str = "asset-inventory.md";
const SYS_FUNCTIONAL_VIEW_MD: &str = "functional-view.md";
const SYS_COMPONENT_VIEW_MD: &str = "component-view.md";
const SYS_DEPLOYMENT_VIEW_MD: &str = "deployment-view.md";
const SYS_TECHNOLOGY_VIEW_MD: &str = "technology-view.md";
const SYS_INTEGRATION_VIEW_MD: &str = "integration-view.md";
const SYS_RISK_REGISTER_MD: &str = "risk-register.md";
const SYS_ASSESSMENT_EVIDENCE_MD: &str = "assessment-evidence.md";

// SupplyChainAnalysis
const ANALYSIS_OVERVIEW_MD: &str = "analysis-overview.md";
const SBOM_BUNDLE_MD: &str = "sbom-bundle.md";
const VULNERABILITY_TRIAGE_MD: &str = "vulnerability-triage.md";
const LICENSE_COMPLIANCE_MD: &str = "license-compliance.md";
const LEGACY_POSTURE_MD: &str = "legacy-posture.md";
const POLICY_DECISIONS_MD: &str = "policy-decisions.md";
const SUPPLY_CHAIN_ANALYSIS_EVIDENCE_MD: &str = "analysis-evidence.md";

// ── Mode contracts ────────────────────────────────────────────────────────────

/// Returns the artifact requirements for the [`SecurityAssessment`](crate::domain::mode::Mode::SecurityAssessment) mode.
pub(super) fn security_assessment() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            ASSESSMENT_OVERVIEW_MD,
            &[SUMMARY, "Assessment Scope", "In-Scope Assets", "Trust Boundaries", "Out Of Scope"],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            THREAT_MODEL_MD,
            &[SUMMARY, "Threat Inventory", "Attacker Goals", "Boundary Threats"],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            RISK_REGISTER_MD,
            &[SUMMARY, "Risk Findings", "Likelihood And Impact", "Proposed Owners"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            MITIGATIONS_MD,
            &[SUMMARY, "Recommended Controls", "Tradeoffs", "Sequencing Notes"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            ASSUMPTIONS_AND_GAPS_MD,
            &[SUMMARY, ASSUMPTIONS, EVIDENCE_GAPS, "Unobservable Surfaces"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            COMPLIANCE_ANCHORS_MD,
            &[SUMMARY, "Applicable Standards", "Control Families", "Scope Limits"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            ASSESSMENT_EVIDENCE_MD,
            &[SUMMARY, SOURCE_INPUTS, INDEPENDENT_CHECKS, DEFERRED_VERIFICATION],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`SystemAssessment`](crate::domain::mode::Mode::SystemAssessment) mode.
pub(super) fn system_assessment() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            ASSESSMENT_OVERVIEW_MD,
            &[
                SUMMARY,
                "Assessment Objective",
                "Stakeholders",
                "Primary Concerns",
                "Assessment Posture",
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            SYS_COVERAGE_MAP_MD,
            &[
                SUMMARY,
                "Stakeholder Concerns",
                "Assessed Views",
                "Partial Or Skipped Coverage",
                "Confidence By Surface",
            ],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            SYS_ASSET_INVENTORY_MD,
            &[
                SUMMARY,
                "Assessed Assets",
                "Critical Dependencies",
                "Boundary Notes",
                "Ownership Signals",
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            SYS_FUNCTIONAL_VIEW_MD,
            &[
                SUMMARY,
                "Responsibilities",
                "Primary Flows",
                "Observed Boundaries",
                "Confidence Notes",
            ],
            &[GateKind::Architecture],
        ),
        requirement(
            SYS_COMPONENT_VIEW_MD,
            &[SUMMARY, COMPONENTS, "Responsibilities", "Interfaces", "Confidence Notes"],
            &[GateKind::Architecture, GateKind::ReleaseReadiness],
        ),
        requirement(
            SYS_DEPLOYMENT_VIEW_MD,
            &[
                SUMMARY,
                "Execution Environments",
                "Network And Runtime Boundaries",
                "Deployment Signals",
                "Coverage Gaps",
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            SYS_TECHNOLOGY_VIEW_MD,
            &[
                SUMMARY,
                "Technology Stack",
                "Platform Dependencies",
                "Version Or Lifecycle Signals",
                EVIDENCE_GAPS,
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            SYS_INTEGRATION_VIEW_MD,
            &[
                SUMMARY,
                "Integrations",
                "Data Exchanges",
                "Trust And Failure Boundaries",
                "Inference Notes",
            ],
            &[GateKind::Architecture, GateKind::Risk],
        ),
        requirement(
            SYS_RISK_REGISTER_MD,
            &[SUMMARY, "Observed Risks", "Risk Triggers", "Impact Notes", "Likely Follow-On Modes"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            SYS_ASSESSMENT_EVIDENCE_MD,
            &[
                SUMMARY,
                "Observed Findings",
                "Inferred Findings",
                "Assessment Gaps",
                "Evidence Sources",
            ],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

/// Returns the artifact requirements for the [`SupplyChainAnalysis`](crate::domain::mode::Mode::SupplyChainAnalysis) mode.
pub(super) fn supply_chain_analysis() -> Vec<ArtifactRequirement> {
    vec![
        requirement(
            ANALYSIS_OVERVIEW_MD,
            &[
                SUMMARY,
                "Declared Scope",
                "Licensing Posture",
                "Distribution Model",
                "Ecosystems In Scope",
                "Out Of Scope Components",
            ],
            &[GateKind::Risk],
        ),
        requirement(
            SBOM_BUNDLE_MD,
            &[
                SUMMARY,
                "Scanner Selection Rationale",
                "SBOM Outputs",
                "Scanner Decisions",
                "Coverage Gaps",
            ],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            VULNERABILITY_TRIAGE_MD,
            &[SUMMARY, "Findings By Severity", "Exploitability Notes", "Triage Decisions"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            LICENSE_COMPLIANCE_MD,
            &[SUMMARY, "Compatibility Classes", "Flagged Incompatibilities", "Obligations"],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            LEGACY_POSTURE_MD,
            &[
                SUMMARY,
                "Outdated Dependencies",
                "End Of Life Signals",
                "Abandonment Signals",
                "Modernization Slices",
            ],
            &[GateKind::Risk, GateKind::ReleaseReadiness],
        ),
        requirement(
            POLICY_DECISIONS_MD,
            &[SUMMARY, "Scanner Decisions", "Coverage Gaps", DEFERRED_VERIFICATION],
            &[GateKind::ReleaseReadiness],
        ),
        requirement(
            SUPPLY_CHAIN_ANALYSIS_EVIDENCE_MD,
            &[SUMMARY, SOURCE_INPUTS, INDEPENDENT_CHECKS, DEFERRED_VERIFICATION],
            &[GateKind::ReleaseReadiness],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn security_assessment_has_expected_artifact_count() {
        assert_eq!(security_assessment().len(), 7);
    }

    #[test]
    fn security_assessment_primary_is_overview() {
        assert_eq!(security_assessment()[0].file_name, ASSESSMENT_OVERVIEW_MD);
    }

    #[test]
    fn security_assessment_all_artifacts_are_required() {
        assert!(security_assessment().iter().all(|r| r.required));
    }

    #[test]
    fn system_assessment_has_expected_artifact_count() {
        assert_eq!(system_assessment().len(), 10);
    }

    #[test]
    fn system_assessment_primary_is_overview() {
        assert_eq!(system_assessment()[0].file_name, ASSESSMENT_OVERVIEW_MD);
    }

    #[test]
    fn supply_chain_analysis_has_expected_artifact_count() {
        assert_eq!(supply_chain_analysis().len(), 7);
    }

    #[test]
    fn supply_chain_analysis_primary_is_overview() {
        assert_eq!(supply_chain_analysis()[0].file_name, ANALYSIS_OVERVIEW_MD);
    }
}
