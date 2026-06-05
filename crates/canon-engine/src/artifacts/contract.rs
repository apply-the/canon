//! Artifact contracts for all governed modes.
//!
//! The dispatch function [`contract_for_mode`] is the single entry-point. Each
//! mode group lives in a dedicated sub-module so file sizes stay manageable and
//! mode-specific changes are isolated:
//!
//! | Module | Modes |
//! |---|---|
//! | [`authoring`] | Requirements, Discovery, SystemShaping |
//! | [`delivery`] | Backlog, Change, Implementation, Refactor |
//! | [`governance`] | Architecture, Review, PrReview, Verification |
//! | [`analysis`] | SecurityAssessment, SystemAssessment, SupplyChainAnalysis |
//! | [`operations`] | Incident, Migration |
//! | [`domain`] | DomainLanguage, DomainModel |

mod analysis;
mod authoring;
mod delivery;
mod domain;
mod governance;
mod operations;
pub(super) mod sections;

use crate::domain::artifact::{
    ArtifactContract, ArtifactFormat, ArtifactRequirement, artifact_slug, is_packet_sidecar,
    prefixed_artifact_name,
};
use crate::domain::gate::GateKind;
use crate::domain::mode::Mode;
use crate::domain::run::{
    BacklogHandoffAvailability, BacklogPlanningContext, ClosureAssessment,
    ClosureDecompositionScope,
};
use crate::domain::verification::VerificationLayer;

/// Return the full [`ArtifactContract`] for the given [`Mode`].
///
/// The contract lists every expected body artifact in canonical delivery order,
/// the required Markdown sections each artifact must contain, and the gate kinds
/// that must be satisfied before the artifact is accepted. Sidecar artifacts
/// (`view-manifest.json`, `packet-metadata.json`) are appended last and excluded
/// from ordering and primary-artifact resolution by [`is_packet_sidecar`].
pub fn contract_for_mode(mode: Mode) -> ArtifactContract {
    let files = match mode {
        Mode::Requirements => authoring::requirements(),
        Mode::Discovery => authoring::discovery(),
        Mode::SystemShaping => authoring::system_shaping(),
        Mode::Backlog => delivery::backlog(),
        Mode::Change => delivery::change(),
        Mode::Implementation => delivery::implementation(),
        Mode::Refactor => delivery::refactor(),
        Mode::Architecture => governance::architecture(),
        Mode::Review => governance::review(),
        Mode::PrReview => governance::pr_review(),
        Mode::Verification => governance::verification(),
        Mode::SecurityAssessment => analysis::security_assessment(),
        Mode::SystemAssessment => analysis::system_assessment(),
        Mode::SupplyChainAnalysis => analysis::supply_chain_analysis(),
        Mode::Incident => operations::incident(),
        Mode::Migration => operations::migration(),
        Mode::DomainLanguage => domain::domain_language(),
        Mode::DomainModel => domain::domain_model(),
        Mode::Debugging => delivery::debugging(),
        Mode::Brainstorming => authoring::brainstorming(),
        Mode::PolicyShaping => governance::policy_shaping(),
    };

    build_contract(files)
}

/// Appends the mandatory `packet-metadata.json` sidecar (when absent), applies
/// numeric prefixes to body artifacts, and wraps the result in an
/// [`ArtifactContract`].
fn build_contract(mut files: Vec<ArtifactRequirement>) -> ArtifactContract {
    if !files.iter().any(|r| artifact_slug(&r.file_name) == "packet-metadata.json") {
        files.push(requirement_with_format(
            "packet-metadata.json",
            ArtifactFormat::Json,
            &[],
            &[GateKind::ReleaseReadiness],
        ));
    }

    let mut reader_facing_index = 0;
    let prefixed_files = files
        .into_iter()
        .map(|mut req| {
            let bare_name = artifact_slug(&req.file_name).to_string();
            req.file_name = if is_packet_sidecar(&bare_name) {
                bare_name
            } else {
                reader_facing_index += 1;
                prefixed_artifact_name(reader_facing_index, &bare_name)
            };
            req
        })
        .collect();

    ArtifactContract {
        version: 1,
        artifact_requirements: prefixed_files,
        required_verification_layers: vec![VerificationLayer::SelfCritique],
    }
}

