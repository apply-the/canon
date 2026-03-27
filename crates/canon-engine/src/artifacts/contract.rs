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
        Mode::BrownfieldChange => vec![
            requirement(
                "system-slice.md",
                &["Summary", "System Slice", "Excluded Areas"],
                &[GateKind::Exploration, GateKind::Architecture, GateKind::ReleaseReadiness],
            ),
            requirement(
                "legacy-invariants.md",
                &["Summary", "Legacy Invariants", "Forbidden Normalization"],
                &[GateKind::BrownfieldPreservation, GateKind::Risk, GateKind::ReleaseReadiness],
            ),
            requirement(
                "change-surface.md",
                &["Summary", "Change Surface", "Ownership"],
                &[
                    GateKind::BrownfieldPreservation,
                    GateKind::Architecture,
                    GateKind::ReleaseReadiness,
                ],
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
