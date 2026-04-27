use crate::domain::artifact::{ArtifactContract, ArtifactFormat, ArtifactRequirement};
use crate::domain::gate::GateKind;
use crate::domain::mode::Mode;
use crate::domain::run::{ClosureAssessment, ClosureDecompositionScope};
use crate::domain::verification::VerificationLayer;

pub fn contract_for_mode(mode: Mode) -> ArtifactContract {
    let files = match mode {
        Mode::Requirements => vec![
            requirement(
                "problem-statement.md",
                &["Summary", "Problem", "Outcome"],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "constraints.md",
                &["Summary", "Constraints", "Non-Negotiables"],
                &[GateKind::Exploration, GateKind::Risk, GateKind::Architecture],
            ),
            requirement(
                "options.md",
                &["Summary", "Options", "Recommended Path"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "tradeoffs.md",
                &["Summary", "Tradeoffs", "Consequences"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "scope-cuts.md",
                &["Summary", "Scope Cuts", "Deferred Work"],
                &[GateKind::Exploration, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-checklist.md",
                &["Summary", "Decision Checklist", "Open Questions"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Discovery => vec![
            requirement(
                "problem-map.md",
                &[
                    "Summary",
                    "Problem Domain",
                    "Repo Surface",
                    "Immediate Tensions",
                    "Downstream Handoff",
                ],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "unknowns-and-assumptions.md",
                &["Summary", "Unknowns", "Assumptions", "Validation Targets", "Confidence Levels"],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "context-boundary.md",
                &[
                    "Summary",
                    "In-Scope Context",
                    "Repo Surface",
                    "Out-of-Scope Context",
                    "Translation Trigger",
                ],
                &[GateKind::Exploration, GateKind::ReleaseReadiness],
            ),
            requirement(
                "exploration-options.md",
                &["Summary", "Options", "Constraints", "Recommended Direction", "Next-Phase Shape"],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "decision-pressure-points.md",
                &[
                    "Summary",
                    "Pressure Points",
                    "Blocking Decisions",
                    "Open Questions",
                    "Recommended Owner",
                ],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::SystemShaping => vec![
            requirement(
                "system-shape.md",
                &["Summary", "System Shape", "Boundary Decisions", "Domain Responsibilities"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "domain-model.md",
                &[
                    "Summary",
                    "Candidate Bounded Contexts",
                    "Core And Supporting Domain Hypotheses",
                    "Ubiquitous Language",
                    "Domain Invariants",
                    "Boundary Risks And Open Questions",
                ],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "architecture-outline.md",
                &["Summary", "Structural Options", "Selected Boundaries", "Rationale"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "capability-map.md",
                &["Summary", "Capabilities", "Dependencies", "Gaps"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "delivery-options.md",
                &["Summary", "Delivery Phases", "Sequencing Rationale", "Risk per Phase"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "risk-hotspots.md",
                &["Summary", "Hotspots", "Mitigation Status", "Unresolved Risks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Change => vec![
            requirement(
                "system-slice.md",
                &["Summary", "System Slice", "Domain Slice", "Excluded Areas"],
                &[GateKind::Exploration, GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "legacy-invariants.md",
                &["Summary", "Legacy Invariants", "Domain Invariants", "Forbidden Normalization"],
                &[GateKind::ChangePreservation, GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "change-surface.md",
                &["Summary", "Change Surface", "Ownership", "Cross-Context Risks"],
                &[GateKind::ChangePreservation, GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "implementation-plan.md",
                &["Summary", "Implementation Plan", "Sequencing"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "validation-strategy.md",
                &["Summary", "Validation Strategy", "Independent Checks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-record.md",
                &[
                    "Summary",
                    "Decision Record",
                    "Boundary Tradeoffs",
                    "Consequences",
                    "Unresolved Questions",
                ],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Backlog => vec![
            requirement(
                "backlog-overview.md",
                &[
                    "Summary",
                    "Scope",
                    "Planning Horizon",
                    "Source Inputs",
                    "Delivery Intent",
                    "Decomposition Posture",
                ],
                &[GateKind::Exploration, GateKind::Risk],
            ),
            requirement(
                "epic-tree.md",
                &["Summary", "Epic Tree", "Scope Boundaries", "Source Trace Links"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "capability-to-epic-map.md",
                &["Summary", "Capability Mapping", "Source Trace Links", "Planning Gaps"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "dependency-map.md",
                &["Summary", "Dependencies", "Blocking Edges", "External Dependencies"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "delivery-slices.md",
                &["Summary", "Delivery Slices", "Slice Boundaries", "Dependency Links"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "sequencing-plan.md",
                &["Summary", "Sequencing", "Ordering Rationale", "Readiness Signals"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "acceptance-anchors.md",
                &["Summary", "Acceptance Anchors", "Source Trace Links", "Deferred Detail"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "planning-risks.md",
                &["Summary", "Closure Findings", "Planning Risks", "Follow-Up Triggers"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Incident => vec![
            requirement(
                "incident-frame.md",
                &[
                    "Summary",
                    "Incident Scope",
                    "Trigger And Current State",
                    "Operational Constraints",
                ],
                &[GateKind::Risk, GateKind::IncidentContainment, GateKind::Architecture],
            ),
            requirement(
                "hypothesis-log.md",
                &["Summary", "Known Facts", "Working Hypotheses", "Evidence Gaps"],
                &[GateKind::IncidentContainment, GateKind::Risk],
            ),
            requirement(
                "blast-radius-map.md",
                &["Summary", "Impacted Surfaces", "Propagation Paths", "Confidence And Unknowns"],
                &[GateKind::IncidentContainment, GateKind::Architecture],
            ),
            requirement(
                "containment-plan.md",
                &["Summary", "Immediate Actions", "Ordered Sequence", "Stop Conditions"],
                &[GateKind::IncidentContainment, GateKind::ReleaseReadiness],
            ),
            requirement(
                "incident-decision-record.md",
                &["Summary", "Decision Points", "Approved Actions", "Deferred Actions"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "follow-up-verification.md",
                &["Summary", "Verification Checks", "Release Readiness", "Follow-Up Work"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Implementation => vec![
            requirement(
                "task-mapping.md",
                &["Summary", "Task Mapping", "Bounded Changes"],
                &[GateKind::ImplementationReadiness, GateKind::ReleaseReadiness],
            ),
            requirement(
                "mutation-bounds.md",
                &["Summary", "Mutation Bounds", "Allowed Paths"],
                &[GateKind::Risk, GateKind::ImplementationReadiness],
            ),
            requirement(
                "implementation-notes.md",
                &["Summary", "Executed Changes", "Task Linkage"],
                &[GateKind::ImplementationReadiness, GateKind::ReleaseReadiness],
            ),
            requirement(
                "completion-evidence.md",
                &["Summary", "Completion Evidence", "Remaining Risks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "validation-hooks.md",
                &["Summary", "Safety-Net Evidence", "Independent Checks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "rollback-notes.md",
                &["Summary", "Rollback Triggers", "Rollback Steps"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Migration => vec![
            requirement(
                "source-target-map.md",
                &["Summary", "Current State", "Target State", "Transition Boundaries"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "compatibility-matrix.md",
                &[
                    "Summary",
                    "Guaranteed Compatibility",
                    "Temporary Incompatibilities",
                    "Coexistence Rules",
                ],
                &[GateKind::Architecture, GateKind::MigrationSafety],
            ),
            requirement(
                "sequencing-plan.md",
                &["Summary", "Ordered Steps", "Parallelizable Work", "Cutover Criteria"],
                &[GateKind::MigrationSafety, GateKind::ReleaseReadiness],
            ),
            requirement(
                "fallback-plan.md",
                &["Summary", "Rollback Triggers", "Fallback Paths", "Re-Entry Criteria"],
                &[GateKind::MigrationSafety, GateKind::Risk],
            ),
            requirement(
                "migration-verification-report.md",
                &["Summary", "Verification Checks", "Residual Risks", "Release Readiness"],
                &[GateKind::MigrationSafety, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-record.md",
                &["Summary", "Migration Decisions", "Deferred Decisions", "Approval Notes"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
        ],
        Mode::Refactor => vec![
            requirement(
                "preserved-behavior.md",
                &["Summary", "Preserved Behavior", "Approved Exceptions"],
                &[GateKind::ChangePreservation, GateKind::ReleaseReadiness],
            ),
            requirement(
                "refactor-scope.md",
                &["Summary", "Refactor Scope", "Allowed Paths"],
                &[GateKind::ChangePreservation, GateKind::Risk],
            ),
            requirement(
                "structural-rationale.md",
                &["Summary", "Structural Rationale", "Untouched Surface"],
                &[GateKind::Exploration, GateKind::ChangePreservation],
            ),
            requirement(
                "regression-evidence.md",
                &["Summary", "Safety-Net Evidence", "Regression Findings"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "contract-drift-check.md",
                &["Summary", "Contract Drift", "Reviewer Notes"],
                &[GateKind::Architecture, GateKind::ChangePreservation],
            ),
            requirement(
                "no-feature-addition.md",
                &["Summary", "Feature Audit", "Decision"],
                &[GateKind::ChangePreservation, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Architecture => vec![
            requirement(
                "architecture-decisions.md",
                &[
                    "Summary",
                    "Decision",
                    "Constraints",
                    "Decision Drivers",
                    "Recommendation",
                    "Consequences",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "invariants.md",
                &["Summary", "Invariants", "Rationale", "Verification Hooks"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "tradeoff-matrix.md",
                &[
                    "Summary",
                    "Options Considered",
                    "Evaluation Criteria",
                    "Pros",
                    "Cons",
                    "Why Not The Others",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "boundary-map.md",
                &["Summary", "Boundaries", "Ownership", "Crossing Rules"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "context-map.md",
                &[
                    "Summary",
                    "Bounded Contexts",
                    "Context Relationships",
                    "Integration Seams",
                    "Anti-Corruption Candidates",
                    "Ownership Boundaries",
                    "Shared Invariants",
                ],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "readiness-assessment.md",
                &["Summary", "Readiness Status", "Blockers", "Accepted Risks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "system-context.md",
                &["System Context"],
                &[GateKind::Architecture, GateKind::Exploration],
            ),
            requirement("container-view.md", &["Containers"], &[GateKind::Architecture]),
            requirement(
                "component-view.md",
                &["Components"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Review => vec![
            requirement(
                "review-brief.md",
                &["Summary", "Review Target", "Evidence Basis"],
                &[GateKind::Risk, GateKind::Architecture],
            ),
            requirement(
                "boundary-assessment.md",
                &["Summary", "Boundary Findings", "Ownership Notes"],
                &[GateKind::Architecture, GateKind::ReviewDisposition],
            ),
            requirement(
                "missing-evidence.md",
                &["Summary", "Missing Evidence", "Collection Priorities"],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-impact.md",
                &["Summary", "Decision Impact", "Reversibility Concerns"],
                &[GateKind::Architecture, GateKind::ReviewDisposition],
            ),
            requirement(
                "review-disposition.md",
                &["Summary", "Final Disposition", "Accepted Risks"],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
        ],
        Mode::Verification => vec![
            requirement(
                "invariants-checklist.md",
                &["Summary", "Claims Under Test", "Invariant Checks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "contract-matrix.md",
                &["Summary", "Contract Assumptions", "Verification Outcome"],
                &[GateKind::ReleaseReadiness],
            ),
            requirement(
                "adversarial-review.md",
                &["Summary", "Challenge Findings", "Contradictions"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "verification-report.md",
                &["Summary", "Verified Claims", "Rejected Claims", "Overall Verdict"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "unresolved-findings.md",
                &["Summary", "Open Findings", "Required Follow-Up"],
                &[GateKind::ReleaseReadiness],
            ),
        ],
        Mode::PrReview => vec![
            requirement(
                "pr-analysis.md",
                &[
                    "Summary",
                    "Scope Summary",
                    "Changed Modules",
                    "Inferred Intent",
                    "Surprising Surface Area",
                ],
                &[GateKind::Risk, GateKind::ReviewDisposition],
            ),
            requirement(
                "boundary-check.md",
                &[
                    "Summary",
                    "Boundary Findings",
                    "Ownership Breaks",
                    "Unauthorized Structural Impact",
                ],
                &[GateKind::Architecture, GateKind::ReviewDisposition],
            ),
            requirement(
                "conventional-comments.md",
                &["Summary", "Evidence Posture", "Conventional Comments", "Traceability"],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
            requirement(
                "duplication-check.md",
                &["Summary", "Duplicate Behavior", "Canonical Owner Conflicts"],
                &[GateKind::ReviewDisposition],
            ),
            requirement(
                "contract-drift.md",
                &["Summary", "Interface Drift", "Compatibility Concerns"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "missing-tests.md",
                &[
                    "Summary",
                    "Missing Invariant Checks",
                    "Missing Contract Checks",
                    "Weak or Mirrored Tests",
                ],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-impact.md",
                &[
                    "Summary",
                    "Implied Decisions",
                    "Absent Decision Records",
                    "Reversibility Concerns",
                ],
                &[GateKind::Risk, GateKind::ReviewDisposition],
            ),
            requirement(
                "review-summary.md",
                &[
                    "Summary",
                    "Severity",
                    "Must-Fix Findings",
                    "Accepted Risks",
                    "Final Disposition",
                ],
                &[GateKind::ReviewDisposition, GateKind::ReleaseReadiness],
            ),
        ],
    };

    ArtifactContract {
        version: 1,
        artifact_requirements: files,
        required_verification_layers: vec![VerificationLayer::SelfCritique],
    }
}

pub fn backlog_contract_for_closure(
    contract: &ArtifactContract,
    closure_assessment: &ClosureAssessment,
) -> ArtifactContract {
    if matches!(closure_assessment.decomposition_scope, ClosureDecompositionScope::RiskOnlyPacket) {
        let mut filtered = contract.clone();
        filtered.artifact_requirements.retain(|requirement| {
            matches!(requirement.file_name.as_str(), "backlog-overview.md" | "planning-risks.md")
        });
        filtered
    } else {
        contract.clone()
    }
}

pub fn validate_artifact(requirement: &ArtifactRequirement, contents: &str) -> Vec<String> {
    let mut blockers = Vec::new();

    for section in &requirement.required_sections {
        if !contains_required_heading(contents, section) {
            blockers
                .push(format!("{} is missing required section `{section}`", requirement.file_name));
        }
    }

    blockers
}

fn contains_required_heading(contents: &str, section: &str) -> bool {
    let expected = format!("## {section}");
    contents.lines().any(|line| line.trim() == expected)
}

pub fn validate_release_bundle(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
) -> Vec<String> {
    let mut blockers = Vec::new();

    for requirement in &contract.artifact_requirements {
        match artifacts.iter().find(|(file_name, _)| file_name == &requirement.file_name) {
            Some((_, contents)) => blockers.extend(validate_artifact(requirement, contents)),
            None => blockers.push(format!("missing required artifact `{}`", requirement.file_name)),
        }
    }

    blockers
}

fn requirement(
    file_name: &str,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    ArtifactRequirement {
        file_name: file_name.to_string(),
        format: ArtifactFormat::Markdown,
        required_sections: required_sections.iter().map(ToString::to_string).collect(),
        gates: gates.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_artifact_requires_exact_h2_headings() {
        let requirement = requirement("task-mapping.md", &["Task Mapping"], &[]);

        let blockers = validate_artifact(
            &requirement,
            "# Implementation Brief\n\n### Task Mapping\n\n- this should not satisfy the contract\n",
        );

        assert_eq!(
            blockers,
            vec!["task-mapping.md is missing required section `Task Mapping`".to_string()]
        );
    }

    #[test]
    fn validate_release_bundle_reports_missing_artifacts_and_sections() {
        let contract = ArtifactContract {
            version: 1,
            artifact_requirements: vec![
                requirement("task-mapping.md", &["Task Mapping"], &[]),
                requirement("mutation-bounds.md", &["Mutation Bounds"], &[]),
            ],
            required_verification_layers: vec![VerificationLayer::SelfCritique],
        };

        let blockers = validate_release_bundle(
            &contract,
            &[("task-mapping.md".to_string(), "## Task Mapping\n\n- bounded slice\n".to_string())],
        );

        assert!(blockers.contains(&"missing required artifact `mutation-bounds.md`".to_string()));

        let blockers = validate_release_bundle(
            &contract,
            &[
                ("task-mapping.md".to_string(), "## Task Mapping\n\n- bounded slice\n".to_string()),
                (
                    "mutation-bounds.md".to_string(),
                    "## Summary\n\nmissing the canonical heading\n".to_string(),
                ),
            ],
        );

        assert!(blockers.contains(
            &"mutation-bounds.md is missing required section `Mutation Bounds`".to_string()
        ));
    }
}