/// Returns a filtered contract for backlog packets where closure scope is risk-only.
pub fn backlog_contract_for_closure(
    contract: &ArtifactContract,
    closure_assessment: &ClosureAssessment,
) -> ArtifactContract {
    if matches!(closure_assessment.decomposition_scope, ClosureDecompositionScope::RiskOnlyPacket) {
        let mut filtered = contract.clone();
        filtered.artifact_requirements.retain(|r| {
            matches!(
                crate::domain::artifact::artifact_slug(&r.file_name),
                "backlog-overview.md" | "planning-risks.md"
            ) || is_packet_sidecar(&r.file_name)
        });
        filtered
    } else {
        contract.clone()
    }
}

/// Returns the effective backlog contract for the current planning context,
/// filtering optional handoff surfaces when no downstream handoff is available.
pub fn backlog_contract_for_planning_context(
    contract: &ArtifactContract,
    planning_context: &BacklogPlanningContext,
) -> ArtifactContract {
    let mut filtered = backlog_contract_for_closure(contract, &planning_context.closure_assessment);
    if !matches!(planning_context.handoff_availability, BacklogHandoffAvailability::Available) {
        filtered.artifact_requirements.retain(|requirement| {
            crate::domain::artifact::artifact_slug(&requirement.file_name) != "execution-handoff.md"
        });
    }
    filtered
}

/// Returns a filtered contract for architecture packets, enabling only artifacts
/// supported by the given context summary.
pub fn architecture_contract_for_context(
    contract: &ArtifactContract,
    context_summary: &str,
) -> ArtifactContract {
    let mut filtered = contract.clone();
    filtered.artifact_requirements.retain(|r| {
        r.required
            || crate::artifacts::markdown::architecture_artifact_enabled(
                &r.file_name,
                context_summary,
            )
    });

    let mut reader_facing_index = 0;
    for r in &mut filtered.artifact_requirements {
        let bare_name = artifact_slug(&r.file_name).to_string();
        r.file_name = if is_packet_sidecar(&bare_name) {
            bare_name
        } else {
            reader_facing_index += 1;
            prefixed_artifact_name(reader_facing_index, &bare_name)
        };
    }

    filtered
}

/// Validates that a single artifact's contents satisfy the required sections in the given requirement.
pub fn validate_artifact(requirement: &ArtifactRequirement, contents: &str) -> Vec<String> {
    requirement
        .required_sections
        .iter()
        .filter(|section| !contains_required_heading(contents, section))
        .map(|section| format!("{} is missing required section `{section}`", requirement.file_name))
        .collect()
}

fn contains_required_heading(contents: &str, section: &str) -> bool {
    let expected = format!("## {section}");
    contents.lines().any(|line| line.trim() == expected)
}

/// Validates all required artifacts in a release bundle against their contracts.
pub fn validate_release_bundle(
    contract: &ArtifactContract,
    artifacts: &[(String, String)],
) -> Vec<String> {
    let mut blockers = Vec::new();

    for req in &contract.artifact_requirements {
        match artifacts.iter().find(|(file_name, _)| file_name == &req.file_name) {
            Some((_, contents)) => blockers.extend(validate_artifact(req, contents)),
            None if req.required => {
                blockers.push(format!("missing required artifact `{}`", req.file_name))
            }
            None => {}
        }
    }

    blockers
}

// ── Builder helpers (used by sub-modules via `super::`) ───────────────────────

pub(super) fn requirement(
    file_name: &str,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    requirement_with_format(file_name, ArtifactFormat::Markdown, required_sections, gates)
}

pub(super) fn requirement_with_format(
    file_name: &str,
    format: ArtifactFormat,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    ArtifactRequirement {
        file_name: file_name.to_string(),
        format,
        required_sections: required_sections.iter().map(|s| (*s).to_string()).collect(),
        gates: gates.to_vec(),
        required: true,
    }
}

