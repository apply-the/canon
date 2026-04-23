use crate::domain::artifact::{ArtifactContract, ArtifactFormat, ArtifactRequirement};
use crate::domain::gate::GateKind;
use crate::domain::mode::Mode;
use crate::domain::verification::VerificationLayer;

pub fn contract_for_mode(mode: Mode) -> ArtifactContract {
    let files = match mode {
        Mode::Requirements => vec![
            requirement(
                "problem-statement.md",
                &["Summary", "Problem", "Boundary", "Success Signal"],
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
                    "Repo Signals",
                    "Problem Domain",
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
                &["Summary", "System Slice", "Excluded Areas"],
                &[GateKind::Exploration, GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "legacy-invariants.md",
                &["Summary", "Legacy Invariants", "Forbidden Normalization"],
                &[GateKind::ChangePreservation, GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "change-surface.md",
                &["Summary", "Change Surface", "Ownership"],
                &[GateKind::ChangePreservation, GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "implementation-plan.md",
                &["Summary", "Plan", "Sequencing"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "validation-strategy.md",
                &["Summary", "Validation Strategy", "Independent Checks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "decision-record.md",
                &["Summary", "Decision", "Consequences", "Unresolved Questions"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
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
                &["Summary", "Decisions", "Tradeoffs", "Consequences", "Unresolved Questions"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "invariants.md",
                &["Summary", "Invariants", "Rationale", "Verification Hooks"],
                &[GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "tradeoff-matrix.md",
                &["Summary", "Options", "Evaluation Criteria", "Scores", "Selected Option"],
                &[GateKind::Architecture, GateKind::Risk],
            ),
            requirement(
                "boundary-map.md",
                &["Summary", "Boundaries", "Ownership", "Crossing Rules"],
                &[GateKind::Exploration, GateKind::Architecture],
            ),
            requirement(
                "readiness-assessment.md",
                &["Summary", "Readiness Status", "Blockers", "Accepted Risks"],
                &[GateKind::Risk, GateKind::ReleaseReadiness],
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
                &["Summary", "Open Findings", "Required Follow-up"],
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
        other => vec![ArtifactRequirement {
            file_name: format!("{}.md", other.as_str()),
            format: ArtifactFormat::Markdown,
            required_sections: vec!["Summary".to_string()],
            gates: vec![GateKind::ReleaseReadiness],
        }],
    };

    ArtifactContract {
        version: 1,
        artifact_requirements: files,
        required_verification_layers: vec![VerificationLayer::SelfCritique],
    }
}

pub fn validate_artifact(requirement: &ArtifactRequirement, contents: &str) -> Vec<String> {
    let mut blockers = Vec::new();

    for section in &requirement.required_sections {
        let heading = format!("## {section}");
        if !contents.contains(&heading) {
            blockers
                .push(format!("{} is missing required section `{section}`", requirement.file_name));
        }
    }

    blockers
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
