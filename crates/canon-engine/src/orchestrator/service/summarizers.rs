//! Mode-result summarizers, action-chip builder, and related helpers.
//!
//! All functions here produce display-ready data from `PersistedArtifact` slices
//! and run state; they do not write to disk or mutate domain state.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::domain::mode::Mode;
use crate::domain::run::RunState;
use crate::persistence::store::PersistedArtifact;

use super::{ActionChip, ModeResultSummary, ResultActionSummary};

#[derive(Debug, Deserialize)]
struct PacketMetadataPrimaryArtifact {
    primary_artifact: String,
}

/// Resolve the primary body artifact from a packet's artifact slice.
///
/// The function first reads the `primary_artifact` field from the
/// `packet-metadata.json` sidecar (if present and parseable), then falls back to
/// a linear scan for an artifact whose slug matches `fallback_slug`. Returns `None`
/// when neither lookup succeeds.
fn packet_primary_artifact<'a>(
    artifacts: &'a [PersistedArtifact],
    fallback_slug: &str,
) -> Option<&'a PersistedArtifact> {
    let metadata_primary = artifacts
        .iter()
        .find(|artifact| artifact.record.slug() == "packet-metadata.json")
        .and_then(|artifact| {
            serde_json::from_str::<PacketMetadataPrimaryArtifact>(&artifact.contents)
                .ok()
                .map(|metadata| metadata.primary_artifact)
        });

    metadata_primary
        .and_then(|file_name| {
            artifacts.iter().find(|artifact| artifact.record.file_name == file_name)
        })
        .or_else(|| artifacts.iter().find(|artifact| artifact.record.slug() == fallback_slug))
}

// ── Mode-result dispatch ──────────────────────────────────────────────────────

pub(crate) fn summarize_mode_result(
    mode: Mode,
    artifacts: &[PersistedArtifact],
) -> Option<ModeResultSummary> {
    match mode {
        Mode::Requirements => authoring::summarize_requirements_mode_result(artifacts),
        Mode::Discovery => authoring::summarize_discovery_mode_result(artifacts),
        Mode::SystemShaping => authoring::summarize_system_shaping_mode_result(artifacts),
        Mode::Architecture => governance::summarize_architecture_mode_result(artifacts),
        Mode::SystemAssessment => analysis::summarize_system_assessment_mode_result(artifacts),
        Mode::Change => delivery::summarize_change_mode_result(artifacts),
        Mode::Backlog => delivery::summarize_backlog_mode_result(artifacts),
        Mode::Incident => operations::summarize_incident_mode_result(artifacts),
        Mode::SecurityAssessment => analysis::summarize_security_assessment_mode_result(artifacts),
        Mode::Implementation => delivery::summarize_implementation_mode_result(artifacts),
        Mode::Migration => delivery::summarize_migration_mode_result(artifacts),
        Mode::SupplyChainAnalysis => {
            analysis::summarize_supply_chain_analysis_mode_result(artifacts)
        }
        Mode::Refactor => delivery::summarize_refactor_mode_result(artifacts),
        Mode::Review => governance::summarize_review_mode_result(artifacts),
        Mode::Verification => governance::summarize_verification_mode_result(artifacts),
        Mode::PrReview => governance::summarize_pr_review_mode_result(artifacts),
        Mode::DomainLanguage => domain::summarize_domain_language_mode_result(artifacts),
        Mode::DomainModel => domain::summarize_domain_model_mode_result(artifacts),
    }
}

// ── Action chip builder ───────────────────────────────────────────────────────