pub(super) fn optional_requirement(
    file_name: &str,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    optional_requirement_with_format(file_name, ArtifactFormat::Markdown, required_sections, gates)
}

pub(super) fn optional_requirement_with_format(
    file_name: &str,
    format: ArtifactFormat,
    required_sections: &[&str],
    gates: &[GateKind],
) -> ArtifactRequirement {
    ArtifactRequirement {
        file_name: file_name.to_string(),
        format,
        required_sections: required_sections.iter().map(|s| (*s).to_string()).collect(),
        gates: gates.to_vec(),
        required: false,
    }
}

#[cfg(test)]
mod tests {
    use super::{architecture_contract_for_context, contract_for_mode};
    use crate::domain::mode::Mode;

    #[test]
    fn architecture_contract_for_context_excludes_unmentioned_optional_views() {
        let contract = contract_for_mode(Mode::Architecture);

        let filtered = architecture_contract_for_context(
            &contract,
            "# Architecture Brief\n\nDecision focus: bounded analytics CLI.\nConstraint: preserve Canon runtime contracts.\n",
        );

        let slugs = filtered.artifact_requirements.iter().map(|r| r.slug()).collect::<Vec<_>>();

        assert!(!slugs.contains(&"component-view.md"));
        assert!(!slugs.contains(&"component-view.mmd"));
        assert!(!slugs.contains(&"dynamic-view.md"));
        assert!(!slugs.contains(&"dynamic-view.mmd"));
        assert_eq!(slugs.last(), Some(&"packet-metadata.json"));
        assert!(filtered.artifact_requirements.iter().any(|r| r.file_name == "view-manifest.json"));
        assert!(
            filtered.artifact_requirements.iter().any(|r| r.file_name == "packet-metadata.json")
        );
    }

    #[test]
    fn contract_for_mode_appends_packet_metadata_sidecar_for_all_modes() {
        use crate::domain::artifact::artifact_slug;
        use crate::domain::mode::Mode;
        for &mode in Mode::all() {
            let contract = contract_for_mode(mode);
            assert!(
                contract
                    .artifact_requirements
                    .iter()
                    .any(|r| artifact_slug(&r.file_name) == "packet-metadata.json"),
                "mode {mode:?} is missing packet-metadata.json sidecar"
            );
        }
    }

    #[test]
    fn validate_artifact_reports_missing_sections() {
        use super::{requirement, validate_artifact};
        use crate::domain::gate::GateKind;
        let req = requirement("test.md", &["Summary", "Problem"], &[GateKind::Risk]);
        let contents = "## Summary\n\nsome text\n";
        let errors = validate_artifact(&req, contents);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Problem"));
    }

    #[test]
    fn backlog_contract_for_planning_context_removes_execution_handoff_when_unavailable() {
        use super::backlog_contract_for_planning_context;
        use crate::domain::run::{
            BacklogGranularity, BacklogHandoffAvailability, BacklogPlanningContext,
            ClosureAssessment,
        };

        let contract = contract_for_mode(Mode::Backlog);
        let context = BacklogPlanningContext {
            mode: "backlog".to_string(),
            delivery_intent: "test".to_string(),
            desired_granularity: BacklogGranularity::EpicOnly,
            planning_horizon: None,
            source_refs: vec![],
            priority_inputs: vec![],
            constraints: vec![],
            out_of_scope: vec![],
            closure_assessment: ClosureAssessment::sufficient(),
            handoff_availability: BacklogHandoffAvailability::Unavailable,
            handoff_findings: vec![],
            slice_ids: vec![],
            execution_handoff: None,
        };

        let filtered = backlog_contract_for_planning_context(&contract, &context);
        let slugs = filtered.artifact_requirements.iter().map(|r| r.slug()).collect::<Vec<_>>();
        assert!(!slugs.contains(&"execution-handoff.md"));
    }
}