pub(crate) fn build_action_chips_for(
    state: RunState,
    approval_targets: &[String],
    primary_artifact_path: &str,
    run_id: &str,
) -> Vec<ActionChip> {
    let mut chips: Vec<ActionChip> = Vec::new();
    let open_primary_artifact_is_recommended = !primary_artifact_path.is_empty()
        && (matches!(state, RunState::Blocked)
            || (matches!(state, RunState::AwaitingApproval) && !approval_targets.is_empty()));

    if !primary_artifact_path.is_empty() {
        let mut args = BTreeMap::new();
        args.insert("PATH".to_string(), primary_artifact_path.to_string());
        chips.push(ActionChip {
            id: "open-primary-artifact".to_string(),
            label: "Open primary artifact".to_string(),
            skill: "host:open-file".to_string(),
            intent: "Inspect".to_string(),
            prefilled_args: args,
            required_user_inputs: Vec::new(),
            visibility_condition: "primary_artifact_path is non-empty".to_string(),
            recommended: open_primary_artifact_is_recommended,
            text_fallback: format!("Open the primary artifact at {primary_artifact_path}."),
        });
    }

    if matches!(state, RunState::AwaitingApproval | RunState::Completed) && !run_id.is_empty() {
        let mut args = BTreeMap::new();
        args.insert("RUN_ID".to_string(), run_id.to_string());
        chips.push(ActionChip {
            id: "inspect-evidence".to_string(),
            label: "Inspect evidence".to_string(),
            skill: "canon-inspect-evidence".to_string(),
            intent: "Inspect".to_string(),
            prefilled_args: args,
            required_user_inputs: Vec::new(),
            visibility_condition: "state is AwaitingApproval or Completed".to_string(),
            recommended: matches!(state, RunState::AwaitingApproval)
                && !approval_targets.is_empty()
                && primary_artifact_path.is_empty(),
            text_fallback: format!(
                "Inspect evidence for run {run_id}: `canon inspect evidence --run {run_id}`."
            ),
        });
    }

    if matches!(state, RunState::AwaitingApproval) && !run_id.is_empty() {
        for target in approval_targets {
            let mut args = BTreeMap::new();
            args.insert("RUN_ID".to_string(), run_id.to_string());
            args.insert("TARGET".to_string(), target.clone());
            chips.push(ActionChip {
                id: format!("approve-{}", target.replace(':', "-")),
                label: "Approve generation...".to_string(),
                skill: "canon-approve".to_string(),
                intent: "GovernedAction".to_string(),
                prefilled_args: args,
                required_user_inputs: vec![
                    "BY".to_string(),
                    "DECISION".to_string(),
                    "RATIONALE".to_string(),
                ],
                visibility_condition:
                    "state is AwaitingApproval and Canon issued an approval target".to_string(),
                recommended: false,
                text_fallback: format!(
                    "Approve target {target} for run {run_id}: `canon approve --run {run_id} --target {target} --by <BY> --decision <DECISION> --rationale <RATIONALE>`."
                ),
            });
        }

        if approval_targets.is_empty() {
            let mut args = BTreeMap::new();
            args.insert("RUN_ID".to_string(), run_id.to_string());
            chips.push(ActionChip {
                id: "resume-run".to_string(),
                label: "Resume run".to_string(),
                skill: "canon-resume".to_string(),
                intent: "GovernedAction".to_string(),
                prefilled_args: args,
                required_user_inputs: Vec::new(),
                visibility_condition:
                    "state is AwaitingApproval and Canon has no remaining approval targets"
                        .to_string(),
                recommended: true,
                text_fallback: format!(
                    "Resume run {run_id} to continue post-approval execution: `canon resume --run {run_id}`."
                ),
            });
        }
    }

    chips
}

// ── Primary-artifact action ───────────────────────────────────────────────────

pub(crate) fn primary_artifact_action_for(path: &str) -> ResultActionSummary {
    ResultActionSummary {
        id: "open-primary-artifact".to_string(),
        label: "Open primary artifact".to_string(),
        host_action: "open-file".to_string(),
        target: path.to_string(),
        text_fallback: format!("Open the primary artifact at {path}."),
    }
}

fn packet_output_quality_headline(
    packet_name: &str,
    missing_context_markers: usize,
    caution_count: usize,
    caution_label: &str,
    publishable_target: &str,
) -> String {
    if missing_context_markers > 0 {
        format!(
            "{packet_name} packet is structurally complete only and still carries {missing_context_markers} explicit missing-context marker(s)."
        )
    } else if caution_count > 0 {
        format!(
            "{packet_name} packet is materially useful but still carries {caution_count} {caution_label} before {publishable_target}."
        )
    } else {
        format!("{packet_name} packet is publishable for {publishable_target}.")
    }
}

fn packet_output_quality_artifact_prefix(
    missing_context_markers: usize,
    caution_count: usize,
    caution_label: &str,
) -> String {
    if missing_context_markers > 0 {
        format!(
            "Primary artifact is structurally complete only and still carries {missing_context_markers} missing-context marker(s)."
        )
    } else if caution_count > 0 {
        format!(
            "Primary artifact is materially useful but still carries {caution_count} {caution_label}."
        )
    } else {
        "Primary artifact is publishable.".to_string()
    }
}

// ── Mode-summarizer submodules ───────────────────────────────────────────────

mod analysis;
mod authoring;
mod delivery;
mod domain;
mod governance;
mod operations;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::artifact::{ArtifactFormat, ArtifactRecord};

    fn make_artifact(file_name: &str, contents: &str) -> PersistedArtifact {
        PersistedArtifact {
            record: ArtifactRecord {
                file_name: file_name.to_string(),
                relative_path: format!("artifacts/run-test/mode/{file_name}"),
                format: ArtifactFormat::Markdown,
                provenance: None,
            },
            contents: contents.to_string(),
        }
    }

    #[test]
    fn build_action_chips_for_includes_open_artifact_chip() {
        let chips = build_action_chips_for(
            RunState::Completed,
            &[],
            ".canon/artifacts/task-mapping.md",
            "run-123",
        );
        assert!(chips.iter().any(|c| c.id == "open-primary-artifact"));
    }

    #[test]
    fn build_action_chips_for_includes_inspect_evidence_when_awaiting_approval() {
        let chips = build_action_chips_for(
            RunState::AwaitingApproval,
            &["gate:execution".to_string()],
            ".canon/artifacts/task-mapping.md",
            "run-123",
        );
        assert!(chips.iter().any(|c| c.id == "inspect-evidence"));
    }

    #[test]
    fn build_action_chips_for_includes_approve_chip_for_each_target() {
        let chips = build_action_chips_for(
            RunState::AwaitingApproval,
            &["gate:execution".to_string()],
            ".canon/artifacts/task-mapping.md",
            "run-123",
        );
        assert!(chips.iter().any(|c| c.id == "approve-gate-execution"));
    }

    #[test]
    fn build_action_chips_for_includes_resume_chip_when_no_targets() {
        let chips =
            build_action_chips_for(RunState::AwaitingApproval, &[], ".canon/artifact.md", "run-1");
        assert!(chips.iter().any(|c| c.id == "resume-run"));
        let resume = chips.iter().find(|c| c.id == "resume-run").unwrap();
        assert!(resume.recommended);
    }

    #[test]
    fn approve_chip_text_fallback_includes_runnable_command() {
        let chips = build_action_chips_for(
            RunState::AwaitingApproval,
            &["gate:execution".to_string()],
            "",
            "run-42",
        );
        let approve = chips.iter().find(|c| c.id == "approve-gate-execution").unwrap();
        assert!(
            approve.text_fallback.contains("canon approve --run run-42 --target gate:execution"),
            "text_fallback should embed the exact canon approve command: {}",
            approve.text_fallback
        );
        assert!(approve.text_fallback.contains("run-42"));
    }

    #[test]
    fn summarize_requirements_mode_result_returns_none_when_primary_missing() {
        let artifacts = vec![make_artifact("constraints.md", "## Constraints\n- item")];
        assert!(summarize_mode_result(Mode::Requirements, &artifacts).is_none());
    }

    #[test]
    fn summarize_requirements_mode_result_produces_summary() {
        let artifacts = vec![make_artifact(
            "problem-statement.md",
            "## Problem\nReduce auth latency.\n\n## Summary\nFix auth.",
        )];
        let summary = summarize_mode_result(Mode::Requirements, &artifacts);
        assert!(summary.is_some());
        let s = summary.unwrap();
        assert!(s.headline.contains("Requirements packet"));
        assert!(s.primary_artifact_title == "Problem Statement");
    }

    #[test]
    fn packet_output_quality_headline_marks_materially_useful_when_caveats_remain() {
        let headline = packet_output_quality_headline(
            "Requirements",
            0,
            2,
            "open question set(s)",
            "downstream review",
        );

        assert!(headline.contains("materially useful"));
        assert!(headline.contains("2 open question set(s)"));
    }

    #[test]
    fn packet_output_quality_headline_marks_publishable_when_complete() {
        let headline = packet_output_quality_headline(
            "Architecture",
            0,
            0,
            "",
            "downstream implementation or review",
        );

        assert!(headline.contains("publishable"));
        assert!(headline.contains("downstream implementation or review"));
    }

    #[test]
    fn summarize_implementation_mode_result_returns_none_when_primary_missing() {
        let artifacts = vec![make_artifact("mutation-bounds.md", "## Allowed Paths\n- src/**")];
        assert!(summarize_mode_result(Mode::Implementation, &artifacts).is_none());
    }

    #[test]
    fn summarize_implementation_mode_result_headline_indicates_missing_context() {
        let artifacts = vec![make_artifact("task-mapping.md", "## Task Mapping\n- task 1")];
        let summary = summarize_mode_result(Mode::Implementation, &artifacts).unwrap();
        // mutation-bounds and validation-hooks are missing, so there should be missing markers
        assert!(summary.headline.contains("missing-context marker"));
    }

    #[test]
    fn primary_artifact_action_for_populates_correct_fields() {
        let action = primary_artifact_action_for(".canon/artifacts/task-mapping.md");
        assert_eq!(action.id, "open-primary-artifact");
        assert_eq!(action.target, ".canon/artifacts/task-mapping.md");
        assert!(action.text_fallback.contains(".canon/artifacts/task-mapping.md"));
    }

    #[test]
    fn summarize_mode_result_returns_incident_and_migration_summaries() {
        let incident_artifacts = vec![
            make_artifact(
                "incident-frame.md",
                "## Summary\nContain payment outage.\n\n## Incident Scope\n- payments-api\n\n## Trigger And Current State\n- active outage\n\n## Operational Constraints\n- no schema changes\n",
            ),
            make_artifact(
                "blast-radius-map.md",
                "## Summary\nBlast radius bounded to payments.\n\n## Impacted Surfaces\n- payments-api\n\n## Propagation Paths\n- checkout flow\n\n## Confidence And Unknowns\n- medium confidence\n",
            ),
            make_artifact(
                "containment-plan.md",
                "## Summary\nContain by disabling async retries.\n\n## Immediate Actions\n- disable retries\n\n## Ordered Sequence\n- step 1\n\n## Stop Conditions\n- errors stable\n",
            ),
        ];
        let migration_artifacts = vec![
            make_artifact(
                "source-target-map.md",
                "## Summary\nMove auth traffic to v2.\n\n## Current State\n- v1 auth\n\n## Target State\n- v2 auth\n\n## Transition Boundaries\n- login flow\n",
            ),
            make_artifact(
                "compatibility-matrix.md",
                "## Summary\nCompatibility tracked.\n\n## Guaranteed Compatibility\n- tokens valid\n\n## Temporary Incompatibilities\n- admin UI\n\n## Coexistence Rules\n- dual-write\n",
            ),
            make_artifact(
                "fallback-plan.md",
                "## Summary\nRollback to v1.\n\n## Rollback Triggers\n- auth failures\n\n## Fallback Paths\n- route to v1\n\n## Re-Entry Criteria\n- fix deployed\n",
            ),
        ];

        let incident_summary = summarize_mode_result(Mode::Incident, &incident_artifacts)
            .expect("incident summary should exist once the operational mode is implemented");
        assert!(incident_summary.headline.to_ascii_lowercase().contains("incident"));
        assert_eq!(incident_summary.execution_posture.as_deref(), Some("recommendation-only"));

        let migration_summary = summarize_mode_result(Mode::Migration, &migration_artifacts)
            .expect("migration summary should exist once the operational mode is implemented");
        assert!(migration_summary.headline.to_ascii_lowercase().contains("migration"));
        assert_eq!(migration_summary.execution_posture.as_deref(), Some("recommendation-only"));
    }

    #[test]
    fn summarize_review_mode_result_surfaces_evidence_bounded_posture() {
        let artifacts = vec![
            make_artifact(
                "review-brief.md",
                "## Review Target\n\n- bounded package only\n\n## Evidence Basis\n\n- packet is grounded in authored evidence.",
            ),
            make_artifact(
                "review-disposition.md",
                "## Final Disposition\n\nStatus: ready-with-review-notes\n\nRationale: bounded review packet is ready for inspection.\n\n## Accepted Risks\n\n- residual review notes remain bounded to this package.",
            ),
            make_artifact(
                "boundary-assessment.md",
                "## Boundary Findings\n\n- No boundary expansion beyond the authored review target was detected.",
            ),
            make_artifact(
                "missing-evidence.md",
                "## Missing Evidence\n\nStatus: evidence-bounded\n\n- No critical evidence gaps remain from the authored review packet.",
            ),
        ];

        let summary = summarize_mode_result(Mode::Review, &artifacts).expect("review summary");
        assert!(summary.headline.contains("evidence-bounded"));
        assert!(summary.headline.contains("no boundary expansion"));
        assert!(summary.artifact_packet_summary.contains("evidence-bounded"));
    }

    #[test]
    fn summarize_review_mode_result_requires_explicit_disposition_when_pending() {
        let artifacts = vec![
            make_artifact(
                "review-brief.md",
                "## Review Target\n\n- bounded package only\n\n## Evidence Basis\n\n- packet is grounded in authored evidence.",
            ),
            make_artifact(
                "review-disposition.md",
                "## Final Disposition\n\nStatus: awaiting-disposition\n\nRationale: explicit owner sign-off is still pending.\n\n## Accepted Risks\n\n- No accepted risks recorded while disposition is still pending.",
            ),
            make_artifact(
                "boundary-assessment.md",
                "## Boundary Findings\n\n- shared release gate still needs owner confirmation.",
            ),
            make_artifact(
                "missing-evidence.md",
                "## Missing Evidence\n\nStatus: evidence-open\n\n- operator acknowledgment is still pending.",
            ),
        ];

        let summary = summarize_mode_result(Mode::Review, &artifacts).expect("review summary");
        assert!(summary.headline.contains("requires explicit disposition"));
        assert!(summary.artifact_packet_summary.contains("boundary finding set(s)"));
    }

    #[test]
    fn summarize_review_mode_result_surfaces_explicit_approval_branch() {
        let artifacts = vec![
            make_artifact(
                "review-brief.md",
                "## Review Target\n\n- bounded package only\n\n## Evidence Basis\n\n- packet is grounded in authored evidence.",
            ),
            make_artifact(
                "review-disposition.md",
                "## Final Disposition\n\nStatus: accepted-with-approval\n\nRationale: residual concerns were explicitly approved.\n\n## Accepted Risks\n\n- follow-up verification stays attached to this packet.",
            ),
            make_artifact(
                "boundary-assessment.md",
                "## Boundary Findings\n\n- rollout note remains bounded to the current package.",
            ),
            make_artifact(
                "missing-evidence.md",
                "## Missing Evidence\n\nStatus: evidence-reviewed\n\n- no additional evidence collection is required before publication.",
            ),
        ];

        let summary = summarize_mode_result(Mode::Review, &artifacts).expect("review summary");
        assert!(summary.headline.contains("explicit approval"));
        assert!(summary.artifact_packet_summary.contains("`accepted-with-approval`"));
    }

    #[test]
    fn summarize_review_mode_result_reports_missing_context_markers() {
        let artifacts = vec![
            make_artifact(
                "review-brief.md",
                "## Review Target\n\n- bounded package only\n\n## Evidence Basis\n\n- packet is grounded in authored evidence.",
            ),
            make_artifact("review-disposition.md", "## Notes\n\n- summary still needs structure."),
        ];

        let summary = summarize_mode_result(Mode::Review, &artifacts).expect("review summary");
        assert!(summary.headline.contains("missing-context marker"));
        assert!(summary.artifact_packet_summary.contains("packet is readable"));
    }

    #[test]
    fn summarize_verification_mode_result_surfaces_no_direct_contradiction_posture() {
        let artifacts = vec![
            make_artifact(
                "verification-report.md",
                "## Verified Claims\n\n- rollback remains bounded\n\n## Rejected Claims\n\n- none recorded\n\n## Overall Verdict\n\nStatus: supported\n\nRationale: current evidence covers the authored claim set.",
            ),
            make_artifact(
                "unresolved-findings.md",
                "## Open Findings\n\nStatus: no-open-findings\n\n- No unresolved findings remain from the current verification packet.",
            ),
            make_artifact(
                "invariants-checklist.md",
                "## Claims Under Test\n\n- rollback remains bounded\n- operator evidence stays attached to the packet",
            ),
            make_artifact("adversarial-review.md", "## Contradictions\n\n- none recorded"),
        ];

        let summary =
            summarize_mode_result(Mode::Verification, &artifacts).expect("verification summary");
        assert!(summary.headline.contains("no direct contradictions"));
        assert!(summary.artifact_packet_summary.contains("no-direct-contradiction"));
    }

    #[test]
    fn summarize_verification_mode_result_recognizes_packet_no_contradiction_phrase() {
        let artifacts = vec![
            make_artifact(
                "verification-report.md",
                "## Verified Claims\n\n- rollback remains bounded\n\n## Rejected Claims\n\n- none recorded\n\n## Overall Verdict\n\nStatus: supported\n\nRationale: current evidence covers the authored claim set.",
            ),
            make_artifact(
                "unresolved-findings.md",
                "## Open Findings\n\nStatus: no-open-findings\n\n- No unresolved findings remain from the current verification packet.",
            ),
            make_artifact(
                "invariants-checklist.md",
                "## Claims Under Test\n\n- rollback remains bounded\n- operator evidence stays attached to the packet",
            ),
            make_artifact(
                "adversarial-review.md",
                "## Contradictions\n\n- No direct contradiction was identified against the current verification packet.",
            ),
        ];

        let summary =
            summarize_mode_result(Mode::Verification, &artifacts).expect("verification summary");
        assert!(summary.headline.contains("no direct contradictions"));
        assert!(summary.artifact_packet_summary.contains("no-direct-contradiction"));
    }

    #[test]
    fn summarize_verification_mode_result_blocks_release_when_findings_remain_open() {
        let artifacts = vec![
            make_artifact(
                "verification-report.md",
                "## Verified Claims\n\n- rollback remains bounded\n\n## Rejected Claims\n\n- none recorded\n\n## Overall Verdict\n\nStatus: supported-with-follow-up\n\nRationale: remaining checks are bounded and explicit.",
            ),
            make_artifact(
                "unresolved-findings.md",
                "## Open Findings\n\nStatus: unresolved-findings-open\n\n- capture operator rollback evidence\n- attach downstream verification trace",
            ),
            make_artifact(
                "invariants-checklist.md",
                "## Claims Under Test\n\n- rollback remains bounded\n- operator evidence stays attached to the packet",
            ),
            make_artifact("adversarial-review.md", "## Contradictions\n\n- none recorded"),
        ];

        let summary =
            summarize_mode_result(Mode::Verification, &artifacts).expect("verification summary");
        assert!(summary.headline.contains("blocked release readiness"));
        assert!(summary.artifact_packet_summary.contains("unresolved finding set(s)"));
    }

    #[test]
    fn summarize_verification_mode_result_reports_recorded_contradictions() {
        let artifacts = vec![
            make_artifact(
                "verification-report.md",
                "## Verified Claims\n\n- rollback remains bounded\n\n## Rejected Claims\n\n- none recorded\n\n## Overall Verdict\n\nStatus: contested\n\nRationale: a contradiction was recorded against the current packet.",
            ),
            make_artifact(
                "unresolved-findings.md",
                "## Open Findings\n\nStatus: no-open-findings\n\n- No unresolved findings remain from the current verification packet.",
            ),
            make_artifact(
                "invariants-checklist.md",
                "## Claims Under Test\n\n- rollback remains bounded\n- operator evidence stays attached to the packet",
            ),
            make_artifact(
                "adversarial-review.md",
                "## Contradictions\n\n- rollback drifted outside the approved boundary.",
            ),
        ];

        let summary =
            summarize_mode_result(Mode::Verification, &artifacts).expect("verification summary");
        assert!(summary.headline.contains("contradiction set(s)"));
        assert!(summary.artifact_packet_summary.contains("contradiction set(s)"));
    }

    #[test]
    fn summarize_verification_mode_result_reports_missing_context_markers() {
        let artifacts = vec![
            make_artifact(
                "verification-report.md",
                "## Verified Claims\n\n- rollback remains bounded",
            ),
            make_artifact(
                "invariants-checklist.md",
                "## Claims Under Test\n\n- rollback remains bounded",
            ),
        ];

        let summary =
            summarize_mode_result(Mode::Verification, &artifacts).expect("verification summary");
        assert!(summary.headline.contains("missing-context marker"));
        assert!(summary.artifact_packet_summary.contains("packet is readable"));
    }

    #[test]
    fn summarize_pr_review_mode_result_derives_review_note_severity_from_placeholders() {
        let artifacts = vec![
            make_artifact(
                "pr-analysis.md",
                "## Changed Modules\n\n- crates/canon-engine/src/orchestrator/service/summarizers/governance.rs\n\n## Inferred Intent\n\n- preserve governed summary behavior while tightening wording.",
            ),
            make_artifact(
                "review-summary.md",
                "## Final Disposition\n\nStatus: ready-with-review-notes\n\nRationale: bounded review notes remain.\n\n## Severity\n\n- triaged without escalation.\n\n## Must-Fix Findings\n\n- No must-fix findings remain.\n\n## Accepted Risks\n\n- Follow-up wording can be tightened in docs.",
            ),
        ];

        let summary = summarize_mode_result(Mode::PrReview, &artifacts).expect("pr review summary");
        assert!(summary.headline.contains("review note(s) and no unresolved must-fix findings"));
        assert!(summary.artifact_packet_summary.contains("`review-notes` severity"));
    }

    #[test]
    fn summarize_pr_review_mode_result_derives_must_fix_severity_when_disposition_is_pending() {
        let artifacts = vec![
            make_artifact(
                "pr-analysis.md",
                "## Changed Modules\n\n- crates/canon-engine/src/orchestrator/service/summarizers/governance.rs\n- crates/canon-engine/src/orchestrator/service/summarizers.rs\n\n## Inferred Intent\n\n- keep the review packet bounded to the changed summarizer surface.",
            ),
            make_artifact(
                "review-summary.md",
                "## Final Disposition\n\nStatus: awaiting-disposition\n\nRationale: release owner sign-off is still pending.\n\n## Severity\n\n- pending final disposition.\n\n## Must-Fix Findings\n\n- release owner sign-off is still pending.\n\n## Accepted Risks\n\n- No accepted risks recorded.",
            ),
        ];

        let summary = summarize_mode_result(Mode::PrReview, &artifacts).expect("pr review summary");
        assert!(summary.headline.contains("waiting for explicit disposition"));
        assert!(summary.artifact_packet_summary.contains("`must-fix` severity"));
    }

    #[test]
    fn summarize_pr_review_mode_result_surfaces_explicit_approval_branch() {
        let artifacts = vec![
            make_artifact(
                "pr-analysis.md",
                "## Changed Modules\n\n- crates/canon-engine/src/orchestrator/service/summarizers/governance.rs\n\n## Inferred Intent\n\n- preserve governed summary behavior while closing the remaining concern.",
            ),
            make_artifact(
                "review-summary.md",
                "## Final Disposition\n\nStatus: accepted-with-approval\n\nRationale: the remaining concern was explicitly accepted.\n\n## Severity\n\nOverall severity: must-fix\n\nMust-fix findings: 1\n\nReview notes: 0\n\n## Must-Fix Findings\n\n- release owner accepted the single remaining concern.\n\n## Accepted Risks\n\n- No accepted risks recorded.",
            ),
        ];

        let summary = summarize_mode_result(Mode::PrReview, &artifacts).expect("pr review summary");
        assert!(summary.headline.contains("explicit approval"));
        assert!(summary.artifact_packet_summary.contains("`accepted-with-approval`"));
    }

    #[test]
    fn summarize_pr_review_mode_result_reports_missing_context_and_uses_inferred_intent_excerpt() {
        let artifacts = vec![
            make_artifact(
                "pr-analysis.md",
                "## Changed Modules\n\n- crates/canon-engine/src/orchestrator/service/summarizers/governance.rs\n\n## Inferred Intent\n\n- preserve bounded rollout guidance for the governed PR summary.",
            ),
            make_artifact(
                "review-summary.md",
                "## Notes\n\n- partial export still needs structure.",
            ),
        ];

        let summary = summarize_mode_result(Mode::PrReview, &artifacts).expect("pr review summary");
        assert!(summary.headline.contains("missing-context marker"));
        assert!(summary.artifact_packet_summary.contains("still carries"));
        assert!(summary.result_excerpt.contains("preserve bounded rollout guidance"));
    }

    #[test]
    fn summarize_architecture_mode_result_uses_overview_when_present() {
        let artifacts = vec![make_artifact(
            "architecture-overview.md",
            "## Primary Decision\n- Use C4 views.\n",
        )];
        let summary = summarize_mode_result(Mode::Architecture, &artifacts).unwrap();
        assert_eq!(summary.primary_artifact_title, "Architecture Overview");
    }

    #[test]
    fn summarize_architecture_mode_result_falls_back_to_decisions_when_overview_absent() {
        let artifacts = vec![make_artifact(
            "architecture-decisions.md",
            "## Decision\n- Use a layered architecture.\n",
        )];
        let summary = summarize_mode_result(Mode::Architecture, &artifacts).unwrap();
        assert_eq!(summary.primary_artifact_title, "Architecture Decisions");
    }

    #[test]
    fn summarize_architecture_mode_result_returns_none_when_both_primary_artifacts_absent() {
        let artifacts = vec![make_artifact("invariants.md", "## Invariants\n- keep stable\n")];
        assert!(summarize_mode_result(Mode::Architecture, &artifacts).is_none());
    }
}
